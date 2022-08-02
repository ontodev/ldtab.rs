use serde_json::{Value};
use serde_json::json; 
use crate::owl2ofn::expression_transducer as expression_transducer;
use horned_owl::model::{Axiom, Build, Class, SubClassOf, ClassAssertion, ClassExpression, DeclareClass, DeclareNamedIndividual, DisjointClasses, DisjointUnion, EquivalentClasses, EquivalentObjectProperties, NamedIndividual, ObjectProperty, ObjectPropertyDomain, ObjectPropertyExpression, SubObjectPropertyExpression, SubObjectPropertyOf, TransitiveObjectProperty, Individual, ObjectPropertyAssertion, AnonymousIndividual, DataProperty, DataRange, Datatype, Literal, FacetRestriction, Facet};


pub fn translate(axiom : &Axiom) -> Value {

    match axiom {
        Axiom::OntologyAnnotation(x) => json!("[TODO ontology annotation]"),
        Axiom::Import(x) => json!("[TODO import]"),
        Axiom::DeclareClass(x) => json!("[TODO declare class]"),
        Axiom::DeclareObjectProperty(x) => json!("[TODO declare object property]"),
        Axiom::DeclareAnnotationProperty(x) => json!("[TODO declare annotation property]"),
        Axiom::DeclareDataProperty(x) => json!("[TODO declare data property]"),
        Axiom::DeclareNamedIndividual(x) => json!("[TODO declare named individual]"),
        Axiom::DeclareDatatype(x) => json!("[TODO declare datatype]"),

        Axiom::SubClassOf(x) => translate_subclass_of(x),
        Axiom::EquivalentClasses(x) => translate_equivalent_classes(x),
        Axiom::DisjointClasses(x) => translate_disjoint_classes(x),
        Axiom::DisjointUnion(x) => json!("[TODO disjoint union]"),

        Axiom::SubObjectPropertyOf(x) => json!("[TODO sub object property of]"),
        Axiom::EquivalentObjectProperties(x) => json!("[TODO equivalent object properties]"),
        Axiom::DisjointObjectProperties(x) => json!("[TODO disjoint object properties]"),
        Axiom::InverseObjectProperties(x) => json!("[TODO inverse object properties]"),
        Axiom::ObjectPropertyDomain(x) => json!("[TODO object property domain]"),
        Axiom::ObjectPropertyRange(x) => json!("[TODO object property range]"),
        Axiom::FunctionalObjectProperty(x) => json!("[TODO functional object property]"),
        Axiom::InverseFunctionalObjectProperty(x) => json!("[TODO inverse functional object property]"),
        Axiom::ReflexiveObjectProperty(x) => json!("[TODO reflexive object property]"),
        Axiom::IrreflexiveObjectProperty(x) => json!("[TODO irreflexive object property]"),
        Axiom::SymmetricObjectProperty(x) => json!("[TODO symmetric object property]"),
        Axiom::AsymmetricObjectProperty(x) => json!("[TODO assymmetric object property]"),
        Axiom::TransitiveObjectProperty(x) => json!("[TODO transitive object property]"),
        Axiom::SubDataPropertyOf(x) => json!("[TODO sub data property]"),
        Axiom::EquivalentDataProperties(x) => json!("[TODO equivalent data property]"),
        Axiom::DisjointDataProperties(x) => json!("[TODO disjoint data property]"),
        Axiom::DataPropertyDomain(x) => json!("[TODO data property domain]"),
        Axiom::DataPropertyRange(x) => json!("[TODO data property range]"),
        Axiom::FunctionalDataProperty(x) => json!("[TODO functional data property]"),
        Axiom::DatatypeDefinition(x) => json!("[TODO data type definition]"),
        Axiom::HasKey(x) => json!("[TODO has key]"),
        Axiom::SameIndividual(x) => json!("[TODO same individual]"),
        Axiom::DifferentIndividuals(x) => json!("[TODO different individuals]"),
        Axiom::ClassAssertion(x) => json!("[TODO class assertion]"),
        Axiom::ObjectPropertyAssertion(x) => json!("[TODO object property assertion]"),
        Axiom::NegativeObjectPropertyAssertion(x) => json!("[TODO negative object property assertion]"),
        Axiom::DataPropertyAssertion(x) => json!("[TODO data property assertion]"),
        Axiom::NegativeDataPropertyAssertion(x) => json!("[TODO negative data property assertion]"),
        Axiom::AnnotationAssertion(x) => json!("[TODO annotation assertion]"),
        Axiom::SubAnnotationPropertyOf(x) => json!("[TODO sub annotation property of]"),
        Axiom::AnnotationPropertyDomain(x) => json!("[TODO annotation property domain]"),
        Axiom::AnnotationPropertyRange(x) => json!("[TODO annotation property range]"), 
    } 
}

pub fn translate_subclass_of(axiom: &SubClassOf) -> Value { 

    let operator = Value::String(String::from("SubClassOf"));
    let subclass = expression_transducer::translate_class_expression(&axiom.sub);
    let superclass = expression_transducer::translate_class_expression(&axiom.sup);
    let v = vec![operator, subclass, superclass];
    Value::Array(v) 
}

pub fn translate_disjoint_classes(axiom : &DisjointClasses) -> Value {
    let operator = Value::String(String::from("DisjointClasses"));
    let classes = axiom.0.clone();
    let mut operands : Vec<Value> = classes.into_iter()
                                         .map(|x| expression_transducer::translate_class_expression(&x))
                                         .collect(); 
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_equivalent_classes(axiom : &EquivalentClasses) -> Value {
    let operator = Value::String(String::from("EquivalentClasses"));
    let classes = axiom.0.clone();
    let mut operands : Vec<Value> = classes.into_iter()
                                         .map(|x| expression_transducer::translate_class_expression(&x))
                                         .collect(); 
    operands.insert(0,operator);
    Value::Array(operands) 
}
