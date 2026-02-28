use anyhow::{Context, Result};
use clap::ArgMatches;
use horned_owl::io::owx::reader::*;
use horned_owl::model::*;
use horned_owl::ontology::set::SetOntology;
use rayon::prelude::*;
use serde_json::{json, Map, Value};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

use crate::owl_2_ofn;

use super::curify::{curify_triples, generate_blank_node_id, invert_prefix_map};
use super::db::{insert_triples_to_db, load_prefix_map};
use super::triple::{
    extract_value_and_datatype, is_ldtab_blanknode, ldtab_2_triple,
    LdTabTriple, DATATYPE_IRI, UNKNOWN_VALUE,
};

const RDF_TYPE: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>";
const OWL_VERSION_IRI: &str = "<http://www.w3.org/2002/07/owl#versionIRI>";
const OWL_ONTOLOGY: &str = "<http://www.w3.org/2002/07/owl#Ontology>";
const OWL_DISJOINT_WITH: &str = "<http://www.w3.org/2002/07/owl#disjointWith>";
const OWL_EQUIVALENT_CLASS: &str = "<http://www.w3.org/2002/07/owl#equivalentClass>";
const OWL_UNION_OF: &str = "<http://www.w3.org/2002/07/owl#unionOf>";
const RDFS_SUBCLASS_OF: &str = "<http://www.w3.org/2000/01/rdf-schema#subClassOf>";
const SWRL_VARIABLE: &str = "<http://www.w3.org/2003/11/swrl#Variable>";

pub async fn import(sub_matches: &ArgMatches) -> Result<()> {
    let database = sub_matches.get_one::<String>("database");
    let ontology = sub_matches.get_one::<String>("ontology");

    // This check should be redundant, as clap requires both arguments
    if database.is_none() || ontology.is_none() {
        anyhow::bail!("You must provide both a database and an ontology.");
    }

    //these unwraps are safe
    let database_path = database.unwrap();
    let ontology_path = ontology.unwrap();

    let file = File::open(ontology_path)?;
    let reader = BufReader::new(file);
    let build = Build::<ArcStr>::new();

    match read_with_build(reader, &build) {
        Ok((ontology, _prefix_mapping)) => {
            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(database_path)
                .await
                .context("Failed to connect to the database")?;

            import_ontology(&ontology, &pool).await?;
        }
        Err(e) => {
            eprintln!("Failed to read ontology: {:?}", e);
        }
    }
    Ok(())
}

async fn import_ontology(ontology: &SetOntology<ArcStr>, pool: &SqlitePool) -> Result<()> {
    let iri_value = extract_ontology_iri(ontology);
    let prefix_map = load_prefix_map(pool).await?;
    let prefix2iri = invert_prefix_map(&prefix_map);

    let count = ontology.iter().count();
    println!("Number of axioms: {}", count);

    let (imports, ontology_annotations, dl_safe_rules, ontology_id, normal) =
        split_ontology_components(ontology);

    let start = Instant::now();

    // Process axiom types into triples
    let mut ldtab_triples = process_normal_axioms(&normal);
    ldtab_triples.extend(process_ontology_id(&ontology_id));
    ldtab_triples.extend(process_imports(&imports, &iri_value));
    ldtab_triples.extend(process_ontology_annotations(&ontology_annotations, &iri_value));
    ldtab_triples.extend(process_swrl_rules(&dl_safe_rules, &prefix2iri));

    // Handle blank nodes
    let ldtab_triples = process_blank_nodes(&ldtab_triples, &prefix2iri);

    // curify all triples
    let ldtab_triples = curify_triples(&ldtab_triples, &prefix_map);

    let duration = start.elapsed();
    println!("OWL2LDTab took: {:?}", duration);

    // Insert into database
    let time = Instant::now();
    insert_triples_to_db(&ldtab_triples, pool).await?;
    let duration = time.elapsed();
    println!("Sqlite took: {:?}", duration);

    Ok(())
}

fn extract_ontology_iri(ontology: &SetOntology<ArcStr>) -> Value {
    let id = ontology.i();
    let i = id.clone().the_ontology_id().unwrap().iri.unwrap();
    let ii = i.get(0..);
    let iri = "<".to_string() + ii.unwrap() + ">";
    Value::String(String::from(iri))
}

fn split_ontology_components<'a>(
    ontology: &'a SetOntology<ArcStr>,
) -> (
    Vec<&'a AnnotatedComponent<ArcStr>>,
    Vec<&'a AnnotatedComponent<ArcStr>>,
    Vec<&'a AnnotatedComponent<ArcStr>>,
    Vec<&'a AnnotatedComponent<ArcStr>>,
    Vec<&'a AnnotatedComponent<ArcStr>>,
) {
    let mut imports = Vec::new();
    let mut ontology_annotations = Vec::new();
    let mut dl_safe_rules = Vec::new();
    let mut ontology_id = Vec::new();
    let mut normal = Vec::new();

    ontology.iter().for_each(|ann_axiom| {
        let component = &ann_axiom.component;
        match component {
            Component::Import(_) => imports.push(ann_axiom),
            Component::OntologyAnnotation(_) => ontology_annotations.push(ann_axiom),
            Component::Rule(_) => dl_safe_rules.push(ann_axiom),
            Component::OntologyID(_) => ontology_id.push(ann_axiom),
            _ => normal.push(ann_axiom),
        }
    });

    (imports, ontology_annotations, dl_safe_rules, ontology_id, normal)
}

fn process_normal_axioms(
    normal: &[&AnnotatedComponent<ArcStr>],
) -> Vec<LdTabTriple> {
    normal
        .par_iter()
        .map(|ann_axiom| owl_2_ldtab(ann_axiom))
        .collect()
}

fn process_ontology_id(ontology_id: &[&AnnotatedComponent<ArcStr>]) -> Vec<LdTabTriple> {
    let mut triples = Vec::new();

    for ann_axiom in ontology_id {
        let ofn = owl_2_ofn::transducer::translate(ann_axiom);
        let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);

        if ldtab["predicate"] == OWL_VERSION_IRI {
            let mut t = ldtab_2_triple(&ldtab);
            t.predicate = json!(RDF_TYPE);
            t.object = json!(OWL_ONTOLOGY);
            triples.push(t);
        }

        if ldtab["object"] != UNKNOWN_VALUE {
            triples.push(ldtab_2_triple(&ldtab));
        }
    }

    triples
}

fn process_imports(
    imports: &[&AnnotatedComponent<ArcStr>],
    iri_value: &Value,
) -> Vec<LdTabTriple> {
    imports
        .iter()
        .map(|ann_axiom| {
            let ofn = owl_2_ofn::transducer::translate(ann_axiom);
            let ofn = Value::Array(vec![ofn[0].clone(), iri_value.clone(), ofn[1].clone()]);
            let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);
            ldtab_2_triple(&ldtab)
        })
        .collect()
}

fn process_ontology_annotations(
    ontology_annotations: &[&AnnotatedComponent<ArcStr>],
    iri_value: &Value,
) -> Vec<LdTabTriple> {
    ontology_annotations
        .iter()
        .map(|ann_axiom| {
            let ofn = owl_2_ofn::transducer::translate(ann_axiom);
            let ofn = Value::Array(vec![ofn[0].clone(), iri_value.clone(), ofn[1].clone()]);
            let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);
            ldtab_2_triple(&ldtab)
        })
        .collect()
}

fn process_swrl_rules(
    dl_safe_rules: &[&AnnotatedComponent<ArcStr>],
    prefix2iri: &HashMap<String, String>,
) -> Vec<LdTabTriple> {
    let mut triples = Vec::new();

    // Collect all variables from rules
    let variables: HashSet<Variable<ArcStr>> = dl_safe_rules
        .iter()
        .flat_map(|ann_axiom| {
            let rule = match &ann_axiom.component {
                Component::Rule(r) => r,
                other => panic!("Expected a Rule-component, but found: {:?}", other),
            };
            get_rule_variables(rule)
        })
        .collect();

    // Create type declarations for variables
    for var in &variables {
        let var_iri = format!("<{}>", var.0);
        triples.push(LdTabTriple::new(var_iri, RDF_TYPE, SWRL_VARIABLE, DATATYPE_IRI));
    }

    // Process each rule
    for ann_axiom in dl_safe_rules {
        let ofn = owl_2_ofn::transducer::translate(ann_axiom);
        let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);

        // Build blank node content
        let mut m = Map::new();
        for triple in ldtab.as_array().unwrap() {
            let predicate = triple.get("predicate").unwrap().as_str().unwrap();
            let object = triple.get("object").unwrap();
            let datatype = triple.get("datatype").unwrap().as_str().unwrap();
            let ooo = json!([{"datatype": datatype, "object": object}]);
            m.insert(predicate.to_string(), ooo);
        }

        let blank = Value::Object(m);
        let blank_node = generate_blank_node_id(&blank, prefix2iri);

        for triple in ldtab.as_array().unwrap() {
            let t = LdTabTriple::new(
                blank_node.clone(),
                triple.get("predicate").unwrap().clone(),
                triple.get("object").unwrap().clone(),
                triple.get("datatype").unwrap().clone(),
            )
            .annotation(triple.get("annotation").unwrap().clone());
            triples.push(t);
        }
    }

    triples
}

/// Processes blank nodes in triples, expanding JSON objects as subjects.
fn process_blank_nodes(
    ldtab_triples: &[LdTabTriple],
    prefix2iri: &HashMap<String, String>,
) -> Vec<LdTabTriple> {
    let mut new_ldtab_triples: Vec<LdTabTriple> = Vec::new();

    for t in ldtab_triples {
        let s = &t.subject;
        let o = &t.object;

        if is_ldtab_blanknode(s) {
            if let Value::Object(map) = o {
                for (key, value) in map.iter() {
                    let (value_v, datatype) = extract_value_and_datatype(value, &t.datatype);
                    new_ldtab_triples.push(
                        LdTabTriple::new(t.subject.clone(), key.clone(), value_v, datatype)
                    );
                }
            } else {
                new_ldtab_triples.push(t.clone());
            }
        } else if s.is_object() {
            if let Value::Object(map) = s {
                let mut m = map.clone();
                let ooo = json!([{"datatype": t.datatype, "object": o.clone()}]);

                if let Some(p_str) = t.predicate.as_str() {
                    match p_str {
                        OWL_DISJOINT_WITH => { m.insert(OWL_DISJOINT_WITH.to_string(), ooo); }
                        RDFS_SUBCLASS_OF => { m.insert(RDFS_SUBCLASS_OF.to_string(), ooo); }
                        OWL_EQUIVALENT_CLASS => { m.insert(OWL_EQUIVALENT_CLASS.to_string(), ooo); }
                        OWL_UNION_OF => { m.insert(OWL_UNION_OF.to_string(), ooo); }
                        _ => {}
                    }
                }

                let blank = Value::Object(m);
                let blank_node = generate_blank_node_id(&blank, prefix2iri);

                for (key, value) in map.iter() {
                    let (value_v, datatype) = extract_value_and_datatype(value, &t.datatype);
                    new_ldtab_triples.push(
                        LdTabTriple::new(blank_node.clone(), key.clone(), value_v, datatype)
                    );
                }

                new_ldtab_triples.push(
                    LdTabTriple::new(
                        blank_node,
                        t.predicate.clone(),
                        t.object.clone(),
                        t.datatype.clone(),
                    )
                    .annotation(t.annotation.clone())
                );
            }
        } else {
            new_ldtab_triples.push(t.clone());
        }
    }

    new_ldtab_triples
}

fn owl_2_ldtab(
    ann_axiom: &AnnotatedComponent<ArcStr>,
) -> LdTabTriple {
    let ofn = owl_2_ofn::transducer::translate(ann_axiom);
    let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);
    ldtab_2_triple(&ldtab)
}

pub fn get_rule_variables<A>(rule: &Rule<A>) -> HashSet<Variable<A>>
where
    A: ForIRI,
{
    let mut vars = HashSet::new();

    for atom in rule.head.iter().chain(rule.body.iter()) {
        match *atom {
            Atom::BuiltInAtom { ref args, .. } => {
                for arg in args {
                    if let DArgument::Variable(ref var) = arg {
                        vars.insert(var.clone());
                    }
                }
            }
            Atom::ClassAtom { ref arg, .. } => {
                if let IArgument::Variable(ref var) = arg {
                    vars.insert(var.clone());
                }
            }
            Atom::DataPropertyAtom { ref args, .. } => {
                let (d1, d2) = args;
                if let DArgument::Variable(ref var) = d1 {
                    vars.insert(var.clone());
                }
                if let DArgument::Variable(ref var) = d2 {
                    vars.insert(var.clone());
                }
            }
            Atom::DataRangeAtom { ref arg, .. } => {
                if let DArgument::Variable(ref var) = arg {
                    vars.insert(var.clone());
                }
            }
            Atom::DifferentIndividualsAtom(ref i1, ref i2)
            | Atom::SameIndividualAtom(ref i1, ref i2) => {
                if let IArgument::Variable(ref var) = i1 {
                    vars.insert(var.clone());
                }
                if let IArgument::Variable(ref var) = i2 {
                    vars.insert(var.clone());
                }
            }
            Atom::ObjectPropertyAtom { ref args, .. } => {
                let (i1, i2) = args;
                if let IArgument::Variable(ref var) = i1 {
                    vars.insert(var.clone());
                }
                if let IArgument::Variable(ref var) = i2 {
                    vars.insert(var.clone());
                }
            }
        }
    }

    vars
}
