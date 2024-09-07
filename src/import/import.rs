use anyhow::{Context, Result};
use clap::ArgMatches;
use horned_bin::parse_path;
use horned_owl::io::ParserConfiguration;
use horned_owl::model::*;
use horned_owl::ontology::set::SetOntology;
use regex::Regex;
use serde_json::Value;
use serde_json::json;
use sqlx::{sqlite::SqlitePoolOptions, QueryBuilder, Row, SqlitePool};
use std::collections::HashMap;
use std::path::Path;

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

    //parse ontology using horned_owl
    match parse_path(Path::new(&ontology_path), ParserConfiguration::default()) {
        Ok(parsed) => {
            let ont = &parsed.decompose().0;

            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(database_path)
                .await
                .context("Failed to connect to the database")?;

            import_ontology(ont, &pool).await?;
        }
        Err(error) => anyhow::bail!("Failed to parse ontology: {}", error),
    }

    Ok(())
}

async fn import_ontology(ontology: &SetOntology<RcStr>, pool: &SqlitePool) -> Result<()> {
    let id = ontology.i();
    //let iri = id.clone().the_ontology_id().unwrap().iri.unwrap();
    //let iri_value = Value::String(String::from(iri.get(0..).unwrap()));

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

    // Initialize an empty HashMap
    let mut map = HashMap::new();

    // Iterate over the rows and insert them into the HashMap
    for row in rows {
        let base: String = row.get("base");
        let prefix: String = row.get("prefix");
        map.insert(base, prefix);
    }

    //convert OWL to LDTab
    let mut ldtab_triples = Vec::new();
    let count = ontology.iter().count();
    println!("Number of axioms: {}", count);
    for ann_axiom in ontology.iter() {

        //1. translate Horned OWL to OFN S-expression
        let ofn = owl_2_ofn::transducer::translate(ann_axiom);
        let ofn = match ofn[0].as_str() {

            Some("Import") => Value::Array(vec![ofn[0].clone(), iri_value.clone(), ofn[1].clone()]),
            Some("OntologyAnnotation") => {
                Value::Array(vec![ofn[0].clone(), iri_value.clone(), ofn[1].clone()])
            }
            _ => ofn.clone(),
        };

        let ofn_curified = curify_with(&ofn, &map);

        //2. translate OFN S-Expression to LDTab ThickTriple
        let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn_curified);

        //An "Ontology" object in Horned-OWL gets translated into two LDTab triples
        if ldtab["predicate"] == "owl:versionIRI" {
            let mut t = ldtab.clone();
            t["predicate"] = json!("rdf:type");
            t["object"] = json!("owl:Ontology");

            ldtab_triples.push(ldtab_2_tuple(&t).unwrap());
        }

        let ldtab_tuple = ldtab_2_tuple(&ldtab).unwrap();

        ldtab_triples.push(ldtab_tuple);
    }

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
    Ok(())
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
    let subject = value.get("subject").unwrap().as_str().unwrap();
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

fn curify_with(ofn: &Value, iri2prefix: &HashMap<String, String>) -> Value {
    match ofn {
        Value::Array(vec) => {
            // Create a new array with the replaced values
            let new_vec: Vec<Value> = vec
                .iter()
                .map(|item| curify_with(item, iri2prefix))
                .collect();
            Value::Array(new_vec)
        }
        Value::String(s) => {
            // Replace all matching substrings in the string
            Value::String(replace_substrings(s, iri2prefix))
        }
        _ => ofn.clone(), // This case shouldn't occur, but we clone in case
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
    let annotation = value.get("annotation").unwrap();
    match annotation {
        Value::Object(map) => {
            if map.is_empty() {
                String::new()
            } else {
                annotation.to_string()
            }
        }
        _ => String::new(),
    }
}
