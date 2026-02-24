use crate::owl_2_ofn::annotation_transducer;
use crate::owl_2_ofn::expression_transducer;
use crate::owl_2_ofn::util::{format_iri, ofn_list};
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
    OntologyID, ArcStr, ReflexiveObjectProperty, Rule, SameIndividual, SubAnnotationPropertyOf,
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
pub fn translate(axiom: &Component<ArcStr>) -> Value {
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
        Component::Rule(x) => translate_rule(x),
    }
}

pub fn translate_subclass_of(axiom: &SubClassOf<ArcStr>) -> Value {
    let subclass = expression_transducer::translate_class_expression(&axiom.sub);
    let superclass = expression_transducer::translate_class_expression(&axiom.sup);
    ofn_list("SubClassOf", vec![subclass, superclass])
}

pub fn translate_disjoint_union(axiom: &DisjointUnion<ArcStr>) -> Value {
    let lhs = expression_transducer::translate_class(&axiom.0);
    let mut operands: Vec<Value> = axiom.1.iter()
        .map(|x| expression_transducer::translate_class_expression(x))
        .collect();
    operands.insert(0, lhs);
    ofn_list("DisjointUnion", operands)
}

pub fn translate_disjoint_classes(axiom: &DisjointClasses<ArcStr>) -> Value {
    let operands: Vec<Value> = axiom.0.iter()
        .map(|x| expression_transducer::translate_class_expression(x))
        .collect();
    ofn_list("DisjointClasses", operands)
}

pub fn translate_equivalent_classes(axiom: &EquivalentClasses<ArcStr>) -> Value {
    let operands: Vec<Value> = axiom.0.iter()
        .map(|x| expression_transducer::translate_class_expression(x))
        .collect();
    ofn_list("EquivalentClasses", operands)
}

pub fn translate_object_property_axiom(
    operator: &str,
    property: &ObjectPropertyExpression<ArcStr>,
) -> Value {
    let argument = expression_transducer::translate_object_property_expression(property);
    ofn_list(operator, vec![argument])
}

pub fn translate_reflexive_object_property(axiom: &ReflexiveObjectProperty<ArcStr>) -> Value {
    translate_object_property_axiom("ReflexiveObjectProperty", &axiom.0)
}

pub fn translate_irreflexive_object_property(axiom: &IrreflexiveObjectProperty<ArcStr>) -> Value {
    translate_object_property_axiom("IrreflexiveObjectProperty", &axiom.0)
}

pub fn translate_symmetric_object_property(axiom: &SymmetricObjectProperty<ArcStr>) -> Value {
    translate_object_property_axiom("SymmetricObjectProperty", &axiom.0)
}

pub fn translate_asymmetric_object_property(axiom: &AsymmetricObjectProperty<ArcStr>) -> Value {
    translate_object_property_axiom("AsymmetricObjectProperty", &axiom.0)
}

pub fn translate_transitive_object_property(axiom: &TransitiveObjectProperty<ArcStr>) -> Value {
    translate_object_property_axiom("TransitiveObjectProperty", &axiom.0)
}

pub fn translate_functional_object_property(axiom: &FunctionalObjectProperty<ArcStr>) -> Value {
    translate_object_property_axiom("FunctionalObjectProperty", &axiom.0)
}

pub fn translate_inverse_functional_object_property(
    axiom: &InverseFunctionalObjectProperty<ArcStr>,
) -> Value {
    translate_object_property_axiom("InverseFunctionalObjectProperty", &axiom.0)
}

pub fn translate_object_property_domain(axiom: &ObjectPropertyDomain<ArcStr>) -> Value {
    let property = expression_transducer::translate_object_property_expression(&axiom.ope);
    let domain = expression_transducer::translate_class_expression(&axiom.ce);
    ofn_list("ObjectPropertyDomain", vec![property, domain])
}

pub fn translate_object_property_range(axiom: &ObjectPropertyRange<ArcStr>) -> Value {
    let property = expression_transducer::translate_object_property_expression(&axiom.ope);
    let range = expression_transducer::translate_class_expression(&axiom.ce);
    ofn_list("ObjectPropertyRange", vec![property, range])
}

pub fn translate_inverse_properties(axiom: &InverseObjectProperties<ArcStr>) -> Value {
    let lhs = expression_transducer::translate_object_property(&axiom.0);
    let rhs = expression_transducer::translate_object_property(&axiom.1);
    ofn_list("InverseObjectProperties", vec![lhs, rhs])
}

pub fn translate_disjoint_object_properties(axiom: &DisjointObjectProperties<ArcStr>) -> Value {
    let operands: Vec<Value> = axiom.0.iter()
        .map(|x| expression_transducer::translate_object_property_expression(x))
        .collect();
    ofn_list("DisjointObjectProperties", operands)
}

pub fn translate_equivalent_object_properties(axiom: &EquivalentObjectProperties<ArcStr>) -> Value {
    let operands: Vec<Value> = axiom.0.iter()
        .map(|x| expression_transducer::translate_object_property_expression(x))
        .collect();
    ofn_list("EquivalentObjectProperties", operands)
}

pub fn translate_sub_object_property(axiom: &SubObjectPropertyOf<ArcStr>) -> Value {
    let lhs = expression_transducer::translate_sub_object_property_expression(&axiom.sub);
    let rhs = expression_transducer::translate_object_property_expression(&axiom.sup);
    ofn_list("SubObjectPropertyOf", vec![lhs, rhs])
}

pub fn wrap_declaration(v: Value) -> Value {
    ofn_list("Declaration", vec![v])
}

pub fn translate_class_declaration(axiom: &DeclareClass<ArcStr>) -> Value {
    let class = expression_transducer::translate_class(&axiom.0);
    wrap_declaration(ofn_list("Class", vec![class]))
}

pub fn translate_object_property_declaration(axiom: &DeclareObjectProperty<ArcStr>) -> Value {
    let property = expression_transducer::translate_object_property(&axiom.0);
    wrap_declaration(ofn_list("ObjectProperty", vec![property]))
}

pub fn translate_data_property_declaration(axiom: &DeclareDataProperty<ArcStr>) -> Value {
    let property = expression_transducer::translate_data_property(&axiom.0);
    wrap_declaration(ofn_list("DataProperty", vec![property]))
}

pub fn translate_named_individual_declaration(axiom: &DeclareNamedIndividual<ArcStr>) -> Value {
    let individual = expression_transducer::translate_named_individual(&axiom.0);
    wrap_declaration(ofn_list("NamedIndividual", vec![individual]))
}

pub fn translate_datatype_declaration(axiom: &DeclareDatatype<ArcStr>) -> Value {
    let datatype = expression_transducer::translate_datatype(&axiom.0);
    wrap_declaration(ofn_list("Datatype", vec![datatype]))
}

pub fn translate_import(axiom: &Import<ArcStr>) -> Value {
    let a = json!(axiom.0.get(0..));
    ofn_list("Import", vec![a])
}

pub fn translate_sub_data_property_of(axiom: &SubDataPropertyOf<ArcStr>) -> Value {
    let sub = expression_transducer::translate_data_property(&axiom.sub);
    let sup = expression_transducer::translate_data_property(&axiom.sup);
    ofn_list("SubDataPropertyOf", vec![sub, sup])
}

pub fn translate_equivalent_data_properties(axiom: &EquivalentDataProperties<ArcStr>) -> Value {
    let operands: Vec<Value> = axiom.0.iter()
        .map(|x| expression_transducer::translate_data_property(x))
        .collect();
    ofn_list("EquivalentDataProperties", operands)
}

pub fn translate_disjoint_data_properties(axiom: &DisjointDataProperties<ArcStr>) -> Value {
    let operands: Vec<Value> = axiom.0.iter()
        .map(|x| expression_transducer::translate_data_property(x))
        .collect();
    ofn_list("DisjointDataProperties", operands)
}

pub fn translate_data_property_domain(axiom: &DataPropertyDomain<ArcStr>) -> Value {
    let property = expression_transducer::translate_data_property(&axiom.dp);
    let domain = expression_transducer::translate_class_expression(&axiom.ce);
    ofn_list("DataPropertyDomain", vec![property, domain])
}

pub fn translate_data_property_range(axiom: &DataPropertyRange<ArcStr>) -> Value {
    let property = expression_transducer::translate_data_property(&axiom.dp);
    let range = expression_transducer::translate_data_range(&axiom.dr);
    ofn_list("DataPropertyRange", vec![property, range])
}

pub fn translate_functional_data_property(axiom: &FunctionalDataProperty<ArcStr>) -> Value {
    let property = expression_transducer::translate_data_property(&axiom.0);
    ofn_list("FunctionalDataProperty", vec![property])
}

pub fn translate_datatype_definition(axiom: &DatatypeDefinition<ArcStr>) -> Value {
    let datatype = expression_transducer::translate_datatype(&axiom.kind);
    let range = expression_transducer::translate_data_range(&axiom.range);
    ofn_list("DatatypeDefinition", vec![datatype, range])
}

pub fn translate_has_key(axiom: &HasKey<ArcStr>) -> Value {
    let operator = Value::String(String::from("HasKey"));
    let ce = expression_transducer::translate_class_expression(&axiom.ce);

    //NB: horned owl doesn't distinguish between object and data properties in hasKey
    let operands: Vec<Value> = axiom.vpe.iter()
        .map(|x| expression_transducer::translate_property_expression(x))
        .collect();

    // empty vector for third HasKey argument (datatype properties) 
    let dummy = Vec::new();

    let mut res = Vec::new();
    res.push(operator);
    res.push(ce);
    res.push(Value::Array(operands));
    res.push(Value::Array(dummy)); // no annotation support yet

    Value::Array(res)
}

pub fn translate_same_individual(axiom: &SameIndividual<ArcStr>) -> Value {
    let operands: Vec<Value> = axiom.0.iter()
        .map(|x| expression_transducer::translate_individual(x))
        .collect();
    ofn_list("SameIndividual", operands)
}

pub fn translate_different_individuals(axiom: &DifferentIndividuals<ArcStr>) -> Value {
    let operands: Vec<Value> = axiom.0.iter()
        .map(|x| expression_transducer::translate_individual(x))
        .collect();
    ofn_list("DifferentIndividuals", operands)
}

pub fn translate_class_assertion(axiom: &ClassAssertion<ArcStr>) -> Value {
    let individual = expression_transducer::translate_individual(&axiom.i);
    let class = expression_transducer::translate_class_expression(&axiom.ce);
    ofn_list("ClassAssertion", vec![class, individual])
}

pub fn translate_object_property_assertion(axiom: &ObjectPropertyAssertion<ArcStr>) -> Value {
    let from = expression_transducer::translate_individual(&axiom.from);
    let to = expression_transducer::translate_individual(&axiom.to);
    let property = expression_transducer::translate_object_property_expression(&axiom.ope);
    ofn_list("ObjectPropertyAssertion", vec![property, from, to])
}

pub fn translate_negative_object_property_assertion(
    axiom: &NegativeObjectPropertyAssertion<ArcStr>,
) -> Value {
    let from = expression_transducer::translate_individual(&axiom.from);
    let to = expression_transducer::translate_individual(&axiom.to);
    let property = expression_transducer::translate_object_property_expression(&axiom.ope);
    ofn_list("NegativeObjectPropertyAssertion", vec![property, from, to])
}

pub fn translate_data_property_assertion(axiom: &DataPropertyAssertion<ArcStr>) -> Value {
    let from = expression_transducer::translate_individual(&axiom.from);
    let to = expression_transducer::translate_literal(&axiom.to);
    let property = expression_transducer::translate_data_property(&axiom.dp);
    ofn_list("DataPropertyAssertion", vec![property, from, to])
}

pub fn translate_negative_data_property_assertion(
    axiom: &NegativeDataPropertyAssertion<ArcStr>,
) -> Value {
    let from = expression_transducer::translate_individual(&axiom.from);
    let to = expression_transducer::translate_literal(&axiom.to);
    let property = expression_transducer::translate_data_property(&axiom.dp);
    ofn_list("NegativeDataPropertyAssertion", vec![property, from, to])
}

pub fn translate_annotation_assertion(axiom: &AnnotationAssertion<ArcStr>) -> Value {
    let subject = annotation_transducer::translate_annotation_subject(&axiom.subject);
    let property = annotation_transducer::translate_annotation_property(&axiom.ann.ap);
    let value = annotation_transducer::translate_annotation_value(&axiom.ann.av);
    ofn_list("AnnotationAssertion", vec![property, subject, value])
}

pub fn translate_ontology_annotation(axiom: &OntologyAnnotation<ArcStr>) -> Value {
    let annotation = annotation_transducer::translate_annotation(&axiom.0);
    ofn_list("OntologyAnnotation", vec![annotation])
}

pub fn translate_declare_annotation_property(axiom: &DeclareAnnotationProperty<ArcStr>) -> Value {
    let annotation = annotation_transducer::translate_annotation_property(&axiom.0);
    wrap_declaration(ofn_list("AnnotationProperty", vec![annotation]))
}

pub fn translate_sub_annotation_property_of(axiom: &SubAnnotationPropertyOf<ArcStr>) -> Value {
    let sub = annotation_transducer::translate_annotation_property(&axiom.sub);
    let sup = annotation_transducer::translate_annotation_property(&axiom.sup);
    ofn_list("SubAnnotationPropertyOf", vec![sub, sup])
}

pub fn translate_annotation_property_domain(axiom: &AnnotationPropertyDomain<ArcStr>) -> Value {
    let property = annotation_transducer::translate_annotation_property(&axiom.ap);
    let iri = format_iri(axiom.iri.get(0..).unwrap());
    ofn_list("AnnotationPropertyDomain", vec![property, iri])
}

pub fn translate_annotation_property_range(axiom: &AnnotationPropertyRange<ArcStr>) -> Value {
    let property = annotation_transducer::translate_annotation_property(&axiom.ap);
    let iri = format_iri(axiom.iri.get(0..).unwrap());
    ofn_list("AnnotationPropertyRange", vec![property, iri])
}

pub fn translate_doc_iri(axiom: &DocIRI<ArcStr>) -> Value {
    let iri = format_iri(axiom.0.get(0..).unwrap());
    ofn_list("DocIRI", vec![iri])
}

pub fn translate_rule(axiom: &Rule<ArcStr>) -> Value {
    let body_operands: Vec<Value> = axiom.body.iter()
        .map(|atom| expression_transducer::translate_atom(atom))
        .collect();
    let head_operands: Vec<Value> = axiom.head.iter()
        .map(|atom| expression_transducer::translate_atom(atom))
        .collect();
    ofn_list("DLSafeRule", vec![
        ofn_list("Body", body_operands),
        ofn_list("Head", head_operands),
    ])
}

pub fn translate_ontology_id(axiom: &OntologyID<ArcStr>) -> Value {
    let iri = format_iri(axiom.iri.as_deref().unwrap_or("unknown"));
    let viri = format_iri(axiom.viri.as_deref().unwrap_or("unknown"));
    ofn_list("Ontology", vec![iri, viri])
}
