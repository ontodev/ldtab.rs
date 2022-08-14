use horned_owl::command::parse_path;
use serde_json::{Value};
use serde_json::json; 
use std::path::Path;
use std::rc::Rc;
use horned_owl::model::{IRI, AnnotatedAxiom, Ontology, Axiom, Build, Class, ClassAssertion, ClassExpression, DeclareClass, DeclareNamedIndividual, DisjointClasses, DisjointUnion, EquivalentClasses, EquivalentObjectProperties, NamedIndividual, ObjectProperty, ObjectPropertyDomain, ObjectPropertyExpression, SubObjectPropertyExpression, SubObjectPropertyOf, TransitiveObjectProperty, Individual as OWLIndividual, ObjectPropertyAssertion, Import, OntologyAnnotation};
use horned_owl::model::ClassExpression::ObjectUnionOf;
use horned_owl::ontology::set::SetOntology;


use rio_xml::{RdfXmlParser, RdfXmlError};
use rio_api::parser::TriplesParser;
use rio_api::model::*;

use crate::owl2ofn::transducer as owl2ofn; 
use crate::owl2ofn::annotation_transducer as annotation_transducer; 

extern crate wiring_rs; 

use im::{HashSet, hashset};

pub fn ontology_2_ldtab(ontology : &SetOntology) {
    //TODO a SetOntology has the field "doc_iri" - however this doesn't seem to be always set?
    let id = ontology.id();
    let iri = id.clone().iri.unwrap(); //TODO can I assume that this exists?

    for ann_axiom in ontology.iter() { 

        let axiom = &ann_axiom.axiom;
        let annotations = &ann_axiom.ann;

        let inspect = match axiom {
            Axiom::Import(x) => translate_ontology_import(&x, &iri),
            Axiom::OntologyAnnotation(x) => translate_ontology_annotation(&x, &iri),
            _ => owl_2_thick(&ann_axiom),
        };

        //TODO insert into database 
        println!("Input {:?}", ann_axiom);
        println!("Output {}", inspect);
        println!("===");
    }

}

pub fn translate_ontology_annotation(axiom : &OntologyAnnotation, ontology_iri : &IRI) -> Value {
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
           "annotation":"Null"
       }) 
}

pub fn translate_ontology_import(axiom : &Import, ontology_iri : &IRI) -> Value {

    //TODO: sort this
       json!({"assertion":"1",
           "retraction":"0",
           "graph":"graph",
           "subject":ontology_iri.get(0..),
           "predicate":"owl:imports",
           "object":axiom.0.get(0..),
           "datatype":"_IRI",
           "annotation":"Null"
       }) 
}

pub fn owl_2_thick(annotated_axiom : &AnnotatedAxiom) -> Value {

    let ofn = owl2ofn::translate(annotated_axiom);
    let thick_triple = wiring_rs::ofn2ldtab::ofn_parser::translate_triple(&ofn);

    thick_triple
}
