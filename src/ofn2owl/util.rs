use serde_json::{Value};
use regex::Regex;

pub fn is_literal(v: &Value) -> bool {
    match v.as_str() {
        Some(x) => is_literal_string(x),
        None => false 
    }
}

pub fn is_anonynous_individual(v : &Value) -> bool {
    match v.as_str() {
        Some(x) => is_literal_string(x),
        None => false 
    } 
}

pub fn is_anonymous_individual(s : &str) -> bool {
    let anonymous = Regex::new("^(.*)_:(.+)$").unwrap();
    if anonymous.is_match(s) {
        true
    } else  {
        false
    } 
}

pub fn is_literal_string(s : &str) -> bool {

    let literal = Regex::new("^\"(.+)\"(.*)$").unwrap();
    //let simple = Regex::new("^\"(.+)\"$").unwrap();
    //let language_tag = Regex::new("^\"(.+)\"@(.*)$").unwrap();
    //let datatype = Regex::new("^\"(.+)\"\\^\\^(.*)$").unwrap();

    if literal.is_match(s) {
        true
    } else  {
        false
    } 
}
