use horned_owl::command::parse_path;
use std::path::Path;
use std::env;

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

    match parse_path(Path::new(&args[1])) {
        Ok(parsed) => {
                let ont = &parsed.decompose().0;
                
                let r_test = round::ontology::ontology_2_ldtab(ont);

                //serialise a horned OWL ontology
                //let ax_map = mapped::from(ont.clone());
                //let ax_map = mapped::from(r_test.clone());

                //let buffer = File::create("out.owl").unwrap(); 
                //let res = serialise(buffer, &ax_map, None);
                //println!("{:?}", res); 
        }
        Err(error) => { dbg!("ERROR: {}", error); }
    } 
}
