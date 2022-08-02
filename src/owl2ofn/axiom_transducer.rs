use serde_json::{Value};
use serde_json::json; 
use crate::owl2ofn::expression_transducer as expression_transducer;
use horned_owl::model::{Axiom, Build, Class, SubClassOf, ClassAssertion, ClassExpression, DeclareClass, DeclareObjectProperty, DeclareDatatype, DeclareDataProperty, DeclareNamedIndividual, DisjointClasses, DisjointUnion, EquivalentClasses, EquivalentObjectProperties, NamedIndividual, ObjectProperty, ObjectPropertyDomain, ObjectPropertyExpression, SubObjectPropertyExpression, SubObjectPropertyOf, TransitiveObjectProperty, Individual, ObjectPropertyAssertion, AnonymousIndividual, DataProperty, DataRange, Datatype, Literal, FacetRestriction, Facet, ReflexiveObjectProperty, IrreflexiveObjectProperty, SymmetricObjectProperty, AsymmetricObjectProperty, ObjectPropertyRange, InverseObjectProperties, FunctionalObjectProperty, InverseFunctionalObjectProperty, DisjointObjectProperties};


pub fn translate(axiom : &Axiom) -> Value {

    match axiom {
        Axiom::OntologyAnnotation(x) => json!("[TODO ontology annotation]"),
        Axiom::Import(x) => json!("[TODO import]"),

        Axiom::DeclareClass(x) => translate_class_declaration(x),
        Axiom::DeclareObjectProperty(x) => translate_object_property_declaration(x),
        Axiom::DeclareAnnotationProperty(x) => json!("[TODO declare annotation property]"),
        Axiom::DeclareDataProperty(x) => translate_data_property_declaration(x),
        Axiom::DeclareNamedIndividual(x) => translate_named_individual_declaration(x),
        Axiom::DeclareDatatype(x) => translate_datatype_declaration(x),

        Axiom::SubClassOf(x) => translate_subclass_of(x),
        Axiom::EquivalentClasses(x) => translate_equivalent_classes(x),
        Axiom::DisjointClasses(x) => translate_disjoint_classes(x),
        Axiom::DisjointUnion(x) => translate_disjoint_union(x),

        Axiom::SubObjectPropertyOf(x) => translate_sub_object_property(x),
        Axiom::EquivalentObjectProperties(x) => translate_equivalent_object_properties(x),
        Axiom::DisjointObjectProperties(x) => translate_disjoint_object_properties(x),
        Axiom::InverseObjectProperties(x) => translate_inverse_properties(x),
        Axiom::ObjectPropertyDomain(x) => translate_object_property_domain(x),
        Axiom::ObjectPropertyRange(x) => translate_object_property_range(x),
        Axiom::FunctionalObjectProperty(x) => translate_functional_object_property(x),
        Axiom::InverseFunctionalObjectProperty(x) => translate_inverse_functional_object_property(x),
        Axiom::ReflexiveObjectProperty(x) => translate_reflexive_object_property(x),
        Axiom::IrreflexiveObjectProperty(x) => translate_irreflexive_object_property(x),
        Axiom::SymmetricObjectProperty(x) => translate_symmetric_object_property(x),
        Axiom::AsymmetricObjectProperty(x) => translate_asymmetric_object_property(x),
        Axiom::TransitiveObjectProperty(x) => translate_transitive_object_property(x),

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

pub fn translate_disjoint_union(axiom : &DisjointUnion) -> Value {
    let operator = Value::String(String::from("DisjointUnion"));
    let lhs = axiom.0.clone();
    let lhs = expression_transducer::translate_class(&lhs);
    let arguments = axiom.1.clone();
    let mut operands : Vec<Value> = arguments.into_iter()
                                         .map(|x| expression_transducer::translate_class_expression(&x))
                                         .collect(); 

    operands.insert(0,lhs);
    operands.insert(0,operator);
    Value::Array(operands) 
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

pub fn translate_object_property_axiom(operator : &str, property : &ObjectPropertyExpression) -> Value {

    let operator = Value::String(String::from(operator));
    let argument = expression_transducer::translate_property_expression(property);

    let mut res = vec![operator];
    res.push(argument);
    Value::Array(res) 
}

pub fn translate_reflexive_object_property(axiom : &ReflexiveObjectProperty) -> Value {
    translate_object_property_axiom("ReflexiveObjectProperty", &axiom.0.clone()) 
}

pub fn translate_irreflexive_object_property(axiom : &IrreflexiveObjectProperty) -> Value {
    translate_object_property_axiom("IrreflexiveObjectProperty", &axiom.0.clone()) 
}

pub fn translate_symmetric_object_property(axiom : &SymmetricObjectProperty) -> Value {
    translate_object_property_axiom("SymmetricObjectProperty", &axiom.0.clone()) 
}

pub fn translate_asymmetric_object_property(axiom : &AsymmetricObjectProperty) -> Value {
    translate_object_property_axiom("AsymmetricObjectProperty", &axiom.0.clone()) 
}

pub fn translate_transitive_object_property(axiom : &TransitiveObjectProperty) -> Value {
    translate_object_property_axiom("TransitiveObjectProperty", &axiom.0.clone()) 
}

pub fn translate_functional_object_property(axiom : &FunctionalObjectProperty) -> Value {
    translate_object_property_axiom("FunctionalObjectProperty", &axiom.0.clone()) 
}

pub fn translate_inverse_functional_object_property(axiom : &InverseFunctionalObjectProperty) -> Value {
    translate_object_property_axiom("InverseFunctionalObjectProperty", &axiom.0.clone()) 
}

pub fn translate_object_property_domain(axiom : &ObjectPropertyDomain) -> Value {
    let property = expression_transducer::translate_property_expression(&axiom.ope.clone());
    let domain = expression_transducer::translate_class_expression(&axiom.ce.clone());

    let operator = Value::String(String::from("ObjectPropertyDomain"));

    let mut res = vec![operator];
    res.push(property);
    res.push(domain);
    Value::Array(res) 
}

pub fn translate_object_property_range(axiom : &ObjectPropertyRange) -> Value {

    let operator = Value::String(String::from("ObjectPropertyRange"));
    let property = expression_transducer::translate_property_expression(&axiom.ope.clone());
    let domain = expression_transducer::translate_class_expression(&axiom.ce.clone()); 

    let mut res = vec![operator];
    res.push(property);
    res.push(domain);
    Value::Array(res) 
}

pub fn translate_inverse_properties(axiom : &InverseObjectProperties) -> Value {

    let operator = Value::String(String::from("InverseObjectProperties"));
    let lhs = expression_transducer::translate_object_property(&axiom.0.clone());
    let rhs = expression_transducer::translate_object_property(&axiom.1.clone());

    let mut res = vec![operator];
    res.push(lhs);
    res.push(rhs);
    Value::Array(res) 
}

pub fn translate_disjoint_object_properties(axiom : &DisjointObjectProperties) -> Value {
    let operator = Value::String(String::from("DisjointObjectProperties"));

    let arguments = axiom.0.clone();
    let mut operands : Vec<Value> = arguments.into_iter()
                                         .map(|x| expression_transducer::translate_property_expression(&x))
                                         .collect(); 
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_equivalent_object_properties(axiom : &EquivalentObjectProperties) -> Value {
    let operator = Value::String(String::from("EquivalentObjectProperties"));

    let arguments = axiom.0.clone();
    let mut operands : Vec<Value> = arguments.into_iter()
                                         .map(|x| expression_transducer::translate_property_expression(&x))
                                         .collect(); 
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_sub_object_property(axiom : &SubObjectPropertyOf) -> Value {
    let operator = Value::String(String::from("SubObjectPropertyOf"));
    let lhs = expression_transducer::translate_sub_object_property_expression(&axiom.sub);

    let rhs = expression_transducer::translate_property_expression(&axiom.sup); 

    let v = vec![operator, lhs, rhs];
    Value::Array(v) 
}

pub fn wrap_declaration(v : &Value) -> Value {

    let declaration = Value::String(String::from("Declaration")); 
    let res = vec![declaration, v.clone()]; 
    Value::Array(res) 
}

pub fn translate_class_declaration(axiom : &DeclareClass) -> Value {

    let operator = Value::String(String::from("Class"));
    let class = expression_transducer::translate_class(&axiom.0.clone());

    let v = vec![operator, class];
    let v =  Value::Array(v);
    wrap_declaration(&v) 
}

pub fn translate_object_property_declaration(axiom : &DeclareObjectProperty) -> Value {

    let operator = Value::String(String::from("ObjectProperty"));
    let property = expression_transducer::translate_object_property(&axiom.0.clone());

    let v = vec![operator, property];
    let v =  Value::Array(v);
    wrap_declaration(&v) 
}

pub fn translate_data_property_declaration(axiom : &DeclareDataProperty) -> Value {

    let operator = Value::String(String::from("DataProperty"));
    let property = expression_transducer::translate_data_property(&axiom.0.clone());

    let v = vec![operator, property];
    let v =  Value::Array(v);
    wrap_declaration(&v) 
}

pub fn translate_named_individual_declaration(axiom : &DeclareNamedIndividual) -> Value {

    let operator = Value::String(String::from("NamedIndividual"));
    let property = expression_transducer::translate_named_individual(&axiom.0.clone());

    let v = vec![operator, property];
    let v =  Value::Array(v);
    wrap_declaration(&v) 
}

pub fn translate_datatype_declaration(axiom : &DeclareDatatype) -> Value {

    let operator = Value::String(String::from("Datatype"));
    let property = expression_transducer::translate_datatype(&axiom.0.clone());

    let v = vec![operator, property];
    let v =  Value::Array(v);
    wrap_declaration(&v) 
}


