use rio_xml::{RdfXmlParser, RdfXmlError};
use rio_api::parser::TriplesParser;
use rio_api::model::NamedNode;
use std::path::Path;

use std::io::BufReader;
use std::fs::File;

pub fn parse(file: &str) {

    let ex_file = b"<?xml version=\"1.0\"?>
<rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\" xmlns:schema=\"http://schema.org/\">
 <rdf:Description rdf:about=\"http://example.com/foo\">
   <rdf:type rdf:resource=\"http://schema.org/Person\" />
   <schema:name>Foo</schema:name>
 </rdf:Description>
 <schema:Person rdf:about=\"http://example.com/bar\" schema:name=\"Bar\" />
</rdf:RDF>";

    let path = Path::new(file);

    let rdf_type = NamedNode { iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" };
    let mut count = 0;
    //RdfXmlParser::new(ex_file.as_ref(), None).parse_all(&mut |t| {
    RdfXmlParser::new(BufReader::new(File::open(path).unwrap()), None).parse_all(&mut |t| {
        if t.predicate == rdf_type {
            count += 1;
        }
        Ok(()) as Result<(), RdfXmlError>
    }).unwrap();
    println!("Count is {:?}", count); 

}
