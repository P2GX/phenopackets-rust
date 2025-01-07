use phenopackets_dev::schema::v2;

#[test]
fn ontology_class() {
    let seizure = v2::core::OntologyClass {
        id: "HP:0001250".into(),
        label: "Seizure".into(),
    };

    assert_eq!(&seizure.id, "HP:0001250");
    assert_eq!(&seizure.label, "Seizure");
}

#[test]
fn phenotypic_feature() {
    let pf = v2::core::PhenotypicFeature {
        r#type: Some(v2::core::OntologyClass {
            id: "HP:0001250".into(),
            label: "Seizure".into(),
        }),
        excluded: false,
        description: "Description".into(),
        onset: None,
        resolution: None,
        modifiers: vec![],
        severity: None,
        evidence: vec![],
    };

    assert_eq!(&pf.r#type.as_ref().unwrap().id, "HP:0001250");
    assert_eq!(&pf.r#type.as_ref().unwrap().label, "Seizure");
    assert_eq!(&pf.excluded, &false);
    assert_eq!(&pf.excluded, &false);
}
