use serde_json::{Value};
use regex::Regex;

pub fn is_literal(v: &Value) -> bool {
    match v {
        Value::String(x) => is_literal_string(x),
        _ => false 
    }
}

pub fn is_anonynous_individual(v : &Value) -> bool {
    match v {
        Value::String(x) => is_literal_string(x),
        _ => false,
    }
}

pub fn is_anonymous_individual(s : &str) -> bool {
    let anonymous = Regex::new("^(.*)_:(.+)$").unwrap(); 
    anonymous.is_match(s)
}

pub fn is_literal_string(s : &str) -> bool {

    //NB: "(?s)" sets a flag so that . matches \n 
    //let literal = Regex::new("^\"(.*)\"(.*)$").unwrap();
    let literal = Regex::new("(?s)^\"(.*)\"(.*)$").unwrap();

    literal.is_match(s) 
}
