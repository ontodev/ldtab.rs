use crate::ofn_2_owl::annotation_transducer;
use crate::ofn_2_owl::expression_transducer;
use crate::ofn_2_owl::util::{build, extract_iri_str};
use horned_owl::model::{
    Annotation, AnnotationAssertion, AnnotationProperty, AnnotationPropertyDomain,
    AnnotationPropertyRange, AsymmetricObjectProperty, Class, ClassAssertion,
    ClassExpression, Component, DataProperty, DataPropertyAssertion, DataPropertyDomain,
    DataPropertyRange, DatatypeDefinition, DeclareAnnotationProperty, DeclareClass,
    DeclareDataProperty, DeclareDatatype, DeclareNamedIndividual, DeclareObjectProperty,
    DifferentIndividuals, DisjointClasses, DisjointDataProperties, DisjointObjectProperties,
    DisjointUnion, EquivalentClasses, EquivalentDataProperties, EquivalentObjectProperties,
    FunctionalDataProperty, FunctionalObjectProperty, Import, Individual,
    InverseFunctionalObjectProperty, InverseObjectProperties, IrreflexiveObjectProperty,
    NamedIndividual, NegativeDataPropertyAssertion, NegativeObjectPropertyAssertion,
    ObjectProperty, ObjectPropertyAssertion, ObjectPropertyDomain, ObjectPropertyExpression,
    ObjectPropertyRange, RcStr, ReflexiveObjectProperty, SameIndividual, SubAnnotationPropertyOf,
    SubClassOf, SubDataPropertyOf, SubObjectPropertyOf, SymmetricObjectProperty,
    TransitiveObjectProperty,
};
use serde_json::Value;

pub fn translate_axiom(v: &Value) -> Component<RcStr> {
    match v[0].as_str() {
        //Ontology annotation
        //import
        Some("Declaration") => translate_declaration(v),
        Some("SubClassOf") => translate_subclass_of(v),
        Some("EquivalentClasses") => translate_equivalent_classes(v),
        Some("DisjointClasses") => translate_disjoint_classes(v),
        Some("DisjointUnion") => translate_disjoint_union(v),
        Some("SubObjectPropertyOf") => translate_sub_object_property_of(v),
        Some("EquivalentObjectProperties") => translate_equivalent_object_properties(v),
        Some("DisjointObjectProperties") => translate_disjoint_object_properties(v),
        Some("InverseObjectProperties") => translate_inverse_object_properties(v),
        Some("ObjectPropertyDomain") => translate_object_property_domain(v),
        Some("ObjectPropertyRange") => translate_object_property_range(v),
        Some("FunctionalObjectProperty") => translate_functional_object_property(v),
        Some("InverseFunctionalObjectProperty") => translate_inverse_functional_object_property(v),
        Some("ReflexiveObjectProperty") => translate_reflexive_object_property(v),
        Some("IrreflexiveObjectProperty") => translate_irreflexive_object_property(v),
        Some("SymmetricObjectProperty") => translate_symmetric_object_property(v),
        Some("AsymmetricObjectProperty") => translate_asymmetric_object_property(v),
        Some("TransitiveObjectProperty") => translate_transitive_object_property(v),

        Some("SubDataPropertyOf") => translate_sub_dataproperty_of(v),
        Some("EquivalentDataProperties") => translate_equivalent_data_properties(v),
        Some("DisjointDataProperties") => translate_disjoint_data_properties(v),
        Some("DataPropertyDomain") => translate_data_property_domain(v),
        Some("DataPropertyRange") => translate_data_property_range(v),
        Some("FunctionalDataProperty") => translate_functional_data_property(v),

        Some("DatatypeDefinition") => translate_datatype_definition(v),
        //Some("HasKey") => translate_has_key(v), //no property type support in OFN S
        Some("HasKey") => panic!("HasKey operator currently not supported"),

        Some("SameIndividual") => translate_same_individual(v),
        Some("DifferentIndividuals") => translate_different_individuals(v),

        Some("ClassAssertion") => translate_class_assertion(v),
        Some("ObjectPropertyAssertion") => translate_object_property_assertion(v),
        Some("NegativeObjectPropertyAssertion") => translate_negative_object_property_assertion(v),
        Some("DataPropertyAssertion") => translate_data_property_assertion(v),
        Some("NegativeDataPropertyAssertion") => translate_negative_data_property_assertion(v),

        Some("AnnotationAssertion") => translate_annotation_assertion(v),
        Some("SubAnnotationPropertyOf") => translate_sub_annotation_assertion(v),
        Some("AnnotationPropertyDomain") => translate_annotation_property_domain(v),
        Some("AnnotationPropertyRange") => translate_annotation_property_range(v),
        Some("Import") => translate_import(v),

        Some(_) => panic!("Not a valid OWL axiom operator"),
        None => panic!("Not a valid (typed) OFN S-expression"),
    }
}

pub fn translate_named_class(v: &Value) -> Class<RcStr> {
    build().class(extract_iri_str(v)).into()
}

pub fn translate_import(v: &Value) -> Component<RcStr> {
    let import = build().iri(extract_iri_str(&v[2])).into();

    let axiom = Import(import);
    Component::Import(axiom)
}

pub fn translate_object_property(v: &Value) -> ObjectProperty<RcStr> {
    build().object_property(extract_iri_str(v)).into()
}

pub fn translate_annotation_property(v: &Value) -> AnnotationProperty<RcStr> {
    build().annotation_property(extract_iri_str(v)).into()
}

pub fn translate_named_individual(v: &Value) -> NamedIndividual<RcStr> {
    build().named_individual(extract_iri_str(v)).into()
}

//TODO refactor this into expression_transducer

pub fn translate_subclass_of(v: &Value) -> Component<RcStr> {
    let sub = expression_transducer::translate_class_expression(&v[1]);
    let sup = expression_transducer::translate_class_expression(&v[2]);
    let axiom = SubClassOf { sub: sub, sup: sup };
    Component::SubClassOf(axiom)
}

pub fn translate_equivalent_classes(v: &Value) -> Component<RcStr> {
    let operands: Vec<ClassExpression<RcStr>> = (&(v.as_array().unwrap())[1..])
        .into_iter()
        .map(|x| expression_transducer::translate_class_expression(&x))
        .collect();
    let axiom = EquivalentClasses { 0: operands };
    Component::EquivalentClasses(axiom)
}

pub fn translate_disjoint_classes(v: &Value) -> Component<RcStr> {
    let operands: Vec<ClassExpression<RcStr>> = (&(v.as_array().unwrap())[1..])
        .into_iter()
        .map(|x| expression_transducer::translate_class_expression(&x))
        .collect();
    let axiom = DisjointClasses { 0: operands };
    Component::DisjointClasses(axiom)
}

pub fn translate_disjoint_union(v: &Value) -> Component<RcStr> {
    //NB: we need a (named) class here - not a class expression
    //let lhs = expression_transducer::translate_class_expression(&v[1]);
    let lhs = translate_named_class(&v[1]);

    let operands: Vec<ClassExpression<RcStr>> = (&(v.as_array().unwrap())[2..])
        .into_iter()
        .map(|x| expression_transducer::translate_class_expression(&x))
        .collect();
    let axiom = DisjointUnion {
        0: lhs,
        1: operands,
    };
    Component::DisjointUnion(axiom)
}

pub fn translate_class_declaration(v: &Value) -> Component<RcStr> {
    let class = translate_named_class(&v[1]);
    let axiom = DeclareClass { 0: class };
    Component::DeclareClass(axiom)
}

pub fn translate_object_property_declaration(v: &Value) -> Component<RcStr> {
    let property = translate_object_property(&v[1]);
    let axiom = DeclareObjectProperty { 0: property };
    Component::DeclareObjectProperty(axiom)
}

pub fn translate_data_property_declaration(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_data_property(&v[1]);
    let axiom = DeclareDataProperty { 0: property };
    Component::DeclareDataProperty(axiom)
}

pub fn translate_annotation_property_declaration(v: &Value) -> Component<RcStr> {
    let property = translate_annotation_property(&v[1]);
    let axiom = DeclareAnnotationProperty { 0: property };
    Component::DeclareAnnotationProperty(axiom)
}

pub fn translate_named_individual_declaration(v: &Value) -> Component<RcStr> {
    let individual = translate_named_individual(&v[1]);
    let axiom = DeclareNamedIndividual { 0: individual };
    Component::DeclareNamedIndividual(axiom)
}

pub fn translate_datatype_declaration(v: &Value) -> Component<RcStr> {
    let datatype = expression_transducer::translate_datatype(&v[1]);
    let axiom = DeclareDatatype { 0: datatype };
    Component::DeclareDatatype(axiom)
}

pub fn translate_declaration(v: &Value) -> Component<RcStr> {
    let unwrapped_declaration = v[1].clone();
    match unwrapped_declaration[0].as_str() {
        Some("Class") => translate_class_declaration(&unwrapped_declaration),
        Some("ObjectProperty") => translate_object_property_declaration(&unwrapped_declaration),
        Some("DataProperty") => translate_data_property_declaration(&unwrapped_declaration),
        Some("AnnotationProperty") => {
            translate_annotation_property_declaration(&unwrapped_declaration)
        }
        Some("NamedIndividual") => translate_named_individual_declaration(&unwrapped_declaration),
        Some("Datatype") => translate_datatype_declaration(&unwrapped_declaration),
        _ => panic!(),
    }
}

pub fn translate_sub_object_property_of(v: &Value) -> Component<RcStr> {
    let lhs = expression_transducer::translate_sub_object_property_expression(&v[1]);
    let rhs = expression_transducer::translate_object_property_expression(&v[2]);
    let axiom = SubObjectPropertyOf { sub: lhs, sup: rhs };
    Component::SubObjectPropertyOf(axiom)
}

pub fn translate_equivalent_object_properties(v: &Value) -> Component<RcStr> {
    let operands: Vec<ObjectPropertyExpression<RcStr>> = (&(v.as_array().unwrap())[1..])
        .into_iter()
        .map(|x| expression_transducer::translate_object_property_expression(&x))
        .collect();
    let axiom = EquivalentObjectProperties(operands);
    Component::EquivalentObjectProperties(axiom)
}

pub fn translate_disjoint_object_properties(v: &Value) -> Component<RcStr> {
    let operands: Vec<ObjectPropertyExpression<RcStr>> = (&(v.as_array().unwrap())[1..])
        .into_iter()
        .map(|x| expression_transducer::translate_object_property_expression(&x))
        .collect();
    let axiom = DisjointObjectProperties(operands);
    Component::DisjointObjectProperties(axiom)
}

pub fn translate_inverse_object_properties(v: &Value) -> Component<RcStr> {
    let lhs = translate_object_property(&v[1]);
    let rhs = translate_object_property(&v[2]);
    let axiom = InverseObjectProperties { 0: lhs, 1: rhs };
    Component::InverseObjectProperties(axiom)
}

pub fn translate_object_property_domain(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let domain = expression_transducer::translate_class_expression(&v[2]);
    let axiom = ObjectPropertyDomain {
        ope: property,
        ce: domain,
    };
    Component::ObjectPropertyDomain(axiom)
}

pub fn translate_object_property_range(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let domain = expression_transducer::translate_class_expression(&v[2]);
    let axiom = ObjectPropertyRange {
        ope: property,
        ce: domain,
    };
    Component::ObjectPropertyRange(axiom)
}

pub fn translate_functional_object_property(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = FunctionalObjectProperty { 0: property };
    Component::FunctionalObjectProperty(axiom)
}

pub fn translate_inverse_functional_object_property(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = InverseFunctionalObjectProperty { 0: property };
    Component::InverseFunctionalObjectProperty(axiom)
}

pub fn translate_reflexive_object_property(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = ReflexiveObjectProperty { 0: property };
    Component::ReflexiveObjectProperty(axiom)
}

pub fn translate_irreflexive_object_property(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = IrreflexiveObjectProperty { 0: property };
    Component::IrreflexiveObjectProperty(axiom)
}

pub fn translate_symmetric_object_property(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = SymmetricObjectProperty { 0: property };
    Component::SymmetricObjectProperty(axiom)
}

pub fn translate_asymmetric_object_property(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = AsymmetricObjectProperty { 0: property };
    Component::AsymmetricObjectProperty(axiom)
}

pub fn translate_transitive_object_property(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = TransitiveObjectProperty { 0: property };
    Component::TransitiveObjectProperty(axiom)
}

pub fn translate_sub_dataproperty_of(v: &Value) -> Component<RcStr> {
    let lhs = expression_transducer::translate_data_property(&v[1]);
    let rhs = expression_transducer::translate_data_property(&v[2]);

    let axiom = SubDataPropertyOf { sub: lhs, sup: rhs };
    Component::SubDataPropertyOf(axiom)
}

pub fn translate_equivalent_data_properties(v: &Value) -> Component<RcStr> {
    let operands: Vec<DataProperty<RcStr>> = (&(v.as_array().unwrap())[1..])
        .into_iter()
        .map(|x| expression_transducer::translate_data_property(&x))
        .collect();
    let axiom = EquivalentDataProperties(operands);
    Component::EquivalentDataProperties(axiom)
}

pub fn translate_disjoint_data_properties(v: &Value) -> Component<RcStr> {
    let operands: Vec<DataProperty<RcStr>> = (&(v.as_array().unwrap())[1..])
        .into_iter()
        .map(|x| expression_transducer::translate_data_property(&x))
        .collect();
    let axiom = DisjointDataProperties(operands);
    Component::DisjointDataProperties(axiom)
}

pub fn translate_data_property_domain(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_data_property(&v[1]);
    let domain = expression_transducer::translate_class_expression(&v[2]);

    let axiom = DataPropertyDomain {
        dp: property,
        ce: domain,
    };
    Component::DataPropertyDomain(axiom)
}

pub fn translate_data_property_range(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_data_property(&v[1]);
    let range = expression_transducer::translate_data_range(&v[2]);

    let axiom = DataPropertyRange {
        dp: property,
        dr: range,
    };
    Component::DataPropertyRange(axiom)
}

pub fn translate_functional_data_property(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_data_property(&v[1]);

    let axiom = FunctionalDataProperty { 0: property };
    Component::FunctionalDataProperty(axiom)
}

pub fn translate_datatype_definition(v: &Value) -> Component<RcStr> {
    let kind = expression_transducer::translate_datatype(&v[1]);
    let range = expression_transducer::translate_data_range(&v[2]);

    let axiom = DatatypeDefinition {
        kind: kind,
        range: range,
    };
    Component::DatatypeDefinition(axiom)
}

//TODO cannot translate this without knowing the type of the property arguments ..
//pub fn translate_has_key(v : &Value) -> Component {
//
//    let class_expression = expression_transducer::translate_class_expression(&v[1]);
//    let operands: Vec<PropertyExpression> = (&(v.as_array().unwrap())[1..])
//                                               .into_iter()
//                                               .map(|x| expression_transducer::translate_data_property(&x))
//                                               .collect();
//
//}

pub fn translate_same_individual(v: &Value) -> Component<RcStr> {
    let operands: Vec<Individual<RcStr>> = (&(v.as_array().unwrap())[1..])
        .into_iter()
        .map(|x| expression_transducer::translate_individual(&x))
        .collect();

    let axiom = SameIndividual { 0: operands };
    Component::SameIndividual(axiom)
}

pub fn translate_different_individuals(v: &Value) -> Component<RcStr> {
    let operands: Vec<Individual<RcStr>> = (&(v.as_array().unwrap())[1..])
        .into_iter()
        .map(|x| expression_transducer::translate_individual(&x))
        .collect();

    let axiom = DifferentIndividuals { 0: operands };
    Component::DifferentIndividuals(axiom)
}

pub fn translate_class_assertion(v: &Value) -> Component<RcStr> {
    let class_expression = expression_transducer::translate_class_expression(&v[1]);
    let individual = expression_transducer::translate_individual(&v[2]);

    let axiom = ClassAssertion {
        ce: class_expression,
        i: individual,
    };
    Component::ClassAssertion(axiom)
}

pub fn translate_object_property_assertion(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let from = expression_transducer::translate_individual(&v[2]);
    let to = expression_transducer::translate_individual(&v[3]);

    let axiom = ObjectPropertyAssertion {
        ope: property,
        from: from,
        to: to,
    };
    Component::ObjectPropertyAssertion(axiom)
}

pub fn translate_negative_object_property_assertion(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let from = expression_transducer::translate_individual(&v[2]);
    let to = expression_transducer::translate_individual(&v[3]);

    let axiom = NegativeObjectPropertyAssertion {
        ope: property,
        from: from,
        to: to,
    };
    Component::NegativeObjectPropertyAssertion(axiom)
}

pub fn translate_data_property_assertion(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_data_property(&v[1]);
    let from = expression_transducer::translate_individual(&v[2]);
    let to = expression_transducer::translate_literal(&v[3]);

    let axiom = DataPropertyAssertion {
        dp: property,
        from: from,
        to: to,
    };
    Component::DataPropertyAssertion(axiom)
}

pub fn translate_negative_data_property_assertion(v: &Value) -> Component<RcStr> {
    let property = expression_transducer::translate_data_property(&v[1]);
    let from = expression_transducer::translate_individual(&v[2]);
    let to = expression_transducer::translate_literal(&v[3]);

    let axiom = NegativeDataPropertyAssertion {
        dp: property,
        from: from,
        to: to,
    };
    Component::NegativeDataPropertyAssertion(axiom)
}

pub fn translate_annotation_assertion(v: &Value) -> Component<RcStr> {
    let property = annotation_transducer::translate_annotation_property(&v[1]);
    let subject = annotation_transducer::translate_annotation_subject(&v[2]);
    let value = annotation_transducer::translate_annotation_value(&v[3]);

    let annotation = Annotation {
        ap: property,
        av: value,
    };

    let axiom = AnnotationAssertion {
        subject: subject,
        ann: annotation,
    };
    Component::AnnotationAssertion(axiom)
}

pub fn translate_sub_annotation_assertion(v: &Value) -> Component<RcStr> {
    let sub = annotation_transducer::translate_annotation_property(&v[1]);
    let sup = annotation_transducer::translate_annotation_property(&v[2]);

    let axiom = SubAnnotationPropertyOf { sub: sub, sup: sup };
    Component::SubAnnotationPropertyOf(axiom)
}

pub fn translate_annotation_property_domain(v: &Value) -> Component<RcStr> {
    let property = annotation_transducer::translate_annotation_property(&v[1]);

    let iri = build().iri(extract_iri_str(&v[2]));

    let axiom = AnnotationPropertyDomain {
        ap: property,
        iri: iri,
    };
    Component::AnnotationPropertyDomain(axiom)
}

pub fn translate_annotation_property_range(v: &Value) -> Component<RcStr> {
    let property = annotation_transducer::translate_annotation_property(&v[1]);

    let iri = build().iri(extract_iri_str(&v[2]));

    let axiom = AnnotationPropertyRange {
        ap: property,
        iri: iri,
    };
    Component::AnnotationPropertyRange(axiom)
}
