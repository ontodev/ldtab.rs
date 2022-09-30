use serde_json::{Value};
use serde_json::json; 
use horned_owl::model::{IRI, AnnotatedAxiom, Ontology, Axiom, Import, OntologyAnnotation, RcStr};
use horned_owl::ontology::set::SetOntology;


use crate::owl2ofn::transducer as owl2ofn; 
use crate::ofn2owl::transducer as ofn2owl; 
use crate::owl2ofn::annotation_transducer as annotation_transducer; 

extern crate wiring_rs; 


pub fn ontology_2_ldtab(ontology : &SetOntology<RcStr>) {
    //TODO a SetOntology has the field "doc_iri" - however this doesn't seem to be always set?
    let id = ontology.id();
    let iri = id.clone().iri.unwrap(); //TODO can I assume that this exists?

    for ann_axiom in ontology.iter() { 

        let axiom = &ann_axiom.axiom;
        let annotations = &ann_axiom.ann;

        if !annotations.is_empty() {

            println!("0. Horned OWL: {:?}", ann_axiom);

            //translate horned OWL to LDTab Thick Triple
            let inspect = match axiom {

                //two special cases:
                Axiom::Import(x) => translate_ontology_import(&x, &iri),
                Axiom::OntologyAnnotation(x) => translate_ontology_annotation(&x, &iri),

                _ => owl_2_thick(&ann_axiom),
            };


        }
    }

}

//the following translations are two special cases that cannot be
//translated with the standard translation
//(because some related information is split in Horned OWL's data model)
pub fn translate_ontology_annotation(axiom : &OntologyAnnotation<RcStr>, ontology_iri : &IRI<RcStr>) -> Value {
    //An ontolog annotation could be encoded as a OFN S-expression
    //via
    //["ThickTriple", s, p, o]
    //or
    //["Ontology", ["ThickTriple", s, p, o]]]
    //
    let annotation = axiom.0.clone();

    let property = annotation.ap;
    let value = annotation.av;
    let value_ofn = annotation_transducer::translate_annotation_value(&value);
    let value_ldtab = wiring_rs::ofn2ldtab::annotation_translation::translate_value(&value_ofn);
    let value_datatype = wiring_rs::ofn2ldtab::util::translate_datatype(&value_ofn);

    //TODO: recrusive annotations (currently not supported in horned OWL)
    //TODO: sort this
       json!({"assertion":"1",
           "retraction":"0",
           "graph":"graph",
           "subject":ontology_iri.get(0..),
           "predicate":property.0.get(0..),
           "object":value_ldtab,
           "datatype":value_datatype,
           "annotation":"Null" //TODO
       }) 
}

pub fn translate_ontology_import(axiom : &Import<RcStr>, ontology_iri : &IRI<RcStr>) -> Value {

    //TODO: sort this
       json!({"assertion":"1",
           "retraction":"0",
           "graph":"graph",
           "subject":ontology_iri.get(0..),
           "predicate":"owl:imports",
           "object":axiom.0.get(0..),
           "datatype":"_IRI",
           "annotation":"Null" //TODO
       }) 
}

//translate a Horned OWL axiom to a LDTab ThickTriple
pub fn owl_2_thick(annotated_axiom : &AnnotatedAxiom<RcStr>) -> Value {

    //horned OWL to OFN S-expression
    let ofn = owl2ofn::translate(annotated_axiom);
    println!("1 OFN: {}", ofn);

    //OFN S-expression to LDTab ThickTriple
    let thick_triple = wiring_rs::ofn2ldtab::ofn_parser::translate_triple(&ofn);

    thick_triple
}
