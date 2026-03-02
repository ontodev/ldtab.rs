use anyhow::Result;
use crate::ofn_2_owl::expression_transducer;
use crate::ofn_2_owl::util;
use crate::ofn_2_owl::util::{build, extract_iri_str};
use horned_owl::model::{
    Annotation, AnnotationProperty, AnnotationSubject, AnnotationValue, ArcStr,
};
use serde_json::Value;

pub fn translate_annotation(v: &Value) -> Result<Annotation<ArcStr>> {
    let property = translate_annotation_property(&v[1])?;
    let value = translate_annotation_value(&v[2])?;
    Ok(Annotation {
        ap: property,
        av: value,
    })
}

pub fn translate_annotation_property(v: &Value) -> Result<AnnotationProperty<ArcStr>> {
    Ok(build().annotation_property(extract_iri_str(v)?))
}

pub fn translate_annotation_value(v: &Value) -> Result<AnnotationValue<ArcStr>> {
    if util::is_literal(v) {
        let value = expression_transducer::translate_literal(v)?;
        Ok(AnnotationValue::Literal(value))
    } else {
        let iri = build().iri(extract_iri_str(v)?);
        Ok(AnnotationValue::IRI(iri))
    }
}

pub fn translate_annotation_subject(v: &Value) -> Result<AnnotationSubject<ArcStr>> {
    if util::is_anonynous_individual(v) {
        let individual = expression_transducer::translate_anonymous_individual(v)?;
        Ok(AnnotationSubject::AnonymousIndividual(individual))
    } else {
        let iri = build().iri(extract_iri_str(v)?);
        Ok(AnnotationSubject::IRI(iri))
    }
}
