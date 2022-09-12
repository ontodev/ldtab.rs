use serde_json::{Value};
use serde_json::json; 
use crate::ofn2owl::expression_transducer as expression_transducer;
use crate::ofn2owl::annotation_transducer as annotation_transducer;
use horned_owl::model::{Annotation, Individual, AnnotationProperty, Datatype, NamedIndividual, DataProperty, ObjectProperty, Build, Class, ClassExpression, AnnotatedAxiom, Axiom, SubClassOf, ClassAssertion, DeclareClass, DeclareObjectProperty, DeclareDatatype, DeclareDataProperty, DeclareNamedIndividual, DisjointClasses, DisjointUnion, EquivalentClasses, EquivalentObjectProperties, ObjectPropertyDomain, ObjectPropertyExpression, SubObjectPropertyOf, TransitiveObjectProperty, ObjectPropertyAssertion, ReflexiveObjectProperty, IrreflexiveObjectProperty, SymmetricObjectProperty, AsymmetricObjectProperty, ObjectPropertyRange, InverseObjectProperties, FunctionalObjectProperty, InverseFunctionalObjectProperty, DisjointObjectProperties, Import, SubDataPropertyOf, EquivalentDataProperties, DisjointDataProperties, DataPropertyDomain, DataPropertyRange, FunctionalDataProperty, DatatypeDefinition, HasKey, SameIndividual, DifferentIndividuals, NegativeObjectPropertyAssertion, DataPropertyAssertion, NegativeDataPropertyAssertion, AnnotationAssertion, OntologyAnnotation, DeclareAnnotationProperty, SubAnnotationPropertyOf, AnnotationPropertyDomain, AnnotationPropertyRange};

pub fn translate(v : &Value) -> AnnotatedAxiom {
    panic!()

}

pub fn translate_axiom(v : &Value) -> Axiom {

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
        Some("AnnotationPropertyRange") => translate_annotation_property_domain(v),

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


pub fn translate_annotation_property(v : &Value) -> AnnotationProperty {

    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 
    b.annotation_property(iri).into()
}

pub fn translate_named_individual(v : &Value) -> NamedIndividual {

    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 
    b.named_individual(iri).into()
}

//TODO refactor this into expression_transducer 
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

    let property = expression_transducer::translate_data_property(&v[1]);
    let axiom = DeclareDataProperty{0: property};
    Axiom::DeclareDataProperty(axiom)
}

pub fn translate_annotation_property_declaration(v : &Value) -> Axiom {

    let property = translate_annotation_property(&v[1]);
    let axiom = DeclareAnnotationProperty{0: property};
    Axiom::DeclareAnnotationProperty(axiom)
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
        Some("Class") => translate_class_declaration(&unwrapped_declaration),
        Some("ObjectProperty") => translate_object_property_declaration(&unwrapped_declaration),
        Some("DataProperty") => translate_data_property_declaration(&unwrapped_declaration),
        Some("AnnotationProperty") => translate_annotation_property_declaration(&unwrapped_declaration),
        Some("NamedIndividual") => translate_named_individual_declaration(&unwrapped_declaration),
        Some("Datatype") => translate_datatype_declaration(&unwrapped_declaration),
        _ => panic!()
    }
}

pub fn translate_sub_object_property_of(v : &Value) -> Axiom {
    let lhs = expression_transducer::translate_sub_object_property_expression(&v[1]);
    let rhs = expression_transducer::translate_object_property_expression(&v[2]);
    let axiom = SubObjectPropertyOf{sub : lhs,
                                    sup: rhs};
    Axiom::SubObjectPropertyOf(axiom) 
}

pub fn translate_equivalent_object_properties(v : &Value) -> Axiom {

    let operands: Vec<ObjectPropertyExpression> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| expression_transducer::translate_object_property_expression(&x))
                                               .collect(); 
    let axiom = EquivalentObjectProperties(operands);
    Axiom::EquivalentObjectProperties(axiom) 
}

pub fn translate_disjoint_object_properties(v : &Value) -> Axiom {

    let operands: Vec<ObjectPropertyExpression> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| expression_transducer::translate_object_property_expression(&x))
                                               .collect(); 
    let axiom = DisjointObjectProperties(operands);
    Axiom::DisjointObjectProperties(axiom) 
}

pub fn translate_inverse_object_properties(v : &Value) -> Axiom {
    let lhs = translate_object_property(&v[1]);
    let rhs = translate_object_property(&v[2]);
    let axiom = InverseObjectProperties { 0: lhs, 1: rhs};
    Axiom::InverseObjectProperties(axiom) 
}

pub fn translate_object_property_domain(v : &Value) -> Axiom {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let domain = expression_transducer::translate_class_expression(&v[2]);
    let axiom = ObjectPropertyDomain{ope: property, ce : domain };
    Axiom::ObjectPropertyDomain(axiom) 
}

pub fn translate_object_property_range(v : &Value) -> Axiom {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let domain = expression_transducer::translate_class_expression(&v[2]);
    let axiom = ObjectPropertyRange{ope: property, ce : domain };
    Axiom::ObjectPropertyRange(axiom) 
}

pub fn translate_functional_object_property(v : &Value) -> Axiom {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = FunctionalObjectProperty{0: property};
    Axiom::FunctionalObjectProperty(axiom) 
}

pub fn translate_inverse_functional_object_property(v : &Value) -> Axiom {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = InverseFunctionalObjectProperty{0: property};
    Axiom::InverseFunctionalObjectProperty(axiom) 
}

pub fn translate_reflexive_object_property(v : &Value) -> Axiom {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = ReflexiveObjectProperty{0: property};
    Axiom::ReflexiveObjectProperty(axiom) 
}

pub fn translate_irreflexive_object_property(v : &Value) -> Axiom {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = IrreflexiveObjectProperty{0: property};
    Axiom::IrreflexiveObjectProperty(axiom) 
}

pub fn translate_symmetric_object_property(v : &Value) -> Axiom {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = SymmetricObjectProperty{0: property};
    Axiom::SymmetricObjectProperty(axiom) 
}

pub fn translate_asymmetric_object_property(v : &Value) -> Axiom {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = AsymmetricObjectProperty{0: property};
    Axiom::AsymmetricObjectProperty(axiom) 
}

pub fn translate_transitive_object_property(v : &Value) -> Axiom {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let axiom = TransitiveObjectProperty{0: property};
    Axiom::TransitiveObjectProperty(axiom) 
}

pub fn translate_sub_dataproperty_of(v : &Value) -> Axiom {
    let lhs = expression_transducer::translate_data_property(&v[1]);
    let rhs = expression_transducer::translate_data_property(&v[2]);

    let axiom = SubDataPropertyOf{sub : lhs, sup : rhs};
    Axiom::SubDataPropertyOf(axiom) 
}

pub fn translate_equivalent_data_properties(v : &Value) -> Axiom {

    let operands: Vec<DataProperty> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| expression_transducer::translate_data_property(&x))
                                               .collect(); 
    let axiom = EquivalentDataProperties(operands);
    Axiom::EquivalentDataProperties(axiom) 
}

pub fn translate_disjoint_data_properties(v : &Value) -> Axiom {

    let operands: Vec<DataProperty> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| expression_transducer::translate_data_property(&x))
                                               .collect(); 
    let axiom = DisjointDataProperties(operands);
    Axiom::DisjointDataProperties(axiom) 

}

pub fn translate_data_property_domain(v : &Value) -> Axiom {
    let property = expression_transducer::translate_data_property(&v[1]);
    let domain = expression_transducer::translate_class_expression(&v[2]);

    let axiom = DataPropertyDomain{dp: property, ce: domain};
    Axiom::DataPropertyDomain(axiom)

}

pub fn translate_data_property_range(v : &Value) -> Axiom {
    let property = expression_transducer::translate_data_property(&v[1]);
    let range = expression_transducer::translate_data_range(&v[2]);

    let axiom = DataPropertyRange{dp: property, dr: range};
    Axiom::DataPropertyRange(axiom) 
}

pub fn translate_functional_data_property(v : &Value) -> Axiom {

    let property = expression_transducer::translate_data_property(&v[1]);

    let axiom = FunctionalDataProperty{0: property};
    Axiom::FunctionalDataProperty(axiom)
}

pub fn translate_datatype_definition(v : &Value) -> Axiom {

    let kind = translate_datatype(&v[1]);
    let range = expression_transducer::translate_data_range(&v[2]);

    let axiom = DatatypeDefinition{kind : kind, range : range};
    Axiom::DatatypeDefinition(axiom)

}

//TODO cannot translate this without knowing the type of the property arguments ..
//pub fn translate_has_key(v : &Value) -> Axiom {
//
//    let class_expression = expression_transducer::translate_class_expression(&v[1]);
//    let operands: Vec<PropertyExpression> = (&(v.as_array().unwrap())[1..])
//                                               .into_iter()
//                                               .map(|x| expression_transducer::translate_data_property(&x))
//                                               .collect(); 
//
//}

pub fn translate_same_individual(v : &Value) -> Axiom {

    let operands: Vec<Individual> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| expression_transducer::translate_individual(&x))
                                               .collect(); 

    let axiom = SameIndividual{0 : operands};
    Axiom::SameIndividual(axiom)
}


pub fn translate_different_individuals(v : &Value) -> Axiom {

    let operands: Vec<Individual> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| expression_transducer::translate_individual(&x))
                                               .collect(); 

    let axiom = DifferentIndividuals{0 : operands};
    Axiom::DifferentIndividuals(axiom)
}


pub fn translate_class_assertion(v : &Value) -> Axiom {

    let class_expression = expression_transducer::translate_class_expression(&v[1]);
    let individual = expression_transducer::translate_individual(&v[2]);

    let axiom = ClassAssertion{ce: class_expression, i: individual};
    Axiom::ClassAssertion(axiom) 
}

pub fn translate_object_property_assertion(v : &Value) -> Axiom {

    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let from = expression_transducer::translate_individual(&v[2]);
    let to = expression_transducer::translate_individual(&v[3]);

    let axiom = ObjectPropertyAssertion{ope: property, from: from, to: to};
    Axiom::ObjectPropertyAssertion(axiom) 
}

pub fn translate_negative_object_property_assertion(v : &Value) -> Axiom {
    let property = expression_transducer::translate_object_property_expression(&v[1]);
    let from = expression_transducer::translate_individual(&v[2]);
    let to = expression_transducer::translate_individual(&v[3]);

    let axiom = NegativeObjectPropertyAssertion{ope: property, from: from, to: to};
    Axiom::NegativeObjectPropertyAssertion(axiom) 

}

pub fn translate_data_property_assertion(v : &Value) -> Axiom {

    let property = expression_transducer::translate_data_property(&v[1]);
    let from = expression_transducer::translate_individual(&v[2]);
    let to = expression_transducer::translate_literal(&v[3]);

    let axiom = DataPropertyAssertion{dp: property, from: from, to: to};
    Axiom::DataPropertyAssertion(axiom) 
}

pub fn translate_negative_data_property_assertion(v : &Value) -> Axiom {

    let property = expression_transducer::translate_data_property(&v[1]);
    let from = expression_transducer::translate_individual(&v[2]);
    let to = expression_transducer::translate_literal(&v[3]);

    let axiom = NegativeDataPropertyAssertion{dp: property, from: from, to: to};
    Axiom::NegativeDataPropertyAssertion(axiom) 
}

pub fn translate_annotation_assertion(v : &Value) -> Axiom {

    let property = annotation_transducer::translate_annotation_property(&v[1]);
    let subject = annotation_transducer::translate_annotation_subject(&v[2]);
    let value = annotation_transducer::translate_annotation_value(&v[3]);

    let annotation = Annotation{ap : property, av :value};

    let axiom = AnnotationAssertion{subject : subject, ann: annotation};
    Axiom::AnnotationAssertion(axiom) 
}

pub fn translate_sub_annotation_assertion(v : &Value) -> Axiom {
    let sub = annotation_transducer::translate_annotation_property(&v[1]);
    let sup = annotation_transducer::translate_annotation_property(&v[2]);

    let axiom = SubAnnotationPropertyOf{sub : sub, sup : sup };
    Axiom::SubAnnotationPropertyOf(axiom) 
}

pub fn translate_annotation_property_domain(v : &Value) -> Axiom {

    let property = annotation_transducer::translate_annotation_property(&v[1]);

    let b = Build::new();
    let string = v[2].as_str().unwrap();
    let iri = b.iri(string); 

    let axiom = AnnotationPropertyDomain{ap : property, iri : iri};
    Axiom::AnnotationPropertyDomain(axiom) 
}

pub fn translate_annotation_property_range(v : &Value) -> Axiom {

    let property = annotation_transducer::translate_annotation_property(&v[1]);

    let b = Build::new();
    let string = v[2].as_str().unwrap();
    let iri = b.iri(string); 

    let axiom = AnnotationPropertyRange{ap : property, iri : iri};
    Axiom::AnnotationPropertyRange(axiom) 
}
