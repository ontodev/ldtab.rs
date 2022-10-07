use serde_json::{Value};
use serde_json::json; 
use horned_owl::model::{Build, Annotation, AnnotationProperty, AnnotationValue, AnnotationSubject, ArcStr, RcStr};
use crate::ofn_2_owl::expression_transducer as expression_transducer;
use crate::ofn_2_owl::util as util;
use std::collections::BTreeSet;

pub fn translate_annotation(v : &Value) -> Annotation<RcStr> {
    let property = translate_annotation_property(&v[1]);
    let value = translate_annotation_value(&v[2]);
    let annotation = Annotation{ap : property,
                                av : value};
    annotation 
}

pub fn translate_annotation_property(v : &Value) -> AnnotationProperty<RcStr> {
    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 

    b.annotation_property(iri.clone())
}

pub fn translate_annotation_value(v : &Value) -> AnnotationValue<RcStr> {
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

pub fn translate_annotation_subject(v : &Value) -> AnnotationSubject<RcStr> {
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
