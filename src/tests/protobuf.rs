//! Tests of (de)serialization of the protobuf binary format.

/// Test v1 phenopackets.
mod v1 {

    use crate::{
        schema::v1::{Cohort, Family, Phenopacket},
        tests::read_path,
    };
    use prost::Message;

    const V2_PHENOPACKET: &str = "data/v1/phenopacket.pb";
    const V2_FAMILY: &str = "data/v1/family.pb";
    const V2_COHORT: &str = "data/v1/cohort.pb";

    #[test]
    fn decode_phenopacket() {
        let buf = read_path(V2_PHENOPACKET);
        let actual = Phenopacket::decode(&buf[..]).unwrap();

        assert_eq!(&actual.id, "comprehensive-phenopacket-id");
    }

    #[test]
    fn decode_family() {
        let buf = read_path(V2_FAMILY);
        let family = Family::decode(&buf[..]).unwrap();

        assert_eq!(&family.id, "comprehensive-family-id");
    }

    #[test]
    fn decode_cohort() {
        let buf = read_path(V2_COHORT);
        let family = Cohort::decode(&buf[..]).unwrap();

        assert_eq!(&family.id, "comprehensive-cohort-id");
    }
}

/// Test top-level elements of v2 Phenopacket Schema.
mod v2 {

    use crate::{
        schema::v2::{Cohort, Family, Phenopacket},
        tests::{examples::v2, read_path},
    };
    use prost::Message;

    const V2_PHENOPACKET: &str = "data/v2/phenopacket.pb";
    const V2_FAMILY: &str = "data/v2/family.pb";
    const V2_COHORT: &str = "data/v2/cohort.pb";

    #[test]
    fn decode_phenopacket() {
        let buf = read_path(V2_PHENOPACKET);
        let actual = Phenopacket::decode(&*buf).unwrap();

        let expected = v2::phenopacket();

        assert_eq!(actual, expected);

        // assert_eq!(actual.id, expected.id);
        // assert_eq!(actual.subject, expected.subject);
        // assert_eq!(actual.phenotypic_features, expected.phenotypic_features);
        // assert_eq!(actual.measurements, expected.measurements);
        // assert_eq!(actual.biosamples, expected.biosamples);
        // assert_eq!(actual.interpretations, expected.interpretations);
        // assert_eq!(actual.diseases, expected.diseases);
        // assert_eq!(actual.medical_actions, expected.medical_actions);
        // assert_eq!(actual.files, expected.files);
        // assert_eq!(actual.meta_data, expected.meta_data);
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
