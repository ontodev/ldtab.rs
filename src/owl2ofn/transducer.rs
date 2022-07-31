use serde_json::{Value};
use serde_json::json; 
use horned_owl::model::{Axiom, Build, Class, SubClassOf, ClassAssertion, ClassExpression, DeclareClass, DeclareNamedIndividual, DisjointClasses, DisjointUnion, EquivalentClasses, EquivalentObjectProperties, NamedIndividual, ObjectProperty, ObjectPropertyDomain, ObjectPropertyExpression, SubObjectPropertyExpression, SubObjectPropertyOf, TransitiveObjectProperty, Individual, ObjectPropertyAssertion, AnonymousIndividual, DataProperty, DataRange, Datatype, Literal, FacetRestriction, Facet};


pub fn translate_property_expression(expression: &ObjectPropertyExpression) -> Value { 

     match expression {
         ObjectPropertyExpression::ObjectProperty(x) => translate_object_property(&x),
         //TODO: this means I cannot nest the inverse operator?
         ObjectPropertyExpression::InverseObjectProperty(x) => translate_object_property(&x),
     } 
}

pub fn translate_object_property(property: &ObjectProperty) -> Value { 
    let a = property.0.get(0..);
    json!(a)
}

pub fn translate_data_property(property: &DataProperty) -> Value { 
    let a = property.0.get(0..);
    json!(a)
}

pub fn translate_class(class : &Class) -> Value {
    let a = class.0.get(0..);
    json!(a) 
}

//TODO: not sure this is correct
pub fn translate_anonymous_individual(a : &AnonymousIndividual) -> Value { 
    let an = a.0.get(0..);
    json!(an) 
}

pub fn translate_named_individual(a : &NamedIndividual) -> Value { 
    let an = a.0.get(0..);
    json!(an) 
}

pub fn translate_individual(individual : &Individual) -> Value {

     match individual {
         Individual::Anonymous(x) => translate_anonymous_individual(&x),
         Individual::Named(x) => translate_named_individual(&x),
     } 
}

pub fn translate_literal(literal : &Literal) -> Value { 
        match literal {
            Literal::Simple{literal} => json!(literal),
            Literal::Language{literal, lang} => json!(format!("{}@{}", literal, lang)),
            Literal::Datatype{literal, datatype_iri} => json!(format!("{}^^{}", literal, datatype_iri.get(0..).unwrap()))
        }
}

pub fn translate_n_ary_operator(operator : &str, arguments : &Vec<ClassExpression>) -> Value {
    let mut operands : Vec<Value> = arguments.into_iter()
                                         .map(|x| translate_class_expression(&x))
                                         .collect(); 
    let operator = Value::String(String::from(operator));
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_object_one_of(arguments : &Vec<Individual>) -> Value {
    let mut operands : Vec<Value> = arguments.into_iter()
                                         .map(|x| translate_individual(&x))
                                         .collect(); 
    let operator = Value::String(String::from("ObjectOneOf"));
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_object_complement(argument : &Box<ClassExpression>) -> Value {
    let expression : ClassExpression = *argument.clone();
    let argument = translate_class_expression(&expression);

    let operator = Value::String(String::from("ObjectComplementOf"));
    let mut res = vec![operator];
    res.push(argument);

    Value::Array(res) 
}

pub fn translate_object_some_values_from(property : &ObjectPropertyExpression, filler : &Box<ClassExpression> ) -> Value {
    let expression : ClassExpression = *filler.clone();

    let operator = Value::String(String::from("ObjectSomeValuesFrom"));
    let filler = translate_class_expression(&expression);
    let property = translate_property_expression(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(filler);

    Value::Array(res) 
}

pub fn translate_object_all_values_from(property : &ObjectPropertyExpression, filler : &Box<ClassExpression> ) -> Value {
    let expression : ClassExpression = *filler.clone();

    let operator = Value::String(String::from("ObjectAllValuesFrom"));
    let filler = translate_class_expression(&expression);
    let property = translate_property_expression(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(filler);

    Value::Array(res) 
}

pub fn translate_object_has_value(property : &ObjectPropertyExpression, value : &Individual ) -> Value {

    let operator = Value::String(String::from("ObjectAllValuesFrom"));
    let value = translate_individual(value);
    let property = translate_property_expression(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(value);

    Value::Array(res) 
}

pub fn translate_object_has_self(property : &ObjectPropertyExpression) -> Value {

    let operator = Value::String(String::from("ObjectHasSelf"));
    let property = translate_property_expression(property);

    let mut res = vec![operator];
    res.push(property);

    Value::Array(res) 
}

pub fn translate_object_cardinality(operator : &str, cardinality : &u32, property : &ObjectPropertyExpression, filler : &Box<ClassExpression> ) -> Value {

    let expression : ClassExpression = *filler.clone();

    let operator = Value::String(String::from(operator));
    let cardinality = json!(cardinality);
    let filler = translate_class_expression(&expression);
    let property = translate_property_expression(property);

    let mut res = vec![operator];
    res.push(cardinality);
    res.push(property);
    res.push(filler);

    Value::Array(res) 
}

pub fn translate_datatype(datatype : &Datatype) -> Value {
    let a = datatype.0.get(0..);
    json!(a) 
}

pub fn translate_data_intersection_of(arguments : &Vec<DataRange>) -> Value {
    let mut operands : Vec<Value> = arguments.into_iter()
                                         .map(|x| translate_data_range(&x))
                                         .collect(); 
    let operator = Value::String(String::from("DataIntersectionOf"));
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_data_union_of(arguments : &Vec<DataRange>) -> Value {
    let mut operands : Vec<Value> = arguments.into_iter()
                                         .map(|x| translate_data_range(&x))
                                         .collect(); 
    let operator = Value::String(String::from("DataUnionOf"));
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_data_complement_of(argument : &Box<DataRange>) -> Value {

    let range : DataRange = *argument.clone();
    let argument = translate_data_range(&range);

    let operator = Value::String(String::from("DataComplementOf"));
    let mut res = vec![operator];
    res.push(argument);

    Value::Array(res) 
}

pub fn translate_data_one_of(arguments : &Vec<Literal>) -> Value {

    let mut operands : Vec<Value> = arguments.into_iter()
                                         .map(|x| translate_literal(&x))
                                         .collect(); 

    let operator = Value::String(String::from("DataOneOf"));
    operands.insert(0,operator);
    Value::Array(operands) 
}


pub fn translate_facet(facet : &Facet) -> Value {
    match facet  {
        Facet::Length => json!("Length"),
        Facet::MinLength => json!("MinLength"),
        Facet::MaxLength => json!("MaxLength"),
        Facet::Pattern => json!("Pattern"),
        Facet::MinInclusive => json!("MinInclusive"),
        Facet::MinExclusive => json!("MinExclusive"),
        Facet::MaxInclusive => json!("MaxInclusive"),
        Facet::MaxExclusive => json!("MaxExclusive"),
        Facet::TotalDigits => json!("TotalDigits"),
        Facet::FractionDigits => json!("FractionDigits"),
        Facet::LangRange => json!("LangRange"),
    }
}

pub fn translate_facet_restriction(facet_restriction : &FacetRestriction) -> Value {

    let operator = Value::String(String::from("FaceetRestriction"));

    let facet = facet_restriction.f.clone();
    let facet = translate_facet(&facet);

    let literal = facet_restriction.l.clone();
    let literal = translate_literal(&literal);

    let mut res = vec![operator];
    res.push(facet);
    res.push(literal);

    Value::Array(res) 
}

pub fn translate_datatype_restriction(datatype : &Datatype, facets : &Vec<FacetRestriction>) -> Value {
    let operator = Value::String(String::from("DatatypeRestriction"));
    let datatype = translate_datatype(datatype); 
    let mut operands : Vec<Value> = facets.into_iter()
                                         .map(|x| translate_facet_restriction(&x))
                                         .collect(); 

    operands.insert(0,datatype);
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_data_range(range : &DataRange) -> Value { 

    match range {
        DataRange::Datatype(x) => translate_datatype(&x),
        DataRange::DataIntersectionOf(x) =>  translate_data_intersection_of(&x),
        DataRange::DataUnionOf(x) =>  translate_data_union_of(&x),
        DataRange::DataComplementOf(x) => translate_data_complement_of(&x),
        DataRange::DataOneOf(x) => translate_data_one_of(&x),
        DataRange::DatatypeRestriction(datatype,facets) => translate_datatype_restriction(&datatype, &facets),
    }
}

pub fn translate_data_some_values_from(property : &DataProperty, filler : &DataRange ) -> Value {


    let operator = Value::String(String::from("DataSomeValuesFrom"));
    let filler = translate_data_range(&filler);
    let property = translate_data_property(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(filler);

    Value::Array(res) 
}

pub fn translate_data_all_values_from(property : &DataProperty, filler : &DataRange ) -> Value {


    let operator = Value::String(String::from("DataAllValuesFrom"));
    let filler = translate_data_range(&filler);
    let property = translate_data_property(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(filler);

    Value::Array(res) 
}

pub fn translate_data_has_value(property : &DataProperty, literal : &Literal ) -> Value {

    let operator = Value::String(String::from("ObjectAllValuesFrom"));
    let value = translate_literal(literal);
    let property = translate_data_property(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(value);

    Value::Array(res) 
}

pub fn translate_data_cardinality(operator : &str, cardinality : &u32, property : &DataProperty, filler : &DataRange ) -> Value { 

    let operator = Value::String(String::from(operator));
    let cardinality = json!(cardinality);
    let filler = translate_data_range(filler);
    let property = translate_data_property(property);

    let mut res = vec![operator];
    res.push(cardinality);
    res.push(property);
    res.push(filler);

    Value::Array(res) 
}


pub fn translate_class_expression(expression: &ClassExpression) -> Value { 

     match expression {
         ClassExpression::Class(x) => translate_class(&x),
         ClassExpression::ObjectIntersectionOf(x) =>  translate_n_ary_operator("ObjectIntersectionOf", &x),
         ClassExpression::ObjectUnionOf(x) => translate_n_ary_operator("ObjectUnionOf", &x),
         ClassExpression::ObjectComplementOf(x) => translate_object_complement(x),
         ClassExpression::ObjectOneOf(x) => translate_object_one_of(&x),
         ClassExpression::ObjectSomeValuesFrom{ope,bce} => translate_object_some_values_from(&ope, bce),
         ClassExpression::ObjectAllValuesFrom{ope,bce} => translate_object_all_values_from(&ope, bce),
         ClassExpression::ObjectHasValue{ope,i} => translate_object_has_value(&ope, &i),
         ClassExpression::ObjectHasSelf(p) => translate_object_has_self(&p),
         ClassExpression::ObjectMinCardinality{n, ope,bce} => translate_object_cardinality("ObjectMinCardinality", &n, &ope, bce),
         ClassExpression::ObjectMaxCardinality{n, ope,bce} => translate_object_cardinality("ObjectMaxCardinality", &n, &ope, bce),
         ClassExpression::ObjectExactCardinality{n, ope,bce} => translate_object_cardinality("ObjectExactCardinality", &n, &ope, bce),
         ClassExpression::DataSomeValuesFrom{dp,dr} => translate_data_some_values_from(&dp, &dr),
         ClassExpression::DataAllValuesFrom{dp,dr} => translate_data_all_values_from(&dp, &dr),
         ClassExpression::DataHasValue{dp,l} => translate_data_has_value(&dp, &l),
         ClassExpression::DataMinCardinality{n, dp,dr} => translate_data_cardinality("DataMinCardinality", &n, &dp, &dr),
         ClassExpression::DataMaxCardinality{n, dp,dr} => translate_data_cardinality("DataMaxCardinality", &n, &dp, &dr),
         ClassExpression::DataExactCardinality{n, dp,dr} => translate_data_cardinality("DataExactCardinality", &n, &dp, &dr), 
    }
}

//TODO: provide translation for all axiom types
pub fn translate_subclass_of(axiom: &SubClassOf) -> Value { 

    let operator = Value::String(String::from("SubClassOf"));
    let subclass = translate_class_expression(&axiom.sub);
    let superclass = translate_class_expression(&axiom.sup);
    let v = vec![operator, subclass, superclass];
    Value::Array(v) 
}

pub fn translate_equivalent_classes(axiom : &EquivalentClasses) -> Value {
    let operator = Value::String(String::from("EquivalentClasses"));
    let classes = axiom.0.clone();
    let mut operands : Vec<Value> = classes.into_iter()
                                         .map(|x| translate_class_expression(&x))
                                         .collect(); 
    operands.insert(0,operator);
    Value::Array(operands) 
}
