//! # Rust bindings for Phenopacket Schema
//! 
//! More documentation will arrive soon.

// Include the `phenopackets` module, which is generated from the Phenopacket Schema proto files.
pub mod phenopackets {
    pub mod schema {
        pub mod v1 {
            pub mod core {
                include!(concat!(env!("OUT_DIR"), "/org.phenopackets.schema.v1.core.rs"));
            }
            include!(concat!(env!("OUT_DIR"), "/org.phenopackets.schema.v1.rs"));
        }
        pub mod v2 {
            pub mod core {
                include!(concat!(env!("OUT_DIR"), "/org.phenopackets.schema.v2.core.rs"));
            }
            include!(concat!(env!("OUT_DIR"), "/org.phenopackets.schema.v2.rs"));
        }
    }
}

pub mod ga4gh {
    pub mod vrsatile {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/org.ga4gh.vrsatile.v1.rs"));
        }
    }
    pub mod vrs {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/org.ga4gh.vrs.v1.rs"));
        }
    }
}

#[cfg(test)]
mod test {

    use crate::phenopackets::schema::v2;

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
}
