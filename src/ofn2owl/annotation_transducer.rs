use serde_json::{Value};
use serde_json::json; 
use horned_owl::model::{Build, Annotation, AnnotationProperty, AnnotationValue, AnnotationSubject};
use crate::ofn2owl::expression_transducer as expression_transducer;
use crate::ofn2owl::util as util;
use std::collections::BTreeSet;

pub fn translate_annotation_property(v : &Value) -> AnnotationProperty {
    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 
    
    b.annotation_property(iri)
}

pub fn translate_annotation_value(v : &Value) -> AnnotationValue {
    if util::is_literal(v) {
        let value = expression_transducer::translate_literal(v);
        AnnotationValue::Literal(value) 
    } else { 
        let b = Build::new();
        let string = v.as_str().unwrap();
        let iri = b.iri(string); 
        AnnotationValue::IRI(iri) 
    } 
}

pub fn translate_annotation_subject(v : &Value) -> AnnotationSubject {
    if util::is_anonynous_individual(v) {
        let individual = expression_transducer::translate_anonymous_individual(v);
        AnnotationSubject::AnonymousIndividual(individual) 
    } else { 
        let b = Build::new();
        let string = v.as_str().unwrap();
        let iri = b.iri(string); 
        AnnotationSubject::IRI(iri) 
    } 
}
