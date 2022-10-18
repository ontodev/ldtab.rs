use serde_json::{Value};
use crate::ofn_2_owl::axiom_transducer as axiom_transducer;
use crate::ofn_2_owl::annotation_transducer as annotation_transducer;
use horned_owl::model::{AnnotatedAxiom, RcStr};
use std::collections::BTreeSet;


/// Given an OFN S-expression encoding an OWL axiom,
/// return it's representation in Horned OWL
///
/// #Examples
///
/// let ofn = vec!["SubClassOf", "ex:A", "ex:B"];
/// let ofn = json!(ofn);
/// let axiom = ofn_2_owl::transducer::translate(&ofn);
///
/// println!("{:?}", axiom); 

pub fn translate(ofn : &Value) -> AnnotatedAxiom<RcStr> {

    //split logic from annotation
    let owl = get_owl(ofn);
    let annotations = get_annotations(ofn);

    //translate logical component
    let axiom = axiom_transducer::translate_axiom(&owl);

    //translate annotation component
    let mut annotation_set = BTreeSet::new();
    for annotation in annotations {
        let ann = annotation_transducer::translate_annotation(&annotation);
        annotation_set.insert(ann);
    }

    //merge logical and annotation component
    let annotated_axiom = AnnotatedAxiom{axiom : axiom,
                                         ann: annotation_set};

    annotated_axiom 
}



//TODO: reuse wiring (ofn2ldtab/annotation_translation)
pub fn get_owl(ofn : &Value) -> Value {

    let mut res = Vec::new();
    let original = &ofn.as_array().unwrap()[0..];
    for element in original { 
        if !is_annotation(element){
            res.push(element.clone());
        } 
    } 
    Value::Array(res) 
}

pub fn is_annotation(v : &Value) -> bool { 
    match v.clone() { 
        Value::Array(x) => { 
            match x[0].as_str(){
                Some("Annotation") => true,
                Some(_) => false,
                None => false, 
            }
        }
        _ => false,
    }
}

pub fn has_annotation(v : &Value) -> bool { 
    match v.clone() {
        Value::Array(x) => is_annotation(&x[1]), //look into second argument
        _ => false,
    } 
}

pub fn get_annotations(ofn : &Value) -> Vec<Value> {

    if has_annotation(&ofn) {

        let mut res = Vec::new();
        let candidates = &ofn.as_array().unwrap()[0..];
        for candidate in candidates  {
            if is_annotation(candidate){
                res.push(candidate.clone());
            } 
        }
        res
    } else {
        Vec::new()//empty vector
    } 
}
