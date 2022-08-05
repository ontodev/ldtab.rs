use serde_json::{Value};
use serde_json::json; 
use crate::owl2ofn::axiom_transducer as axiom_transducer;
use crate::owl2ofn::annotation_transducer as annotation_transducer;
use horned_owl::model::{AnnotatedAxiom, Axiom};

pub fn translate(axiom : &AnnotatedAxiom) -> Value{

    let mut logical_axiom = axiom_transducer::translate(&axiom.axiom);

    if !axiom.ann.is_empty() { 
        let annotations = annotation_transducer::translate_annotation_set(&axiom.ann); 
        let logical_axiom_vec = logical_axiom.as_array_mut().unwrap();
        logical_axiom_vec.insert(1,annotations);
        json!(logical_axiom_vec)
    } else { 
        logical_axiom
    }
}
