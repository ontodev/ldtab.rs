use horned_owl::command::parse_path;
use std::path::Path;
use std::rc::Rc;
use horned_owl::model::{Axiom, Build, Class, ClassAssertion, ClassExpression, DeclareClass, DeclareNamedIndividual, DisjointClasses, DisjointUnion, EquivalentClasses, EquivalentObjectProperties, NamedIndividual, ObjectProperty, ObjectPropertyDomain, ObjectPropertyExpression, SubObjectPropertyExpression, SubObjectPropertyOf, TransitiveObjectProperty, Individual as OWLIndividual, ObjectPropertyAssertion};
use horned_owl::model::ClassExpression::ObjectUnionOf;
use horned_owl::ontology::set::SetOntology;


use rio_xml::{RdfXmlParser, RdfXmlError};
use rio_api::parser::TriplesParser;
use rio_api::model::*;

use im::{HashSet, hashset};

pub mod xml;


pub fn print_expression(expr: &ClassExpression) {
    println!("{:?}", expr); 
}

pub fn print_ontology(ontology: &SetOntology) {

    for ann_axiom in ontology.iter() {
        let axiom = &ann_axiom.axiom;
        match axiom {
             Axiom::SubClassOf(ax) => { println!("SubClassOf:");
                 print_expression(&ax.sub);
                 print_expression(&ax.sup); 
             },
             _ => println!("Something else"),
        };
    }
}

fn main() {

    match parse_path(Path::new("ontology.owl")) {
        Ok(parsed) => {
                let ont = &parsed.decompose().0;
                print_ontology(ont);
        }
        Err(error) => { dbg!("ERROR: {}", error); }
    } 

    xml::parser::parse("ontology.owl");

}
