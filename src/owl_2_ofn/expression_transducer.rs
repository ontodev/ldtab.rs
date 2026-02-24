use horned_owl::model::{
    AnonymousIndividual, Atom, Class, ClassExpression, DArgument, DataProperty, DataRange,
    Datatype, FacetRestriction, IArgument, Individual, Literal, NamedIndividual, ObjectProperty,
    ObjectPropertyExpression, PropertyExpression, ArcStr, SubObjectPropertyExpression,
};
use horned_owl::vocab::Facet;
use serde_json::json;
use serde_json::Value;

use crate::owl_2_ofn::util::format_iri;

pub fn translate_sub_object_property_expression(
    expression: &SubObjectPropertyExpression<ArcStr>,
) -> Value {
    match expression {
        SubObjectPropertyExpression::ObjectPropertyChain(x) => {
            let operator = Value::String(String::from("ObjectPropertyChain"));
            let mut operands: Vec<Value> = x
                .into_iter()
                .map(|x| translate_object_property_expression(&x))
                .collect();
            operands.insert(0, operator);
            Value::Array(operands)
        }
        SubObjectPropertyExpression::ObjectPropertyExpression(x) => {
            translate_object_property_expression(&x)
        }
    }
}

pub fn translate_property_expression(expression: &PropertyExpression<ArcStr>) -> Value {
    match expression {
        PropertyExpression::ObjectPropertyExpression(x) => translate_object_property_expression(&x),
        PropertyExpression::DataProperty(x) => translate_data_property(&x),
        PropertyExpression::AnnotationProperty(_x) => json!("TODO"), //TODO
    }
}

pub fn translate_object_property_expression(expression: &ObjectPropertyExpression<ArcStr>) -> Value {
    match expression {
        ObjectPropertyExpression::ObjectProperty(x) => translate_object_property(&x),
        ObjectPropertyExpression::InverseObjectProperty(x) => translate_inverse_object_property(&x),
    }
}

pub fn translate_inverse_object_property(property: &ObjectProperty<ArcStr>) -> Value {
    let operator = Value::String(String::from("ObjectInverseOf"));
    let mut res = vec![operator];

    //let operand = Value::String(String::from(property.0.get(0..).unwrap()));
    let operand = translate_object_property(&property);

    res.push(operand);

    Value::Array(res)
}

pub fn translate_object_property(property: &ObjectProperty<ArcStr>) -> Value {
    format_iri(property.0.get(0..).unwrap())
}

pub fn translate_data_property(property: &DataProperty<ArcStr>) -> Value {
    format_iri(property.0.get(0..).unwrap())
}

pub fn translate_class(class: &Class<ArcStr>) -> Value {
    format_iri(class.0.get(0..).unwrap())
}

//TODO: not sure this is correct
pub fn translate_anonymous_individual(a: &AnonymousIndividual<ArcStr>) -> Value {
    format_iri(a.0.get(0..).unwrap())
}

pub fn translate_named_individual(a: &NamedIndividual<ArcStr>) -> Value {
    format_iri(a.0.get(0..).unwrap())
}

//TODO this is an IRI
pub fn translate_individual(individual: &Individual<ArcStr>) -> Value {
    match individual {
        Individual::Anonymous(x) => translate_anonymous_individual(&x),
        Individual::Named(x) => translate_named_individual(&x),
    }
}

pub fn translate_literal(literal: &Literal<ArcStr>) -> Value {
    match literal {
        //we need to use double quotes here to mark a string as a literal
        Literal::Simple { literal } => json!(format!("\"{}\"", literal)),
        //Literal::Simple { literal } => json!(format!("{}", literal)),
        //{ if literal.is_empty() {
        //    json!("") } else {
        //    json!(format!("\"{}\"",literal))}},
        Literal::Language { literal, lang } => json!(format!("\"{}\"@{}", literal, lang)),
        Literal::Datatype {
            literal,
            datatype_iri,
        } => {
            let iri = format!("<{}>", datatype_iri.get(0..).unwrap());
            json!(format!("\"{}\"^^{}", literal, iri))
        }
    }
}

pub fn translate_n_ary_operator(operator: &str, arguments: &Vec<ClassExpression<ArcStr>>) -> Value {
    let mut operands: Vec<Value> = arguments
        .into_iter()
        .map(|x| translate_class_expression(&x))
        .collect();
    let operator = Value::String(String::from(operator));
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_object_one_of(arguments: &Vec<Individual<ArcStr>>) -> Value {
    let mut operands: Vec<Value> = arguments
        .into_iter()
        .map(|x| translate_individual(&x))
        .collect();
    let operator = Value::String(String::from("ObjectOneOf"));
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_object_complement(argument: &Box<ClassExpression<ArcStr>>) -> Value {
    let expression: ClassExpression<ArcStr> = *argument.clone();
    let argument = translate_class_expression(&expression);

    let operator = Value::String(String::from("ObjectComplementOf"));
    let mut res = vec![operator];
    res.push(argument);

    Value::Array(res)
}

pub fn translate_object_some_values_from(
    property: &ObjectPropertyExpression<ArcStr>,
    filler: &Box<ClassExpression<ArcStr>>,
) -> Value {
    let expression: ClassExpression<ArcStr> = *filler.clone();

    let operator = Value::String(String::from("ObjectSomeValuesFrom"));
    let filler = translate_class_expression(&expression);
    let property = translate_object_property_expression(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(filler);

    Value::Array(res)
}

pub fn translate_object_all_values_from(
    property: &ObjectPropertyExpression<ArcStr>,
    filler: &Box<ClassExpression<ArcStr>>,
) -> Value {
    let expression: ClassExpression<ArcStr> = *filler.clone();

    let operator = Value::String(String::from("ObjectAllValuesFrom"));
    let filler = translate_class_expression(&expression);
    let property = translate_object_property_expression(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(filler);

    Value::Array(res)
}

pub fn translate_object_has_value(
    property: &ObjectPropertyExpression<ArcStr>,
    value: &Individual<ArcStr>,
) -> Value {
    let operator = Value::String(String::from("ObjectHasValue"));
    let value = translate_individual(value);
    let property = translate_object_property_expression(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(value);

    Value::Array(res)
}

pub fn translate_object_has_self(property: &ObjectPropertyExpression<ArcStr>) -> Value {
    let operator = Value::String(String::from("ObjectHasSelf"));
    let property = translate_object_property_expression(property);

    let mut res = vec![operator];
    res.push(property);

    Value::Array(res)
}

fn is_owl_thing(value: &Value) -> bool {
    // OWL:Thing as a string (IRI)
    let owl_thing_iri = "<http://www.w3.org/2002/07/owl#Thing>";

    // Check if the serde_json::Value is a string and compare it to OWL:Thing IRI
    match value {
        Value::String(iri) => iri == owl_thing_iri,
        _ => false,
    }
}

pub fn translate_object_cardinality(
    operator: &str,
    cardinality: &u32,
    property: &ObjectPropertyExpression<ArcStr>,
    filler: &Box<ClassExpression<ArcStr>>,
) -> Value {
    let expression: ClassExpression<ArcStr> = *filler.clone();

    let operator = Value::String(String::from(operator));
    let cardinality = json!(cardinality.to_string());
    let filler = translate_class_expression(&expression);
    let property = translate_object_property_expression(property);

    let mut res = vec![operator];
    res.push(cardinality);
    res.push(property);

    if filler != Value::Null && !is_owl_thing(&filler) {
        res.push(filler);
    }

    Value::Array(res)
}

pub fn translate_datatype(datatype: &Datatype<ArcStr>) -> Value {
    format_iri(datatype.0.get(0..).unwrap())
}

pub fn translate_data_intersection_of(arguments: &Vec<DataRange<ArcStr>>) -> Value {
    let mut operands: Vec<Value> = arguments
        .into_iter()
        .map(|x| translate_data_range(&x))
        .collect();
    let operator = Value::String(String::from("DataIntersectionOf"));
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_data_union_of(arguments: &Vec<DataRange<ArcStr>>) -> Value {
    let mut operands: Vec<Value> = arguments
        .into_iter()
        .map(|x| translate_data_range(&x))
        .collect();
    let operator = Value::String(String::from("DataUnionOf"));
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_data_complement_of(argument: &Box<DataRange<ArcStr>>) -> Value {
    let range: DataRange<ArcStr> = *argument.clone();
    let argument = translate_data_range(&range);

    let operator = Value::String(String::from("DataComplementOf"));
    let mut res = vec![operator];
    res.push(argument);

    Value::Array(res)
}

pub fn translate_data_one_of(arguments: &Vec<Literal<ArcStr>>) -> Value {
    let mut operands: Vec<Value> = arguments
        .into_iter()
        .map(|x| translate_literal(&x))
        .collect();

    let operator = Value::String(String::from("DataOneOf"));
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_facet(facet: &Facet) -> Value {
    match facet {
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

pub fn translate_facet_restriction(facet_restriction: &FacetRestriction<ArcStr>) -> Value {
    let operator = Value::String(String::from("FacetRestriction"));

    let facet = facet_restriction.f.clone();
    let facet = translate_facet(&facet);

    let literal = facet_restriction.l.clone();
    let literal = translate_literal(&literal);

    let mut res = vec![operator];
    res.push(facet);
    res.push(literal);

    Value::Array(res)
}

pub fn translate_datatype_restriction(
    datatype: &Datatype<ArcStr>,
    facets: &Vec<FacetRestriction<ArcStr>>,
) -> Value {
    let operator = Value::String(String::from("DatatypeRestriction"));
    let datatype = translate_datatype(datatype);
    let mut operands: Vec<Value> = facets
        .into_iter()
        .map(|x| translate_facet_restriction(&x))
        .collect();

    operands.insert(0, datatype);
    operands.insert(0, operator);
    Value::Array(operands)
}

pub fn translate_data_range(range: &DataRange<ArcStr>) -> Value {
    match range {
        DataRange::Datatype(x) => translate_datatype(&x),
        DataRange::DataIntersectionOf(x) => translate_data_intersection_of(&x),
        DataRange::DataUnionOf(x) => translate_data_union_of(&x),
        DataRange::DataComplementOf(x) => translate_data_complement_of(&x),
        DataRange::DataOneOf(x) => translate_data_one_of(&x),
        DataRange::DatatypeRestriction(datatype, facets) => {
            translate_datatype_restriction(&datatype, &facets)
        }
    }
}

pub fn translate_data_some_values_from(
    property: &DataProperty<ArcStr>,
    filler: &DataRange<ArcStr>,
) -> Value {
    let operator = Value::String(String::from("DataSomeValuesFrom"));
    let filler = translate_data_range(&filler);
    let property = translate_data_property(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(filler);

    Value::Array(res)
}

pub fn translate_data_all_values_from(
    property: &DataProperty<ArcStr>,
    filler: &DataRange<ArcStr>,
) -> Value {
    let operator = Value::String(String::from("DataAllValuesFrom"));
    let filler = translate_data_range(&filler);
    let property = translate_data_property(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(filler);

    Value::Array(res)
}

pub fn translate_data_has_value(property: &DataProperty<ArcStr>, literal: &Literal<ArcStr>) -> Value {
    let operator = Value::String(String::from("DataHasValue"));
    let value = translate_literal(literal);
    let property = translate_data_property(property);

    let mut res = vec![operator];
    res.push(property);
    res.push(value);

    Value::Array(res)
}

fn is_rdfs_literal(value: &Value) -> bool {
    // RDFS:Literal as a string (IRI)
    let rdfs_literal_iri = "<http://www.w3.org/2000/01/rdf-schema#Literal>";

    // Check if the serde_json::Value is a string and compare it to RDFS:Literal IRI
    match value {
        Value::String(iri) => iri == rdfs_literal_iri,
        _ => false,
    }
}

pub fn translate_data_cardinality(
    operator: &str,
    cardinality: &u32,
    property: &DataProperty<ArcStr>,
    filler: &DataRange<ArcStr>,
) -> Value {
    let operator = Value::String(String::from(operator));
    let cardinality = json!(cardinality.to_string());
    let filler = translate_data_range(filler);
    let property = translate_data_property(property);

    let mut res = vec![operator];
    res.push(cardinality);
    res.push(property);

    if filler != Value::Null && !is_rdfs_literal(&filler) {
        res.push(filler);
    }

    Value::Array(res)
}

pub fn translate_atom(atom: &Atom<ArcStr>) -> Value {
    match atom {
        Atom::BuiltInAtom { pred, args } => {
            let operator = Value::String(String::from("BuiltInAtom"));

            let a = pred.get(0..);
            let pred = Value::String(String::from(a.unwrap()));
            let mut dargs = Vec::new();
            for darg in args {
                dargs.push(translate_darg(darg));
            }

            let mut v = vec![operator, pred];
            v.extend(dargs);
            Value::Array(v)
        }
        Atom::ClassAtom { pred, arg } => {
            let operator = Value::String(String::from("ClassAtom"));
            let expression = translate_class_expression(pred);
            let arg = translate_iarg(arg);

            let v = vec![operator, expression, arg];
            Value::Array(v)
        }
        Atom::DataPropertyAtom { pred, args } => {
            let operator = Value::String(String::from("DataPropertyAtom"));
            let property = translate_data_property(pred);
            let arg1 = translate_darg(&args.0);
            let arg2 = translate_darg(&args.1);

            let v = vec![operator, property, arg1, arg2];
            Value::Array(v)
        }
        Atom::DataRangeAtom { pred, arg } => {
            let operator = Value::String(String::from("DataRangeAtom"));
            let range = translate_data_range(pred);
            let arg = translate_darg(arg);

            let v = vec![operator, range, arg];
            Value::Array(v)
        }
        Atom::DifferentIndividualsAtom(arg1, arg2) => {
            let operator = Value::String(String::from("DifferentIndividualsAtom"));
            let v = vec![operator, translate_iarg(arg1), translate_iarg(arg2)];
            Value::Array(v)
        }
        Atom::ObjectPropertyAtom { pred, args } => {
            let operator = Value::String(String::from("ObjectPropertyAtom"));
            let property = translate_object_property_expression(pred);
            let arg1 = translate_iarg(&args.0);
            let arg2 = translate_iarg(&args.1);

            let v = vec![operator, property, arg1, arg2];
            Value::Array(v)
        }
        Atom::SameIndividualAtom(arg1, arg2) => {
            let operator = Value::String(String::from("SameIndividualAtom"));
            let v = vec![operator, translate_iarg(arg1), translate_iarg(arg2)];
            Value::Array(v)
        }
    }
}

pub fn translate_iarg(iarg: &IArgument<ArcStr>) -> Value {
    match iarg {
        IArgument::Variable(x) => {
            let operator = Value::String(String::from("Variable"));
            let v = vec![operator, Value::String(String::from(x))];
            Value::Array(v)
        }
        IArgument::Individual(x) => translate_individual(x),
    }
}

pub fn translate_darg(darg: &DArgument<ArcStr>) -> Value {
    match darg {
        DArgument::Variable(x) => {
            let operator = Value::String(String::from("Variable"));
            let v = vec![operator, Value::String(String::from(x))];
            Value::Array(v)
        }
        DArgument::Literal(x) => translate_literal(x),
    }
}

pub fn translate_class_expression(expression: &ClassExpression<ArcStr>) -> Value {
    match expression {
        ClassExpression::Class(x) => translate_class(&x),
        ClassExpression::ObjectIntersectionOf(x) => {
            translate_n_ary_operator("ObjectIntersectionOf", &x)
        }
        ClassExpression::ObjectUnionOf(x) => translate_n_ary_operator("ObjectUnionOf", &x),
        ClassExpression::ObjectComplementOf(x) => translate_object_complement(x),
        ClassExpression::ObjectOneOf(x) => translate_object_one_of(&x),
        ClassExpression::ObjectSomeValuesFrom { ope, bce } => {
            translate_object_some_values_from(&ope, bce)
        }
        ClassExpression::ObjectAllValuesFrom { ope, bce } => {
            translate_object_all_values_from(&ope, bce)
        }
        ClassExpression::ObjectHasValue { ope, i } => translate_object_has_value(&ope, &i),
        ClassExpression::ObjectHasSelf(p) => translate_object_has_self(&p),
        ClassExpression::ObjectMinCardinality { n, ope, bce } => {
            translate_object_cardinality("ObjectMinCardinality", &n, &ope, bce)
        }
        ClassExpression::ObjectMaxCardinality { n, ope, bce } => {
            translate_object_cardinality("ObjectMaxCardinality", &n, &ope, bce)
        }
        ClassExpression::ObjectExactCardinality { n, ope, bce } => {
            translate_object_cardinality("ObjectExactCardinality", &n, &ope, bce)
        }
        ClassExpression::DataSomeValuesFrom { dp, dr } => translate_data_some_values_from(&dp, &dr),
        ClassExpression::DataAllValuesFrom { dp, dr } => translate_data_all_values_from(&dp, &dr),
        ClassExpression::DataHasValue { dp, l } => translate_data_has_value(&dp, &l),
        ClassExpression::DataMinCardinality { n, dp, dr } => {
            translate_data_cardinality("DataMinCardinality", &n, &dp, &dr)
        }
        ClassExpression::DataMaxCardinality { n, dp, dr } => {
            translate_data_cardinality("DataMaxCardinality", &n, &dp, &dr)
        }
        ClassExpression::DataExactCardinality { n, dp, dr } => {
            translate_data_cardinality("DataExactCardinality", &n, &dp, &dr)
        }
    }
}
