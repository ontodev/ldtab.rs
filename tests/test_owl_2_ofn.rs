use horned_owl::model::{Build, Axiom, SubClassOf, ClassExpression, EquivalentClasses, DisjointClasses, DisjointUnion, DeclareClass, DeclareObjectProperty, DeclareAnnotationProperty, DeclareDataProperty, DeclareNamedIndividual, DeclareDatatype, SubObjectPropertyOf, SubObjectPropertyExpression, EquivalentObjectProperties, InverseObjectProperties, ObjectPropertyDomain, ObjectPropertyRange, FunctionalObjectProperty};
use serde_json::json; 
use ldtab_rs::owl2ofn::axiom_transducer as axiom_transducer;

//setup horned OWL axiom
//translate it to OFN S-expression

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
fn test_inverse_object_property_domain() { 
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
fn test_inverse_object_property_range() { 
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

