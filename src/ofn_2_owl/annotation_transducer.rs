use crate::ofn_2_owl::expression_transducer;
use crate::ofn_2_owl::util;
use crate::ofn_2_owl::util::{build, extract_iri_str};
use horned_owl::model::{
    Annotation, AnnotationProperty, AnnotationSubject, AnnotationValue, RcStr,
};
use serde_json::Value;

pub fn translate_annotation(v: &Value) -> Annotation<RcStr> {
    let property = translate_annotation_property(&v[1]);
    let value = translate_annotation_value(&v[2]);
    let annotation = Annotation {
        ap: property,
        av: value,
    };
    annotation
}

pub fn translate_annotation_property(v: &Value) -> AnnotationProperty<RcStr> {
    build().annotation_property(extract_iri_str(v))
}

pub fn translate_annotation_value(v: &Value) -> AnnotationValue<RcStr> {
    if util::is_literal(v) {
        let value = expression_transducer::translate_literal(v);
        AnnotationValue::Literal(value)
    } else {
        let iri = build().iri(extract_iri_str(v));
        AnnotationValue::IRI(iri)
    }
}

pub fn translate_annotation_subject(v: &Value) -> AnnotationSubject<RcStr> {
    if util::is_anonynous_individual(v) {
        let individual = expression_transducer::translate_anonymous_individual(v);
        AnnotationSubject::AnonymousIndividual(individual)
    } else {
        let iri = build().iri(extract_iri_str(v));
        AnnotationSubject::IRI(iri)
    }
}
