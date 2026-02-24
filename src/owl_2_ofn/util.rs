use serde_json::json;
use serde_json::Value;

pub fn format_iri(iri: &str) -> Value {
    json!(format!("<{}>", iri))
}
