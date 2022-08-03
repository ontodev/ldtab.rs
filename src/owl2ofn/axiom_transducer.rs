use serde_json::{Value};
use serde_json::json; 
use crate::owl2ofn::expression_transducer as expression_transducer;
use crate::owl2ofn::annotation_transducer as annotation_transducer;
use horned_owl::model::{Axiom, SubClassOf, ClassAssertion, DeclareClass, DeclareObjectProperty, DeclareDatatype, DeclareDataProperty, DeclareNamedIndividual, DisjointClasses, DisjointUnion, EquivalentClasses, EquivalentObjectProperties, ObjectPropertyDomain, ObjectPropertyExpression, SubObjectPropertyOf, TransitiveObjectProperty, ObjectPropertyAssertion, ReflexiveObjectProperty, IrreflexiveObjectProperty, SymmetricObjectProperty, AsymmetricObjectProperty, ObjectPropertyRange, InverseObjectProperties, FunctionalObjectProperty, InverseFunctionalObjectProperty, DisjointObjectProperties, Import, SubDataPropertyOf, EquivalentDataProperties, DisjointDataProperties, DataPropertyDomain, DataPropertyRange, FunctionalDataProperty, DatatypeDefinition, HasKey, SameIndividual, DifferentIndividuals, NegativeObjectPropertyAssertion, DataPropertyAssertion, NegativeDataPropertyAssertion, AnnotationAssertion, OntologyAnnotation, DeclareAnnotationProperty, SubAnnotationPropertyOf, AnnotationPropertyDomain, AnnotationPropertyRange};


pub fn translate(axiom : &Axiom) -> Value {

    match axiom {
        Axiom::OntologyAnnotation(x) => translate_ontology_annotation(x),
        Axiom::Import(x) => translate_import(x),

        Axiom::DeclareClass(x) => translate_class_declaration(x),
        Axiom::DeclareObjectProperty(x) => translate_object_property_declaration(x),
        Axiom::DeclareAnnotationProperty(x) => translate_declare_annotation_property(x),
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

        Axiom::SubDataPropertyOf(x) => translate_sub_data_property_of(x),
        Axiom::EquivalentDataProperties(x) => translate_equivalent_data_properties(x),
        Axiom::DisjointDataProperties(x) => translate_disjoint_data_properties(x),
        Axiom::DataPropertyDomain(x) => translate_data_property_domain(x),
        Axiom::DataPropertyRange(x) => translate_data_property_range(x),
        Axiom::FunctionalDataProperty(x) => translate_functional_data_property(x),

        Axiom::DatatypeDefinition(x) => translate_datatype_definition(x),
        Axiom::HasKey(x) => translate_has_key(x),

        Axiom::SameIndividual(x) => translate_same_individual(x),
        Axiom::DifferentIndividuals(x) => translate_different_individuals(x),

        Axiom::ClassAssertion(x) => translate_class_assertion(x),
        Axiom::ObjectPropertyAssertion(x) => translate_object_property_assertion(x),
        Axiom::NegativeObjectPropertyAssertion(x) => translate_negative_object_property_assertion(x),
        Axiom::DataPropertyAssertion(x) => translate_data_property_assertion(x),
        Axiom::NegativeDataPropertyAssertion(x) => translate_negative_data_property_assertion(x),

        Axiom::AnnotationAssertion(x) => translate_annotation_assertion(x),
        Axiom::SubAnnotationPropertyOf(x) => translate_sub_annotation_property_of(x), 
        Axiom::AnnotationPropertyDomain(x) => translate_annotation_property_domain(x),
        Axiom::AnnotationPropertyRange(x) => translate_annotation_property_range(x), 
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
    let argument = expression_transducer::translate_object_property_expression(property);

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
    let property = expression_transducer::translate_object_property_expression(&axiom.ope.clone());
    let domain = expression_transducer::translate_class_expression(&axiom.ce.clone());

    let operator = Value::String(String::from("ObjectPropertyDomain"));

    let mut res = vec![operator];
    res.push(property);
    res.push(domain);
    Value::Array(res) 
}

pub fn translate_object_property_range(axiom : &ObjectPropertyRange) -> Value {

    let operator = Value::String(String::from("ObjectPropertyRange"));
    let property = expression_transducer::translate_object_property_expression(&axiom.ope.clone());
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
                                         .map(|x| expression_transducer::translate_object_property_expression(&x))
                                         .collect(); 
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_equivalent_object_properties(axiom : &EquivalentObjectProperties) -> Value {
    let operator = Value::String(String::from("EquivalentObjectProperties"));

    let arguments = axiom.0.clone();
    let mut operands : Vec<Value> = arguments.into_iter()
                                         .map(|x| expression_transducer::translate_object_property_expression(&x))
                                         .collect(); 
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_sub_object_property(axiom : &SubObjectPropertyOf) -> Value {
    let operator = Value::String(String::from("SubObjectPropertyOf"));
    let lhs = expression_transducer::translate_sub_object_property_expression(&axiom.sub);

    let rhs = expression_transducer::translate_object_property_expression(&axiom.sup); 

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

pub fn translate_import(axiom : &Import) -> Value {
    let operator = Value::String(String::from("Import"));
    let a = json!(axiom.0.get(0..));
    let v = vec![operator, a];
    Value::Array(v)
}

pub fn translate_sub_data_property_of(axiom : &SubDataPropertyOf) -> Value {

    let operator = Value::String(String::from("SubDataPropertyOf"));
    let sub = expression_transducer::translate_data_property(&axiom.sub);
    let sup = expression_transducer::translate_data_property(&axiom.sup);

    let v = vec![operator, sub, sup];
    Value::Array(v) 
}

pub fn translate_equivalent_data_properties(axiom : &EquivalentDataProperties) -> Value {

    let operator = Value::String(String::from("EquivalentDataProperties"));
    let arguments = axiom.0.clone();
    let mut operands : Vec<Value> = arguments.into_iter()
                                         .map(|x| expression_transducer::translate_data_property(&x))
                                         .collect(); 
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_disjoint_data_properties(axiom : &DisjointDataProperties) -> Value {

    let operator = Value::String(String::from("DisjointDataProperties"));
    let arguments = axiom.0.clone();
    let mut operands : Vec<Value> = arguments.into_iter()
                                         .map(|x| expression_transducer::translate_data_property(&x))
                                         .collect(); 
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_data_property_domain(axiom : &DataPropertyDomain) -> Value {

    let operator = Value::String(String::from("DataPropertyDomain"));
    let property = expression_transducer::translate_data_property(&axiom.dp);
    let domain = expression_transducer::translate_class_expression(&axiom.ce);

    let v = vec![operator, property, domain];
    Value::Array(v) 
}

pub fn translate_data_property_range(axiom : &DataPropertyRange) -> Value {

    let operator = Value::String(String::from("DataPropertyRange"));
    let property = expression_transducer::translate_data_property(&axiom.dp);
    let range = expression_transducer::translate_data_range(&axiom.dr);

    let v = vec![operator, property, range];
    Value::Array(v) 
}

pub fn translate_functional_data_property(axiom : &FunctionalDataProperty) -> Value {

    let operator = Value::String(String::from("FunctionalDataProperty"));
    let property = expression_transducer::translate_data_property(&axiom.0);

    let v = vec![operator, property];
    Value::Array(v) 
}

pub fn translate_datatype_definition(axiom : &DatatypeDefinition) -> Value {

    let operator = Value::String(String::from("DatatypeDefinition"));
    let datatype = expression_transducer::translate_datatype(&axiom.kind);
    let range = expression_transducer::translate_data_range(&axiom.range);

    let v = vec![operator, datatype, range];
    Value::Array(v) 
}

pub fn translate_has_key(axiom : &HasKey) -> Value {
    let operator = Value::String(String::from("HasKey"));
    let ce = expression_transducer::translate_class_expression(&axiom.ce);
    let properties = axiom.vpe.clone();

    let mut operands : Vec<Value> = properties.into_iter()
                                         .map(|x| expression_transducer::translate_property_expression(&x))
                                         .collect(); 

    operands.insert(0,ce);
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_same_individual(axiom : &SameIndividual) -> Value {
    let operator = Value::String(String::from("SameIndividual"));
    let individuals = axiom.0.clone();
    let mut operands : Vec<Value> = individuals.into_iter()
                                         .map(|x| expression_transducer::translate_individual(&x))
                                         .collect(); 
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_different_individuals(axiom : &DifferentIndividuals) -> Value {
    let operator = Value::String(String::from("DifferentIndividuals"));
    let individuals = axiom.0.clone();
    let mut operands : Vec<Value> = individuals.into_iter()
                                         .map(|x| expression_transducer::translate_individual(&x))
                                         .collect(); 
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_class_assertion(axiom : &ClassAssertion) -> Value {
    let operator = Value::String(String::from("ClassAssertion"));
    let individual = expression_transducer::translate_individual(&axiom.i);
    let class = expression_transducer::translate_class_expression(&axiom.ce);

    let v = vec![operator, class, individual];
    Value::Array(v) 
}

pub fn translate_object_property_assertion(axiom : &ObjectPropertyAssertion) -> Value {
    let operator = Value::String(String::from("ObjectPropertyAssertion"));
    let from = expression_transducer::translate_individual(&axiom.from);
    let to = expression_transducer::translate_individual(&axiom.to);
    let property = expression_transducer::translate_object_property_expression(&axiom.ope);

    let v = vec![operator, property, from, to];
    Value::Array(v) 
}

pub fn translate_negative_object_property_assertion(axiom : &NegativeObjectPropertyAssertion) -> Value {
    let operator = Value::String(String::from("NegativeObjectPropertyAssertion"));
    let from = expression_transducer::translate_individual(&axiom.from);
    let to = expression_transducer::translate_individual(&axiom.to);
    let property = expression_transducer::translate_object_property_expression(&axiom.ope);

    let v = vec![operator, property, from, to];
    Value::Array(v) 
}

pub fn translate_data_property_assertion(axiom : &DataPropertyAssertion) -> Value {
    let operator = Value::String(String::from("DataPropertyAssertion"));
    let from = expression_transducer::translate_individual(&axiom.from);
    let to = expression_transducer::translate_literal(&axiom.to);
    let property = expression_transducer::translate_data_property(&axiom.dp);

    let v = vec![operator, property, from, to];
    Value::Array(v) 
}

pub fn translate_negative_data_property_assertion(axiom : &NegativeDataPropertyAssertion) -> Value {
    let operator = Value::String(String::from("NegativeDataPropertyAssertion"));
    let from = expression_transducer::translate_individual(&axiom.from);
    let to = expression_transducer::translate_literal(&axiom.to);
    let property = expression_transducer::translate_data_property(&axiom.dp);

    let v = vec![operator, property, from, to];
    Value::Array(v) 
}

pub fn translate_annotation_assertion(axiom : &AnnotationAssertion) -> Value {

    let operator = Value::String(String::from("AnnotationAssertion"));
    let subject = annotation_transducer::translate_annotation_subject(&axiom.subject);
    let annotation = annotation_transducer::translate_annotation(&axiom.ann);

    let v = vec![operator, subject, annotation];
    Value::Array(v) 
}

pub fn translate_ontology_annotation(axiom : &OntologyAnnotation) -> Value {
    let operator = Value::String(String::from("OntologyAnnotation"));
    let annotation = annotation_transducer::translate_annotation(&axiom.0);
    let v = vec![operator, annotation];
    Value::Array(v) 
}

pub fn translate_declare_annotation_property(axiom : &DeclareAnnotationProperty) -> Value {
    let operator = Value::String(String::from("AnnotationProperty"));
    let annotation = annotation_transducer::translate_annotation_property(&axiom.0);
    let v = vec![operator, annotation];
    let v =  Value::Array(v);
    wrap_declaration(&v) 
}

pub fn translate_sub_annotation_property_of(axiom : &SubAnnotationPropertyOf) -> Value {
    let operator = Value::String(String::from("SubAnnotationPropertyOf"));
    let sub = annotation_transducer::translate_annotation_property(&axiom.sub);
    let sup = annotation_transducer::translate_annotation_property(&axiom.sup);
    let v = vec![operator, sub, sup];
    Value::Array(v) 
}

pub fn translate_annotation_property_domain(axiom : &AnnotationPropertyDomain) -> Value {
    let operator = Value::String(String::from("AnnotationPropertyDomain"));
    let property = annotation_transducer::translate_annotation_property(&axiom.ap);
    let iri = json!(axiom.iri.get(0..));
    let v = vec![operator, property, iri];
    Value::Array(v) 
}

pub fn translate_annotation_property_range(axiom : &AnnotationPropertyRange) -> Value {
    let operator = Value::String(String::from("AnnotationPropertyRange"));
    let property = annotation_transducer::translate_annotation_property(&axiom.ap);
    let iri = json!(axiom.iri.get(0..));
    let v = vec![operator, property, iri];
    Value::Array(v) 
}



