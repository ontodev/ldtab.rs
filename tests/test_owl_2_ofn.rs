use anyhow::{Context, Result};
use horned_owl::model::{
    AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationPropertyDomain,
    AnnotationPropertyRange, AnnotationSubject, AnnotationValue, ArcStr, AsymmetricObjectProperty,
    Build, ClassAssertion, ClassExpression, Component, DataPropertyAssertion, DataPropertyDomain,
    DataPropertyRange, DataRange, DatatypeDefinition, DeclareAnnotationProperty, DeclareClass,
    DeclareDataProperty, DeclareDatatype, DeclareNamedIndividual, DeclareObjectProperty,
    DifferentIndividuals, DisjointClasses, DisjointDataProperties, DisjointObjectProperties,
    DisjointUnion, DocIRI, EquivalentClasses, EquivalentDataProperties, EquivalentObjectProperties,
    FacetRestriction, FunctionalDataProperty, FunctionalObjectProperty, HasKey, Import,
    InverseFunctionalObjectProperty, InverseObjectProperties, IrreflexiveObjectProperty, Literal,
    NegativeDataPropertyAssertion, NegativeObjectPropertyAssertion, ObjectPropertyAssertion,
    ObjectPropertyDomain, ObjectPropertyExpression, ObjectPropertyRange, OntologyAnnotation,
    OntologyID, PropertyExpression, ReflexiveObjectProperty, SameIndividual, SubAnnotationPropertyOf,
    SubClassOf, SubDataPropertyOf, SubObjectPropertyExpression, SubObjectPropertyOf,
    SymmetricObjectProperty, TransitiveObjectProperty,
};
use horned_owl::vocab::Facet;
use std::collections::BTreeSet;

fn assert_roundtrip(axiom: AnnotatedComponent<ArcStr>) -> Result<()> {
    let original_ofn = ldtab_rs::owl_2_ofn::transducer::translate(&axiom);
    let roundtripped_axiom = ldtab_rs::ofn_2_owl::transducer::translate(&original_ofn)
        .with_context(|| format!("failed to parse OFN: {original_ofn}"))?;
    let roundtripped_ofn = ldtab_rs::owl_2_ofn::transducer::translate(&roundtripped_axiom);

    assert_eq!(roundtripped_axiom, axiom);
    assert_eq!(roundtripped_ofn, original_ofn);
    Ok(())
}

fn plain(component: Component<ArcStr>) -> AnnotatedComponent<ArcStr> {
    AnnotatedComponent {
        component,
        ann: BTreeSet::new(),
    }
}

fn run_cases(cases: Vec<(&'static str, AnnotatedComponent<ArcStr>)>) -> Result<()> {
    for (name, axiom) in cases {
        assert_roundtrip(axiom).with_context(|| format!("roundtrip case failed: {name}"))?;
    }
    Ok(())
}

#[test]
fn roundtrip_declaration_axioms() -> Result<()> {
    let b = Build::new();
    run_cases(vec![
        (
            "DeclareClass",
            plain(Component::DeclareClass(DeclareClass(
                b.class("http://example.org/C1"),
            ))),
        ),
        (
            "DeclareObjectProperty",
            plain(Component::DeclareObjectProperty(DeclareObjectProperty(
                b.object_property("http://example.org/p1"),
            ))),
        ),
        (
            "DeclareDataProperty",
            plain(Component::DeclareDataProperty(DeclareDataProperty(
                b.data_property("http://example.org/dp1"),
            ))),
        ),
        (
            "DeclareAnnotationProperty",
            plain(Component::DeclareAnnotationProperty(DeclareAnnotationProperty(
                b.annotation_property("http://example.org/ap1"),
            ))),
        ),
        (
            "DeclareNamedIndividual",
            plain(Component::DeclareNamedIndividual(DeclareNamedIndividual(
                b.named_individual("http://example.org/i1"),
            ))),
        ),
        (
            "DeclareDatatype",
            plain(Component::DeclareDatatype(DeclareDatatype(
                b.datatype("http://example.org/dt1"),
            ))),
        ),
    ])
}

#[test]
fn roundtrip_class_axioms() -> Result<()> {
    let b = Build::new();
    run_cases(vec![
        (
            "SubClassOf",
            plain(Component::SubClassOf(SubClassOf {
                sub: b.class("http://example.org/C1").into(),
                sup: b.class("http://example.org/C2").into(),
            })),
        ),
        (
            "EquivalentClasses",
            plain(Component::EquivalentClasses(EquivalentClasses(vec![
                b.class("http://example.org/C1").into(),
                b.class("http://example.org/C2").into(),
                b.class("http://example.org/C3").into(),
            ]))),
        ),
        (
            "DisjointClasses",
            plain(Component::DisjointClasses(DisjointClasses(vec![
                b.class("http://example.org/C1").into(),
                b.class("http://example.org/C2").into(),
                b.class("http://example.org/C3").into(),
            ]))),
        ),
        (
            "DisjointUnion",
            plain(Component::DisjointUnion(DisjointUnion(
                b.class("http://example.org/C0"),
                vec![
                    b.class("http://example.org/C1").into(),
                    b.class("http://example.org/C2").into(),
                    b.class("http://example.org/C3").into(),
                ],
            ))),
        ),
    ])
}

#[test]
fn roundtrip_object_property_axioms() -> Result<()> {
    let b = Build::new();
    run_cases(vec![
        (
            "SubObjectPropertyOf-simple",
            plain(Component::SubObjectPropertyOf(SubObjectPropertyOf {
                sub: SubObjectPropertyExpression::ObjectPropertyExpression(
                    b.object_property("http://example.org/p1").into(),
                ),
                sup: b.object_property("http://example.org/p2").into(),
            })),
        ),
        (
            "SubObjectPropertyOf-chain",
            plain(Component::SubObjectPropertyOf(SubObjectPropertyOf {
                sub: SubObjectPropertyExpression::ObjectPropertyChain(vec![
                    b.object_property("http://example.org/p1").into(),
                    b.object_property("http://example.org/p2").into(),
                ]),
                sup: b.object_property("http://example.org/p3").into(),
            })),
        ),
        (
            "SubObjectPropertyOf-inverse-sub",
            plain(Component::SubObjectPropertyOf(SubObjectPropertyOf {
                sub: SubObjectPropertyExpression::ObjectPropertyExpression(
                    ObjectPropertyExpression::InverseObjectProperty(
                        b.object_property("http://example.org/p1"),
                    ),
                ),
                sup: b.object_property("http://example.org/p2").into(),
            })),
        ),
        (
            "EquivalentObjectProperties",
            plain(Component::EquivalentObjectProperties(EquivalentObjectProperties(
                vec![
                    b.object_property("http://example.org/p1").into(),
                    b.object_property("http://example.org/p2").into(),
                    b.object_property("http://example.org/p3").into(),
                ],
            ))),
        ),
        (
            "DisjointObjectProperties",
            plain(Component::DisjointObjectProperties(DisjointObjectProperties(
                vec![
                    b.object_property("http://example.org/p1").into(),
                    b.object_property("http://example.org/p2").into(),
                ],
            ))),
        ),
        (
            "InverseObjectProperties",
            plain(Component::InverseObjectProperties(InverseObjectProperties(
                b.object_property("http://example.org/p1"),
                b.object_property("http://example.org/p2"),
            ))),
        ),
        (
            "ObjectPropertyDomain",
            plain(Component::ObjectPropertyDomain(ObjectPropertyDomain {
                ope: b.object_property("http://example.org/p1").into(),
                ce: b.class("http://example.org/C1").into(),
            })),
        ),
        (
            "ObjectPropertyRange",
            plain(Component::ObjectPropertyRange(ObjectPropertyRange {
                ope: b.object_property("http://example.org/p1").into(),
                ce: b.class("http://example.org/C2").into(),
            })),
        ),
        (
            "FunctionalObjectProperty",
            plain(Component::FunctionalObjectProperty(FunctionalObjectProperty(
                b.object_property("http://example.org/p1").into(),
            ))),
        ),
        (
            "InverseFunctionalObjectProperty",
            plain(Component::InverseFunctionalObjectProperty(
                InverseFunctionalObjectProperty(
                    b.object_property("http://example.org/p1").into(),
                ),
            )),
        ),
        (
            "ReflexiveObjectProperty",
            plain(Component::ReflexiveObjectProperty(ReflexiveObjectProperty(
                b.object_property("http://example.org/p1").into(),
            ))),
        ),
        (
            "IrreflexiveObjectProperty",
            plain(Component::IrreflexiveObjectProperty(IrreflexiveObjectProperty(
                b.object_property("http://example.org/p1").into(),
            ))),
        ),
        (
            "SymmetricObjectProperty",
            plain(Component::SymmetricObjectProperty(SymmetricObjectProperty(
                b.object_property("http://example.org/p1").into(),
            ))),
        ),
        (
            "AsymmetricObjectProperty",
            plain(Component::AsymmetricObjectProperty(AsymmetricObjectProperty(
                b.object_property("http://example.org/p1").into(),
            ))),
        ),
        (
            "TransitiveObjectProperty",
            plain(Component::TransitiveObjectProperty(TransitiveObjectProperty(
                b.object_property("http://example.org/p1").into(),
            ))),
        ),
    ])
}

#[test]
fn roundtrip_data_property_axioms() -> Result<()> {
    let b = Build::new();
    run_cases(vec![
        (
            "SubDataPropertyOf",
            plain(Component::SubDataPropertyOf(SubDataPropertyOf {
                sub: b.data_property("http://example.org/dp1"),
                sup: b.data_property("http://example.org/dp2"),
            })),
        ),
        (
            "EquivalentDataProperties",
            plain(Component::EquivalentDataProperties(EquivalentDataProperties(
                vec![
                    b.data_property("http://example.org/dp1"),
                    b.data_property("http://example.org/dp2"),
                    b.data_property("http://example.org/dp3"),
                ],
            ))),
        ),
        (
            "DisjointDataProperties",
            plain(Component::DisjointDataProperties(DisjointDataProperties(
                vec![
                    b.data_property("http://example.org/dp1"),
                    b.data_property("http://example.org/dp2"),
                ],
            ))),
        ),
        (
            "DataPropertyDomain",
            plain(Component::DataPropertyDomain(DataPropertyDomain {
                dp: b.data_property("http://example.org/dp1"),
                ce: b.class("http://example.org/C1").into(),
            })),
        ),
        (
            "DataPropertyRange",
            plain(Component::DataPropertyRange(DataPropertyRange {
                dp: b.data_property("http://example.org/dp1"),
                dr: DataRange::Datatype(b.datatype("http://example.org/dt1")),
            })),
        ),
        (
            "FunctionalDataProperty",
            plain(Component::FunctionalDataProperty(FunctionalDataProperty(
                b.data_property("http://example.org/dp1"),
            ))),
        ),
        (
            "DatatypeDefinition",
            plain(Component::DatatypeDefinition(DatatypeDefinition {
                kind: b.datatype("http://example.org/customDatatype"),
                range: DataRange::DataUnionOf(vec![
                    DataRange::Datatype(b.datatype(
                        "http://www.w3.org/2001/XMLSchema#string",
                    )),
                    DataRange::Datatype(b.datatype(
                        "http://www.w3.org/2001/XMLSchema#integer",
                    )),
                ]),
            })),
        ),
    ])
}

#[test]
fn roundtrip_assertion_axioms() -> Result<()> {
    let b = Build::new();
    run_cases(vec![
        (
            "SameIndividual",
            plain(Component::SameIndividual(SameIndividual(vec![
                b.named_individual("http://example.org/i1").into(),
                b.named_individual("http://example.org/i2").into(),
                b.named_individual("http://example.org/i3").into(),
            ]))),
        ),
        (
            "DifferentIndividuals",
            plain(Component::DifferentIndividuals(DifferentIndividuals(vec![
                b.named_individual("http://example.org/i1").into(),
                b.named_individual("http://example.org/i2").into(),
                b.named_individual("http://example.org/i3").into(),
            ]))),
        ),
        (
            "ClassAssertion",
            plain(Component::ClassAssertion(ClassAssertion {
                ce: ClassExpression::ObjectIntersectionOf(vec![
                    b.class("http://example.org/C1").into(),
                    b.class("http://example.org/C2").into(),
                ]),
                i: b.named_individual("http://example.org/i1").into(),
            })),
        ),
        (
            "ObjectPropertyAssertion",
            plain(Component::ObjectPropertyAssertion(ObjectPropertyAssertion {
                ope: b.object_property("http://example.org/p1").into(),
                from: b.named_individual("http://example.org/i1").into(),
                to: b.named_individual("http://example.org/i2").into(),
            })),
        ),
        (
            "NegativeObjectPropertyAssertion",
            plain(Component::NegativeObjectPropertyAssertion(
                NegativeObjectPropertyAssertion {
                    ope: b.object_property("http://example.org/p1").into(),
                    from: b.named_individual("http://example.org/i1").into(),
                    to: b.named_individual("http://example.org/i2").into(),
                },
            )),
        ),
        (
            "DataPropertyAssertion-simple-literal",
            plain(Component::DataPropertyAssertion(DataPropertyAssertion {
                dp: b.data_property("http://example.org/dp1"),
                from: b.named_individual("http://example.org/i1").into(),
                to: Literal::Simple {
                    literal: "sample".to_string(),
                },
            })),
        ),
        (
            "DataPropertyAssertion-language-literal",
            plain(Component::DataPropertyAssertion(DataPropertyAssertion {
                dp: b.data_property("http://example.org/dp1"),
                from: b.named_individual("http://example.org/i1").into(),
                to: Literal::Language {
                    literal: "bonjour".to_string(),
                    lang: "fr".to_string(),
                },
            })),
        ),
        (
            "DataPropertyAssertion-typed-literal",
            plain(Component::DataPropertyAssertion(DataPropertyAssertion {
                dp: b.data_property("http://example.org/dp1"),
                from: b.named_individual("http://example.org/i1").into(),
                to: Literal::Datatype {
                    literal: "42".to_string(),
                    datatype_iri: b.iri("http://www.w3.org/2001/XMLSchema#integer"),
                },
            })),
        ),
        (
            "NegativeDataPropertyAssertion",
            plain(Component::NegativeDataPropertyAssertion(
                NegativeDataPropertyAssertion {
                    dp: b.data_property("http://example.org/dp1"),
                    from: b.named_individual("http://example.org/i1").into(),
                    to: Literal::Datatype {
                        literal: "0".to_string(),
                        datatype_iri: b.iri("http://www.w3.org/2001/XMLSchema#integer"),
                    },
                },
            )),
        ),
    ])
}

#[test]
fn roundtrip_annotation_axioms() -> Result<()> {
    let b = Build::new();
    run_cases(vec![
        (
            "AnnotationAssertion-iri-value",
            plain(Component::AnnotationAssertion(AnnotationAssertion {
                subject: AnnotationSubject::IRI(b.iri("http://example.org/C1")),
                ann: Annotation {
                    ap: b.annotation_property("http://example.org/ap1"),
                    av: AnnotationValue::IRI(b.iri("http://example.org/reference")),
                },
            })),
        ),
        (
            "AnnotationAssertion-literal-value",
            plain(Component::AnnotationAssertion(AnnotationAssertion {
                subject: AnnotationSubject::IRI(b.iri("http://example.org/C1")),
                ann: Annotation {
                    ap: b.annotation_property("http://example.org/ap1"),
                    av: AnnotationValue::Literal(Literal::Simple {
                        literal: "text".to_string(),
                    }),
                },
            })),
        ),
        (
            "SubAnnotationPropertyOf",
            plain(Component::SubAnnotationPropertyOf(SubAnnotationPropertyOf {
                sub: b.annotation_property("http://example.org/ap1"),
                sup: b.annotation_property("http://example.org/ap2"),
            })),
        ),
        (
            "AnnotationPropertyDomain",
            plain(Component::AnnotationPropertyDomain(AnnotationPropertyDomain {
                ap: b.annotation_property("http://example.org/ap1"),
                iri: b.iri("http://example.org/C1"),
            })),
        ),
        (
            "AnnotationPropertyRange",
            plain(Component::AnnotationPropertyRange(AnnotationPropertyRange {
                ap: b.annotation_property("http://example.org/ap1"),
                iri: b.iri("http://example.org/C2"),
            })),
        ),
    ])?;

    let mut ann = BTreeSet::new();
    ann.insert(Annotation {
        ap: b.annotation_property("http://example.org/ap1"),
        av: AnnotationValue::IRI(b.iri("http://example.org/reference")),
    });
    ann.insert(Annotation {
        ap: b.annotation_property("http://example.org/ap2"),
        av: AnnotationValue::Literal(Literal::Language {
            literal: "hello".to_string(),
            lang: "en".to_string(),
        }),
    });

    assert_roundtrip(AnnotatedComponent {
        component: Component::SubClassOf(SubClassOf {
            sub: b.class("http://example.org/C1").into(),
            sup: b.class("http://example.org/C2").into(),
        }),
        ann,
    })
}

#[test]
fn roundtrip_metadata_and_key_axioms() -> Result<()> {
    let b = Build::new();
    run_cases(vec![
        (
            "Import",
            plain(Component::Import(Import(
                b.iri("http://example.org/imported-ontology"),
            ))),
        ),
        (
            "OntologyID",
            plain(Component::OntologyID(OntologyID {
                iri: Some(b.iri("http://example.org/ontology")),
                viri: Some(b.iri("http://example.org/ontology/1.0.0")),
            })),
        ),
        (
            "DocIRI",
            plain(Component::DocIRI(DocIRI(
                b.iri("http://example.org/ontology.owl"),
            ))),
        ),
        (
            "OntologyAnnotation",
            plain(Component::OntologyAnnotation(OntologyAnnotation(Annotation {
                ap: b.annotation_property("http://example.org/ap1"),
                av: AnnotationValue::IRI(b.iri("http://example.org/reference")),
            }))),
        ),
        (
            "HasKey",
            plain(Component::HasKey(HasKey {
                ce: b.class("http://example.org/C1").into(),
                vpe: vec![
                    PropertyExpression::ObjectPropertyExpression(
                        b.object_property("http://example.org/p1").into(),
                    ),
                    PropertyExpression::DataProperty(
                        b.data_property("http://example.org/dp1"),
                    ),
                ],
            })),
        ),
    ])
}

#[test]
fn roundtrip_class_expression_constructors() -> Result<()> {
    let b = Build::new();
    let c1 = b.class("http://example.org/C1");
    let c2 = b.class("http://example.org/C2");
    let p1 = b.object_property("http://example.org/p1");
    let i1 = b.named_individual("http://example.org/i1");
    let i2 = b.named_individual("http://example.org/i2");
    let dp1 = b.data_property("http://example.org/dp1");
    let xsd_string = b.datatype("http://www.w3.org/2001/XMLSchema#string");
    let xsd_integer = b.datatype("http://www.w3.org/2001/XMLSchema#integer");

    run_cases(vec![
        (
            "ObjectIntersectionOf",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::ObjectIntersectionOf(vec![
                    c1.clone().into(),
                    c2.clone().into(),
                ]),
            })),
        ),
        (
            "ObjectUnionOf",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::ObjectUnionOf(vec![c1.clone().into(), c2.clone().into()]),
            })),
        ),
        (
            "ObjectComplementOf",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::ObjectComplementOf(Box::new(c2.clone().into())),
            })),
        ),
        (
            "ObjectOneOf",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::ObjectOneOf(vec![i1.clone().into(), i2.clone().into()]),
            })),
        ),
        (
            "ObjectSomeValuesFrom",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::ObjectSomeValuesFrom {
                    ope: p1.clone().into(),
                    bce: Box::new(c2.clone().into()),
                },
            })),
        ),
        (
            "ObjectAllValuesFrom",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::ObjectAllValuesFrom {
                    ope: p1.clone().into(),
                    bce: Box::new(c2.clone().into()),
                },
            })),
        ),
        (
            "ObjectHasValue",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::ObjectHasValue {
                    ope: p1.clone().into(),
                    i: i1.clone().into(),
                },
            })),
        ),
        (
            "ObjectHasSelf",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::ObjectHasSelf(p1.clone().into()),
            })),
        ),
        (
            "ObjectMinCardinality-qualified",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::ObjectMinCardinality {
                    n: 1,
                    ope: p1.clone().into(),
                    bce: Box::new(c2.clone().into()),
                },
            })),
        ),
        (
            "ObjectMaxCardinality-qualified",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::ObjectMaxCardinality {
                    n: 2,
                    ope: p1.clone().into(),
                    bce: Box::new(c2.clone().into()),
                },
            })),
        ),
        (
            "ObjectExactCardinality-qualified",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::ObjectExactCardinality {
                    n: 3,
                    ope: p1.clone().into(),
                    bce: Box::new(c2.clone().into()),
                },
            })),
        ),
        (
            "DataSomeValuesFrom",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::DataSomeValuesFrom {
                    dp: dp1.clone(),
                    dr: DataRange::Datatype(xsd_string.clone()),
                },
            })),
        ),
        (
            "DataAllValuesFrom",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::DataAllValuesFrom {
                    dp: dp1.clone(),
                    dr: DataRange::Datatype(xsd_string.clone()),
                },
            })),
        ),
        (
            "DataHasValue",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::DataHasValue {
                    dp: dp1.clone(),
                    l: Literal::Simple {
                        literal: "sample".to_string(),
                    },
                },
            })),
        ),
        (
            "DataMinCardinality-qualified",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::DataMinCardinality {
                    n: 1,
                    dp: dp1.clone(),
                    dr: DataRange::Datatype(xsd_integer.clone()),
                },
            })),
        ),
        (
            "DataMaxCardinality-qualified",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::DataMaxCardinality {
                    n: 2,
                    dp: dp1.clone(),
                    dr: DataRange::Datatype(xsd_integer.clone()),
                },
            })),
        ),
        (
            "DataExactCardinality-qualified",
            plain(Component::SubClassOf(SubClassOf {
                sub: c1.clone().into(),
                sup: ClassExpression::DataExactCardinality {
                    n: 3,
                    dp: dp1.clone(),
                    dr: DataRange::Datatype(xsd_integer.clone()),
                },
            })),
        ),
    ])
}

#[test]
fn roundtrip_data_range_constructors() -> Result<()> {
    let b = Build::new();
    let xsd_string = b.datatype("http://www.w3.org/2001/XMLSchema#string");
    let xsd_integer = b.datatype("http://www.w3.org/2001/XMLSchema#integer");
    let dp1 = b.data_property("http://example.org/dp1");

    run_cases(vec![
        (
            "DataIntersectionOf",
            plain(Component::DataPropertyRange(DataPropertyRange {
                dp: dp1.clone(),
                dr: DataRange::DataIntersectionOf(vec![
                    DataRange::Datatype(xsd_string.clone()),
                    DataRange::Datatype(xsd_integer.clone()),
                ]),
            })),
        ),
        (
            "DataUnionOf",
            plain(Component::DataPropertyRange(DataPropertyRange {
                dp: dp1.clone(),
                dr: DataRange::DataUnionOf(vec![
                    DataRange::Datatype(xsd_string.clone()),
                    DataRange::Datatype(xsd_integer.clone()),
                ]),
            })),
        ),
        (
            "DataComplementOf",
            plain(Component::DataPropertyRange(DataPropertyRange {
                dp: dp1.clone(),
                dr: DataRange::DataComplementOf(Box::new(DataRange::Datatype(
                    xsd_string.clone(),
                ))),
            })),
        ),
        (
            "DataOneOf",
            plain(Component::DataPropertyRange(DataPropertyRange {
                dp: dp1.clone(),
                dr: DataRange::DataOneOf(vec![
                    Literal::Simple {
                        literal: "a".to_string(),
                    },
                    Literal::Simple {
                        literal: "b".to_string(),
                    },
                ]),
            })),
        ),
        (
            "DatatypeRestriction",
            plain(Component::DataPropertyRange(DataPropertyRange {
                dp: dp1,
                dr: DataRange::DatatypeRestriction(
                    xsd_integer,
                    vec![FacetRestriction {
                        f: Facet::MinInclusive,
                        l: Literal::Datatype {
                            literal: "1".to_string(),
                            datatype_iri: b.iri("http://www.w3.org/2001/XMLSchema#integer"),
                        },
                    }],
                ),
            })),
        ),
    ])
}
