use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;

use horned_owl::model::{Build, ClassExpression, DataRange, RcStr};

static ANONYMOUS_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(.*)_:(.+)$").unwrap());
static LITERAL_RE: Lazy<Regex> = Lazy::new(|| Regex::new("(?s)^\"(.*)\"(.*)$").unwrap());

pub fn is_literal(v: &Value) -> bool {
    match v {
        Value::String(x) => is_literal_string(x),
        _ => false,
    }
}

pub fn is_anonynous_individual(v: &Value) -> bool {
    match v {
        Value::String(x) => is_literal_string(x),
        _ => false,
    }
}

pub fn is_anonymous_individual(s: &str) -> bool {
    ANONYMOUS_RE.is_match(s)
}

pub fn is_literal_string(s: &str) -> bool {
    LITERAL_RE.is_match(s)
}

//TODO: check that the string is a valid IRI
pub fn extract_iri_str(v: &Value) -> &str {
    v.as_str().expect("Expected an IRI string")
}

pub fn build() -> Build<RcStr> {
    Build::new()
}

pub fn default_class_filler() -> ClassExpression<RcStr> {
    build().class("http://www.w3.org/2002/07/owl#Thing").into()
}

pub fn default_data_filler() -> DataRange<RcStr> {
    DataRange::Datatype(build().datatype("rdfs:Literal"))
}
