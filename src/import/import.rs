use anyhow::{Context, Result};
use clap::ArgMatches;
use std::collections::HashSet;
use horned_bin::parse_path;
use horned_owl::io::ParserConfiguration;
use horned_owl::io::owx::reader::*;
use horned_owl::model::*;
use horned_owl::ontology::set::SetOntology;
use rayon::prelude::*;
use regex::Regex;
use serde_json::json;
use serde_json::{Value, Map, from_str};
use sqlx::{sqlite::SqlitePoolOptions, QueryBuilder, Row, SqlitePool};
use std::collections::HashMap;
use std::path::Path;
use std::io::{BufReader, Result as IoResult};
use std::fs::File;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use sha2::{Sha256, Digest};

use std::time::Instant;

use crate::owl_2_ofn;

const SQLITE_MAX_VARIABLE_NUMBER: usize = 999;
const NUM_COLUMNS: usize = 8;


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
        Ok((ontology, prefix_mapping)) => {
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
    let id = ontology.i();

    let i = id.clone().the_ontology_id().unwrap().iri.unwrap();
    let ii = i.get(0..);
    let iri = "<".to_string() + ii.unwrap() + ">";
    let iri_value = Value::String(String::from(iri));

    //load prefixes
    let rows = sqlx::query(
        r#"
        SELECT * FROM prefix
        "#,
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch rows from the table")?;

    //initialize map for prefixes
    let mut map = HashMap::new();
    for row in rows {
        let base: String = row.get("base");
        let prefix: String = row.get("prefix");
        map.insert(base, prefix);
    }

    let prefix_map = map.clone();

    //convert OWL to LDTab
    let mut ldtab_triples = Vec::new();
    let count = ontology.iter().count();
    println!("Number of axioms: {}", count);


    //split ontology
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
        }});

    let start = Instant::now();
    ldtab_triples.par_extend(
        normal.par_iter()
        .map(|ann_axiom| owl_2_ldtab(ann_axiom, &map))
        .filter_map(|res| match res {
            Ok(t) => Some(t),
            Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        }));

    //translate ontology ID
    ontology_id.iter().for_each(|ann_axiom| {
        let ofn = owl_2_ofn::transducer::translate(ann_axiom);
        let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);

        //An "Ontology" object in Horned-OWL gets translated into two LDTab triples
        if ldtab["predicate"] == "<http://www.w3.org/2002/07/owl#versionIRI>" {
            let mut t = ldtab.clone();
            t["predicate"] = json!("<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>");
            t["object"] = json!("<http://www.w3.org/2002/07/owl#Ontology>");

            ldtab_triples.push(ldtab_2_tuple(&t).unwrap())
        }

        if !(ldtab["object"] == "<unknown>"){
            ldtab_triples.push(ldtab_2_tuple(&ldtab).unwrap());
        }
    });

    //handle imports (the ontology's iri is not available in Horned-OWL's construct, so we add it here)
    imports.iter().for_each(|ann_axiom| {
        let ofn = owl_2_ofn::transducer::translate(ann_axiom);
        let ofn = Value::Array(vec![ofn[0].clone(), iri_value.clone(), ofn[1].clone()]);

        let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);

        ldtab_triples.push(ldtab_2_tuple(&ldtab).unwrap());
    });

    //handle ontology annotations (the ontology's iri is not available in Horned-OWL's construct, so we add it here)
    ontology_annotations.iter().for_each(|ann_axiom| {
        let ofn = owl_2_ofn::transducer::translate(ann_axiom);
        let ofn = Value::Array(vec![ofn[0].clone(), iri_value.clone(), ofn[1].clone()]);

        let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);

        ldtab_triples.push(ldtab_2_tuple(&ldtab).unwrap());
    });


    //handle SWRL rules (a single rule is split into multiple LDTab triples)
    let variables: HashSet<Variable<ArcStr>> = dl_safe_rules
        .iter()
        .flat_map(|ann_axiom| {
            let rule = match &ann_axiom.component {
                Component::Rule(r) => r,
                other => panic!(
                    "Expected a Rule‐component, but found: {:?}", 
                    other
                ),
            };
            get_rule_variables(rule)
        })
        .collect();
    for var in &variables {
        let var_iri = format!("<{}>", var.0);
        let ldtab = json!({
            "assertion":"1",
            "retraction": "0",
            "graph": "graph",
            "subject": var_iri,
            "predicate": "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>",
            "object": "<http://www.w3.org/2003/11/swrl#Variable>",
            "datatype": "_IRI",
            "annotation": Value::Null
        });

        ldtab_triples.push(ldtab_2_tuple(&ldtab).unwrap());

    }

    dl_safe_rules.iter().for_each(|ann_axiom| {

        let ofn = owl_2_ofn::transducer::translate(ann_axiom);

        let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);

        //handle blank nodes in rules
        let mut m = Map::new();
        for triple in ldtab.as_array().unwrap() {
            let predicate = triple.get("predicate").unwrap().as_str().unwrap();
            let object = triple.get("object").unwrap();
            let datatype = triple.get("datatype").unwrap().as_str().unwrap();
            let ooo = json!([{"datatype":datatype,"object":object}]);

            m.insert(predicate.to_string(), ooo);
        }

        let blank = Value::Object(m);
        let blank_node = generate_blank_node_id(&blank, &prefix_map);

        for triple in ldtab.as_array().unwrap() {
            let ldtab = json!({
                "assertion":"1",
                "retraction": "0",
                "graph": "graph",
                "subject": blank_node,
                "predicate": triple.get("predicate").unwrap(),
                "object": triple.get("object").unwrap(),
                "datatype": triple.get("datatype").unwrap(),
                "annotation": triple.get("annotation").unwrap()
            });

            ldtab_triples.push(ldtab_2_tuple(&ldtab).unwrap());
        }
    });



    let mut new_ldtab_triples = Vec::new();

    ldtab_triples.iter().for_each(|t| {

        //handle ldtab blank nodes 
        let s = parse_json_from_string(&t.3);
        let p = parse_json_from_string(&t.4);
        let o =
            if t.6 == "_JSONMAP" || t.6 == "_JSONLIST" {
                parse_json_from_string(&t.5)
            } else {
                Value::String(t.5.clone())
            };

        if is_ldtab_blanknode(&s) {
            //&& datatype_obj.unwrap().as_str().unwrap() == "_JSONMAP" { //subject is already a blank node

            if let Value::Object(map) = o.clone() {

                for (key, value) in map.iter() {
                    let (value_v, datatype) = extract_value_and_datatype(value, &t.6);

                    let ldtab = json!({
                        "assertion":"1",
                        "retraction": "0",
                        "graph": "graph",
                        "subject": t.3,
                        "predicate": key,
                        "object": value_v,
                        "datatype": datatype,
                        "annotation": Value::Null
                    });
                    new_ldtab_triples.push(ldtab_2_tuple(&ldtab).unwrap());
                }
            } else {
                //normal triple with blank node subject
                new_ldtab_triples.push(t.clone());
            }

        }

        if s.is_object() {
            //blank node as subject
            //&& p == "<http://www.w3.org/2002/07/owl#disjointWith>"
            if let Value::Object(map) = s.clone()  {

                let mut m = map.clone();

                let ooo = json!([{"datatype":t.6,"object":o.clone()}]);


                if p == "owl:disjointWith" {
                    m.insert("<http://www.w3.org/2002/07/owl#disjointWith>".to_string(), ooo.clone());
                }

                if p == "rdfs:subClassOf" {
                    m.insert("rdfs:subClassOf".to_string(), ooo.clone());
                }

                if p == "owl:equivalentClass" {
                    m.insert("owl:equivalentClass".to_string(), ooo.clone());
                }

                if p == "owl:unionOf" {
                    m.insert("owl:unionOf".to_string(), ooo.clone());
                }

                let blank = Value::Object(m);
                let blank_node = generate_blank_node_id(&blank, &prefix_map);

                for (key, value) in map.iter() {
                    let (value_v, datatype) = extract_value_and_datatype(value, &t.6);

                    let ldtab = json!({
                        "assertion":"1",
                        "retraction": "0",
                        "graph": "graph",
                        "subject": blank_node,
                        "predicate": key,
                        "object": value_v,
                        "datatype": datatype,
                        "annotation": Value::Null
                    });
                    new_ldtab_triples.push(ldtab_2_tuple(&ldtab).unwrap());
                }

                let ldtab = json!({
                    "assertion": "1",
                    "retraction": "0",
                    "graph": "graph",
                    "subject": blank_node,
                    "predicate": t.4,
                    "object": parse_json_from_string(&t.5),
                    "datatype": t.6,
                    "annotation": parse_json_from_string(&t.7)
                });
                new_ldtab_triples.push(ldtab_2_tuple(&ldtab).unwrap());
            }
        }

        //normal triple
        if !is_ldtab_blanknode(&s) && !s.is_object() {
            new_ldtab_triples.push(t.clone());
        }

    });

    //run curify_ldtab_with on all ldtab_triples
    let mut ldtab_triples = Vec::new();

    new_ldtab_triples.iter().for_each(|t| {

        let o =
            if t.6 == "_JSONMAP" || t.6 == "_JSONLIST" {
                parse_json_from_string(&t.5)
            } else {
                Value::String(t.5.clone())
            };

        let ldtab = json!({
            "assertion": "1",
            "retraction": "0",
            "graph": t.2,
            "subject": t.3,
            "predicate": t.4,
            "object": o,
            "datatype": t.6,
            "annotation": t.7
        });


        let ldtab_curified = curify_ldtab_with(&ldtab, &map);

        let tuple = ldtab_2_tuple(&ldtab_curified).unwrap();
        ldtab_triples.push(tuple);

    });

    let duration = start.elapsed();
    println!("OWL2LDTab took: {:?}", duration);

    let time = Instant::now();

    // Insert LDTab triples into the database
    // (in chunks - the QueryBuilder can only handle a limited number of parameters at once)
    let mut start = 0;
    while start < ldtab_triples.len() {
        let end = std::cmp::min(
            start + SQLITE_MAX_VARIABLE_NUMBER / NUM_COLUMNS,
            ldtab_triples.len(),
        );
        let chunk = &ldtab_triples[start..end];

        //TODO: parameterize table
        let mut query_builder = QueryBuilder::new(
            "INSERT INTO statement (assertion, retraction, graph, subject, predicate, object, datatype, annotation) ",
        );

        query_builder.push_values(chunk, |mut b, tuple| {
            b.push_bind(tuple.0)
                .push_bind(tuple.1)
                .push_bind(&tuple.2)
                .push_bind(&tuple.3)
                .push_bind(&tuple.4)
                .push_bind(&tuple.5)
                .push_bind(&tuple.6)
                .push_bind(&tuple.7);
        });

        query_builder
            .build()
            .execute(pool)
            .await
            .context("Failed to insert ldtab_triples into the database")?;

        start = end;
    }

    let duration = time.elapsed();
    println!("Sqlite took: {:?}", duration);

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


fn owl_2_ldtab(ann_axiom: &AnnotatedComponent<ArcStr>, map : &HashMap<String, String>) -> std::io::Result<(i32, i32, String, String, String, String, String, String)> {

        let ofn = owl_2_ofn::transducer::translate(ann_axiom);

        let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);

        let ldtab_curified = curify_ldtab_with(&ldtab, map);

        ldtab_2_tuple(&ldtab_curified)
}

//TODO: wiring doesn't pull apart literals and language tags/datatypes
fn ldtab_2_tuple(
    value: &Value,
) -> std::io::Result<(i32, i32, String, String, String, String, String, String)> {
    //println!("value: {:?}", value);

    // Extract "subject", "predicate", and "object" from the JSON object
    //let assertion = value.get("assertion").unwrap().as_i64().unwrap();
    //let retraction = value.get("retraction").unwrap().as_i64().unwrap();
    let assertion = string_to_i32(value.get("assertion").unwrap()).unwrap();
    let retraction = string_to_i32(value.get("retraction").unwrap()).unwrap();

    let graph = value.get("graph").unwrap().as_str().unwrap();

    //let subject = value.get("subject").unwrap().as_str().unwrap();
    let subject = json_value_to_string(value.get("subject").unwrap());

    let predicate = value.get("predicate").unwrap().as_str().unwrap();
    let object = json_value_to_string(value.get("object").unwrap()); //Object is already a String
    let datatype = value.get("datatype").unwrap().as_str().unwrap();
    let annotation = get_annotation(value);

    Ok((
        assertion as i32,
        retraction as i32,
        graph.to_string(),
        subject.to_string(),
        predicate.to_string(),
        object,
        datatype.to_string(),
        annotation,
    ))
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
    let datatype = Regex::new("^\"(?s)(.*)\"\\^\\^(.*)$").unwrap();

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
    } else if datatype.is_match(input) {
        match datatype.captures(input) {
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
            if x.contains_key("datatype") && x.get("datatype").unwrap().as_str() == Some("_IRI") {
                let obj = x.get("object").unwrap().clone();
                return (obj, "_IRI".to_string());
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
    let datatype = Regex::new("^\"(?s)(.*)\"\\^\\^(.*)$").unwrap();

    // Expand whole-string CURIEs like "ex:Foo" -> "<http://...Foo>"
    if let Some(expanded) = expand_curie_to_iri(input, prefix2iri) {
        return expanded;
    }

    // Expand datatype CURIEs inside typed literals like "\"x\"^^ex:dt"
    if datatype.is_match(input) {
        if let Some(caps) = datatype.captures(input) {
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
