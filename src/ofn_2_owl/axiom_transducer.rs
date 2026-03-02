use anyhow::{bail, Context, Result};
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
    ObjectPropertyRange, ArcStr, ReflexiveObjectProperty, SameIndividual, SubAnnotationPropertyOf,
    SubClassOf, SubDataPropertyOf, SubObjectPropertyOf, SymmetricObjectProperty,
    TransitiveObjectProperty,
};
use serde_json::Value;

pub fn translate_axiom(v: &Value) -> Result<Component<ArcStr>> {
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
        Some("HasKey") => bail!("HasKey operator currently not supported"),

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

        Some(op) => bail!("Not a valid OWL axiom operator: {}", op),
        None => bail!("Not a valid (typed) OFN S-expression: missing operator"),
    }
}

pub fn translate_named_class(v: &Value) -> Result<Class<ArcStr>> {
    Ok(build().class(extract_iri_str(v)?).into())
}

pub fn translate_import(v: &Value) -> Result<Component<ArcStr>> {
    let import = build().iri(extract_iri_str(&v[2])?).into();

    let axiom = Import(import);
    Ok(Component::Import(axiom))
}

pub fn translate_object_property(v: &Value) -> Result<ObjectProperty<ArcStr>> {
    Ok(build().object_property(extract_iri_str(v)?).into())
}

pub fn translate_annotation_property(v: &Value) -> Result<AnnotationProperty<ArcStr>> {
    Ok(build().annotation_property(extract_iri_str(v)?).into())
}

pub fn translate_named_individual(v: &Value) -> Result<NamedIndividual<ArcStr>> {
    Ok(build().named_individual(extract_iri_str(v)?).into())
}

//TODO refactor this into expression_transducer

pub fn translate_subclass_of(v: &Value) -> Result<Component<ArcStr>> {
    let sub = expression_transducer::translate_class_expression(&v[1])?;
    let sup = expression_transducer::translate_class_expression(&v[2])?;
    let axiom = SubClassOf { sub, sup };
    Ok(Component::SubClassOf(axiom))
}

pub fn translate_equivalent_classes(v: &Value) -> Result<Component<ArcStr>> {
    let operands: Vec<ClassExpression<ArcStr>> = v.as_array()
        .context("Expected array for EquivalentClasses")?[1..]
        .iter()
        .map(|x| expression_transducer::translate_class_expression(x))
        .collect::<Result<_>>()?;
    let axiom = EquivalentClasses(operands);
    Ok(Component::EquivalentClasses(axiom))
}

pub fn translate_disjoint_classes(v: &Value) -> Result<Component<ArcStr>> {
    let operands: Vec<ClassExpression<ArcStr>> = v.as_array()
        .context("Expected array for DisjointClasses")?[1..]
        .iter()
        .map(|x| expression_transducer::translate_class_expression(x))
        .collect::<Result<_>>()?;
    let axiom = DisjointClasses(operands);
    Ok(Component::DisjointClasses(axiom))
}

pub fn translate_disjoint_union(v: &Value) -> Result<Component<ArcStr>> {
    let lhs = translate_named_class(&v[1])?;

    let operands: Vec<ClassExpression<ArcStr>> = v.as_array()
        .context("Expected array for DisjointUnion")?[2..]
        .iter()
        .map(|x| expression_transducer::translate_class_expression(x))
        .collect::<Result<_>>()?;
    let axiom = DisjointUnion(lhs, operands);
    Ok(Component::DisjointUnion(axiom))
}

pub fn translate_class_declaration(v: &Value) -> Result<Component<ArcStr>> {
    let class = translate_named_class(&v[1])?;
    let axiom = DeclareClass(class);
    Ok(Component::DeclareClass(axiom))
}

pub fn translate_object_property_declaration(v: &Value) -> Result<Component<ArcStr>> {
    let property = translate_object_property(&v[1])?;
    let axiom = DeclareObjectProperty(property);
    Ok(Component::DeclareObjectProperty(axiom))
}

pub fn translate_data_property_declaration(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_data_property(&v[1])?;
    let axiom = DeclareDataProperty(property);
    Ok(Component::DeclareDataProperty(axiom))
}

pub fn translate_annotation_property_declaration(v: &Value) -> Result<Component<ArcStr>> {
    let property = translate_annotation_property(&v[1])?;
    let axiom = DeclareAnnotationProperty(property);
    Ok(Component::DeclareAnnotationProperty(axiom))
}

pub fn translate_named_individual_declaration(v: &Value) -> Result<Component<ArcStr>> {
    let individual = translate_named_individual(&v[1])?;
    let axiom = DeclareNamedIndividual(individual);
    Ok(Component::DeclareNamedIndividual(axiom))
}

pub fn translate_datatype_declaration(v: &Value) -> Result<Component<ArcStr>> {
    let datatype = expression_transducer::translate_datatype(&v[1])?;
    let axiom = DeclareDatatype(datatype);
    Ok(Component::DeclareDatatype(axiom))
}

pub fn translate_declaration(v: &Value) -> Result<Component<ArcStr>> {
    let declaration = &v[1];
    match declaration[0].as_str() {
        Some("Class") => translate_class_declaration(declaration),
        Some("ObjectProperty") => translate_object_property_declaration(declaration),
        Some("DataProperty") => translate_data_property_declaration(declaration),
        Some("AnnotationProperty") => translate_annotation_property_declaration(declaration),
        Some("NamedIndividual") => translate_named_individual_declaration(declaration),
        Some("Datatype") => translate_datatype_declaration(declaration),
        _ => bail!("Unknown declaration type"),
    }
}

pub fn translate_sub_object_property_of(v: &Value) -> Result<Component<ArcStr>> {
    let lhs = expression_transducer::translate_sub_object_property_expression(&v[1])?;
    let rhs = expression_transducer::translate_object_property_expression(&v[2])?;
    let axiom = SubObjectPropertyOf { sub: lhs, sup: rhs };
    Ok(Component::SubObjectPropertyOf(axiom))
}

pub fn translate_equivalent_object_properties(v: &Value) -> Result<Component<ArcStr>> {
    let operands: Vec<ObjectPropertyExpression<ArcStr>> = v.as_array()
        .context("Expected array for EquivalentObjectProperties")?[1..]
        .iter()
        .map(|x| expression_transducer::translate_object_property_expression(x))
        .collect::<Result<_>>()?;
    let axiom = EquivalentObjectProperties(operands);
    Ok(Component::EquivalentObjectProperties(axiom))
}

pub fn translate_disjoint_object_properties(v: &Value) -> Result<Component<ArcStr>> {
    let operands: Vec<ObjectPropertyExpression<ArcStr>> = v.as_array()
        .context("Expected array for DisjointObjectProperties")?[1..]
        .iter()
        .map(|x| expression_transducer::translate_object_property_expression(x))
        .collect::<Result<_>>()?;
    let axiom = DisjointObjectProperties(operands);
    Ok(Component::DisjointObjectProperties(axiom))
}

pub fn translate_inverse_object_properties(v: &Value) -> Result<Component<ArcStr>> {
    let lhs = translate_object_property(&v[1])?;
    let rhs = translate_object_property(&v[2])?;
    let axiom = InverseObjectProperties(lhs, rhs);
    Ok(Component::InverseObjectProperties(axiom))
}

pub fn translate_object_property_domain(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_object_property_expression(&v[1])?;
    let domain = expression_transducer::translate_class_expression(&v[2])?;
    let axiom = ObjectPropertyDomain {
        ope: property,
        ce: domain,
    };
    Ok(Component::ObjectPropertyDomain(axiom))
}

pub fn translate_object_property_range(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_object_property_expression(&v[1])?;
    let range = expression_transducer::translate_class_expression(&v[2])?;
    let axiom = ObjectPropertyRange {
        ope: property,
        ce: range,
    };
    Ok(Component::ObjectPropertyRange(axiom))
}

pub fn translate_functional_object_property(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_object_property_expression(&v[1])?;
    let axiom = FunctionalObjectProperty(property);
    Ok(Component::FunctionalObjectProperty(axiom))
}

pub fn translate_inverse_functional_object_property(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_object_property_expression(&v[1])?;
    let axiom = InverseFunctionalObjectProperty(property);
    Ok(Component::InverseFunctionalObjectProperty(axiom))
}

pub fn translate_reflexive_object_property(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_object_property_expression(&v[1])?;
    let axiom = ReflexiveObjectProperty(property);
    Ok(Component::ReflexiveObjectProperty(axiom))
}

pub fn translate_irreflexive_object_property(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_object_property_expression(&v[1])?;
    let axiom = IrreflexiveObjectProperty(property);
    Ok(Component::IrreflexiveObjectProperty(axiom))
}

pub fn translate_symmetric_object_property(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_object_property_expression(&v[1])?;
    let axiom = SymmetricObjectProperty(property);
    Ok(Component::SymmetricObjectProperty(axiom))
}

pub fn translate_asymmetric_object_property(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_object_property_expression(&v[1])?;
    let axiom = AsymmetricObjectProperty(property);
    Ok(Component::AsymmetricObjectProperty(axiom))
}

pub fn translate_transitive_object_property(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_object_property_expression(&v[1])?;
    let axiom = TransitiveObjectProperty(property);
    Ok(Component::TransitiveObjectProperty(axiom))
}

pub fn translate_sub_dataproperty_of(v: &Value) -> Result<Component<ArcStr>> {
    let lhs = expression_transducer::translate_data_property(&v[1])?;
    let rhs = expression_transducer::translate_data_property(&v[2])?;

    let axiom = SubDataPropertyOf { sub: lhs, sup: rhs };
    Ok(Component::SubDataPropertyOf(axiom))
}

pub fn translate_equivalent_data_properties(v: &Value) -> Result<Component<ArcStr>> {
    let operands: Vec<DataProperty<ArcStr>> = v.as_array()
        .context("Expected array for EquivalentDataProperties")?[1..]
        .iter()
        .map(|x| expression_transducer::translate_data_property(x))
        .collect::<Result<_>>()?;
    let axiom = EquivalentDataProperties(operands);
    Ok(Component::EquivalentDataProperties(axiom))
}

pub fn translate_disjoint_data_properties(v: &Value) -> Result<Component<ArcStr>> {
    let operands: Vec<DataProperty<ArcStr>> = v.as_array()
        .context("Expected array for DisjointDataProperties")?[1..]
        .iter()
        .map(|x| expression_transducer::translate_data_property(x))
        .collect::<Result<_>>()?;
    let axiom = DisjointDataProperties(operands);
    Ok(Component::DisjointDataProperties(axiom))
}

pub fn translate_data_property_domain(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_data_property(&v[1])?;
    let domain = expression_transducer::translate_class_expression(&v[2])?;

    let axiom = DataPropertyDomain {
        dp: property,
        ce: domain,
    };
    Ok(Component::DataPropertyDomain(axiom))
}

pub fn translate_data_property_range(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_data_property(&v[1])?;
    let range = expression_transducer::translate_data_range(&v[2])?;

    let axiom = DataPropertyRange {
        dp: property,
        dr: range,
    };
    Ok(Component::DataPropertyRange(axiom))
}

pub fn translate_functional_data_property(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_data_property(&v[1])?;

    let axiom = FunctionalDataProperty(property);
    Ok(Component::FunctionalDataProperty(axiom))
}

pub fn translate_datatype_definition(v: &Value) -> Result<Component<ArcStr>> {
    let kind = expression_transducer::translate_datatype(&v[1])?;
    let range = expression_transducer::translate_data_range(&v[2])?;

    let axiom = DatatypeDefinition { kind, range };
    Ok(Component::DatatypeDefinition(axiom))
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

pub fn translate_same_individual(v: &Value) -> Result<Component<ArcStr>> {
    let operands: Vec<Individual<ArcStr>> = v.as_array()
        .context("Expected array for SameIndividual")?[1..]
        .iter()
        .map(|x| expression_transducer::translate_individual(x))
        .collect::<Result<_>>()?;

    let axiom = SameIndividual(operands);
    Ok(Component::SameIndividual(axiom))
}

pub fn translate_different_individuals(v: &Value) -> Result<Component<ArcStr>> {
    let operands: Vec<Individual<ArcStr>> = v.as_array()
        .context("Expected array for DifferentIndividuals")?[1..]
        .iter()
        .map(|x| expression_transducer::translate_individual(x))
        .collect::<Result<_>>()?;

    let axiom = DifferentIndividuals(operands);
    Ok(Component::DifferentIndividuals(axiom))
}

pub fn translate_class_assertion(v: &Value) -> Result<Component<ArcStr>> {
    let class_expression = expression_transducer::translate_class_expression(&v[1])?;
    let individual = expression_transducer::translate_individual(&v[2])?;

    let axiom = ClassAssertion {
        ce: class_expression,
        i: individual,
    };
    Ok(Component::ClassAssertion(axiom))
}

pub fn translate_object_property_assertion(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_object_property_expression(&v[1])?;
    let from = expression_transducer::translate_individual(&v[2])?;
    let to = expression_transducer::translate_individual(&v[3])?;

    let axiom = ObjectPropertyAssertion {
        ope: property,
        from,
        to,
    };
    Ok(Component::ObjectPropertyAssertion(axiom))
}

pub fn translate_negative_object_property_assertion(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_object_property_expression(&v[1])?;
    let from = expression_transducer::translate_individual(&v[2])?;
    let to = expression_transducer::translate_individual(&v[3])?;

    let axiom = NegativeObjectPropertyAssertion {
        ope: property,
        from,
        to,
    };
    Ok(Component::NegativeObjectPropertyAssertion(axiom))
}

pub fn translate_data_property_assertion(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_data_property(&v[1])?;
    let from = expression_transducer::translate_individual(&v[2])?;
    let to = expression_transducer::translate_literal(&v[3])?;

    let axiom = DataPropertyAssertion {
        dp: property,
        from,
        to,
    };
    Ok(Component::DataPropertyAssertion(axiom))
}

pub fn translate_negative_data_property_assertion(v: &Value) -> Result<Component<ArcStr>> {
    let property = expression_transducer::translate_data_property(&v[1])?;
    let from = expression_transducer::translate_individual(&v[2])?;
    let to = expression_transducer::translate_literal(&v[3])?;

    let axiom = NegativeDataPropertyAssertion {
        dp: property,
        from,
        to,
    };
    Ok(Component::NegativeDataPropertyAssertion(axiom))
}

pub fn translate_annotation_assertion(v: &Value) -> Result<Component<ArcStr>> {
    let property = annotation_transducer::translate_annotation_property(&v[1])?;
    let subject = annotation_transducer::translate_annotation_subject(&v[2])?;
    let value = annotation_transducer::translate_annotation_value(&v[3])?;

    let annotation = Annotation {
        ap: property,
        av: value,
    };

    let axiom = AnnotationAssertion {
        subject,
        ann: annotation,
    };
    Ok(Component::AnnotationAssertion(axiom))
}

pub fn translate_sub_annotation_assertion(v: &Value) -> Result<Component<ArcStr>> {
    let sub = annotation_transducer::translate_annotation_property(&v[1])?;
    let sup = annotation_transducer::translate_annotation_property(&v[2])?;

    let axiom = SubAnnotationPropertyOf { sub, sup };
    Ok(Component::SubAnnotationPropertyOf(axiom))
}

pub fn translate_annotation_property_domain(v: &Value) -> Result<Component<ArcStr>> {
    let property = annotation_transducer::translate_annotation_property(&v[1])?;

    let iri = build().iri(extract_iri_str(&v[2])?);

    let axiom = AnnotationPropertyDomain {
        ap: property,
        iri,
    };
    Ok(Component::AnnotationPropertyDomain(axiom))
}

pub fn translate_annotation_property_range(v: &Value) -> Result<Component<ArcStr>> {
    let property = annotation_transducer::translate_annotation_property(&v[1])?;

    let iri = build().iri(extract_iri_str(&v[2])?);

    let axiom = AnnotationPropertyRange {
        ap: property,
        iri,
    };
    Ok(Component::AnnotationPropertyRange(axiom))
}
