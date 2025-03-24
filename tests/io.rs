use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

mod v1 {

    use super::read_path;

    use phenopackets::schema::v1::{Cohort, Family, Phenopacket};
    use prost::Message;

    const V2_PHENOPACKET: &str = "data/v1/phenopacket.pb";
    const V2_FAMILY: &str = "data/v1/family.pb";
    const V2_COHORT: &str = "data/v1/cohort.pb";

    #[test]
    fn decode_phenopacket() {
        let buf = read_path(V2_PHENOPACKET);
        let pp = Phenopacket::decode(&*buf).unwrap();

        assert_eq!(&pp.id, "comprehensive-phenopacket-id");
    }

    #[test]
    fn decode_family() {
        let buf = read_path(V2_FAMILY);
        let family = Family::decode(&*buf).unwrap();

        assert_eq!(&family.id, "comprehensive-family-id");
    }

    #[test]
    fn decode_cohort() {
        let buf = read_path(V2_COHORT);
        let family = Cohort::decode(&*buf).unwrap();

        assert_eq!(&family.id, "comprehensive-cohort-id");
    }
}

mod v2 {

    use std::str::FromStr;

    use super::read_path;

    use phenopackets::schema::v2::{
        core::{KaryotypicSex, Sex},
        Cohort, Family, Phenopacket,
    };
    use prost::Message;
    use prost_types::Timestamp;

    const V2_PHENOPACKET: &str = "data/v2/phenopacket.pb";
    const V2_FAMILY: &str = "data/v2/family.pb";
    const V2_COHORT: &str = "data/v2/cohort.pb";

    #[test]
    fn decode_phenopacket() {
        let buf = read_path(V2_PHENOPACKET);
        let pp = Phenopacket::decode(&*buf).unwrap();

        assert_eq!(&pp.id, "comprehensive-phenopacket-id");
        let subject = pp.subject.as_ref().unwrap();
        assert_eq!(&subject.id, "14 year-old boy");
        assert_eq!(&subject.alternate_ids, &["boy", "patient", "proband"]);

        assert_eq!(
            // Well-known type
            subject.date_of_birth.as_ref().unwrap(),
            Timestamp::from_str("1970-01-02T10:17:36.000000100Z")
                .as_ref()
                .unwrap(),
        );
        assert_eq!(&subject.sex(), &Sex::Male);
        assert_eq!(&subject.karyotypic_sex(), &KaryotypicSex::Xy);

        let taxonomy = subject.taxonomy.as_ref().unwrap();
        assert_eq!(taxonomy.id, "NCBITaxon:9606");
        assert_eq!(taxonomy.label, "homo sapiens");

        assert_eq!(pp.phenotypic_features.len(), 4);
        assert_eq!(pp.biosamples.len(), 1);
        assert_eq!(pp.interpretations.len(), 1);
        assert_eq!(pp.diseases.len(), 1);
        assert_eq!(pp.files.len(), 1);

        assert!(pp.meta_data.is_some());
    }

    #[test]
    fn decode_family() {
        let buf = read_path(V2_FAMILY);
        let family = Family::decode(&*buf).unwrap();

        assert_eq!(&family.id, "comprehensive-family-id");
    }

    #[test]
    fn decode_cohort() {
        let buf = read_path(V2_COHORT);
        let cohort = Cohort::decode(&*buf).unwrap();

        assert_eq!(&cohort.id, "comprehensive-cohort-id");
    }
}

fn read_path<T: AsRef<Path>>(path: T) -> Vec<u8> {
    let mut buf = vec![];
    if let Ok(f) = File::open(path) {
        let mut b = BufReader::new(f);
        b.read_to_end(&mut buf).unwrap();
    };

    buf
}

#[cfg(feature = "serde")]
mod serde {
    use phenopackets::schema::v2::core::OntologyClass;
    use serde::Serialize;
    use serde_json::Serializer;

    #[test]
    fn serialize_ontology_class_to_json() {
        let seizure = OntologyClass {
            id: "HP:0001250".into(),
            label: "Seizure".into(),
        };

        let mut writer = Vec::new();
        let mut serializer = Serializer::new(&mut writer);

        seizure.serialize(&mut serializer).expect("Should end well");

        let seizure_as_json = String::from_utf8(writer).expect("Should be valid UTF-8");
        assert_eq!(
            seizure_as_json,
            "{\"id\":\"HP:0001250\",\"label\":\"Seizure\"}"
        );
    }
}
