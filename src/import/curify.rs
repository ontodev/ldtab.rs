use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use super::triple::{
    parse_json_from_string, LdTabJsonBuilder, LdTabTriple, DATATYPE_JSONLIST, DATATYPE_JSONMAP,
};

pub(crate) static DATATYPE_LITERAL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^"(?s)(.*)"\^\^(.*)$"#).unwrap());

pub(crate) fn curify_triples(
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

            let annotation = parse_json_from_string(&t.annotation);

            let ldtab = LdTabJsonBuilder::new(
                t.subject.clone(),
                t.predicate.clone(),
                o,
                &t.datatype,
            )
            .graph(&t.graph)
            .annotation(annotation)
            .build();

            let ldtab_curified = curify_ldtab_with(&ldtab, prefix_map);
            super::triple::ldtab_2_triple(&ldtab_curified).unwrap()
        })
        .collect()
}

pub(crate) fn curify_ldtab_with(ldtab: &Value, iri2prefix: &HashMap<String, String>) -> Value {
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
        Value::String(s) => Value::String(replace_substrings(s, iri2prefix)),
        _ => ldtab.clone(),
    }
}

pub(crate) fn generate_blank_node_id(value: &Value, prefix2iri: &HashMap<String, String>) -> String {
    let expanded = uncurify_value(value, prefix2iri);
    let sorted = wiring_rs::ofn_2_ldtab::util::sort_value(&expanded);
    let json_string = sorted.to_string();

    let mut hasher = Sha256::new();
    hasher.update(json_string.as_bytes());
    let hash = hasher.finalize();

    format!("<ldtab:blanknode:{:x}>", hash)
}

pub(crate) fn is_full_iri(s: &str) -> bool {
    s.starts_with('<') && s.ends_with('>')
}

pub(crate) fn invert_prefix_map(iri2prefix: &HashMap<String, String>) -> HashMap<String, String> {
    iri2prefix
        .iter()
        .map(|(iri_base, prefix)| (prefix.clone(), iri_base.clone()))
        .collect()
}

fn try_curify_iri(iri: &str, iri2prefix: &HashMap<String, String>) -> Option<String> {
    let trimmed = &iri[1..iri.len() - 1]; // remove angle brackets
    for (key, value) in iri2prefix {
        if trimmed.starts_with(key) {
            return Some(format!("{}:{}", value, &trimmed[key.len()..]));
        }
    }
    None
}

fn replace_substrings(input: &str, iri2prefix: &HashMap<String, String>) -> String {
    if is_full_iri(input) {
        try_curify_iri(input, iri2prefix).unwrap_or_else(|| input.to_string())
    } else if let Some(x) = DATATYPE_LITERAL_REGEX.captures(input) {
        let literal = &x[1];
        let datatype_iri = &x[2];

        match try_curify_iri(datatype_iri, iri2prefix) {
            Some(curified) => format!("\"{}\"^^{}", literal, curified),
            None => input.to_string(),
        }
    } else {
        input.to_string()
    }
}

fn uncurify_value(ldtab: &Value, prefix2iri: &HashMap<String, String>) -> Value {
    match ldtab {
        Value::Array(vec) => {
            let new_vec: Vec<Value> = vec
                .iter()
                .map(|item| uncurify_value(item, prefix2iri))
                .collect();
            Value::Array(new_vec)
        }
        Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (key, value) in map.iter() {
                let expanded_key = expand_curies(key, prefix2iri);
                new_map.insert(expanded_key, uncurify_value(value, prefix2iri));
            }
            Value::Object(new_map)
        }
        Value::String(s) => Value::String(expand_curies(s, prefix2iri)),
        _ => ldtab.clone(),
    }
}

fn expand_curies(input: &str, prefix2iri: &HashMap<String, String>) -> String {
    if let Some(expanded) = expand_curie_to_iri(input, prefix2iri) {
        return expanded;
    }

    if let Some(caps) = DATATYPE_LITERAL_REGEX.captures(input) {
        let literal = &caps[1];
        let dtype = &caps[2];

        if let Some(expanded_dtype) = expand_curie_to_iri(dtype, prefix2iri) {
            return format!("\"{}\"^^{}", literal, expanded_dtype);
        }
    }

    input.to_string()
}

fn expand_curie_to_iri(curie: &str, prefix2iri: &HashMap<String, String>) -> Option<String> {
    if is_full_iri(curie) {
        return None;
    }

    let mut parts = curie.splitn(2, ':');
    let prefix = parts.next()?;
    let local = parts.next()?;

    let iri_base = prefix2iri.get(prefix)?;
    Some(format!("<{}{}>", iri_base, local))
}
