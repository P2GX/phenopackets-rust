
#[cfg(feature = "serde")]
use ::serde::{Serialize, Deserialize};



// TODO: remove and only keep `src/tests/serde`.
#[cfg(feature = "serde")]
mod serde {
    use serde_json::Value;
    use phenopackets::schema::v2;
    #[test]
    fn test_serde_ontology_class() {
        let seizure = v2::core::OntologyClass {
            id: "HP:0001250".into(),
            label: "Seizure".into(),
        };
        let json_str1 = serde_json::to_string_pretty(&seizure).unwrap();
        let json_str2 = r#"{ "id": "HP:0001250", "label": "Seizure" }"#;
        let v1: Value = serde_json::from_str(&json_str1).unwrap();
        let v2: Value = serde_json::from_str(json_str2).unwrap();
        assert_eq!(v1, v2);
    }
    

}