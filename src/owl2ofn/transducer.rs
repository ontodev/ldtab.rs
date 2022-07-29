use serde_json::{Value};
use serde_json::json; 
use horned_owl::model::{Axiom, Build, Class, SubClassOf, ClassAssertion, ClassExpression, DeclareClass, DeclareNamedIndividual, DisjointClasses, DisjointUnion, EquivalentClasses, EquivalentObjectProperties, NamedIndividual, ObjectProperty, ObjectPropertyDomain, ObjectPropertyExpression, SubObjectPropertyExpression, SubObjectPropertyOf, TransitiveObjectProperty, Individual, ObjectPropertyAssertion, AnonymousIndividual};


//TODO
pub fn translate_property_expression(expression: &ObjectPropertyExpression) -> Value { 
    json!("asd")
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

pub fn translate_n_ary_operator(operator : &str, arguments : &Vec<ClassExpression>) -> Value {
    let mut operands : Vec<Value> = arguments.into_iter()
                                         .map(|x| translate_class_expression(&x))
                                         .collect(); 
    let operator = Value::String(String::from(operator));
    operands.insert(0,operator);
    Value::Array(operands) 
}

pub fn translate_one_of(arguments : &Vec<Individual>) -> Value {
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

pub fn translate_some_values_from(property : &ObjectPropertyExpression, filler : &Box<ClassExpression> ) -> Value {
    let expression : ClassExpression = *filler.clone();

    let operator = Value::String(String::from("ObjectSomeValuesFrom"));
    let filler = translate_class_expression(&expression);
    let property = translate_property_expression(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(filler);

    Value::Array(res) 
}

pub fn translate_all_values_from(property : &ObjectPropertyExpression, filler : &Box<ClassExpression> ) -> Value {
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


pub fn translate_class_expression(expression: &ClassExpression) -> Value { 

     match expression {
         ClassExpression::Class(x) => translate_class(&x),
         ClassExpression::ObjectIntersectionOf(x) =>  translate_n_ary_operator("ObjectIntersectionOf", &x),
         ClassExpression::ObjectUnionOf(x) => translate_n_ary_operator("ObjectUnionOf", &x),
         ClassExpression::ObjectComplementOf(x) => translate_object_complement(x),
         ClassExpression::ObjectOneOf(x) => translate_one_of(&x),
         ClassExpression::ObjectSomeValuesFrom{ope,bce} => translate_some_values_from(&ope, bce),
         ClassExpression::ObjectAllValuesFrom{ope,bce} => translate_all_values_from(&ope, bce),
         ClassExpression::ObjectHasValue{ope,i} => translate_object_has_value(&ope, &i),
         ClassExpression::ObjectHasSelf(p) => translate_object_has_self(&p),
         ClassExpression::ObjectMinCardinality{n, ope,bce} => translate_object_cardinality("ObjectMinCardinality", &n, &ope, bce),

         //TODO: how are unqualified restrictions handled?
         //ObjectMinCardinality
         //ObjectMaxCardinality
         //ObjectExactCardinality
         //
         //DataSomeValuesFrom
         //DataAllValuesFrom
         //DataHasValue
         //
         //DataMinCardinality
         //DataMaxCardinality
         //DataExactCardinality 

         _ => json!("asd"), 
    }
}

pub fn translate_subclass_of(axiom: &SubClassOf) -> Value { 

    let operator = Value::String(String::from("SubClassOf"));
    let subclass = translate_class_expression(&axiom.sub);
    let superclass = translate_class_expression(&axiom.sup);
    let v = vec![operator, subclass, superclass];
    Value::Array(v) 
}
