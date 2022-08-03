use serde_json::{Value};
use serde_json::json; 
use crate::owl2ofn::expression_transducer as expression_transducer;
use horned_owl::model::{Annotation, AnnotationProperty, AnnotationValue, AnnotationSubject};


pub fn translate_annotation(annotation : &Annotation) -> Value {
    let operator = Value::String(String::from("Annotation"));
    let property = translate_annotation_property(&annotation.ap);
    let value = translate_annotation_value(&annotation.av);
    let res = vec![operator, property, value]; 
    Value::Array(res) 
}

pub fn translate_annotation_subject(annotation_subject : &AnnotationSubject) -> Value {
    match annotation_subject {
        AnnotationSubject::IRI(x) => json!(x.get(0..)),
        AnnotationSubject::AnonymousIndividual(x) => expression_transducer::translate_anonymous_individual(x),
    }
}

pub fn translate_annotation_property(property : &AnnotationProperty) -> Value { 
    let a = property.0.get(0..);
    json!(a)
}

pub fn translate_annotation_value(value : &AnnotationValue) -> Value {
    match value {
        AnnotationValue::Literal(x) => expression_transducer::translate_literal(x),
        AnnotationValue::IRI(x) => json!(x.get(0..)),
    } 
}
