use crate::owl_2_ofn::annotation_transducer;
use crate::owl_2_ofn::axiom_transducer;
use horned_owl::model::{AnnotatedComponent, RcStr};
use serde_json::json;
use serde_json::Value;

pub fn translate(axiom: &AnnotatedComponent<RcStr>) -> Value {
    let mut logical_axiom = axiom_transducer::translate(&axiom.component);
    let annotations = &axiom.ann;

    if !annotations.is_empty() {
        let annotation_list = annotation_transducer::translate_annotation_set(&axiom.ann);

        let logical_axiom_vec = logical_axiom.as_array_mut().unwrap();

        for annotation in annotation_list {
            logical_axiom_vec.insert(1, annotation);
        }

        json!(logical_axiom_vec)
    } else {
        logical_axiom
    }
}
