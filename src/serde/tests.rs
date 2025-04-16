/// Test oneof (de)serialization.
mod test_medical_action {
    use crate::generated::org_phenopackets_schema_v2_core::{
        medical_action::Action, time_element::Element, DrugType, MedicalAction, OntologyClass,
        Procedure, TimeElement, Treatment,
    };

    fn procedure() -> MedicalAction {
        MedicalAction {
            treatment_target: None,
            treatment_intent: None,
            response_to_treatment: None,
            adverse_events: vec![],
            treatment_termination_reason: None,
            action: Some(Action::Procedure(Procedure {
                code: Some(OntologyClass {
                    id: "NCIT:C80473".into(),
                    label: "Left Ventricular Assist Device".into(),
                }),
                body_site: None,
                performed: Some(TimeElement {
                    element: Some(Element::Timestamp(
                        "2016-01-01T00:00:00Z".parse().expect("No issues"),
                    )),
                }),
            })),
        }
    }

    #[test]
    fn deserialize_procedure() {
        let val = r#"{
            "procedure": {
              "code": {
                "id": "NCIT:C80473",
                "label": "Left Ventricular Assist Device"
              },
              "performed": {
                "timestamp": "2016-01-01T00:00:00Z"
              }
            }
          }"#;

        let actual: MedicalAction = serde_json::from_str(val).expect("No deserialization issues");

        let expected = procedure();

        assert_eq!(actual, expected);
    }

    #[test]
    fn serialize_procedure() {
        let procedure = procedure();

        let actual = serde_json::to_string(&procedure).expect("Expecting no serialization issues");
        let expected = r#"{"procedure":{"code":{"id":"NCIT:C80473","label":"Left Ventricular Assist Device"},"performed":{"timestamp":"2016-01-01T00:00:00Z"}}}"#;

        assert_eq!(actual, expected);
    }

    fn treatment() -> MedicalAction {
        MedicalAction {
            treatment_target: None,
            treatment_intent: None,
            response_to_treatment: Some(OntologyClass {
                id: "NCIT:C123584".into(),
                label: "Favorable Response".into(),
            }),
            adverse_events: vec![OntologyClass {
                id: "HP:0003198".into(),
                label: "Myopathy".into(),
            }],
            treatment_termination_reason: None,
            action: Some(Action::Treatment(Treatment {
                agent: Some(OntologyClass {
                    id: "DrugCentral:257".into(),
                    label: "Vitamin E".into(),
                }),
                route_of_administration: Some(OntologyClass {
                    id: "NCIT:C38288".into(),
                    label: "Oral Route of Administration".into(),
                }),
                dose_intervals: vec![],
                drug_type: DrugType::UnknownDrugType.into(),
                cumulative_dose: None,
            })),
        }
    }

    #[test]
    fn deserialize_treatment() {
        let val = r#"{
            "treatment": {
              "agent": {
                "id": "DrugCentral:257",
                "label": "Vitamin E"
              },
              "routeOfAdministration": {
                "id": "NCIT:C38288",
                "label": "Oral Route of Administration"
              },
              "drugType": "UNKNOWN_DRUG_TYPE"
            },
            "adverseEvents": [{
                "id": "HP:0003198",
                "label": "Myopathy"
            }],
            "responseToTreatment": {
              "id": "NCIT:C123584",
              "label": "Favorable Response"
            }
          }"#;

        let actual: MedicalAction = serde_json::from_str(val).expect("No deserialization issues");

        let expected = treatment();

        assert_eq!(actual, expected);
    }

    #[test]
    fn serialize_treatment() {
        let treatment = treatment();

        let actual = serde_json::to_string(&treatment).expect("Expecting no serialization issues");
        let expected = r#"{"responseToTreatment":{"id":"NCIT:C123584","label":"Favorable Response"},"adverseEvents":[{"id":"HP:0003198","label":"Myopathy"}],"treatment":{"agent":{"id":"DrugCentral:257","label":"Vitamin E"},"routeOfAdministration":{"id":"NCIT:C38288","label":"Oral Route of Administration"},"drugType":"UNKNOWN_DRUG_TYPE"}}"#;

        assert_eq!(actual, expected);
    }
}

mod test_individual {
    use crate::generated::org_phenopackets_schema_v2_core::{
        time_element::Element, Age, Individual, KaryotypicSex, OntologyClass, Sex, TimeElement,
    };

    /// A minimal individual serializes only the enum fields (`sex` and `karyotypicSex`).
    #[test]
    fn serialize_individual_minimal_fields() {
        let individual = Individual {
            id: "".into(),
            alternate_ids: vec![],
            date_of_birth: None,
            time_at_last_encounter: None,
            vital_status: None,
            sex: Sex::UnknownSex.into(),
            karyotypic_sex: KaryotypicSex::UnknownKaryotype.into(),
            gender: None,
            taxonomy: None,
        };

        let actual = serde_json::to_string(&individual).expect("Expecting no serialization issues");

        assert_eq!(
            actual,
            r#"{"sex":"UNKNOWN_SEX","karyotypicSex":"UNKNOWN_KARYOTYPE"}"#
        )
    }

    #[test]
    fn deserialize_individual() {
        let value = r#"{
        "id": "14 year-old boy",
        "alternateIds": ["boy", "patient", "proband"],
        "dateOfBirth": "1970-01-02T10:17:36.000000100Z",
        "timeAtLastEncounter": {
            "age": {
                "iso8601duration": "P14Y"
            }
        },
        "sex": "MALE",
        "karyotypicSex": "XY",
        "taxonomy": {
            "id": "NCBITaxon:9606",
            "label": "homo sapiens"
        }
        }"#;

        let individual: Individual =
            serde_json::from_str(value).expect("No deserialization issues");

        assert_eq!(&individual.id, "14 year-old boy");
        assert_eq!(&individual.alternate_ids, &["boy", "patient", "proband"]);
        assert_eq!(
            individual.date_of_birth,
            Some(
                "1970-01-02T10:17:36.000000100Z"
                    .parse()
                    .expect("No issues here")
            )
        );
        assert_eq!(
            individual.time_at_last_encounter,
            Some(TimeElement {
                element: Some(Element::Age(Age {
                    iso8601duration: "P14Y".into()
                }))
            })
        );
        assert_eq!(individual.vital_status, None);
        assert_eq!(individual.sex(), Sex::Male);
        assert_eq!(individual.karyotypic_sex(), KaryotypicSex::Xy);
        assert_eq!(individual.gender, None);
        assert_eq!(
            individual.taxonomy,
            Some(OntologyClass {
                id: "NCBITaxon:9606".into(),
                label: "homo sapiens".into()
            })
        );
    }

    #[test]
    fn deserialize_individual_minimal_example() {
        let value = "{}";

        let individual: Individual =
            serde_json::from_str(value).expect("No deserialization issues");

        assert_eq!(&individual.id, "");
        assert!(individual.alternate_ids.is_empty());
        assert_eq!(individual.date_of_birth, None);
        assert_eq!(individual.time_at_last_encounter, None);
        assert_eq!(individual.vital_status, None);
        assert_eq!(individual.sex, 0);
        assert_eq!(individual.karyotypic_sex, 0);
        assert_eq!(individual.gender, None);
        assert_eq!(individual.taxonomy, None);
    }
}

mod test_time_element {
    use crate::generated::org_phenopackets_schema_v2_core::{
        time_element::Element, Age, TimeElement,
    };

    #[test]
    fn serialize_time_element_with_age() {
        let te = TimeElement {
            element: Some(Element::Age(Age {
                iso8601duration: "P14Y".to_string(),
            })),
        };

        let actual = serde_json::to_string(&te).expect("Expecting no serialization issues");

        assert_eq!(actual, r#"{"age":{"iso8601duration":"P14Y"}}"#);
    }

    /// We produce an empty object: `{}`.
    ///
    /// This should not happen in real life, since the oneof field should always be set.
    /// However, Prost wraps the oneof field in an `Option`, so we must test the behavior (here).
    #[test]
    fn serialize_time_element_with_nothing() {
        let te = TimeElement { element: None };

        let actual = serde_json::to_string(&te).expect("Expecting no serialization issues");

        assert_eq!(actual, r#"{}"#);
    }
}
