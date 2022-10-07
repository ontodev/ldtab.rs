use serde_json::{Value};
use serde_json::json; 
use crate::owl_2_ofn::axiom_transducer as axiom_transducer;
use crate::owl_2_ofn::annotation_transducer as annotation_transducer;
use horned_owl::model::{AnnotatedAxiom, Axiom, RcStr};

pub fn translate(axiom : &AnnotatedAxiom<RcStr>) -> Value {

    let mut logical_axiom = axiom_transducer::translate(&axiom.axiom);
    let annotations = &axiom.ann;

    if !annotations.is_empty() { 

        let annotation_list = annotation_transducer::translate_annotation_set(&axiom.ann); 

        let logical_axiom_vec = logical_axiom.as_array_mut().unwrap();

        for annotation in annotation_list {
            logical_axiom_vec.insert(1,annotation); 
        } 

        json!(logical_axiom_vec)

    } else { 
        logical_axiom
    }
}
