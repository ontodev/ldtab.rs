use crate::owl_2_ofn::annotation_transducer;
use crate::owl_2_ofn::expression_transducer;
use horned_owl::model::{
    AnnotationAssertion, AnnotationPropertyDomain, AnnotationPropertyRange,
    AsymmetricObjectProperty, ClassAssertion, Component, DataPropertyAssertion, DataPropertyDomain,
    DataPropertyRange, DatatypeDefinition, DeclareAnnotationProperty, DeclareClass,
    DeclareDataProperty, DeclareDatatype, DeclareNamedIndividual, DeclareObjectProperty,
    DifferentIndividuals, DisjointClasses, DisjointDataProperties, DisjointObjectProperties,
    DisjointUnion, DocIRI, EquivalentClasses, EquivalentDataProperties, EquivalentObjectProperties,
    FunctionalDataProperty, FunctionalObjectProperty, HasKey, Import,
    InverseFunctionalObjectProperty, InverseObjectProperties, IrreflexiveObjectProperty,
    NegativeDataPropertyAssertion, NegativeObjectPropertyAssertion, ObjectPropertyAssertion,
    ObjectPropertyDomain, ObjectPropertyExpression, ObjectPropertyRange, OntologyAnnotation,
    OntologyID, RcStr, ReflexiveObjectProperty, SameIndividual, SubAnnotationPropertyOf,
    SubClassOf, SubDataPropertyOf, SubObjectPropertyOf, SymmetricObjectProperty,
    TransitiveObjectProperty,
};
use serde_json::json;
use serde_json::Value;

///Translates an OWL axiom into an OFN S-expression
///
///# Examples
/// let builder = Build::new();
/// let sub = b.class("http://example/namespace#subclass").into();
/// let sup = b.class("http://example/namespace#superclass").into();
/// let axiom = SubClassOf{sub : sub,
///                        sup : sup};
/// let ofn = translate(&axiom);
/// println("{}", ofn);
pub fn translate(axiom: &Component<RcStr>) -> Value {
    match axiom {
        Component::OntologyAnnotation(x) => translate_ontology_annotation(x),
        Component::Import(x) => translate_import(x),

        Component::DeclareClass(x) => translate_class_declaration(x),
        Component::DeclareObjectProperty(x) => translate_object_property_declaration(x),
        Component::DeclareAnnotationProperty(x) => translate_declare_annotation_property(x),
        Component::DeclareDataProperty(x) => translate_data_property_declaration(x),
        Component::DeclareNamedIndividual(x) => translate_named_individual_declaration(x),
        Component::DeclareDatatype(x) => translate_datatype_declaration(x),

        Component::SubClassOf(x) => translate_subclass_of(x),
        Component::EquivalentClasses(x) => translate_equivalent_classes(x),
        Component::DisjointClasses(x) => translate_disjoint_classes(x),
        Component::DisjointUnion(x) => translate_disjoint_union(x),

        Component::SubObjectPropertyOf(x) => translate_sub_object_property(x),
        Component::EquivalentObjectProperties(x) => translate_equivalent_object_properties(x),
        Component::DisjointObjectProperties(x) => translate_disjoint_object_properties(x),
        Component::InverseObjectProperties(x) => translate_inverse_properties(x),
        Component::ObjectPropertyDomain(x) => translate_object_property_domain(x),
        Component::ObjectPropertyRange(x) => translate_object_property_range(x),
        Component::FunctionalObjectProperty(x) => translate_functional_object_property(x),
        Component::InverseFunctionalObjectProperty(x) => {
            translate_inverse_functional_object_property(x)
        }
        Component::ReflexiveObjectProperty(x) => translate_reflexive_object_property(x),
        Component::IrreflexiveObjectProperty(x) => translate_irreflexive_object_property(x),
        Component::SymmetricObjectProperty(x) => translate_symmetric_object_property(x),
        Component::AsymmetricObjectProperty(x) => translate_asymmetric_object_property(x),
        Component::TransitiveObjectProperty(x) => translate_transitive_object_property(x),

        Component::SubDataPropertyOf(x) => translate_sub_data_property_of(x),
        Component::EquivalentDataProperties(x) => translate_equivalent_data_properties(x),
        Component::DisjointDataProperties(x) => translate_disjoint_data_properties(x),
        Component::DataPropertyDomain(x) => translate_data_property_domain(x),
        Component::DataPropertyRange(x) => translate_data_property_range(x),
        Component::FunctionalDataProperty(x) => translate_functional_data_property(x),

        Component::DatatypeDefinition(x) => translate_datatype_definition(x),
        Component::HasKey(x) => translate_has_key(x),

        Component::SameIndividual(x) => translate_same_individual(x),
        Component::DifferentIndividuals(x) => translate_different_individuals(x),

        Component::ClassAssertion(x) => translate_class_assertion(x),
        Component::ObjectPropertyAssertion(x) => translate_object_property_assertion(x),
        Component::NegativeObjectPropertyAssertion(x) => {
            translate_negative_object_property_assertion(x)
        }
        Component::DataPropertyAssertion(x) => translate_data_property_assertion(x),
        Component::NegativeDataPropertyAssertion(x) => {
            translate_negative_data_property_assertion(x)
        }

        Component::AnnotationAssertion(x) => translate_annotation_assertion(x),
        Component::SubAnnotationPropertyOf(x) => translate_sub_annotation_property_of(x),
        Component::AnnotationPropertyDomain(x) => translate_annotation_property_domain(x),
        Component::AnnotationPropertyRange(x) => translate_annotation_property_range(x),
        Component::OntologyID(x) => translate_ontology_id(x),
        Component::DocIRI(x) => translate_doc_iri(x),
        Component::Rule(_) => Value::Null, //TODO
    }
}

pub fn translate_subclass_of(axiom: &SubClassOf<RcStr>) -> Value {
    let operator = Value::String(String::from("SubClassOf"));
    let subclass = expression_transducer::translate_class_expression(&axiom.sub);
    let superclass = expression_transducer::translate_class_expression(&axiom.sup);
    let v = vec![operator, subclass, superclass];
    Value::Array(v)
}

pub fn translate_disjoint_union(axiom: &DisjointUnion<RcStr>) -> Value {
    let operator = Value::String(String::from("DisjointUnion"));
    let lhs = axiom.0.clone();
    let lhs = expression_transducer::translate_class(&lhs);
    let arguments = axiom.1.clone();
    let mut operands: Vec<Value> = arguments
        .into_iter()
        .map(|x| expression_transducer::translate_class_expression(&x))
        .collect();

    operands.insert(0, lhs);
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_disjoint_classes(axiom: &DisjointClasses<RcStr>) -> Value {
    let operator = Value::String(String::from("DisjointClasses"));
    let classes = axiom.0.clone();
    let mut operands: Vec<Value> = classes
        .into_iter()
        .map(|x| expression_transducer::translate_class_expression(&x))
        .collect();
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_equivalent_classes(axiom: &EquivalentClasses<RcStr>) -> Value {
    let operator = Value::String(String::from("EquivalentClasses"));
    let classes = axiom.0.clone();
    let mut operands: Vec<Value> = classes
        .into_iter()
        .map(|x| expression_transducer::translate_class_expression(&x))
        .collect();
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_object_property_axiom(
    operator: &str,
    property: &ObjectPropertyExpression<RcStr>,
) -> Value {
    let operator = Value::String(String::from(operator));
    let argument = expression_transducer::translate_object_property_expression(property);

    let mut res = vec![operator];
    res.push(argument);
    Value::Array(res)
}

pub fn translate_reflexive_object_property(axiom: &ReflexiveObjectProperty<RcStr>) -> Value {
    translate_object_property_axiom("ReflexiveObjectProperty", &axiom.0.clone())
}

pub fn translate_irreflexive_object_property(axiom: &IrreflexiveObjectProperty<RcStr>) -> Value {
    translate_object_property_axiom("IrreflexiveObjectProperty", &axiom.0.clone())
}

pub fn translate_symmetric_object_property(axiom: &SymmetricObjectProperty<RcStr>) -> Value {
    translate_object_property_axiom("SymmetricObjectProperty", &axiom.0.clone())
}

pub fn translate_asymmetric_object_property(axiom: &AsymmetricObjectProperty<RcStr>) -> Value {
    translate_object_property_axiom("AsymmetricObjectProperty", &axiom.0.clone())
}

pub fn translate_transitive_object_property(axiom: &TransitiveObjectProperty<RcStr>) -> Value {
    translate_object_property_axiom("TransitiveObjectProperty", &axiom.0.clone())
}

pub fn translate_functional_object_property(axiom: &FunctionalObjectProperty<RcStr>) -> Value {
    translate_object_property_axiom("FunctionalObjectProperty", &axiom.0.clone())
}

pub fn translate_inverse_functional_object_property(
    axiom: &InverseFunctionalObjectProperty<RcStr>,
) -> Value {
    translate_object_property_axiom("InverseFunctionalObjectProperty", &axiom.0.clone())
}

pub fn translate_object_property_domain(axiom: &ObjectPropertyDomain<RcStr>) -> Value {
    let property = expression_transducer::translate_object_property_expression(&axiom.ope.clone());
    let domain = expression_transducer::translate_class_expression(&axiom.ce.clone());

    let operator = Value::String(String::from("ObjectPropertyDomain"));

    let mut res = vec![operator];
    res.push(property);
    res.push(domain);
    Value::Array(res)
}

pub fn translate_object_property_range(axiom: &ObjectPropertyRange<RcStr>) -> Value {
    let operator = Value::String(String::from("ObjectPropertyRange"));
    let property = expression_transducer::translate_object_property_expression(&axiom.ope.clone());
    let domain = expression_transducer::translate_class_expression(&axiom.ce.clone());

    let mut res = vec![operator];
    res.push(property);
    res.push(domain);
    Value::Array(res)
}

pub fn translate_inverse_properties(axiom: &InverseObjectProperties<RcStr>) -> Value {
    let operator = Value::String(String::from("InverseObjectProperties"));
    let lhs = expression_transducer::translate_object_property(&axiom.0.clone());
    let rhs = expression_transducer::translate_object_property(&axiom.1.clone());

    let mut res = vec![operator];
    res.push(lhs);
    res.push(rhs);
    Value::Array(res)
}

pub fn translate_disjoint_object_properties(axiom: &DisjointObjectProperties<RcStr>) -> Value {
    let operator = Value::String(String::from("DisjointObjectProperties"));

    let arguments = axiom.0.clone();
    let mut operands: Vec<Value> = arguments
        .into_iter()
        .map(|x| expression_transducer::translate_object_property_expression(&x))
        .collect();
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_equivalent_object_properties(axiom: &EquivalentObjectProperties<RcStr>) -> Value {
    let operator = Value::String(String::from("EquivalentObjectProperties"));

    let arguments = axiom.0.clone();
    let mut operands: Vec<Value> = arguments
        .into_iter()
        .map(|x| expression_transducer::translate_object_property_expression(&x))
        .collect();
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_sub_object_property(axiom: &SubObjectPropertyOf<RcStr>) -> Value {
    let operator = Value::String(String::from("SubObjectPropertyOf"));
    let lhs = expression_transducer::translate_sub_object_property_expression(&axiom.sub);

    let rhs = expression_transducer::translate_object_property_expression(&axiom.sup);

    let v = vec![operator, lhs, rhs];
    Value::Array(v)
}

pub fn wrap_declaration(v: &Value) -> Value {
    let declaration = Value::String(String::from("Declaration"));
    let res = vec![declaration, v.clone()];
    Value::Array(res)
}

pub fn translate_class_declaration(axiom: &DeclareClass<RcStr>) -> Value {
    let operator = Value::String(String::from("Class"));
    let class = expression_transducer::translate_class(&axiom.0.clone());

    let v = vec![operator, class];
    let v = Value::Array(v);
    wrap_declaration(&v)
}

pub fn translate_object_property_declaration(axiom: &DeclareObjectProperty<RcStr>) -> Value {
    let operator = Value::String(String::from("ObjectProperty"));
    let property = expression_transducer::translate_object_property(&axiom.0.clone());

    let v = vec![operator, property];
    let v = Value::Array(v);
    wrap_declaration(&v)
}

pub fn translate_data_property_declaration(axiom: &DeclareDataProperty<RcStr>) -> Value {
    let operator = Value::String(String::from("DataProperty"));
    let property = expression_transducer::translate_data_property(&axiom.0.clone());

    let v = vec![operator, property];
    let v = Value::Array(v);
    wrap_declaration(&v)
}

pub fn translate_named_individual_declaration(axiom: &DeclareNamedIndividual<RcStr>) -> Value {
    let operator = Value::String(String::from("NamedIndividual"));
    let property = expression_transducer::translate_named_individual(&axiom.0.clone());

    let v = vec![operator, property];
    let v = Value::Array(v);
    wrap_declaration(&v)
}

pub fn translate_datatype_declaration(axiom: &DeclareDatatype<RcStr>) -> Value {
    let operator = Value::String(String::from("Datatype"));
    let property = expression_transducer::translate_datatype(&axiom.0.clone());

    let v = vec![operator, property];
    let v = Value::Array(v);
    wrap_declaration(&v)
}

pub fn translate_import(axiom: &Import<RcStr>) -> Value {
    let operator = Value::String(String::from("Import"));
    let a = json!(axiom.0.get(0..));
    let v = vec![operator, a];
    Value::Array(v)
}

pub fn translate_sub_data_property_of(axiom: &SubDataPropertyOf<RcStr>) -> Value {
    let operator = Value::String(String::from("SubDataPropertyOf"));
    let sub = expression_transducer::translate_data_property(&axiom.sub);
    let sup = expression_transducer::translate_data_property(&axiom.sup);

    let v = vec![operator, sub, sup];
    Value::Array(v)
}

pub fn translate_equivalent_data_properties(axiom: &EquivalentDataProperties<RcStr>) -> Value {
    let operator = Value::String(String::from("EquivalentDataProperties"));
    let arguments = axiom.0.clone();
    let mut operands: Vec<Value> = arguments
        .into_iter()
        .map(|x| expression_transducer::translate_data_property(&x))
        .collect();
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_disjoint_data_properties(axiom: &DisjointDataProperties<RcStr>) -> Value {
    let operator = Value::String(String::from("DisjointDataProperties"));
    let arguments = axiom.0.clone();
    let mut operands: Vec<Value> = arguments
        .into_iter()
        .map(|x| expression_transducer::translate_data_property(&x))
        .collect();
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_data_property_domain(axiom: &DataPropertyDomain<RcStr>) -> Value {
    let operator = Value::String(String::from("DataPropertyDomain"));
    let property = expression_transducer::translate_data_property(&axiom.dp);
    let domain = expression_transducer::translate_class_expression(&axiom.ce);

    let v = vec![operator, property, domain];
    Value::Array(v)
}

pub fn translate_data_property_range(axiom: &DataPropertyRange<RcStr>) -> Value {
    let operator = Value::String(String::from("DataPropertyRange"));
    let property = expression_transducer::translate_data_property(&axiom.dp);
    let range = expression_transducer::translate_data_range(&axiom.dr);

    let v = vec![operator, property, range];
    Value::Array(v)
}

pub fn translate_functional_data_property(axiom: &FunctionalDataProperty<RcStr>) -> Value {
    let operator = Value::String(String::from("FunctionalDataProperty"));
    let property = expression_transducer::translate_data_property(&axiom.0);

    let v = vec![operator, property];
    Value::Array(v)
}

pub fn translate_datatype_definition(axiom: &DatatypeDefinition<RcStr>) -> Value {
    let operator = Value::String(String::from("DatatypeDefinition"));
    let datatype = expression_transducer::translate_datatype(&axiom.kind);
    let range = expression_transducer::translate_data_range(&axiom.range);

    let v = vec![operator, datatype, range];
    Value::Array(v)
}

pub fn translate_has_key(axiom: &HasKey<RcStr>) -> Value {
    let operator = Value::String(String::from("HasKey"));
    let ce = expression_transducer::translate_class_expression(&axiom.ce);
    let properties = axiom.vpe.clone();

    let mut operands: Vec<Value> = properties
        .into_iter()
        .map(|x| expression_transducer::translate_property_expression(&x))
        .collect();

    operands.insert(0, ce);
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_same_individual(axiom: &SameIndividual<RcStr>) -> Value {
    let operator = Value::String(String::from("SameIndividual"));
    let individuals = axiom.0.clone();
    let mut operands: Vec<Value> = individuals
        .into_iter()
        .map(|x| expression_transducer::translate_individual(&x))
        .collect();
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_different_individuals(axiom: &DifferentIndividuals<RcStr>) -> Value {
    let operator = Value::String(String::from("DifferentIndividuals"));
    let individuals = axiom.0.clone();
    let mut operands: Vec<Value> = individuals
        .into_iter()
        .map(|x| expression_transducer::translate_individual(&x))
        .collect();
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_class_assertion(axiom: &ClassAssertion<RcStr>) -> Value {
    let operator = Value::String(String::from("ClassAssertion"));
    let individual = expression_transducer::translate_individual(&axiom.i);
    let class = expression_transducer::translate_class_expression(&axiom.ce);

    let v = vec![operator, class, individual];
    Value::Array(v)
}

pub fn translate_object_property_assertion(axiom: &ObjectPropertyAssertion<RcStr>) -> Value {
    let operator = Value::String(String::from("ObjectPropertyAssertion"));
    let from = expression_transducer::translate_individual(&axiom.from);
    let to = expression_transducer::translate_individual(&axiom.to);
    let property = expression_transducer::translate_object_property_expression(&axiom.ope);

    let v = vec![operator, property, from, to];
    Value::Array(v)
}

pub fn translate_negative_object_property_assertion(
    axiom: &NegativeObjectPropertyAssertion<RcStr>,
) -> Value {
    let operator = Value::String(String::from("NegativeObjectPropertyAssertion"));
    let from = expression_transducer::translate_individual(&axiom.from);
    let to = expression_transducer::translate_individual(&axiom.to);
    let property = expression_transducer::translate_object_property_expression(&axiom.ope);

    let v = vec![operator, property, from, to];
    Value::Array(v)
}

pub fn translate_data_property_assertion(axiom: &DataPropertyAssertion<RcStr>) -> Value {
    let operator = Value::String(String::from("DataPropertyAssertion"));
    let from = expression_transducer::translate_individual(&axiom.from);
    let to = expression_transducer::translate_literal(&axiom.to);
    let property = expression_transducer::translate_data_property(&axiom.dp);

    let v = vec![operator, property, from, to];
    Value::Array(v)
}

pub fn translate_negative_data_property_assertion(
    axiom: &NegativeDataPropertyAssertion<RcStr>,
) -> Value {
    let operator = Value::String(String::from("NegativeDataPropertyAssertion"));
    let from = expression_transducer::translate_individual(&axiom.from);
    let to = expression_transducer::translate_literal(&axiom.to);
    let property = expression_transducer::translate_data_property(&axiom.dp);

    let v = vec![operator, property, from, to];
    Value::Array(v)
}

pub fn translate_annotation_assertion(axiom: &AnnotationAssertion<RcStr>) -> Value {
    let operator = Value::String(String::from("AnnotationAssertion"));
    let subject = annotation_transducer::translate_annotation_subject(&axiom.subject);
    let property = annotation_transducer::translate_annotation_property(&axiom.ann.ap);
    let value = annotation_transducer::translate_annotation_value(&axiom.ann.av);

    let v = vec![operator, property, subject, value];
    Value::Array(v)
}

pub fn translate_ontology_annotation(axiom: &OntologyAnnotation<RcStr>) -> Value {
    let operator = Value::String(String::from("OntologyAnnotation"));
    let annotation = annotation_transducer::translate_annotation(&axiom.0);
    let v = vec![operator, annotation];
    Value::Array(v)
}

pub fn translate_declare_annotation_property(axiom: &DeclareAnnotationProperty<RcStr>) -> Value {
    let operator = Value::String(String::from("AnnotationProperty"));
    let annotation = annotation_transducer::translate_annotation_property(&axiom.0);
    let v = vec![operator, annotation];
    let v = Value::Array(v);
    wrap_declaration(&v)
}

pub fn translate_sub_annotation_property_of(axiom: &SubAnnotationPropertyOf<RcStr>) -> Value {
    let operator = Value::String(String::from("SubAnnotationPropertyOf"));
    let sub = annotation_transducer::translate_annotation_property(&axiom.sub);
    let sup = annotation_transducer::translate_annotation_property(&axiom.sup);
    let v = vec![operator, sub, sup];
    Value::Array(v)
}

pub fn translate_annotation_property_domain(axiom: &AnnotationPropertyDomain<RcStr>) -> Value {
    let operator = Value::String(String::from("AnnotationPropertyDomain"));
    let property = annotation_transducer::translate_annotation_property(&axiom.ap);
    let iri = json!(axiom.iri.get(0..));
    let v = vec![operator, property, iri];
    Value::Array(v)
}

pub fn translate_annotation_property_range(axiom: &AnnotationPropertyRange<RcStr>) -> Value {
    let operator = Value::String(String::from("AnnotationPropertyRange"));
    let property = annotation_transducer::translate_annotation_property(&axiom.ap);
    let iri = json!(axiom.iri.get(0..));
    let v = vec![operator, property, iri];
    Value::Array(v)
}

pub fn translate_doc_iri(axiom: &DocIRI<RcStr>) -> Value {
    let operator = Value::String(String::from("DocIRI")); // this is not specified in OWL
    let iri = json!(axiom.0.get(0..));
    let v = vec![operator, iri];
    Value::Array(v)
}

pub fn translate_ontology_id(axiom: &OntologyID<RcStr>) -> Value {
    let operator = Value::String(String::from("Ontology"));
    let iri = json!(axiom.iri.clone().unwrap().get(0..));
    let v = vec![operator, iri];
    Value::Array(v)
}
