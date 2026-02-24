use crate::owl_2_ofn::expression_transducer;
use crate::owl_2_ofn::util::{format_iri, ofn_list};
use horned_owl::model::{
    Annotation, AnnotationProperty, AnnotationSubject, AnnotationValue, ArcStr,
};
use serde_json::Value;
use std::collections::BTreeSet;

pub fn translate_annotation(annotation: &Annotation<ArcStr>) -> Value {
    let property = translate_annotation_property(&annotation.ap);
    let value = translate_annotation_value(&annotation.av);
    ofn_list("Annotation", vec![property, value])
}

pub fn translate_annotation_subject(annotation_subject: &AnnotationSubject<ArcStr>) -> Value {
    match annotation_subject {
        AnnotationSubject::IRI(x) => format_iri(x.get(0..).unwrap()),
        AnnotationSubject::AnonymousIndividual(x) => {
            expression_transducer::translate_anonymous_individual(x)
        }
    }
}

pub fn translate_annotation_property(property: &AnnotationProperty<ArcStr>) -> Value {
    format_iri(property.0.get(0..).unwrap())
}

pub fn translate_annotation_value(value: &AnnotationValue<ArcStr>) -> Value {
    match value {
        AnnotationValue::Literal(x) => expression_transducer::translate_literal(x),
        AnnotationValue::IRI(x) => format_iri(x.get(0..).unwrap()),
        AnnotationValue::AnonymousIndividual(x) => {
            expression_transducer::translate_anonymous_individual(x)
        }
    }
}

pub fn translate_annotation_set(annotation_set: &BTreeSet<Annotation<ArcStr>>) -> Vec<Value> {
    annotation_set.iter().map(|a| translate_annotation(a)).collect()
}

//pub fn translate_annotation_set(annotation_set : &BTreeSet<Annotation>) -> Value {
//        let operator = Value::String(String::from("AnnotationList"));//NB: not OWL
//        let mut res = vec![operator];
//        for annotation in annotation_set {
//            res.push(translate_annotation(annotation));
//        }
//        Value::Array(res)
//}
