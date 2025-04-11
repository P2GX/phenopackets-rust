
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

    #[test]
    fn test_serde_external_reference()  {
        let eref = v2::core::ExternalReference{
            id: "PMID:30962759".to_string(),
            reference: "https://pubmed.ncbi.nlm.nih.gov/30962759".to_string(),
            description: "Recurrent Erythema Nodosum in a Child with a SHOC2 Gene Mutation".to_string(),
        };
        let json_str1 = r#"{ "id": "PMID:30962759", 
                        "reference": "https://pubmed.ncbi.nlm.nih.gov/30962759", 
                        "description": "Recurrent Erythema Nodosum in a Child with a SHOC2 Gene Mutation" }"#;
        let json_str2 = serde_json::to_string_pretty(&eref).unwrap();
        
        let expected: Value = serde_json::from_str(&json_str1).unwrap();
        let observed: Value = serde_json::from_str(&json_str2).unwrap();
        assert_eq!(expected, observed);
    }

    #[test]
    fn test_evidence()  {
        let code = v2::core::OntologyClass {
            id: "ECO:0006017".to_string(),
            label: "author statement from published clinical study used in manual assertion".to_string()
        };
        let ext_ref = v2::core::ExternalReference {
            id: "PMID:30962759".to_string(),
            reference: String::default(),
            description: "Recurrent Erythema Nodosum in a Child with a SHOC2 Gene Mutation".to_string(),
        };
        let evidence = v2::core::Evidence{
            evidence_code: Some(code),
            reference: Some(ext_ref),
        };
        let json_str2 = serde_json::to_string_pretty(&evidence).unwrap();
        let expected_str = r#"{ 
                "evidence_code": 
                { 
                    "id": "ECO:0006017",
                    "label": "author statement from published clinical study used in manual assertion"
                },
                "reference": {
                    "id": "PMID:30962759",
                    "description": "Recurrent Erythema Nodosum in a Child with a SHOC2 Gene Mutation" 
                }
        }"#;
        let expected: Value = serde_json::from_str(&expected_str).unwrap();
        let observed: Value = serde_json::from_str(&json_str2).unwrap();
        assert_eq!(expected, observed);
       /*

 evidenceCode:
        id: "ECO:0006017"
        label: "author statement from published clinical study used in manual assertion"
    reference:
      

       pub struct Evidence {
    /// The encoded evidence type using, for example the Evidence & Conclusion Ontology (ECO - <http://purl.obolibrary.org/obo/eco.owl>)
    /// FHIR mapping: Condition.evidence.code
    #[prost(message, optional, tag = "1")]
    pub evidence_code: ::core::option::Option<OntologyClass>,
    /// FHIR mapping: Condition.evidence.detail
    #[prost(message, optional, tag = "2")]
    pub reference: ::core::option::Option<ExternalReference>,
} */
    }
    

}