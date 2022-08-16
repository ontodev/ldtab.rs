use horned_owl::model::{Build, Axiom, SubClassOf, ClassExpression, EquivalentClasses, DisjointClasses, DisjointUnion, DeclareClass, DeclareObjectProperty, DeclareAnnotationProperty, DeclareDataProperty, DeclareNamedIndividual, DeclareDatatype, SubObjectPropertyOf, SubObjectPropertyExpression, EquivalentObjectProperties, InverseObjectProperties, ObjectPropertyDomain, ObjectPropertyRange, FunctionalObjectProperty, InverseFunctionalObjectProperty, ReflexiveObjectProperty, IrreflexiveObjectProperty, SymmetricObjectProperty, AsymmetricObjectProperty, TransitiveObjectProperty, SubDataPropertyOf, EquivalentDataProperties, DisjointDataProperties, DataPropertyDomain, DataPropertyRange, DataRange, FunctionalDataProperty, DatatypeDefinition, SameIndividual, DifferentIndividuals, ClassAssertion, ObjectPropertyAssertion, NegativeObjectPropertyAssertion, DataPropertyAssertion, Literal, NegativeDataPropertyAssertion, AnnotationAssertion, Annotation, AnnotationValue, AnnotationSubject, SubAnnotationPropertyOf, AnnotationPropertyDomain, AnnotationPropertyRange};
use serde_json::json; 
use ldtab_rs::owl2ofn::axiom_transducer as axiom_transducer;

//setup horned OWL axiom
//translate it to OFN S-expression

///TODO: 
//HasKey
//Import
//OntologyAnnotation 
//SubObjectPropertyOf with property chains

#[test]
fn subclass_of() { 
    let b = Build::new();
    let sub = SubClassOf{sub: b.class("http://www.example.com/op1").into(),
                         sup: b.class("http://www.example.com/op2").into()};
    let sub_axiom = Axiom::SubClassOf(sub);

    let sub_ofn = axiom_transducer::translate(&sub_axiom);

    let sub_ofn_expected = json!(["SubClassOf","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(sub_ofn, sub_ofn_expected);
}

#[test]
fn equivalence_classes_binary() { 
    let b = Build::new();
    let ec = EquivalentClasses
          (vec!(b.class("http://www.example.com/op1").into(),
                b.class("http://www.example.com/op2").into()));
    let ec_axiom = Axiom::EquivalentClasses(ec); 

    let ec_ofn = axiom_transducer::translate(&ec_axiom);

    let ec_ofn_expected = json!(["EquivalentClasses","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(ec_ofn, ec_ofn_expected);
}

#[test]
fn equivalence_classes_nary() { 
    let b = Build::new();
    let ec = EquivalentClasses
          (vec!(b.class("http://www.example.com/op1").into(),
                b.class("http://www.example.com/op2").into(),
                b.class("http://www.example.com/op3").into(),
                b.class("http://www.example.com/op4").into()));
    let ec_axiom = Axiom::EquivalentClasses(ec); 

    let ec_ofn = axiom_transducer::translate(&ec_axiom);

    let ec_ofn_expected = json!(["EquivalentClasses","http://www.example.com/op1",
                                              "http://www.example.com/op2",
                                              "http://www.example.com/op3",
                                              "http://www.example.com/op4"]);

    assert_eq!(ec_ofn, ec_ofn_expected);
}

#[test]
fn disjoint_classes_binary() { 
    let b = Build::new();
    let dc = DisjointClasses
          (vec!(b.class("http://www.example.com/op1").into(),
                b.class("http://www.example.com/op2").into()));
    let dc_axiom = Axiom::DisjointClasses(dc); 

    let dc_ofn = axiom_transducer::translate(&dc_axiom);

    let dc_ofn_expected = json!(["DisjointClasses","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(dc_ofn, dc_ofn_expected);
}

#[test]
fn disjoint_classes_nary() { 
    let b = Build::new();
    let dc = DisjointClasses
          (vec!(b.class("http://www.example.com/op1").into(),
                b.class("http://www.example.com/op2").into(),
                b.class("http://www.example.com/op3").into(),
                b.class("http://www.example.com/op4").into()));
    let dc_axiom = Axiom::DisjointClasses(dc); 

    let dc_ofn = axiom_transducer::translate(&dc_axiom);

    let dc_ofn_expected = json!(["DisjointClasses","http://www.example.com/op1",
                                              "http://www.example.com/op2",
                                              "http://www.example.com/op3",
                                              "http://www.example.com/op4"]);

    assert_eq!(dc_ofn, dc_ofn_expected);
}

#[test]
fn disjoint_union() { 
    let b = Build::new();
    let du = DisjointUnion{
        0: b.class("http://www.example.com/op0").into(),
        1: vec!(b.class("http://www.example.com/op1").into(),
                b.class("http://www.example.com/op2").into(),
                b.class("http://www.example.com/op3").into(),
                b.class("http://www.example.com/op4").into())
           };
    let du_axiom = Axiom::DisjointUnion(du); 

    let du_ofn = axiom_transducer::translate(&du_axiom);

    let du_ofn_expected = json!(["DisjointUnion","http://www.example.com/op0", 
                                              "http://www.example.com/op1",
                                              "http://www.example.com/op2",
                                              "http://www.example.com/op3",
                                              "http://www.example.com/op4"]);

    assert_eq!(du_ofn, du_ofn_expected);
}

#[test]
fn test_class_declaration() { 
    let b = Build::new();
    let dc = DeclareClass{ 0: b.class("http://www.example.com/op1").into() };
    let dc_axiom = Axiom::DeclareClass(dc); 

    let dc_ofn = axiom_transducer::translate(&dc_axiom);

    let dc_ofn_expected = json!(["Declaration",["Class","http://www.example.com/op1"]]);

    assert_eq!(dc_ofn, dc_ofn_expected);
}

#[test]
fn test_object_property_declaration() { 
    let b = Build::new();
    let dp = DeclareObjectProperty{ 0: b.object_property("http://www.example.com/op1").into() };
    let dp_axiom = Axiom::DeclareObjectProperty(dp); 

    let dp_ofn = axiom_transducer::translate(&dp_axiom);

    let dp_ofn_expected = json!(["Declaration",["ObjectProperty","http://www.example.com/op1"]]);

    assert_eq!(dp_ofn, dp_ofn_expected);
}

#[test]
fn test_annotation_property_declaration() { 
    let b = Build::new();
    let ap = DeclareAnnotationProperty{ 0: b.annotation_property("http://www.example.com/op1").into() };
    let ap_axiom = Axiom::DeclareAnnotationProperty(ap); 

    let ap_ofn = axiom_transducer::translate(&ap_axiom);

    let ap_ofn_expected = json!(["Declaration",["AnnotationProperty","http://www.example.com/op1"]]);

    assert_eq!(ap_ofn, ap_ofn_expected);
}

#[test]
fn test_data_property_declaration() { 
    let b = Build::new();
    let dp = DeclareDataProperty{ 0: b.data_property("http://www.example.com/op1").into() };
    let dp_axiom = Axiom::DeclareDataProperty(dp); 

    let dp_ofn = axiom_transducer::translate(&dp_axiom);

    let dp_ofn_expected = json!(["Declaration",["DataProperty","http://www.example.com/op1"]]);

    assert_eq!(dp_ofn, dp_ofn_expected);
}

#[test]
fn test_named_individual_declaration() { 
    let b = Build::new();
    let di = DeclareNamedIndividual{ 0: b.named_individual("http://www.example.com/i1").into() };
    let di_axiom = Axiom::DeclareNamedIndividual(di); 

    let di_ofn = axiom_transducer::translate(&di_axiom);

    let di_ofn_expected = json!(["Declaration",["NamedIndividual","http://www.example.com/i1"]]);

    assert_eq!(di_ofn, di_ofn_expected);
}

#[test]
fn test_data_type_declaration() { 
    let b = Build::new();
    let dt = DeclareDatatype{ 0: b.datatype("http://www.example.com").into() };
    let dt_axiom = Axiom::DeclareDatatype(dt); 

    let dt_ofn = axiom_transducer::translate(&dt_axiom);

    let dt_ofn_expected = json!(["Declaration",["Datatype","http://www.example.com"]]);

    assert_eq!(dt_ofn, dt_ofn_expected);
}

#[test]
fn test_sub_object_property_of() { 
    let b = Build::new();
    let sub = SubObjectPropertyOf{
        sub: SubObjectPropertyExpression::ObjectPropertyExpression(b.object_property("http://www.example.com/op1").into()), 
        sup: b.object_property("http://www.example.com/op2").into()};

    let sub_axiom = Axiom::SubObjectPropertyOf(sub);

    let sub_ofn = axiom_transducer::translate(&sub_axiom);

    let sub_ofn_expected = json!(["SubObjectPropertyOf","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(sub_ofn, sub_ofn_expected);
}


#[test]
fn test_sub_object_property_of_chain() { 
    let b = Build::new();
    let sub = SubObjectPropertyOf{
        sub: SubObjectPropertyExpression::ObjectPropertyChain(
                 vec!(b.object_property("http://www.example.com/op1").into(), 
                      b.object_property("http://www.example.com/op2").into())),
        sup: b.object_property("http://www.example.com/op3").into()};

    let sub_axiom = Axiom::SubObjectPropertyOf(sub);

    let sub_ofn = axiom_transducer::translate(&sub_axiom);

    let sub_ofn_expected = json!(["SubObjectPropertyOf",["ObjectPropertyChain","http://www.example.com/op1","http://www.example.com/op2"],"http://www.example.com/op3"]);

    assert_eq!(sub_ofn, sub_ofn_expected);
}


#[test]
fn test_equivalent_object_properties_binary() { 
    let b = Build::new();
    let ep = EquivalentObjectProperties
          (vec!(b.object_property("http://www.example.com/op1").into(),
                b.object_property("http://www.example.com/op2").into()));
    let ep_axiom = Axiom::EquivalentObjectProperties(ep); 

    let ep_ofn = axiom_transducer::translate(&ep_axiom);

    let ep_ofn_expected = json!(["EquivalentObjectProperties","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(ep_ofn, ep_ofn_expected);
}

#[test]
fn test_equivalent_object_properties_nary() { 
    let b = Build::new();
    let ep = EquivalentObjectProperties
          (vec!(b.object_property("http://www.example.com/op1").into(),
                b.object_property("http://www.example.com/op2").into(),
                b.object_property("http://www.example.com/op3").into(),
                b.object_property("http://www.example.com/op4").into()));
    let ep_axiom = Axiom::EquivalentObjectProperties(ep); 

    let ep_ofn = axiom_transducer::translate(&ep_axiom);

    let ep_ofn_expected = json!(["EquivalentObjectProperties","http://www.example.com/op1",
                                                              "http://www.example.com/op2",
                                                              "http://www.example.com/op3",
                                                              "http://www.example.com/op4"]);

    assert_eq!(ep_ofn, ep_ofn_expected);
}

#[test]
fn test_inverse_object_properties() { 
    let b = Build::new();
    let ip = InverseObjectProperties{0: b.object_property("http://www.example.com/op1").into(),
           1: b.object_property("http://www.example.com/op2").into()};
    let ip_axiom = Axiom::InverseObjectProperties(ip); 

    let ip_ofn = axiom_transducer::translate(&ip_axiom);

    let ip_ofn_expected = json!(["InverseObjectProperties","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(ip_ofn, ip_ofn_expected);
}

#[test]
fn test_object_property_domain() { 
    let b = Build::new();
    let pd = ObjectPropertyDomain{
        ope: b.object_property("http://www.example.com/op1").into(),
        ce: b.class("http://www.example.com/op2").into()};
    let pd_axiom = Axiom::ObjectPropertyDomain(pd); 

    let pd_ofn = axiom_transducer::translate(&pd_axiom);

    let pd_ofn_expected = json!(["ObjectPropertyDomain","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(pd_ofn, pd_ofn_expected);
}

#[test]
fn test_object_property_range() { 
    let b = Build::new();
    let pr = ObjectPropertyRange{
        ope: b.object_property("http://www.example.com/op1").into(),
        ce: b.class("http://www.example.com/op2").into()};
    let pr_axiom = Axiom::ObjectPropertyRange(pr); 

    let pr_ofn = axiom_transducer::translate(&pr_axiom);

    let pr_ofn_expected = json!(["ObjectPropertyRange","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(pr_ofn, pr_ofn_expected);
}

#[test]
fn test_functional_object_property() { 
    let b = Build::new();
    let pr = FunctionalObjectProperty{
        0: b.object_property("http://www.example.com/op1").into()};
    let pr_axiom = Axiom::FunctionalObjectProperty(pr); 

    let pr_ofn = axiom_transducer::translate(&pr_axiom);

    let pr_ofn_expected = json!(["FunctionalObjectProperty","http://www.example.com/op1"]);

    assert_eq!(pr_ofn, pr_ofn_expected);
}

#[test]
fn test_inverse_functional_object_property() { 
    let b = Build::new();
    let pr = InverseFunctionalObjectProperty{
        0: b.object_property("http://www.example.com/op1").into()};
    let pr_axiom = Axiom::InverseFunctionalObjectProperty(pr); 

    let pr_ofn = axiom_transducer::translate(&pr_axiom);

    let pr_ofn_expected = json!(["InverseFunctionalObjectProperty","http://www.example.com/op1"]);

    assert_eq!(pr_ofn, pr_ofn_expected);
}

#[test]
fn test_reflexive_object_property() { 
    let b = Build::new();
    let pr = ReflexiveObjectProperty{
        0: b.object_property("http://www.example.com/op1").into()};
    let pr_axiom = Axiom::ReflexiveObjectProperty(pr); 

    let pr_ofn = axiom_transducer::translate(&pr_axiom);

    let pr_ofn_expected = json!(["ReflexiveObjectProperty","http://www.example.com/op1"]);

    assert_eq!(pr_ofn, pr_ofn_expected);
}

#[test]
fn test_irreflexive_object_property() { 
    let b = Build::new();
    let pr = IrreflexiveObjectProperty{
        0: b.object_property("http://www.example.com/op1").into()};
    let pr_axiom = Axiom::IrreflexiveObjectProperty(pr); 

    let pr_ofn = axiom_transducer::translate(&pr_axiom);

    let pr_ofn_expected = json!(["IrreflexiveObjectProperty","http://www.example.com/op1"]);

    assert_eq!(pr_ofn, pr_ofn_expected);
}

#[test]
fn test_symmetric_object_property() { 
    let b = Build::new();
    let pr = SymmetricObjectProperty{
        0: b.object_property("http://www.example.com/op1").into()};
    let pr_axiom = Axiom::SymmetricObjectProperty(pr); 

    let pr_ofn = axiom_transducer::translate(&pr_axiom);

    let pr_ofn_expected = json!(["SymmetricObjectProperty","http://www.example.com/op1"]);

    assert_eq!(pr_ofn, pr_ofn_expected);
}

#[test]
fn test_asymmetric_object_property() { 
    let b = Build::new();
    let pr = AsymmetricObjectProperty{
        0: b.object_property("http://www.example.com/op1").into()};
    let pr_axiom = Axiom::AsymmetricObjectProperty(pr); 

    let pr_ofn = axiom_transducer::translate(&pr_axiom);

    let pr_ofn_expected = json!(["AsymmetricObjectProperty","http://www.example.com/op1"]);

    assert_eq!(pr_ofn, pr_ofn_expected);
}

#[test]
fn test_transitive_object_property() { 
    let b = Build::new();
    let pr = TransitiveObjectProperty{
        0: b.object_property("http://www.example.com/op1").into()};
    let pr_axiom = Axiom::TransitiveObjectProperty(pr); 

    let pr_ofn = axiom_transducer::translate(&pr_axiom);

    let pr_ofn_expected = json!(["TransitiveObjectProperty","http://www.example.com/op1"]);

    assert_eq!(pr_ofn, pr_ofn_expected);
}

#[test]
fn test_sub_data_property_of() { 
    let b = Build::new();
    let sub = SubDataPropertyOf{ 
        sub: b.data_property("http://www.example.com/op1").into(),
        sup: b.data_property("http://www.example.com/op2").into()};

    let sub_axiom = Axiom::SubDataPropertyOf(sub);

    let sub_ofn = axiom_transducer::translate(&sub_axiom);

    let sub_ofn_expected = json!(["SubDataPropertyOf","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(sub_ofn, sub_ofn_expected);
}

#[test]
fn test_equivalent_data_properties_binary() { 
    let b = Build::new();
    let ep = EquivalentDataProperties
          (vec!(b.data_property("http://www.example.com/op1").into(),
                b.data_property("http://www.example.com/op2").into()));
    let ep_axiom = Axiom::EquivalentDataProperties(ep); 

    let ep_ofn = axiom_transducer::translate(&ep_axiom);

    let ep_ofn_expected = json!(["EquivalentDataProperties","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(ep_ofn, ep_ofn_expected);
}

#[test]
fn test_equivalent_data_properties_nary() { 
    let b = Build::new();
    let ep = EquivalentDataProperties
          (vec!(b.data_property("http://www.example.com/op1").into(),
                b.data_property("http://www.example.com/op2").into(),
                b.data_property("http://www.example.com/op3").into(),
                b.data_property("http://www.example.com/op4").into() ));
    let ep_axiom = Axiom::EquivalentDataProperties(ep); 

    let ep_ofn = axiom_transducer::translate(&ep_axiom);

    let ep_ofn_expected = json!(["EquivalentDataProperties","http://www.example.com/op1",
                                                            "http://www.example.com/op2",
                                                            "http://www.example.com/op3",
                                                            "http://www.example.com/op4",
    ]);

    assert_eq!(ep_ofn, ep_ofn_expected);
}

#[test]
fn test_disjoint_data_properties_binary() { 
    let b = Build::new();
    let ep = DisjointDataProperties
          (vec!(b.data_property("http://www.example.com/op1").into(),
                b.data_property("http://www.example.com/op2").into()));
    let ep_axiom = Axiom::DisjointDataProperties(ep); 

    let ep_ofn = axiom_transducer::translate(&ep_axiom);

    let ep_ofn_expected = json!(["DisjointDataProperties","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(ep_ofn, ep_ofn_expected);
}

#[test]
fn test_disjoint_data_properties_nary() { 
    let b = Build::new();
    let ep = DisjointDataProperties
          (vec!(b.data_property("http://www.example.com/op1").into(),
                b.data_property("http://www.example.com/op2").into(),
                b.data_property("http://www.example.com/op3").into(),
                b.data_property("http://www.example.com/op4").into()));
    let ep_axiom = Axiom::DisjointDataProperties(ep); 

    let ep_ofn = axiom_transducer::translate(&ep_axiom);

    let ep_ofn_expected = json!(["DisjointDataProperties","http://www.example.com/op1",
                                                          "http://www.example.com/op2",
                                                          "http://www.example.com/op3",
                                                          "http://www.example.com/op4"]);

    assert_eq!(ep_ofn, ep_ofn_expected);
}

#[test]
fn test_data_property_domain() { 
    let b = Build::new();
    let dd = DataPropertyDomain
                {dp : b.data_property("http://www.example.com/op1").into(),
                ce: b.class("http://www.example.com/op2").into()};

    let dd_axiom = Axiom::DataPropertyDomain(dd); 

    let dd_ofn = axiom_transducer::translate(&dd_axiom);

    let dd_ofn_expected = json!(["DataPropertyDomain","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(dd_ofn, dd_ofn_expected);
}

#[test]
fn test_data_property_range() { 
    let b = Build::new();
    let dt = b.datatype("http://www.example.com").into();

    let dr = DataPropertyRange
                {dp : b.data_property("http://www.example.com/op1").into(),
                dr: DataRange::Datatype(dt) };

    let dr_axiom = Axiom::DataPropertyRange(dr); 

    let dr_ofn = axiom_transducer::translate(&dr_axiom);

    let dr_ofn_expected = json!(["DataPropertyRange","http://www.example.com/op1","http://www.example.com"]);

    assert_eq!(dr_ofn, dr_ofn_expected);
}

#[test]
fn test_functional_data_property() { 
    let b = Build::new();

    let fp = FunctionalDataProperty
                {0 : b.data_property("http://www.example.com/op1").into() };

    let fp_axiom = Axiom::FunctionalDataProperty(fp); 

    let fp_ofn = axiom_transducer::translate(&fp_axiom);

    let fp_ofn_expected = json!(["FunctionalDataProperty","http://www.example.com/op1"]);

    assert_eq!(fp_ofn, fp_ofn_expected);
}

#[test]
fn test_datatype_definition() { 
    let b = Build::new();
    let dt = b.datatype("http://www.example.com").into();

    let dd = DatatypeDefinition
                {kind : b.datatype("http://www.example.uk").into(),
                range:  DataRange::Datatype(dt) };

    let dd_axiom = Axiom::DatatypeDefinition(dd); 

    let dd_ofn = axiom_transducer::translate(&dd_axiom);

    let dd_ofn_expected = json!(["DatatypeDefinition","http://www.example.uk","http://www.example.com"]);

    assert_eq!(dd_ofn, dd_ofn_expected);
}

#[test]
fn test_same_individual_binary() { 
    let b = Build::new();

    let si = SameIndividual
                {0 :  vec!(b.named_individual("http://www.example.com/op1").into(),
                           b.named_individual("http://www.example.com/op2").into())
                 };

    let si_axiom = Axiom::SameIndividual(si); 

    let si_ofn = axiom_transducer::translate(&si_axiom);

    let si_ofn_expected = json!(["SameIndividual","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(si_ofn, si_ofn_expected);
}

#[test]
fn test_same_individual_nary() { 
    let b = Build::new();

    let si = SameIndividual
                {0 :  vec!(b.named_individual("http://www.example.com/op1").into(),
                           b.named_individual("http://www.example.com/op2").into(),
                           b.named_individual("http://www.example.com/op3").into(),
                           b.named_individual("http://www.example.com/op4").into())
                 };

    let si_axiom = Axiom::SameIndividual(si); 

    let si_ofn = axiom_transducer::translate(&si_axiom);

    let si_ofn_expected = json!(["SameIndividual","http://www.example.com/op1",
                                                  "http://www.example.com/op2",
                                                  "http://www.example.com/op3",
                                                  "http://www.example.com/op4"]);

    assert_eq!(si_ofn, si_ofn_expected);
}

#[test]
fn test_different_individuals_binary() { 
    let b = Build::new();

    let di = DifferentIndividuals
                {0 :  vec!(b.named_individual("http://www.example.com/op1").into(),
                           b.named_individual("http://www.example.com/op2").into())
                 };

    let di_axiom = Axiom::DifferentIndividuals(di); 

    let di_ofn = axiom_transducer::translate(&di_axiom);

    let di_ofn_expected = json!(["DifferentIndividuals","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(di_ofn, di_ofn_expected);
}

#[test]
fn test_different_individuals_nary() { 
    let b = Build::new();

    let di = DifferentIndividuals
                {0 :  vec!(b.named_individual("http://www.example.com/op1").into(),
                           b.named_individual("http://www.example.com/op2").into(),
                           b.named_individual("http://www.example.com/op3").into(),
                           b.named_individual("http://www.example.com/op4").into())
                 };

    let di_axiom = Axiom::DifferentIndividuals(di); 

    let di_ofn = axiom_transducer::translate(&di_axiom);

    let di_ofn_expected = json!(["DifferentIndividuals","http://www.example.com/op1",
                                                        "http://www.example.com/op2",
                                                        "http://www.example.com/op3",
                                                        "http://www.example.com/op4"]);

    assert_eq!(di_ofn, di_ofn_expected);
}

#[test]
fn test_class_assertion() { 
    let b = Build::new();

    let ca = ClassAssertion
                {ce : b.class("http://www.example.com/op1").into(),
                i : b.named_individual("http://www.example.com/op2").into() };

    let ca_axiom = Axiom::ClassAssertion(ca); 

    let ca_ofn = axiom_transducer::translate(&ca_axiom);

    let ca_ofn_expected = json!(["ClassAssertion","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(ca_ofn, ca_ofn_expected);
}

#[test]
fn test_object_property_assertion() { 
    let b = Build::new();

    let oa = ObjectPropertyAssertion
                {ope : b.object_property("http://www.example.com/op1").into(),
                from : b.named_individual("http://www.example.com/op2").into(), 
                to : b.named_individual("http://www.example.com/op3").into() };

    let oa_axiom = Axiom::ObjectPropertyAssertion(oa); 

    let oa_ofn = axiom_transducer::translate(&oa_axiom);

    let oa_ofn_expected = json!(["ObjectPropertyAssertion","http://www.example.com/op1","http://www.example.com/op2","http://www.example.com/op3"]);

    assert_eq!(oa_ofn, oa_ofn_expected);
}

#[test]
fn test_negative_object_property_assertion() { 
    let b = Build::new();

    let oa = NegativeObjectPropertyAssertion
                {ope : b.object_property("http://www.example.com/op1").into(),
                from : b.named_individual("http://www.example.com/op2").into(), 
                to : b.named_individual("http://www.example.com/op3").into() };

    let oa_axiom = Axiom::NegativeObjectPropertyAssertion(oa); 

    let oa_ofn = axiom_transducer::translate(&oa_axiom);

    let oa_ofn_expected = json!(["NegativeObjectPropertyAssertion","http://www.example.com/op1","http://www.example.com/op2","http://www.example.com/op3"]);

    assert_eq!(oa_ofn, oa_ofn_expected);
}


#[test]
fn test_data_property_assertion_simple() { 
    let b = Build::new();
    let simple_literal = Literal::Simple{literal: String::from("literal")};

    let da = DataPropertyAssertion
                {dp : b.data_property("http://www.example.com/op1").into(),
                from : b.named_individual("http://www.example.com/op2").into(), 
                to : simple_literal };

    let da_axiom = Axiom::DataPropertyAssertion(da); 

    let da_ofn = axiom_transducer::translate(&da_axiom);

    let da_ofn_expected = json!(["DataPropertyAssertion","http://www.example.com/op1","http://www.example.com/op2","\"literal\""]);

    assert_eq!(da_ofn, da_ofn_expected);
}

#[test]
fn test_data_property_assertion_language() { 
    let b = Build::new();
    let language_literal = Literal::Language{literal: String::from("literal"),
                                             lang: String::from("en") };

    let da = DataPropertyAssertion
                {dp : b.data_property("http://www.example.com/op1").into(),
                from : b.named_individual("http://www.example.com/op2").into(), 
                to : language_literal };

    let da_axiom = Axiom::DataPropertyAssertion(da); 

    let da_ofn = axiom_transducer::translate(&da_axiom);

    let da_ofn_expected = json!(["DataPropertyAssertion","http://www.example.com/op1","http://www.example.com/op2","\"literal\"@en"]);

    assert_eq!(da_ofn, da_ofn_expected);
}

#[test]
fn test_data_property_assertion_datatype() { 
    let b = Build::new();
    let typed_literal = Literal::Datatype{literal: String::from("literal"),
                                             datatype_iri: b.iri("http://www.example.com") };

    let da = DataPropertyAssertion
                {dp : b.data_property("http://www.example.com/op1").into(),
                from : b.named_individual("http://www.example.com/op2").into(), 
                to : typed_literal };

    let da_axiom = Axiom::DataPropertyAssertion(da); 

    let da_ofn = axiom_transducer::translate(&da_axiom);

    let da_ofn_expected = json!(["DataPropertyAssertion","http://www.example.com/op1","http://www.example.com/op2","\"literal\"^^http://www.example.com"]);

    assert_eq!(da_ofn, da_ofn_expected);
}

#[test]
fn test_negative_data_property_assertion_simple() { 
    let b = Build::new();
    let simple_literal = Literal::Simple{literal: String::from("literal")};

    let da = NegativeDataPropertyAssertion
                {dp : b.data_property("http://www.example.com/op1").into(),
                from : b.named_individual("http://www.example.com/op2").into(), 
                to : simple_literal };

    let da_axiom = Axiom::NegativeDataPropertyAssertion(da); 

    let da_ofn = axiom_transducer::translate(&da_axiom);

    let da_ofn_expected = json!(["NegativeDataPropertyAssertion","http://www.example.com/op1","http://www.example.com/op2","\"literal\""]);

    assert_eq!(da_ofn, da_ofn_expected);
}

#[test]
fn test_negative_data_property_assertion_language() { 
    let b = Build::new();
    let language_literal = Literal::Language{literal: String::from("literal"),
                                             lang: String::from("en") };

    let da = NegativeDataPropertyAssertion
                {dp : b.data_property("http://www.example.com/op1").into(),
                from : b.named_individual("http://www.example.com/op2").into(), 
                to : language_literal };

    let da_axiom = Axiom::NegativeDataPropertyAssertion(da); 

    let da_ofn = axiom_transducer::translate(&da_axiom);

    let da_ofn_expected = json!(["NegativeDataPropertyAssertion","http://www.example.com/op1","http://www.example.com/op2","\"literal\"@en"]);

    assert_eq!(da_ofn, da_ofn_expected);
}

#[test]
fn test_negative_data_property_assertion_datatype() { 
    let b = Build::new();
    let typed_literal = Literal::Datatype{literal: String::from("literal"),
                                             datatype_iri: b.iri("http://www.example.com") };

    let da = NegativeDataPropertyAssertion
                {dp : b.data_property("http://www.example.com/op1").into(),
                from : b.named_individual("http://www.example.com/op2").into(), 
                to : typed_literal };

    let da_axiom = Axiom::NegativeDataPropertyAssertion(da); 

    let da_ofn = axiom_transducer::translate(&da_axiom);

    let da_ofn_expected = json!(["NegativeDataPropertyAssertion","http://www.example.com/op1","http://www.example.com/op2","\"literal\"^^http://www.example.com"]);

    assert_eq!(da_ofn, da_ofn_expected);
}

#[test]
fn test_annotation_assertion() { 
    let b = Build::new();
    let annotation_subject = AnnotationSubject::IRI(b.iri("http://www.example.com/op1"));
    let annotation = Annotation{ ap : b.annotation_property("http://www.example.com/op2"),
                                 av : AnnotationValue::IRI(b.iri("http://www.example.com/op3"))};

    let aa = AnnotationAssertion
                {subject : annotation_subject,
                ann : annotation };

    let aa_axiom = Axiom::AnnotationAssertion(aa); 

    let aa_ofn = axiom_transducer::translate(&aa_axiom);

    let aa_ofn_expected = json!(["AnnotationAssertion","http://www.example.com/op1","http://www.example.com/op2","http://www.example.com/op3"]);

    assert_eq!(aa_ofn, aa_ofn_expected);
}

#[test]
fn test_sub_annotation_property_of() { 
    let b = Build::new();

    let aa = SubAnnotationPropertyOf
                {sub : b.annotation_property("http://www.example.com/op1"),
                 sup : b.annotation_property("http://www.example.com/op2")};

    let aa_axiom = Axiom::SubAnnotationPropertyOf(aa); 

    let aa_ofn = axiom_transducer::translate(&aa_axiom);

    let aa_ofn_expected = json!(["SubAnnotationPropertyOf","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(aa_ofn, aa_ofn_expected);
}

#[test]
fn test_annotation_property_domain() { 
    let b = Build::new();

    let aa = AnnotationPropertyDomain
                {ap : b.annotation_property("http://www.example.com/op1"),
                 iri : b.iri("http://www.example.com/op2")};

    let aa_axiom = Axiom::AnnotationPropertyDomain(aa); 

    let aa_ofn = axiom_transducer::translate(&aa_axiom);

    let aa_ofn_expected = json!(["AnnotationPropertyDomain","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(aa_ofn, aa_ofn_expected);
}

#[test]
fn test_annotation_property_range() { 
    let b = Build::new();

    let aa = AnnotationPropertyRange
                {ap : b.annotation_property("http://www.example.com/op1"),
                 iri : b.iri("http://www.example.com/op2")};

    let aa_axiom = Axiom::AnnotationPropertyRange(aa); 

    let aa_ofn = axiom_transducer::translate(&aa_axiom);

    let aa_ofn_expected = json!(["AnnotationPropertyRange","http://www.example.com/op1","http://www.example.com/op2"]);

    assert_eq!(aa_ofn, aa_ofn_expected);
}

