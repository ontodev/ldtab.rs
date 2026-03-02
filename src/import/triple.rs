use anyhow::{Context, Result};
use serde_json::Value;

// LDTab datatypes
pub(crate) const DATATYPE_IRI: &str = "_IRI";

// Default values
pub(crate) const DEFAULT_GRAPH: &str = "graph";
pub(crate) const UNKNOWN_VALUE: &str = "<unknown>";

#[derive(Debug, Clone)]
pub(crate) struct LdTabTriple {
    pub assertion: i32,
    pub retraction: i32,
    pub graph: String,
    pub subject: Value,
    pub predicate: Value,
    pub object: Value,
    pub datatype: Value,
    pub annotation: Value,
}

impl LdTabTriple {
    /// Create a new triple with default assertion=1, retraction=0, default graph.
    pub fn new(
        subject: impl Into<Value>,
        predicate: impl Into<Value>,
        object: impl Into<Value>,
        datatype: impl Into<Value>,
    ) -> Self {
        Self {
            assertion: 1,
            retraction: 0,
            graph: DEFAULT_GRAPH.to_string(),
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            datatype: datatype.into(),
            annotation: Value::Null,
        }
    }

    pub fn annotation(mut self, annotation: impl Into<Value>) -> Self {
        self.annotation = annotation.into();
        self
    }
}

/// Parse a JSON thick-triple (from wiring_rs) into an LdTabTriple.
pub(crate) fn ldtab_2_triple(value: &Value) -> Result<LdTabTriple> {
    let assertion = value
        .get("assertion")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(1);
    let retraction = value
        .get("retraction")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);

    let graph = value
        .get("graph")
        .and_then(|v| v.as_str())
        .unwrap_or(DEFAULT_GRAPH)
        .to_string();
    let subject = value
        .get("subject")
        .cloned()
        .context("Thick triple missing 'subject' field")?;
    let predicate = value
        .get("predicate")
        .cloned()
        .context("Thick triple missing 'predicate' field")?;
    let object = value
        .get("object")
        .cloned()
        .context("Thick triple missing 'object' field")?;
    let datatype = value
        .get("datatype")
        .cloned()
        .context("Thick triple missing 'datatype' field")?;
    let annotation = get_annotation(value);

    Ok(LdTabTriple {
        assertion,
        retraction,
        graph,
        subject,
        predicate,
        object,
        datatype,
        annotation,
    })
}

pub fn is_ldtab_blanknode(input: &Value) -> bool {
    input
        .as_str()
        .map(|s| s.starts_with("<ldtab:blanknode"))
        .unwrap_or(false)
}

pub(crate) fn extract_value_and_datatype(value: &Value, default_datatype: &Value) -> (Value, Value) {
    match value {
        Value::String(_) => (value.clone(), default_datatype.clone()),
        Value::Array(a) if !a.is_empty() => {
            if let Some(x) = a[0].as_object() {
                if let (Some(datatype), Some(obj)) = (x.get("datatype"), x.get("object")) {
                    return (obj.clone(), datatype.clone());
                }
            }
            (value.clone(), default_datatype.clone())
        }
        Value::Object(x) => {
            if x.get("datatype").and_then(|v| v.as_str()) == Some(DATATYPE_IRI) {
                if let Some(obj) = x.get("object") {
                    return (obj.clone(), Value::String(DATATYPE_IRI.to_string()));
                }
            }
            (value.clone(), default_datatype.clone())
        }
        _ => (value.clone(), default_datatype.clone()),
    }
}

/// Serialize a Value to the string representation used in the database.
pub(crate) fn value_to_db_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        _ => value.to_string(),
    }
}

fn get_annotation(value: &Value) -> Value {
    match value.get("annotation") {
        None | Some(Value::Null) => Value::Null,

        Some(Value::String(s)) => {
            if s.is_empty() {
                Value::Null
            } else if let Ok(inner) = serde_json::from_str::<Value>(s) {
                inner
            } else {
                Value::String(s.clone())
            }
        }

        Some(Value::Object(map)) if map.is_empty() => Value::Null,
        Some(v) => v.clone(),
    }
}
