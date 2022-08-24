use serde_json::{Value};
use horned_owl::model::{Build, Class, ClassExpression,  NamedIndividual, ObjectProperty, ObjectPropertyExpression, SubObjectPropertyExpression, Individual, AnonymousIndividual, DataProperty, DataRange, Datatype, Literal, FacetRestriction, Facet, PropertyExpression};


 //Class | ObjectIntersectionOf | ObjectUnionOf | ObjectComplementOf | ObjectOneOf | ObjectSomeValuesFrom | ObjectAllValuesFrom | ObjectHasValue | ObjectHasSelf | ObjectMinCardinality | ObjectMaxCardinality | ObjectExactCardinality | DataSomeValuesFrom | DataAllValuesFrom | DataHasValue | DataMinCardinality | DataMaxCardinality | DataExactCardinality
 //
pub fn translate_property_expression(v : &Value) -> ObjectPropertyExpression {
     match v[0].as_str() {
         Some("InverseOf") => translate_inverse_of(v), 
         None => translate_named_object_property(v),
         Some(_) => panic!("Incorrect Property Constructor"),
     } 
}

pub fn translate_class_expression(v : &Value) -> ClassExpression { 
     match v[0].as_str() {
         //throw an error for ambiguity
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
         //Some("ObjectHasValue") => translate_has_value(v), 
         //Some("ObjectMinCardinality") => translate_min_cardinality(v), 
         //Some("ObjectMinQualifiedCardinality") => translate_min_qualified_cardinality(v), 
         //Some("ObjectMaxCardinality") => translate_max_cardinality(v), 
         //Some("ObjectMaxQualifiedCardinality") => translate_max_qualified_cardinality(v), 
         //Some("ObjectExactCardinality") => translate_exact_cardinality(v), 
         //Some("ObjectExactQualifiedCardinality") => translate_exact_qualified_cardinality(v), 
         //Some("ObjectHasSelf") => translate_has_self(v), 
         //Some("ObjectIntersectionOf") => translate_intersection_of(v), 
         //Some("ObjectUnionOf") => translate_union_of(v), 
         //Some("ObjectOneOf") => translate_one_of(v), 
         //Some("ObjectComplementOf") => translate_complement_of(v), 
         //Some("ObjectInverseOf") => property_translation::translate_inverse_of(v), 

         //Some("DataSomeValuesFrom") => translate_some_values_from(v), 
         //Some("DataAllValuesFrom") => translate_all_values_from(v), 
         //Some("DataHasValue") => translate_has_value(v), 
         //Some("DataMinCardinality") => translate_min_cardinality(v), 
         //Some("DataMinQualifiedCardinality") => translate_min_qualified_cardinality(v), 
         //Some("DataMaxCardinality") => translate_max_cardinality(v), 
         //Some("DataMaxQualifiedCardinality") => translate_max_qualified_cardinality(v), 
         //Some("DataExactCardinality") => translate_exact_cardinality(v), 
         //Some("DataExactQualifiedCardinality") => translate_exact_qualified_cardinality(v), 
         //Some("DataHasSelf") => translate_has_self(v), 
         //Some("DataIntersectionOf") => translate_intersection_of(v), 
         //Some("DataUnionOf") => translate_union_of(v), 
         //Some("DataOneOf") => translate_one_of(v), 
         //Some("DataComplementOf") => translate_complement_of(v), 

         Some(_) => panic!(),
         None => translate_named_class(v),
     }
} 

pub fn translate_named_object_property(v : &Value) -> ObjectPropertyExpression {
    let b = Build::new();

    let iri = match v {
        Value::String(x) => x,
        _ => panic!("Not a named entity"), 
    }; 

    b.object_property(iri).into() 
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

pub fn translate_object_some_values_from(v : &Value) -> ClassExpression {

    let property = translate_property_expression(&v[1]); 
    let filler: ClassExpression = translate_class_expression(&v[2]); 

    ClassExpression::ObjectSomeValuesFrom {
        ope: property,
        bce : Box::new(filler)
    } 
}

pub fn translate_object_all_values_from(v : &Value) -> ClassExpression {

    let property = translate_property_expression(&v[1]); 
    let filler: ClassExpression = translate_class_expression(&v[2]); 

    ClassExpression::ObjectAllValuesFrom {
        ope: property,
        bce : Box::new(filler)
    } 
}
