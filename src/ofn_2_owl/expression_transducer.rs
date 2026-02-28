use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;
use std::sync::Arc;
use crate::ofn_2_owl::util::{
    build, default_class_filler, default_data_filler, extract_iri_str, parse_number_cardinality,
    parse_string_cardinality,
};
use horned_owl::model::{
    AnonymousIndividual, ClassExpression, DataProperty, DataRange, Datatype, Individual, Literal,
    ObjectPropertyExpression, ArcStr, SubObjectPropertyExpression,
};

static SIMPLE_LITERAL_RE: Lazy<Regex> = Lazy::new(|| Regex::new("(?s)^\"(.*)\"$").unwrap());
static LANGUAGE_TAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new("(?s)^\"(.*)\"@(.*)$").unwrap());
static DATATYPE_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(?s)\"(.*)\"\\^\\^(.*)$").unwrap());

pub fn translate_object_property_expression(v: &Value) -> ObjectPropertyExpression<ArcStr> {
    match v[0].as_str() {
        Some("InverseOf") => translate_inverse_of(v),
        Some("ObjectInverseOf") => translate_inverse_of(v),
        None => translate_named_object_property(v),
        Some(_) => panic!("Incorrect Property Constructor"),
    }
}

pub fn translate_sub_object_property_expression(v: &Value) -> SubObjectPropertyExpression<ArcStr> {
    match v {
        Value::Array(array) => {
            let operands: Vec<ObjectPropertyExpression<ArcStr>> = array[1..]
                .iter()
                .map(|x| translate_object_property_expression(x))
                .collect();
            SubObjectPropertyExpression::ObjectPropertyChain(operands)
        }
        Value::String(_s) => {
            let property = translate_object_property_expression(v);
            SubObjectPropertyExpression::ObjectPropertyExpression(property)
        }
        _ => panic!(),
    }
}

pub fn translate_class_expression(v: &Value) -> ClassExpression<ArcStr> {
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
        Some("ObjectMaxCardinality") => translate_object_max_cardinality(v),
        Some("ObjectExactCardinality") => translate_object_exact_cardinality(v),

        Some("DataMinCardinality") => translate_data_min_cardinality(v),
        Some("DataMaxCardinality") => translate_data_max_cardinality(v),
        Some("DataExactCardinality") => translate_data_exact_cardinality(v),

        Some("ObjectHasSelf") => translate_object_has_self(v),
        Some("ObjectIntersectionOf") => translate_object_intersection_of(v),
        Some("ObjectUnionOf") => translate_object_union_of(v),
        Some("ObjectOneOf") => translate_object_one_of(v),
        Some("ObjectComplementOf") => translate_object_complement_of(v),

        Some("DataSomeValuesFrom") => translate_data_some_values_from(v),
        Some("DataAllValuesFrom") => translate_data_all_values_from(v),
        Some("DataHasValue") => translate_data_has_value(v),

        Some(_) => panic!("Not a valid (typed) OFN S-expression"),
        None => translate_named_class(v),
    }
}

pub fn translate_literal_string(s: &str) -> Literal<ArcStr> {
    if let Some(x) = LANGUAGE_TAG_RE.captures(s) {
        Literal::Language {
            literal: String::from(&x[1]),
            lang: String::from(&x[2]),
        }
    } else if let Some(x) = DATATYPE_RE.captures(s) {
        Literal::Datatype {
            literal: String::from(&x[1]),
            datatype_iri: build().iri(&x[2]),
        }
    } else if let Some(x) = SIMPLE_LITERAL_RE.captures(s) {
        Literal::Simple {
            literal: String::from(&x[1]),
        }
    } else {
        panic!("Not a valid literal: {}", s)
    }
}

pub fn translate_literal(v: &Value) -> Literal<ArcStr> {
    match v.as_str() {
        Some(x) => translate_literal_string(x),
        None => panic!(),
    }
}

pub fn translate_data_range(v: &Value) -> DataRange<ArcStr> {
    match v {
        Value::String(_x) => translate_datatype_as_range(v),
        Value::Array(_x) => {
            match v[0].as_str() {
                Some("Datatype") => translate_datatype_as_range(v),
                Some("DataIntersectionOf") => translate_data_intersection_of(v),
                Some("DataUnionOf") => translate_data_union_of(v),
                Some("DataComplementOf") => translate_data_complement_of(v),
                Some("DataOneOf") => translate_data_one_of(v),
                //TODO
                //Some("DatatypeRestriction") => translate_object_some_values_from(v),
                Some(_) => panic!(),
                None => panic!(),
            }
        }
        _ => panic!(),
    }
}

pub fn translate_data_one_of(v: &Value) -> DataRange<ArcStr> {
    let operands: Vec<Literal<ArcStr>> = v.as_array().unwrap()[1..]
        .iter()
        .map(|x| translate_literal(x))
        .collect();
    DataRange::DataOneOf(operands)
}

pub fn translate_data_complement_of(v: &Value) -> DataRange<ArcStr> {
    let argument: DataRange<ArcStr> = translate_data_range(&v[1]);

    DataRange::DataComplementOf(Box::new(argument))
}

pub fn translate_data_intersection_of(v: &Value) -> DataRange<ArcStr> {
    let operands: Vec<DataRange<ArcStr>> = v.as_array().unwrap()[1..]
        .iter()
        .map(|x| translate_data_range(x))
        .collect();

    DataRange::DataIntersectionOf(operands)
}

pub fn translate_data_union_of(v: &Value) -> DataRange<ArcStr> {
    let operands: Vec<DataRange<ArcStr>> = v.as_array().unwrap()[1..]
        .iter()
        .map(|x| translate_data_range(x))
        .collect();

    DataRange::DataUnionOf(operands)
}

pub fn translate_datatype(v: &Value) -> Datatype<ArcStr> {
    build().datatype(extract_iri_str(v)).into()
}

pub fn translate_datatype_as_range(v: &Value) -> DataRange<ArcStr> {
    DataRange::Datatype(build().datatype(extract_iri_str(v)))
}

pub fn translate_named_object_property(v: &Value) -> ObjectPropertyExpression<ArcStr> {
    build().object_property(extract_iri_str(v)).into()
}

pub fn translate_data_property(v: &Value) -> DataProperty<ArcStr> {
    build().data_property(extract_iri_str(v)).into()
}

pub fn translate_inverse_of(v: &Value) -> ObjectPropertyExpression<ArcStr> {
    let argument = build().object_property(extract_iri_str(&v[1])).into();
    ObjectPropertyExpression::InverseObjectProperty { 0: argument }
}

pub fn translate_named_class(v: &Value) -> ClassExpression<ArcStr> {
    build().class(extract_iri_str(v)).into()
}

pub fn translate_anonymous_individual(v: &Value) -> AnonymousIndividual<ArcStr> {
    let rc: Arc<str> = Arc::from(extract_iri_str(v));
    AnonymousIndividual { 0: rc }
}

pub fn translate_individual(v: &Value) -> Individual<ArcStr> {
    //TODO: handle anonymous individuals
    build().named_individual(extract_iri_str(v)).into()
}

pub fn translate_object_some_values_from(v: &Value) -> ClassExpression<ArcStr> {
    let property = translate_object_property_expression(&v[1]);
    let filler: ClassExpression<ArcStr> = translate_class_expression(&v[2]);

    ClassExpression::ObjectSomeValuesFrom {
        ope: property,
        bce: Box::new(filler),
    }
}

pub fn translate_object_all_values_from(v: &Value) -> ClassExpression<ArcStr> {
    let property = translate_object_property_expression(&v[1]);
    let filler: ClassExpression<ArcStr> = translate_class_expression(&v[2]);

    ClassExpression::ObjectAllValuesFrom {
        ope: property,
        bce: Box::new(filler),
    }
}

pub fn translate_object_has_value(v: &Value) -> ClassExpression<ArcStr> {
    let property = translate_object_property_expression(&v[1]);
    let individual: Individual<ArcStr> = translate_individual(&v[2]);

    ClassExpression::ObjectHasValue {
        ope: property,
        i: individual,
    }
}

pub fn translate_object_min_cardinality(v: &Value) -> ClassExpression<ArcStr> {
    let cardinality = parse_string_cardinality(&v[1]);

    let property = translate_object_property_expression(&v[2]);

    let ofn = v.as_array().unwrap();
    let is_qualified = ofn.len() == 4;

    let filler = if is_qualified {
        translate_class_expression(&v[3])
    } else {
        default_class_filler()
    };

    //let filler: ClassExpression = b.class("http://www.w3.org/2002/07/owl#Thing").into();

    ClassExpression::ObjectMinCardinality {
        n: cardinality as u32,
        ope: property,
        bce: Box::new(filler),
    }
}

pub fn translate_object_max_cardinality(v: &Value) -> ClassExpression<ArcStr> {
    let cardinality = parse_string_cardinality(&v[1]);

    let property = translate_object_property_expression(&v[2]);

    let ofn = v.as_array().unwrap();
    let is_qualified = ofn.len() == 4;

    let filler = if is_qualified {
        translate_class_expression(&v[3])
    } else {
        default_class_filler()
    };

    //let filler: ClassExpression = b.class("http://www.w3.org/2002/07/owl#Thing").into();

    ClassExpression::ObjectMaxCardinality {
        n: cardinality as u32,
        ope: property,
        bce: Box::new(filler),
    }
}

pub fn translate_object_exact_cardinality(v: &Value) -> ClassExpression<ArcStr> {
    let cardinality = parse_string_cardinality(&v[1]);

    let property = translate_object_property_expression(&v[2]);

    let ofn = v.as_array().unwrap();
    let is_qualified = ofn.len() == 4;

    let filler = if is_qualified {
        translate_class_expression(&v[3])
    } else {
        default_class_filler()
    };

    //let filler: ClassExpression = b.class("http://www.w3.org/2002/07/owl#Thing").into();

    ClassExpression::ObjectExactCardinality {
        n: cardinality as u32,
        ope: property,
        bce: Box::new(filler),
    }
}

pub fn translate_object_has_self(v: &Value) -> ClassExpression<ArcStr> {
    let property = translate_object_property_expression(&v[1]);
    ClassExpression::ObjectHasSelf(property)
}

pub fn translate_object_intersection_of(v: &Value) -> ClassExpression<ArcStr> {
    let operands: Vec<ClassExpression<ArcStr>> = v.as_array().unwrap()[1..]
        .iter()
        .map(|x| translate_class_expression(x))
        .collect();

    ClassExpression::ObjectIntersectionOf(operands)
}

pub fn translate_object_union_of(v: &Value) -> ClassExpression<ArcStr> {
    let operands: Vec<ClassExpression<ArcStr>> = v.as_array().unwrap()[1..]
        .iter()
        .map(|x| translate_class_expression(x))
        .collect();

    ClassExpression::ObjectUnionOf(operands)
}

pub fn translate_object_one_of(v: &Value) -> ClassExpression<ArcStr> {
    let operands: Vec<Individual<ArcStr>> = v.as_array().unwrap()[1..]
        .iter()
        .map(|x| translate_individual(x))
        .collect();

    ClassExpression::ObjectOneOf(operands)
}

pub fn translate_object_complement_of(v: &Value) -> ClassExpression<ArcStr> {
    let argument: ClassExpression<ArcStr> = translate_class_expression(&v[1]);

    ClassExpression::ObjectComplementOf(Box::new(argument))
}

pub fn translate_data_some_values_from(v: &Value) -> ClassExpression<ArcStr> {
    let property = translate_data_property(&v[1]);
    let filler: DataRange<ArcStr> = translate_data_range(&v[2]);

    ClassExpression::DataSomeValuesFrom {
        dp: property,
        dr: filler,
    }
}

pub fn translate_data_all_values_from(v: &Value) -> ClassExpression<ArcStr> {
    let property = translate_data_property(&v[1]);
    let filler: DataRange<ArcStr> = translate_data_range(&v[2]);

    ClassExpression::DataAllValuesFrom {
        dp: property,
        dr: filler,
    }
}

pub fn translate_data_has_value(v: &Value) -> ClassExpression<ArcStr> {
    let property = translate_data_property(&v[1]);
    let filler: Literal<ArcStr> = translate_literal(&v[2]);

    ClassExpression::DataHasValue {
        dp: property,
        l: filler,
    }
}

pub fn translate_data_min_cardinality(v: &Value) -> ClassExpression<ArcStr> {
    let cardinality = parse_number_cardinality(&v[1]);

    let property = translate_data_property(&v[2]);

    let ofn = v.as_array().unwrap();
    let is_qualified = ofn.len() == 4;

    let filler = if is_qualified {
        translate_data_range(&v[3])
    } else {
        default_data_filler()
    };

    //let filler: DataRange = DataRange::Datatype(b.datatype("rdfs:Literal"));

    ClassExpression::DataMinCardinality {
        n: cardinality as u32,
        dp: property,
        dr: filler,
    }
}

pub fn translate_data_max_cardinality(v: &Value) -> ClassExpression<ArcStr> {
    let cardinality = parse_number_cardinality(&v[1]);

    let property = translate_data_property(&v[2]);

    let ofn = v.as_array().unwrap();
    let is_qualified = ofn.len() == 4;

    let filler = if is_qualified {
        translate_data_range(&v[3])
    } else {
        default_data_filler()
    };

    ClassExpression::DataMaxCardinality {
        n: cardinality as u32,
        dp: property,
        dr: filler,
    }
}

pub fn translate_data_exact_cardinality(v: &Value) -> ClassExpression<ArcStr> {
    let cardinality = parse_number_cardinality(&v[1]);

    let property = translate_data_property(&v[2]);

    let ofn = v.as_array().unwrap();
    let is_qualified = ofn.len() == 4;
    let filler = if is_qualified {
        translate_data_range(&v[3])
    } else {
        default_data_filler()
    };

    ClassExpression::DataExactCardinality {
        n: cardinality as u32,
        dp: property,
        dr: filler,
    }
}
