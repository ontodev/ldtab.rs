use serde_json::{Value};
use serde_json::json; 
use crate::ofn2owl::expression_transducer as expression_transducer;
use horned_owl::model::{Datatype, NamedIndividual, DataProperty, ObjectProperty, Build, Class, ClassExpression, AnnotatedAxiom, Axiom, SubClassOf, ClassAssertion, DeclareClass, DeclareObjectProperty, DeclareDatatype, DeclareDataProperty, DeclareNamedIndividual, DisjointClasses, DisjointUnion, EquivalentClasses, EquivalentObjectProperties, ObjectPropertyDomain, ObjectPropertyExpression, SubObjectPropertyOf, TransitiveObjectProperty, ObjectPropertyAssertion, ReflexiveObjectProperty, IrreflexiveObjectProperty, SymmetricObjectProperty, AsymmetricObjectProperty, ObjectPropertyRange, InverseObjectProperties, FunctionalObjectProperty, InverseFunctionalObjectProperty, DisjointObjectProperties, Import, SubDataPropertyOf, EquivalentDataProperties, DisjointDataProperties, DataPropertyDomain, DataPropertyRange, FunctionalDataProperty, DatatypeDefinition, HasKey, SameIndividual, DifferentIndividuals, NegativeObjectPropertyAssertion, DataPropertyAssertion, NegativeDataPropertyAssertion, AnnotationAssertion, OntologyAnnotation, DeclareAnnotationProperty, SubAnnotationPropertyOf, AnnotationPropertyDomain, AnnotationPropertyRange};

pub fn translate(v : &Value) -> AnnotatedAxiom {
    panic!()

}

pub fn translate_axiom(v : &Value) -> Axiom {

    match v[0].as_str() {
        Some("Declaration") => translate_declaration(v),
        Some("SubClassOf") => translate_subclass_of(v),
        Some("EquivalentClasses") => translate_equivalent_classes(v),
        Some("DisjointClasses") => translate_disjoint_classes(v),
        Some("DisjointUnion") => translate_disjoint_union(v),
        Some(_) => panic!("Not a valid OWL axiom operator"), 
        None => panic!("Not a valid (typed) OFN S-expression"), 
    } 
}

pub fn translate_named_class(v : &Value) -> Class {

    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 
    b.class(iri).into()
}

pub fn translate_object_property(v : &Value) -> ObjectProperty {

    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 
    b.object_property(iri).into()
}

pub fn translate_data_property(v : &Value) -> DataProperty {

    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 
    b.data_property(iri).into()
}

pub fn translate_named_individual(v : &Value) -> NamedIndividual {

    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 
    b.named_individual(iri).into()
}

pub fn translate_datatype(v : &Value) -> Datatype {

    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 
    b.datatype(iri).into()
}

pub fn translate_subclass_of(v : &Value) -> Axiom {
    let sub =  expression_transducer::translate_class_expression(&v[1]);
    let sup =  expression_transducer::translate_class_expression(&v[2]);
    let axiom = SubClassOf{sub : sub,
                           sup : sup};
    Axiom::SubClassOf(axiom)
}

pub fn translate_equivalent_classes(v : &Value) -> Axiom {

    let operands: Vec<ClassExpression> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| expression_transducer::translate_class_expression(&x))
                                               .collect(); 
    let axiom = EquivalentClasses{0 : operands};
    Axiom::EquivalentClasses(axiom)
}

pub fn translate_disjoint_classes(v : &Value) -> Axiom {

    let operands: Vec<ClassExpression> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| expression_transducer::translate_class_expression(&x))
                                               .collect(); 
    let axiom = DisjointClasses{0 : operands};
    Axiom::DisjointClasses(axiom)
}

pub fn translate_disjoint_union(v : &Value) -> Axiom {

    //NB: we need a (named) class here - not a class expression
    //let lhs = expression_transducer::translate_class_expression(&v[1]);
    let lhs = translate_named_class(&v[1]);

    let operands: Vec<ClassExpression> = (&(v.as_array().unwrap())[2..])
                                               .into_iter()
                                               .map(|x| expression_transducer::translate_class_expression(&x))
                                               .collect(); 
    let axiom = DisjointUnion{0: lhs,
                                1 : operands};
    Axiom::DisjointUnion(axiom)
}

pub fn translate_class_declaration(v : &Value) -> Axiom {

    let class = translate_named_class(&v[1]);
    let axiom = DeclareClass{0: class};
    Axiom::DeclareClass(axiom)
}

pub fn translate_object_property_declaration(v : &Value) -> Axiom {

    let property = translate_object_property(&v[1]);
    let axiom = DeclareObjectProperty{0: property};
    Axiom::DeclareObjectProperty(axiom)
}

pub fn translate_data_property_declaration(v : &Value) -> Axiom {

    let property = translate_data_property(&v[1]);
    let axiom = DeclareDataProperty{0: property};
    Axiom::DeclareDataProperty(axiom)
}

pub fn translate_named_individual_declaration(v : &Value) -> Axiom {

    let individual = translate_named_individual(&v[1]);
    let axiom = DeclareNamedIndividual{0: individual};
    Axiom::DeclareNamedIndividual(axiom)
}

pub fn translate_datatype_declaration(v : &Value) -> Axiom {

    let datatype = translate_datatype(&v[1]);
    let axiom = DeclareDatatype{0: datatype};
    Axiom::DeclareDatatype(axiom)
}

pub fn translate_declaration(v : &Value) -> Axiom {
    let unwrapped_declaration = v[1].clone();
    match unwrapped_declaration[0].as_str() {
        Some("Class") => translate_class_declaration(v),
        Some("ObjectProperty") => translate_object_property_declaration(v),
        Some("DataProperty") => translate_data_property_declaration(v),
        Some("NamedIndividual") => translate_named_individual_declaration(v),
        Some("Datatype") => translate_datatype_declaration(v),
        _ => panic!()
    }
}
