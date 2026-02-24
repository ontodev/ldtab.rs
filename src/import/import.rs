use anyhow::{Context, Result};
use clap::ArgMatches;
use std::collections::HashSet;
use horned_owl::io::owx::reader::*;
use horned_owl::model::*;
use horned_owl::ontology::set::SetOntology;
use rayon::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::json;
use serde_json::{Value, Map};
use sqlx::{sqlite::SqlitePoolOptions, QueryBuilder, Row, SqlitePool};
use std::collections::HashMap;
use std::io::{BufReader};
use std::fs::File;
use sha2::{Sha256, Digest};

use std::time::Instant;

use crate::owl_2_ofn;

const SQLITE_MAX_VARIABLE_NUMBER: usize = 999;
const NUM_COLUMNS: usize = 8;

// RDF/OWL IRIs
const RDF_TYPE: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>";
const OWL_VERSION_IRI: &str = "<http://www.w3.org/2002/07/owl#versionIRI>";
const OWL_ONTOLOGY: &str = "<http://www.w3.org/2002/07/owl#Ontology>";
const OWL_DISJOINT_WITH: &str = "<http://www.w3.org/2002/07/owl#disjointWith>";
const SWRL_VARIABLE: &str = "<http://www.w3.org/2003/11/swrl#Variable>";

// CURIE predicates (used in comparisons after curification)
const CURIE_DISJOINT_WITH: &str = "owl:disjointWith";
const CURIE_SUBCLASS_OF: &str = "rdfs:subClassOf";
const CURIE_EQUIVALENT_CLASS: &str = "owl:equivalentClass";
const CURIE_UNION_OF: &str = "owl:unionOf";

// LDTab datatypes
const DATATYPE_IRI: &str = "_IRI";
const DATATYPE_JSONMAP: &str = "_JSONMAP";
const DATATYPE_JSONLIST: &str = "_JSONLIST";

// Default values
const DEFAULT_GRAPH: &str = "graph";
const UNKNOWN_VALUE: &str = "<unknown>";
const ASSERTION_TRUE: &str = "1";
const _ASSERTION_FALSE: &str = "0";
const _RETRACTION_TRUE: &str = "1";
const RETRACTION_FALSE: &str = "0";

/// Regex to match typed literals
static DATATYPE_LITERAL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^"(?s)(.*)"\^\^(.*)$"#).unwrap());

/// Builder for creating LdTab JSON objects
struct LdTabJsonBuilder {
    graph: String,
    subject: Value,
    predicate: Value,
    object: Value,
    datatype: String,
    annotation: Value,
}

impl LdTabJsonBuilder {

    fn new(
        subject: impl Into<Value>,
        predicate: impl Into<Value>,
        object: impl Into<Value>,
        datatype: &str,
    ) -> Self {
        Self {
            graph: DEFAULT_GRAPH.to_string(),
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            datatype: datatype.to_string(),
            annotation: Value::Null,
        }
    }

    fn graph(mut self, graph: &str) -> Self {
        self.graph = graph.to_string();
        self
    }

    fn annotation(mut self, annotation: impl Into<Value>) -> Self {
        self.annotation = annotation.into();
        self
    }

    fn build(self) -> Value {
        json!({
            "assertion": ASSERTION_TRUE,
            "retraction": RETRACTION_FALSE,
            "graph": self.graph,
            "subject": self.subject,
            "predicate": self.predicate,
            "object": self.object,
            "datatype": self.datatype,
            "annotation": self.annotation
        })
    }
}

#[derive(Debug, Clone)]
struct LdTabTriple {
    assertion: i32,
    retraction: i32,
    graph: String,
    subject: String,
    predicate: String,
    object: String,
    datatype: String,
    annotation: String,
}


pub async fn import(sub_matches: &ArgMatches) -> Result<()> {
    let database = sub_matches.get_one::<String>("database");
    let ontology = sub_matches.get_one::<String>("ontology");

    //This check should be redundant, as clap requires both arguments
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
            // Use `ontology` and `prefix_mapping` as needed
            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(database_path)
                .await
                .context("Failed to connect to the database")?;

            import_ontology(&ontology, &pool).await?;
        },
        Err(e) => {
            eprintln!("Failed to read ontology: {:?}", e);
        }
    }
    Ok(())
}

async fn import_ontology(ontology: &SetOntology<ArcStr>, pool: &SqlitePool) -> Result<()> {
    let iri_value = extract_ontology_iri(ontology);
    let prefix_map = load_prefix_map(pool).await?;

    let count = ontology.iter().count();
    println!("Number of axioms: {}", count);

    // Split ontology into component types
    let (imports, ontology_annotations, dl_safe_rules, ontology_id, normal) = 
        split_ontology_components(ontology);

    let start = Instant::now();

    // Process component types into triples
    let mut ldtab_triples = process_normal_axioms(&normal, &prefix_map);
    ldtab_triples.extend(process_ontology_id(&ontology_id));
    ldtab_triples.extend(process_imports(&imports, &iri_value));
    ldtab_triples.extend(process_ontology_annotations(&ontology_annotations, &iri_value));
    ldtab_triples.extend(process_swrl_rules(&dl_safe_rules, &prefix_map));

    // Handle blank nodes
    let ldtab_triples = process_blank_nodes(&ldtab_triples, &prefix_map);

    // Curify all triples
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

/// Extracts the ontology IRI as a JSON Value.
fn extract_ontology_iri(ontology: &SetOntology<ArcStr>) -> Value {
    let id = ontology.i();
    let i = id.clone().the_ontology_id().unwrap().iri.unwrap();
    let ii = i.get(0..);
    let iri = "<".to_string() + ii.unwrap() + ">";
    Value::String(String::from(iri))
}

/// Loads the prefix map from the database.
async fn load_prefix_map(pool: &SqlitePool) -> Result<HashMap<String, String>> {
    let rows = sqlx::query(
        r#"
        SELECT * FROM prefix
        "#,
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch rows from the table")?;

    let mut map = HashMap::new();
    for row in rows {
        let base: String = row.get("base");
        let prefix: String = row.get("prefix");
        map.insert(base, prefix);
    }
    Ok(map)
}

/// Splits ontology components into separate vectors by type.
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

/// Processes normal axioms in parallel.
fn process_normal_axioms(
    normal: &[&AnnotatedComponent<ArcStr>],
    prefix_map: &HashMap<String, String>,
) -> Vec<LdTabTriple> {
    normal
        .par_iter()
        .map(|ann_axiom| owl_2_ldtab(ann_axiom, prefix_map))
        .filter_map(|res| match res {
            Ok(t) => Some(t),
            Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        })
        .collect()
}

/// Processes ontology ID components.
fn process_ontology_id(ontology_id: &[&AnnotatedComponent<ArcStr>]) -> Vec<LdTabTriple> {
    let mut triples = Vec::new();

    for ann_axiom in ontology_id {
        let ofn = owl_2_ofn::transducer::translate(ann_axiom);
        let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);

        // An "Ontology" object in Horned-OWL gets translated into two LDTab triples
        if ldtab["predicate"] == OWL_VERSION_IRI {
            let mut t = ldtab.clone();
            t["predicate"] = json!(RDF_TYPE);
            t["object"] = json!(OWL_ONTOLOGY);
            triples.push(ldtab_2_triple(&t).unwrap());
        }

        if ldtab["object"] != UNKNOWN_VALUE {
            triples.push(ldtab_2_triple(&ldtab).unwrap());
        }
    }

    triples
}

/// Processes import statements.
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
            ldtab_2_triple(&ldtab).unwrap()
        })
        .collect()
}

/// Processes ontology annotations.
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
            ldtab_2_triple(&ldtab).unwrap()
        })
        .collect()
}

/// Processes SWRL rules including variable declarations.
fn process_swrl_rules(
    dl_safe_rules: &[&AnnotatedComponent<ArcStr>],
    prefix_map: &HashMap<String, String>,
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
        let ldtab = LdTabJsonBuilder::new(var_iri, RDF_TYPE, SWRL_VARIABLE, DATATYPE_IRI)
            .build();
        triples.push(ldtab_2_triple(&ldtab).unwrap());
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
        let blank_node = generate_blank_node_id(&blank, prefix_map);

        for triple in ldtab.as_array().unwrap() {
            let ldtab = LdTabJsonBuilder::new(
                blank_node.clone(),
                triple.get("predicate").unwrap().clone(),
                triple.get("object").unwrap().clone(),
                triple.get("datatype").unwrap().as_str().unwrap(),
            )
            .annotation(triple.get("annotation").unwrap().clone())
            .build();
            triples.push(ldtab_2_triple(&ldtab).unwrap());
        }
    }

    triples
}

/// Processes blank nodes in triples, expanding JSON objects as subjects.
fn process_blank_nodes(
    ldtab_triples: &[LdTabTriple],
    prefix_map: &HashMap<String, String>,
) -> Vec<LdTabTriple> {
    let mut new_ldtab_triples: Vec<LdTabTriple> = Vec::new();

    for t in ldtab_triples {
        let s = parse_json_from_string(&t.subject);
        let p = parse_json_from_string(&t.predicate);
        let o = if t.datatype == DATATYPE_JSONMAP || t.datatype == DATATYPE_JSONLIST {
            parse_json_from_string(&t.object)
        } else {
            Value::String(t.object.clone())
        };

        if is_ldtab_blanknode(&s) {
            if let Value::Object(map) = o.clone() {
                for (key, value) in map.iter() {
                    let (value_v, datatype) = extract_value_and_datatype(value, &t.datatype);

                    let ldtab = LdTabJsonBuilder::new(t.subject.clone(), key.clone(), value_v, &datatype)
                        .build();
                    new_ldtab_triples.push(ldtab_2_triple(&ldtab).unwrap());
                }
            } else {
                new_ldtab_triples.push(t.clone());
            }
        } else if s.is_object() {
            if let Value::Object(map) = s.clone() {
                let mut m = map.clone();
                let ooo = json!([{"datatype": t.datatype, "object": o.clone()}]);

                if p == CURIE_DISJOINT_WITH {
                    m.insert(OWL_DISJOINT_WITH.to_string(), ooo.clone());
                }
                if p == CURIE_SUBCLASS_OF {
                    m.insert(CURIE_SUBCLASS_OF.to_string(), ooo.clone());
                }
                if p == CURIE_EQUIVALENT_CLASS {
                    m.insert(CURIE_EQUIVALENT_CLASS.to_string(), ooo.clone());
                }
                if p == CURIE_UNION_OF {
                    m.insert(CURIE_UNION_OF.to_string(), ooo.clone());
                }

                let blank = Value::Object(m);
                let blank_node = generate_blank_node_id(&blank, prefix_map);

                for (key, value) in map.iter() {
                    let (value_v, datatype) = extract_value_and_datatype(value, &t.datatype);

                    let ldtab = LdTabJsonBuilder::new(blank_node.clone(), key.clone(), value_v, &datatype)
                        .build();
                    new_ldtab_triples.push(ldtab_2_triple(&ldtab).unwrap());
                }

                let ldtab = LdTabJsonBuilder::new(
                    blank_node,
                    t.predicate.clone(),
                    parse_json_from_string(&t.object),
                    &t.datatype,
                )
                .annotation(parse_json_from_string(&t.annotation))
                .build();
                new_ldtab_triples.push(ldtab_2_triple(&ldtab).unwrap());
            }
        } else {
            new_ldtab_triples.push(t.clone());
        }
    }

    new_ldtab_triples
}

/// Applies CURIE compression to all triples.
fn curify_triples(
    triples: &[LdTabTriple],
    prefix_map: &HashMap<String, String>,
) -> Vec<LdTabTriple> {
    triples
        .iter()
        .map(|t| {
            let o = if t.datatype == DATATYPE_JSONMAP || t.datatype == DATATYPE_JSONLIST {
                parse_json_from_string(&t.object)
            } else {
                Value::String(t.object.clone())
            };

            let ldtab = LdTabJsonBuilder::new(
                t.subject.clone(),
                t.predicate.clone(),
                o,
                &t.datatype,
            )
            .graph(&t.graph)
            .annotation(t.annotation.clone())
            .build();

            let ldtab_curified = curify_ldtab_with(&ldtab, prefix_map);
            ldtab_2_triple(&ldtab_curified).unwrap()
        })
        .collect()
}

/// Inserts triples into the database in chunks.
async fn insert_triples_to_db(triples: &[LdTabTriple], pool: &SqlitePool) -> Result<()> {
    let mut start = 0;
    while start < triples.len() {
        let end = std::cmp::min(
            start + SQLITE_MAX_VARIABLE_NUMBER / NUM_COLUMNS,
            triples.len(),
        );
        let chunk = &triples[start..end];

        let mut query_builder = QueryBuilder::new(
            "INSERT INTO statement (assertion, retraction, graph, subject, predicate, object, datatype, annotation) ",
        );

        query_builder.push_values(chunk, |mut b, triple| {
            b.push_bind(triple.assertion)
                .push_bind(triple.retraction)
                .push_bind(&triple.graph)
                .push_bind(&triple.subject)
                .push_bind(&triple.predicate)
                .push_bind(&triple.object)
                .push_bind(&triple.datatype)
                .push_bind(&triple.annotation);
        });

        query_builder
            .build()
            .execute(pool)
            .await
            .context("Failed to insert ldtab_triples into the database")?;

        start = end;
    }
    Ok(())
}

fn parse_json_from_string(s: &str) -> Value {
    match serde_json::from_str(s) {
        Ok(v) => v,
        Err(_) => {
            Value::String(s.to_string())
        }
    }
}

pub fn get_rule_variables<A>(rule: &Rule<A>) -> HashSet<Variable<A>>
where
    A: ForIRI,
{
    let mut vars = HashSet::new();

    // Iterate over every atom in head ∪ body
    for atom in rule.head.iter().chain(rule.body.iter()) {
        match *atom {
            // Built-in atoms have a Vec<DArgument<A>>
            Atom::BuiltInAtom { ref args, .. } => {
                for arg in args {
                    if let DArgument::Variable(ref var) = arg {
                        vars.insert(var.clone());
                    }
                }
            }

            // Class atoms have a single IArgument<A>
            Atom::ClassAtom { ref arg, .. } => {
                if let IArgument::Variable(ref var) = arg {
                    vars.insert(var.clone());
                }
            }

            // DataPropertyAtom has two DArgument<A> fields
            Atom::DataPropertyAtom { ref args, .. } => {
                let (d1, d2) = args;
                if let DArgument::Variable(ref var) = d1 {
                    vars.insert(var.clone());
                }
                if let DArgument::Variable(ref var) = d2 {
                    vars.insert(var.clone());
                }
            }

            // DataRangeAtom has one DArgument<A>
            Atom::DataRangeAtom { ref arg, .. } => {
                if let DArgument::Variable(ref var) = arg {
                    vars.insert(var.clone());
                }
            }

            // DifferentIndividuals and SameIndividual carry two IArgument<A>
            Atom::DifferentIndividualsAtom(ref i1, ref i2)
            | Atom::SameIndividualAtom(ref i1, ref i2) => {
                if let IArgument::Variable(ref var) = i1 {
                    vars.insert(var.clone());
                }
                if let IArgument::Variable(ref var) = i2 {
                    vars.insert(var.clone());
                }
            }

            // ObjectPropertyAtom has two IArgument<A>
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


fn owl_2_ldtab(ann_axiom: &AnnotatedComponent<ArcStr>, map: &HashMap<String, String>) -> std::io::Result<LdTabTriple> {
        let ofn = owl_2_ofn::transducer::translate(ann_axiom);

        let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);

        let ldtab_curified = curify_ldtab_with(&ldtab, map);

        ldtab_2_triple(&ldtab_curified)
}

//TODO: wiring doesn't pull apart literals and language tags/datatypes
fn ldtab_2_triple(value: &Value) -> std::io::Result<LdTabTriple> {
    let assertion = string_to_i32(value.get("assertion").unwrap()).unwrap();
    let retraction = string_to_i32(value.get("retraction").unwrap()).unwrap();

    let graph = value.get("graph").unwrap().as_str().unwrap();
    let subject = json_value_to_string(value.get("subject").unwrap());
    let predicate = value.get("predicate").unwrap().as_str().unwrap();
    let object = json_value_to_string(value.get("object").unwrap());
    let datatype = value.get("datatype").unwrap().as_str().unwrap();
    let annotation = get_annotation(value);

    Ok(LdTabTriple {
        assertion,
        retraction,
        graph: graph.to_string(),
        subject,
        predicate: predicate.to_string(),
        object,
        datatype: datatype.to_string(),
        annotation,
    })
}

fn curify_ldtab_with(ldtab :&Value, iri2prefix: &HashMap<String, String>) -> Value {
    match ldtab {
        Value::Array(vec) => {
            let new_vec: Vec<Value> = vec
                .iter()
                .map(|item| curify_ldtab_with(item, iri2prefix))
                .collect();
            Value::Array(new_vec)
        }
        Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (key, value) in map.iter() {
                let curified_key = replace_substrings(key, iri2prefix);
                new_map.insert(curified_key, curify_ldtab_with(value, iri2prefix));
            }
            Value::Object(new_map)
        }
        Value::String(s) => {

            Value::String(replace_substrings(s, iri2prefix))


        }
        _ => ldtab.clone(), // This case shouldn't occur, but we clone in case
    }
}

fn replace_substrings(input: &str, iri2prefix: &HashMap<String, String>) -> String {
    if is_full_iri(input) {
        let trimmed = &input[1..input.len() - 1]; //remove angle brackets
        let mut result = trimmed.to_string();
        let mut curified = false;
        for (key, value) in iri2prefix {
            if result.starts_with(key) {
                let mut prefix = String::new();
                prefix.push_str(value);
                prefix.push_str(":");
                result = result.replace(key, &prefix);
                curified = true;
            }
        }
        if curified {
            result
        } else {
            input.to_string()
        }
    } else if DATATYPE_LITERAL_REGEX.is_match(input) {
        match DATATYPE_LITERAL_REGEX.captures(input) {
            Some(x) => {
                let literal = format!("{}", &x[1]);
                let datatype_iri = &x[2];

                let trimmed = &datatype_iri[1..datatype_iri.len() - 1]; //remove angle brackets
                let mut result = trimmed.to_string();
                let mut curified = false;

                for (key, value) in iri2prefix {
                    if result.starts_with(key) {
                        let mut prefix = String::new();
                        prefix.push_str(value);
                        prefix.push_str(":");
                        result = result.replace(key, &prefix);
                        curified = true;
                    }
                }

                if curified {
                    format!("\"{}\"^^{}", literal, result)
                } else {
                    input.to_string()
                }
            }

            None => input.to_string(),
        }
    } else {
        input.to_string()
    }
}

fn is_full_iri(s: &str) -> bool {
    s.starts_with('<') && s.ends_with('>')
}

fn string_to_i32(value: &Value) -> Result<i32> {
    // Attempt to extract the value as a string and then parse it as an i32
    match value.as_str() {
        Some(number_str) => match number_str.parse::<i32>() {
            Ok(number) => Ok(number),
            Err(_) => anyhow::bail!("Failed to parse string as i32"),
        },
        None => anyhow::bail!("Failed to extract string value"),
    }
}

//an object can be either a string or another JSON value
fn json_value_to_string(json_value: &Value) -> String {
    match json_value {
        Value::String(s) => format!("{}", s),
        _ => json_value.to_string(),
    }
}

fn get_annotation(value: &Value) -> String {
    match value.get("annotation") {
        None | Some(Value::Null) => String::new(),

        Some(Value::String(s)) => {
            if s.is_empty() {
                String::new()
            } else if let Ok(inner) = serde_json::from_str::<Value>(s) {
                // Parsed successfully: return the actual JSON (no extra escaping)
                inner.to_string()
            } else {
                // Not JSON, just return the raw string
                s.clone()
            }
        }

        Some(Value::Object(map)) if map.is_empty() => String::new(),
        Some(v) => v.to_string(),
    }
}


pub fn is_ldtab_blanknode(input: &Value) -> bool {
    input
        .as_str()
        .map(|s| s.starts_with("<ldtab:blanknode"))
        .unwrap_or(false)
}

fn extract_value_and_datatype(value: &Value, default_datatype: &str) -> (Value, String) {
    match value {
        Value::String(_) => (value.clone(), default_datatype.to_string()),
        Value::Array(a) if !a.is_empty() => {
            if let Some(x) = a[0].as_object() {
                if x.contains_key("datatype") {
                    let datatype = x.get("datatype").unwrap().as_str().unwrap().to_string();
                    let obj = x.get("object").unwrap().clone();
                    return (obj, datatype);
                }
            }
            (value.clone(), default_datatype.to_string())
        }
        Value::Object(x) => {
            if x.contains_key("datatype") && x.get("datatype").unwrap().as_str() == Some(DATATYPE_IRI) {
                let obj = x.get("object").unwrap().clone();
                return (obj, DATATYPE_IRI.to_string());
            }
            (value.clone(), default_datatype.to_string())
        }
        _ => (value.clone(), default_datatype.to_string()),
    }
}

fn generate_blank_node_id(value: &Value, prefix_map: &HashMap<String, String>) -> String {
    let expanded = uncurify_ldtab_with(value, prefix_map);
    let sorted = wiring_rs::ofn_2_ldtab::util::sort_value(&expanded);
    let json_string = sorted.to_string();

    let mut hasher = Sha256::new();
    hasher.update(json_string.as_bytes());
    let hash = hasher.finalize();

    format!("<ldtab:blanknode:{:x}>", hash)
}



fn uncurify_ldtab_with(ldtab: &Value, iri2prefix: &HashMap<String, String>) -> Value {
    // Invert iri2prefix (IRI base -> prefix) into prefix2iri (prefix -> IRI base)
    let prefix2iri: HashMap<String, String> = iri2prefix
        .iter()
        .map(|(iri_base, prefix)| (prefix.clone(), iri_base.clone()))
        .collect();

    match ldtab {
        Value::Array(vec) => {
            let new_vec: Vec<Value> = vec
                .iter()
                .map(|item| uncurify_ldtab_with(item, iri2prefix))
                .collect();
            Value::Array(new_vec)
        }
        Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (key, value) in map.iter() {
                let expanded_key = expand_curies(key, &prefix2iri);
                new_map.insert(expanded_key, uncurify_ldtab_with(value, iri2prefix));
            }
            Value::Object(new_map)
        }
        Value::String(s) => Value::String(expand_curies(s, &prefix2iri)),
        _ => ldtab.clone(),
    }
}

fn expand_curies(input: &str, prefix2iri: &HashMap<String, String>) -> String {
    // Expand whole-string CURIEs like "ex:Foo" -> "<http://...Foo>"
    if let Some(expanded) = expand_curie_to_iri(input, prefix2iri) {
        return expanded;
    }

    // Expand datatype CURIEs inside typed literals like "\"x\"^^ex:dt"
    if DATATYPE_LITERAL_REGEX.is_match(input) {
        if let Some(caps) = DATATYPE_LITERAL_REGEX.captures(input) {
            let literal = &caps[1];
            let dtype = &caps[2];

            if let Some(expanded_dtype) = expand_curie_to_iri(dtype, prefix2iri) {
                return format!("\"{}\"^^{}", literal, expanded_dtype);
            }
        }
    }

    input.to_string()
}

fn expand_curie_to_iri(curie: &str, prefix2iri: &HashMap<String, String>) -> Option<String> {
    // Don't touch full IRIs already written as <...>
    if is_full_iri(curie) {
        return None;
    }

    let mut parts = curie.splitn(2, ':');
    let prefix = parts.next()?;
    let local = parts.next()?; // requires a colon to exist

    let iri_base = prefix2iri.get(prefix)?;
    Some(format!("<{}{}>", iri_base, local))
}
