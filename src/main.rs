use horned_owl::io::ParserConfiguration;
use horned_bin::parse_path;

use std::path::Path;
use std::env;
use horned_owl::ontology::set::SetOntology;
use horned_owl::model::*;
use serde_json::{Value};

//use rio_xml::{RdfXmlParser, RdfXmlError};
//use rio_api::parser::TriplesParser;
//use rio_api::model::*; 
//use std::fs::File; 
//use im::{HashSet, hashset};

pub mod xml;
pub mod owl_2_ofn;
pub mod ofn_2_owl;
pub mod import;
extern crate wiring_rs;

pub mod round;


fn main() {

    let args: Vec<String> = env::args().collect();

    match parse_path(Path::new(&args[1]), ParserConfiguration::default()) {
        Ok(parsed) => {
                let ont = &parsed.decompose().0;

                demo(ont);
                
        }
        Err(error) => { dbg!("ERROR: {}", error); }
    } 
}

fn demo(ontology : &SetOntology<RcStr>)  {

    let id = ontology.id();
    let iri = id.clone().iri.unwrap(); 
    let iri_value = Value::String(String::from(iri.get(0..).unwrap()));

    for ann_axiom in ontology.iter() { 
        println!("Horned OWL: {:?}" , ann_axiom);

        //1. translate Horned OWL to OFN S-expression
        let ofn = owl_2_ofn::transducer::translate(ann_axiom);
        let ofn =
        match ofn[0].as_str() {

            Some("Import") => Value::Array(vec![ofn[0].clone(), iri_value.clone(), ofn[1].clone()]),
            Some("OntologyAnnotation") => Value::Array(vec![ofn[0].clone(), iri_value.clone(), ofn[1].clone()]) ,
            _ => ofn.clone() 
        };
        println!("OFN S-expression: {}" , ofn);

        //2. translate OFN S-Expression to LDTab ThickTriple
        let ldtab = wiring_rs::ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn); 
        println!("LDTab: {}" , ldtab);
        
        //3. Get typing and labeling information from ontology
        //4. Use wiring to type and label expressions
        //5. use ofn_2_ofn module to translate OWL axioms back into Horned OWL

    } 
}
