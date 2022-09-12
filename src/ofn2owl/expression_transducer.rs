use serde_json::{Value};
use regex::Regex;
use std::rc::Rc; 
use horned_owl::model::{Build, Class, ClassExpression,  NamedIndividual, ObjectProperty, ObjectPropertyExpression, SubObjectPropertyExpression, Individual, AnonymousIndividual, DataProperty, DataRange, Datatype, Literal, FacetRestriction, Facet, PropertyExpression};


pub fn translate_object_property_expression(v : &Value) -> ObjectPropertyExpression {
     match v[0].as_str() {
         Some("InverseOf") => translate_inverse_of(v), 
         None => translate_named_object_property(v),
         Some(_) => panic!("Incorrect Property Constructor"),
     } 
}

pub fn translate_sub_object_property_expression(v : &Value) -> SubObjectPropertyExpression {
    match v {
        Value::Array(array) => {
            let operands: Vec<ObjectPropertyExpression> = (&array[1..])
                .into_iter()
                .map(|x| translate_object_property_expression(&x))
                .collect(); 
            SubObjectPropertyExpression::ObjectPropertyChain(operands)
        },
        Value::String(_s) => {
            let property = translate_object_property_expression(v);
            SubObjectPropertyExpression::ObjectPropertyExpression(property)
        },
        _ => panic!()
    }
}

pub fn translate_class_expression(v : &Value) -> ClassExpression { 
     match v[0].as_str() {
         //Some("SomeValuesFrom") => translate_some_values_from(v), 
         //Some("AllValuesFrom") => translate_all_values_from(v), 
         //Some("HasValue") => translate_has_value(v), 
         //Some("MinCardinality") => translate_min_cardinality(v), 
         //Some("MinQualifiedCardinality") => translate_min_qualified_cardinality(v), 
         //Some("MaxCardinality") => translate_max_cardinality(v), 
         //Some("MaxQualifiedCardinality") => translate_max_qualified_cardinality(v), 
         //Some("ExactCardinality") => translate_exact_cardinality(v), 
         //Some("ExactQualifiedCardinality") => translate_exact_qualified_cardinality(v), 
         //Some("HasSelf") => translate_has_self(v), 
         //Some("IntersectionOf") => translate_intersection_of(v), 
         //Some("UnionOf") => translate_union_of(v), 
         //Some("OneOf") => translate_one_of(v), 
         //Some("ComplementOf") => translate_complement_of(v), 
         //Some("InverseOf") => property_translation::translate_inverse_of(v),

         Some("ObjectSomeValuesFrom") => translate_object_some_values_from(v), 
         Some("ObjectAllValuesFrom") => translate_object_all_values_from(v), 
         Some("ObjectHasValue") => translate_object_has_value(v), 
         Some("ObjectMinCardinality") => translate_object_min_cardinality(v), 
         Some("ObjectMinQualifiedCardinality") => translate_object_min_qualified_cardinality(v), 
         Some("ObjectMaxCardinality") => translate_object_max_cardinality(v), 
         Some("ObjectMaxQualifiedCardinality") => translate_object_max_qualified_cardinality(v), 
         Some("ObjectExactCardinality") => translate_object_exact_cardinality(v), 
         Some("ObjectExactQualifiedCardinality") => translate_object_exact_qualified_cardinality(v), 
         Some("ObjectHasSelf") => translate_object_has_self(v), 
         Some("ObjectIntersectionOf") => translate_object_intersection_of(v), 
         Some("ObjectUnionOf") => translate_object_union_of(v), 
         Some("ObjectOneOf") => translate_object_one_of(v), 
         Some("ObjectComplementOf") => translate_object_complement_of(v), 

         Some("DataSomeValuesFrom") => translate_data_some_values_from(v), 
         Some("DataAllValuesFrom") => translate_data_all_values_from(v), 
         Some("DataHasValue") => translate_data_has_value(v), 
         Some("DataMinCardinality") => translate_data_min_cardinality(v), 
         Some("DataMinQualifiedCardinality") => translate_data_min_qualified_cardinality(v), 
         Some("DataMaxCardinality") => translate_data_max_cardinality(v), 
         Some("DataMaxQualifiedCardinality") => translate_data_max_qualified_cardinality(v), 
         Some("DataExactCardinality") => translate_data_exact_cardinality(v), 
         Some("DataExactQualifiedCardinality") => translate_data_exact_qualified_cardinality(v), 

         Some(_) => panic!("Not a valid (typed) OFN S-expression"),
         None => translate_named_class(v),
     }
} 

pub fn translate_literal_string(s: &str) -> Literal {

    let b = Build::new();

    let simple = Regex::new("^\"(.+)\"$").unwrap();
    let language_tag = Regex::new("^\"(.+)\"@(.*)$").unwrap();
    let datatype = Regex::new("^\"(.+)\"\\^\\^(.*)$").unwrap();

    if language_tag.is_match(s) {
        match language_tag.captures(s) {
            Some(x) => Literal::Language{ literal: String::from(&x[1]),
                                          lang: String::from(&x[2]) },
            None => panic!("Not a literal with a language tag")
        } 
    } else if datatype.is_match(s) { 
        match datatype.captures(s){
            Some(x) => Literal::Datatype{literal: String::from(&x[1]),
                                        datatype_iri: b.iri(&x[2]),},
            None => panic!("Not a literal with a datatype")

        } 
    } else if simple.is_match(s) { 
        match simple.captures(s) {
            Some(x) => Literal::Simple{literal: String::from(&x[1])},
            None => panic!("Not a simple literal")
        }
    } else { 
        panic!()
    }
}

pub fn translate_literal(v: &Value) -> Literal {
    match v.as_str() {
        Some(x) => translate_literal_string(x),
        None => panic!() 
    }
}

pub fn translate_data_range(v : &Value) -> DataRange {

    match v {
        Value::String(_x) => translate_datatype(v),
        Value::Array(_x) =>  { 
            match v[0].as_str() {
                Some("Datatype") => translate_datatype(v), 
                Some("DataIntersectionOf") => translate_data_intersection_of(v), 
                Some("DataUnionOf") => translate_data_union_of(v), 
                Some("DataComplementOf") => translate_data_complement_of(v), 
                Some("DataOneOf") => translate_data_one_of(v), 
                //TODO
                //Some("DatatypeRestriction") => translate_object_some_values_from(v), 
                Some(_) => panic!(),
                None => panic!(),
            } 
        } ,
        _ => panic!(),
    } 
}

pub fn translate_data_one_of(v : &Value) -> DataRange { 

    let operands: Vec<Literal> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| translate_literal(&x))
                                               .collect(); 
    DataRange::DataOneOf(operands) 
}

pub fn translate_data_complement_of(v : &Value) -> DataRange { 

    let argument: DataRange = translate_data_range(&v[1]); 

       DataRange::DataComplementOf(Box::new(argument))
}

pub fn translate_data_intersection_of(v : &Value) -> DataRange {
    let operands: Vec<DataRange> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| translate_data_range(&x))
                                               .collect(); 

    DataRange::DataIntersectionOf(operands) 
}

pub fn translate_data_union_of(v : &Value) -> DataRange {
    let operands: Vec<DataRange> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| translate_data_range(&x))
                                               .collect(); 

    DataRange::DataUnionOf(operands) 
}

pub fn translate_datatype(v : &Value) -> DataRange {
    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 
    
    DataRange::Datatype(b.datatype(iri))
}

pub fn translate_named_object_property(v : &Value) -> ObjectPropertyExpression {
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

pub fn translate_inverse_of(v : &Value) -> ObjectPropertyExpression {
    let b = Build::new();

    let ofn_argument = v[1].clone(); 
    let iri = match ofn_argument {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 

    let argument =  b.object_property(iri).into(); 
    ObjectPropertyExpression::InverseObjectProperty{0: argument}
}

pub fn translate_named_class(v : &Value) -> ClassExpression {

    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 

    b.class(iri).into()
}

pub fn translate_anonymous_individual(v : &Value) -> AnonymousIndividual {

    let name = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 

    let s = name.as_str();
    let rc : Rc<str> = Rc::from(s);

    AnonymousIndividual{0: rc} 
}

pub fn translate_individual(v : &Value) -> Individual {

    //TODO: handle anonymous individuals

    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 

    b.named_individual(iri).into()
}

pub fn translate_object_some_values_from(v : &Value) -> ClassExpression {

    let property = translate_object_property_expression(&v[1]); 
    let filler: ClassExpression = translate_class_expression(&v[2]); 

    ClassExpression::ObjectSomeValuesFrom {
        ope: property,
        bce : Box::new(filler)
    } 
}

pub fn translate_object_all_values_from(v : &Value) -> ClassExpression {

    let property = translate_object_property_expression(&v[1]); 
    let filler: ClassExpression = translate_class_expression(&v[2]); 

    ClassExpression::ObjectAllValuesFrom {
        ope: property,
        bce : Box::new(filler)
    } 
}

pub fn translate_object_has_value(v : &Value) -> ClassExpression {

    let property = translate_object_property_expression(&v[1]); 
    let individual: Individual = translate_individual(&v[2]); 

    ClassExpression::ObjectHasValue {
        ope: property,
        i : individual,
    } 
}

pub fn translate_object_min_cardinality(v : &Value) -> ClassExpression { 
    let b = Build::new();

    let cardinality = match v[1].clone() {
        Value::String(x) => {
            let num: i32 = x.parse().unwrap();
            num 
        }, 
        _ => panic!("Not a named entity"), 
    }; 

    let property = translate_object_property_expression(&v[2]); 
    let filler: ClassExpression = b.class("http://www.w3.org/2002/07/owl#Thing").into();

    ClassExpression::ObjectMinCardinality {
        n : cardinality as u32,
        ope: property,
        bce : Box::new(filler)
    } 
}

pub fn translate_object_min_qualified_cardinality(v : &Value) -> ClassExpression { 

    let cardinality = match v[1].clone() {
        Value::String(x) => {
            let num: i32 = x.parse().unwrap();
            num 
        }, 
        _ => panic!("Not a named entity"), 
    }; 

    let property = translate_object_property_expression(&v[2]); 
    let filler: ClassExpression = translate_class_expression(&v[3]); 

    ClassExpression::ObjectMinCardinality {
        n : cardinality as u32,
        ope: property,
        bce : Box::new(filler)
    } 
}

pub fn translate_object_max_cardinality(v : &Value) -> ClassExpression { 

    let b = Build::new();

    let cardinality = match v[1].clone() { 
        Value::String(x) => {
            let num: i32 = x.parse().unwrap();
            num 
        },
        _ => panic!("Not a named entity"), 
    }; 

    let property = translate_object_property_expression(&v[2]); 
    let filler: ClassExpression = b.class("http://www.w3.org/2002/07/owl#Thing").into();

    ClassExpression::ObjectMaxCardinality {
        n : cardinality as u32,
        ope: property,
        bce : Box::new(filler)
    } 
}

pub fn translate_object_max_qualified_cardinality(v : &Value) -> ClassExpression { 

    let cardinality = match v[1].clone() { 
        Value::String(x) => {
            let num: i32 = x.parse().unwrap();
            num 
        },
        _ => panic!("Not a named entity"), 
    }; 

    let property = translate_object_property_expression(&v[2]); 
    let filler: ClassExpression = translate_class_expression(&v[3]); 

    ClassExpression::ObjectMaxCardinality {
        n : cardinality as u32,
        ope: property,
        bce : Box::new(filler)
    } 
}


pub fn translate_object_exact_cardinality(v : &Value) -> ClassExpression { 

    let b = Build::new();

    let cardinality = match v[1].clone() {
        Value::String(x) => {
            let num: i32 = x.parse().unwrap();
            num 
        },
        _ => panic!("Not a named entity"), 
    }; 

    let property = translate_object_property_expression(&v[2]); 
    let filler: ClassExpression = b.class("http://www.w3.org/2002/07/owl#Thing").into();

    ClassExpression::ObjectExactCardinality {
        n : cardinality as u32,
        ope: property,
        bce : Box::new(filler)
    } 
}

pub fn translate_object_exact_qualified_cardinality(v : &Value) -> ClassExpression { 

    let cardinality = match v[1].clone() {
        Value::String(x) => {
            let num: i32 = x.parse().unwrap();
            num 
        }, 
        _ => panic!("Not a named entity"), 
    }; 

    let property = translate_object_property_expression(&v[2]); 
    let filler: ClassExpression = translate_class_expression(&v[3]); 

    ClassExpression::ObjectExactCardinality {
        n : cardinality as u32,
        ope: property,
        bce : Box::new(filler)
    } 
}

pub fn translate_object_has_self(v : &Value) -> ClassExpression { 

    let property = translate_object_property_expression(&v[1]); 
    ClassExpression::ObjectHasSelf(property) 
}

pub fn translate_object_intersection_of(v : &Value) -> ClassExpression { 

    let operands: Vec<ClassExpression> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| translate_class_expression(&x))
                                               .collect(); 

    ClassExpression::ObjectIntersectionOf(operands)
}

pub fn translate_object_union_of(v : &Value) -> ClassExpression { 

    let operands: Vec<ClassExpression> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| translate_class_expression(&x))
                                               .collect(); 

    ClassExpression::ObjectUnionOf(operands)
}

pub fn translate_object_one_of(v : &Value) -> ClassExpression { 

    let operands: Vec<Individual> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| translate_individual(&x))
                                               .collect(); 

    ClassExpression::ObjectOneOf(operands)
}

pub fn translate_object_complement_of(v : &Value) -> ClassExpression { 

    let argument: ClassExpression = translate_class_expression(&v[1]); 

       ClassExpression::ObjectComplementOf(Box::new(argument))
}

pub fn translate_data_some_values_from(v : &Value) -> ClassExpression {

    let property = translate_data_property(&v[1]); 
    let filler: DataRange = translate_data_range(&v[2]); 

    ClassExpression::DataSomeValuesFrom {
        dp: property,
        dr : filler
    } 
}

pub fn translate_data_all_values_from(v : &Value) -> ClassExpression {

    let property = translate_data_property(&v[1]); 
    let filler: DataRange = translate_data_range(&v[2]); 

    ClassExpression::DataAllValuesFrom {
        dp: property,
        dr : filler
    } 
}

pub fn translate_data_has_value(v : &Value) -> ClassExpression {

    let property = translate_data_property(&v[1]); 
    let filler: Literal = translate_literal(&v[2]); 

    ClassExpression::DataHasValue {
        dp: property,
        l: filler
    } 
}

pub fn translate_data_min_cardinality(v : &Value) -> ClassExpression { 

    let b = Build::new();

    let cardinality = match v[1].clone() {
        Value::Number(x) => {
            match x.as_u64() {
                Some(y) => y,
                _ => panic!("Not a valid cardinality"), 
            }
        }, 
        _ => panic!("Not a named entity"), 
    }; 

    let property = translate_data_property(&v[2]); 
    let filler: DataRange = DataRange::Datatype(b.datatype("rdfs:Literal"));

    ClassExpression::DataMinCardinality {
        n : cardinality as u32,
        dp: property,
        dr : filler
    } 
}

pub fn translate_data_min_qualified_cardinality(v : &Value) -> ClassExpression { 

    let cardinality = match v[1].clone() {
        Value::Number(x) => {
            match x.as_u64() {
                Some(y) => y,
                _ => panic!("Not a valid cardinality"), 
            }
        }, 
        _ => panic!("Not a named entity"), 
    }; 

    let property = translate_data_property(&v[2]); 
    let filler: DataRange = translate_data_range(&v[3]);

    ClassExpression::DataMinCardinality {
        n : cardinality as u32,
        dp: property,
        dr : filler
    } 
}

pub fn translate_data_max_cardinality(v : &Value) -> ClassExpression { 

    let b = Build::new();

    let cardinality = match v[1].clone() {
        Value::Number(x) => {
            match x.as_u64() {
                Some(y) => y,
                _ => panic!("Not a valid cardinality"), 
            }
        }, 
        _ => panic!("Not a named entity"), 
    }; 

    let property = translate_data_property(&v[2]); 
    let filler: DataRange = DataRange::Datatype(b.datatype("rdfs:Literal"));

    ClassExpression::DataMaxCardinality {
        n : cardinality as u32,
        dp: property,
        dr : filler
    } 
}

pub fn translate_data_max_qualified_cardinality(v : &Value) -> ClassExpression { 

    let cardinality = match v[1].clone() {
        Value::Number(x) => {
            match x.as_u64() {
                Some(y) => y,
                _ => panic!("Not a valid cardinality"), 
            }
        }, 
        _ => panic!("Not a named entity"), 
    }; 

    let property = translate_data_property(&v[2]); 
    let filler: DataRange = translate_data_range(&v[3]);

    ClassExpression::DataMaxCardinality {
        n : cardinality as u32,
        dp: property,
        dr : filler
    } 
}

pub fn translate_data_exact_cardinality(v : &Value) -> ClassExpression { 

    let b = Build::new();

    let cardinality = match v[1].clone() {
        Value::Number(x) => {
            match x.as_u64() {
                Some(y) => y,
                _ => panic!("Not a valid cardinality"), 
            }
        }, 
        _ => panic!("Not a named entity"), 
    }; 

    let property = translate_data_property(&v[2]); 
    let filler: DataRange = DataRange::Datatype(b.datatype("rdfs:Literal"));

    ClassExpression::DataExactCardinality {
        n : cardinality as u32,
        dp: property,
        dr : filler
    } 
}

pub fn translate_data_exact_qualified_cardinality(v : &Value) -> ClassExpression { 

    let cardinality = match v[1].clone() {
        Value::Number(x) => {
            match x.as_u64() {
                Some(y) => y,
                _ => panic!("Not a valid cardinality"), 
            }
        }, 
        _ => panic!("Not a named entity"), 
    }; 

    let property = translate_data_property(&v[2]); 
    let filler: DataRange = translate_data_range(&v[3]);

    ClassExpression::DataExactCardinality {
        n : cardinality as u32,
        dp: property,
        dr : filler
    } 
}
