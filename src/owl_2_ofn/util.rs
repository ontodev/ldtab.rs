use serde_json::json;
use serde_json::Value;

pub fn format_iri(iri: &str) -> Value {
    json!(format!("<{}>", iri))
}

/// Builds an OFN S-expression array from an operator name and a list of
/// (already-translated) operand Values: `["Operator", op1, op2, ...]`.
pub fn ofn_list(operator: &str, operands: Vec<Value>) -> Value {
    let mut v = vec![Value::String(String::from(operator))];
    v.extend(operands);
    Value::Array(v)
}
