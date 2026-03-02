use anyhow::{bail, Context, Result};
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

pub fn translate_object_property_expression(v: &Value) -> Result<ObjectPropertyExpression<ArcStr>> {
    match v[0].as_str() {
        Some("InverseOf") => translate_inverse_of(v),
        Some("ObjectInverseOf") => translate_inverse_of(v),
        None => translate_named_object_property(v),
        Some(op) => bail!("Incorrect Property Constructor: {}", op),
    }
}

pub fn translate_sub_object_property_expression(v: &Value) -> Result<SubObjectPropertyExpression<ArcStr>> {
    match v {
        Value::Array(array) => {
            let operands: Vec<ObjectPropertyExpression<ArcStr>> = array[1..]
                .iter()
                .map(|x| translate_object_property_expression(x))
                .collect::<Result<_>>()?;
            Ok(SubObjectPropertyExpression::ObjectPropertyChain(operands))
        }
        Value::String(_s) => {
            let property = translate_object_property_expression(v)?;
            Ok(SubObjectPropertyExpression::ObjectPropertyExpression(property))
        }
        _ => bail!("Expected array or string for sub-object-property expression"),
    }
}

pub fn translate_class_expression(v: &Value) -> Result<ClassExpression<ArcStr>> {
    match v[0].as_str() {
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

        Some(op) => bail!("Not a valid class expression operator: {}", op),
        None => translate_named_class(v),
    }
}

pub fn translate_literal_string(s: &str) -> Result<Literal<ArcStr>> {
    if let Some(x) = LANGUAGE_TAG_RE.captures(s) {
        Ok(Literal::Language {
            literal: String::from(&x[1]),
            lang: String::from(&x[2]),
        })
    } else if let Some(x) = DATATYPE_RE.captures(s) {
        Ok(Literal::Datatype {
            literal: String::from(&x[1]),
            datatype_iri: build().iri(&x[2]),
        })
    } else if let Some(x) = SIMPLE_LITERAL_RE.captures(s) {
        Ok(Literal::Simple {
            literal: String::from(&x[1]),
        })
    } else {
        bail!("Not a valid literal: {}", s)
    }
}

pub fn translate_literal(v: &Value) -> Result<Literal<ArcStr>> {
    match v.as_str() {
        Some(x) => translate_literal_string(x),
        None => bail!("Expected a string for literal, got: {}", v),
    }
}

pub fn translate_data_range(v: &Value) -> Result<DataRange<ArcStr>> {
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
                Some(op) => bail!("Not a valid data range operator: {}", op),
                None => bail!("Expected a data range operator string"),
            }
        }
        _ => bail!("Expected string or array for data range, got: {}", v),
    }
}

pub fn translate_data_one_of(v: &Value) -> Result<DataRange<ArcStr>> {
    let operands: Vec<Literal<ArcStr>> = v.as_array()
        .context("Expected array for DataOneOf")?[1..]
        .iter()
        .map(|x| translate_literal(x))
        .collect::<Result<_>>()?;
    Ok(DataRange::DataOneOf(operands))
}

pub fn translate_data_complement_of(v: &Value) -> Result<DataRange<ArcStr>> {
    let argument: DataRange<ArcStr> = translate_data_range(&v[1])?;
    Ok(DataRange::DataComplementOf(Box::new(argument)))
}

pub fn translate_data_intersection_of(v: &Value) -> Result<DataRange<ArcStr>> {
    let operands: Vec<DataRange<ArcStr>> = v.as_array()
        .context("Expected array for DataIntersectionOf")?[1..]
        .iter()
        .map(|x| translate_data_range(x))
        .collect::<Result<_>>()?;

    Ok(DataRange::DataIntersectionOf(operands))
}

pub fn translate_data_union_of(v: &Value) -> Result<DataRange<ArcStr>> {
    let operands: Vec<DataRange<ArcStr>> = v.as_array()
        .context("Expected array for DataUnionOf")?[1..]
        .iter()
        .map(|x| translate_data_range(x))
        .collect::<Result<_>>()?;

    Ok(DataRange::DataUnionOf(operands))
}

pub fn translate_datatype(v: &Value) -> Result<Datatype<ArcStr>> {
    Ok(build().datatype(extract_iri_str(v)?).into())
}

pub fn translate_datatype_as_range(v: &Value) -> Result<DataRange<ArcStr>> {
    Ok(DataRange::Datatype(build().datatype(extract_iri_str(v)?)))
}

pub fn translate_named_object_property(v: &Value) -> Result<ObjectPropertyExpression<ArcStr>> {
    Ok(build().object_property(extract_iri_str(v)?).into())
}

pub fn translate_data_property(v: &Value) -> Result<DataProperty<ArcStr>> {
    Ok(build().data_property(extract_iri_str(v)?).into())
}

pub fn translate_inverse_of(v: &Value) -> Result<ObjectPropertyExpression<ArcStr>> {
    let argument = build().object_property(extract_iri_str(&v[1])?).into();
    Ok(ObjectPropertyExpression::InverseObjectProperty { 0: argument })
}

pub fn translate_named_class(v: &Value) -> Result<ClassExpression<ArcStr>> {
    Ok(build().class(extract_iri_str(v)?).into())
}

pub fn translate_anonymous_individual(v: &Value) -> Result<AnonymousIndividual<ArcStr>> {
    let rc: Arc<str> = Arc::from(extract_iri_str(v)?);
    Ok(AnonymousIndividual { 0: rc })
}

pub fn translate_individual(v: &Value) -> Result<Individual<ArcStr>> {
    //TODO: handle anonymous individuals
    Ok(build().named_individual(extract_iri_str(v)?).into())
}

pub fn translate_object_some_values_from(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let property = translate_object_property_expression(&v[1])?;
    let filler = translate_class_expression(&v[2])?;

    Ok(ClassExpression::ObjectSomeValuesFrom {
        ope: property,
        bce: Box::new(filler),
    })
}

pub fn translate_object_all_values_from(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let property = translate_object_property_expression(&v[1])?;
    let filler = translate_class_expression(&v[2])?;

    Ok(ClassExpression::ObjectAllValuesFrom {
        ope: property,
        bce: Box::new(filler),
    })
}

pub fn translate_object_has_value(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let property = translate_object_property_expression(&v[1])?;
    let individual = translate_individual(&v[2])?;

    Ok(ClassExpression::ObjectHasValue {
        ope: property,
        i: individual,
    })
}

pub fn translate_object_min_cardinality(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let cardinality = parse_string_cardinality(&v[1])?;
    let property = translate_object_property_expression(&v[2])?;

    let ofn = v.as_array().context("Expected array for ObjectMinCardinality")?;
    let is_qualified = ofn.len() == 4;

    let filler = if is_qualified {
        translate_class_expression(&v[3])?
    } else {
        default_class_filler()
    };

    Ok(ClassExpression::ObjectMinCardinality {
        n: cardinality as u32,
        ope: property,
        bce: Box::new(filler),
    })
}

pub fn translate_object_max_cardinality(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let cardinality = parse_string_cardinality(&v[1])?;
    let property = translate_object_property_expression(&v[2])?;

    let ofn = v.as_array().context("Expected array for ObjectMaxCardinality")?;
    let is_qualified = ofn.len() == 4;

    let filler = if is_qualified {
        translate_class_expression(&v[3])?
    } else {
        default_class_filler()
    };

    Ok(ClassExpression::ObjectMaxCardinality {
        n: cardinality as u32,
        ope: property,
        bce: Box::new(filler),
    })
}

pub fn translate_object_exact_cardinality(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let cardinality = parse_string_cardinality(&v[1])?;
    let property = translate_object_property_expression(&v[2])?;

    let ofn = v.as_array().context("Expected array for ObjectExactCardinality")?;
    let is_qualified = ofn.len() == 4;

    let filler = if is_qualified {
        translate_class_expression(&v[3])?
    } else {
        default_class_filler()
    };

    Ok(ClassExpression::ObjectExactCardinality {
        n: cardinality as u32,
        ope: property,
        bce: Box::new(filler),
    })
}

pub fn translate_object_has_self(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let property = translate_object_property_expression(&v[1])?;
    Ok(ClassExpression::ObjectHasSelf(property))
}

pub fn translate_object_intersection_of(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let operands: Vec<ClassExpression<ArcStr>> = v.as_array()
        .context("Expected array for ObjectIntersectionOf")?[1..]
        .iter()
        .map(|x| translate_class_expression(x))
        .collect::<Result<_>>()?;

    Ok(ClassExpression::ObjectIntersectionOf(operands))
}

pub fn translate_object_union_of(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let operands: Vec<ClassExpression<ArcStr>> = v.as_array()
        .context("Expected array for ObjectUnionOf")?[1..]
        .iter()
        .map(|x| translate_class_expression(x))
        .collect::<Result<_>>()?;

    Ok(ClassExpression::ObjectUnionOf(operands))
}

pub fn translate_object_one_of(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let operands: Vec<Individual<ArcStr>> = v.as_array()
        .context("Expected array for ObjectOneOf")?[1..]
        .iter()
        .map(|x| translate_individual(x))
        .collect::<Result<_>>()?;

    Ok(ClassExpression::ObjectOneOf(operands))
}

pub fn translate_object_complement_of(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let argument = translate_class_expression(&v[1])?;
    Ok(ClassExpression::ObjectComplementOf(Box::new(argument)))
}

pub fn translate_data_some_values_from(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let property = translate_data_property(&v[1])?;
    let filler = translate_data_range(&v[2])?;

    Ok(ClassExpression::DataSomeValuesFrom {
        dp: property,
        dr: filler,
    })
}

pub fn translate_data_all_values_from(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let property = translate_data_property(&v[1])?;
    let filler = translate_data_range(&v[2])?;

    Ok(ClassExpression::DataAllValuesFrom {
        dp: property,
        dr: filler,
    })
}

pub fn translate_data_has_value(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let property = translate_data_property(&v[1])?;
    let filler = translate_literal(&v[2])?;

    Ok(ClassExpression::DataHasValue {
        dp: property,
        l: filler,
    })
}

pub fn translate_data_min_cardinality(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let cardinality = parse_number_cardinality(&v[1])?;
    let property = translate_data_property(&v[2])?;

    let ofn = v.as_array().context("Expected array for DataMinCardinality")?;
    let is_qualified = ofn.len() == 4;

    let filler = if is_qualified {
        translate_data_range(&v[3])?
    } else {
        default_data_filler()
    };

    Ok(ClassExpression::DataMinCardinality {
        n: cardinality as u32,
        dp: property,
        dr: filler,
    })
}

pub fn translate_data_max_cardinality(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let cardinality = parse_number_cardinality(&v[1])?;
    let property = translate_data_property(&v[2])?;

    let ofn = v.as_array().context("Expected array for DataMaxCardinality")?;
    let is_qualified = ofn.len() == 4;

    let filler = if is_qualified {
        translate_data_range(&v[3])?
    } else {
        default_data_filler()
    };

    Ok(ClassExpression::DataMaxCardinality {
        n: cardinality as u32,
        dp: property,
        dr: filler,
    })
}

pub fn translate_data_exact_cardinality(v: &Value) -> Result<ClassExpression<ArcStr>> {
    let cardinality = parse_number_cardinality(&v[1])?;
    let property = translate_data_property(&v[2])?;

    let ofn = v.as_array().context("Expected array for DataExactCardinality")?;
    let is_qualified = ofn.len() == 4;
    let filler = if is_qualified {
        translate_data_range(&v[3])?
    } else {
        default_data_filler()
    };

    Ok(ClassExpression::DataExactCardinality {
        n: cardinality as u32,
        dp: property,
        dr: filler,
    })
}
