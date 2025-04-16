/// Test de-/serialization of top-level elements of `v2` Phenopacket Schema
/// from/to JSON format.
mod v2 {

    use crate::{
        schema::v2::{Cohort, Family, Phenopacket},
        tests::examples::v2,
    };

    #[test]
    fn decode_phenopacket() {
        let fpath_pp = "data/v2/phenopacket.json";
        let bytes = std::fs::read(fpath_pp).expect("The test file should be in the repo");

        let actual: Phenopacket =
            serde_json::from_reader(&bytes[..]).expect("Expecting no deserialization issues");

        let expected = v2::phenopacket();

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

        assert_eq!(actual, expected);
    }

    #[test]
    fn phenopacket_json_round_trip() {
        let pp = v2::phenopacket();
        let mut buf: Vec<u8> = Vec::new();

        serde_json::to_writer(&mut buf, &pp).expect("Expecting no serialization issues");

        let decoded: Phenopacket =
            serde_json::from_reader(&buf[..]).expect("Expecting no deserialization issues");

        assert_eq!(pp, decoded);
    }

    #[test]
    #[ignore = "run manually to generate `data/v2/phenopacket.json`"]
    fn encode_phenopacket() {
        let fpath_pp = "data/v2/phenopacket.json";
        let pp = v2::phenopacket();

        let val = serde_json::to_string_pretty(&pp).expect("Expecting no serialization issues");
        std::fs::write(fpath_pp, &val).expect("It should be possible to write into the file");
    }

    #[test]
    fn decode_family() {
        let fpath_pp = "data/v2/family.json";
        let bytes = std::fs::read(fpath_pp).expect("The test file should be in the repo");

        let actual: Family =
            serde_json::from_reader(&bytes[..]).expect("Expecting no deserialization issues");

        let expected = v2::family();

        // assert_eq!(actual.id, expected.id);
        // assert_eq!(actual.proband, expected.proband);
        // assert_eq!(actual.relatives, expected.relatives);
        // assert_eq!(actual.consanguinous_parents, expected.consanguinous_parents);
        // assert_eq!(actual.pedigree, expected.pedigree);
        // assert_eq!(actual.files, expected.files);
        // assert_eq!(actual.meta_data, expected.meta_data);

        assert_eq!(actual, expected);
    }

    #[test]
    fn family_json_round_trip() {
        let expected = v2::family();
        let mut buf: Vec<u8> = Vec::new();

        serde_json::to_writer(&mut buf, &expected).expect("Expecting no serialization issues");

        let decoded: Family =
            serde_json::from_reader(&buf[..]).expect("Expecting no deserialization issues");

        assert_eq!(expected, decoded);
    }

    #[test]
    #[ignore = "run manually to generate `data/v2/family.json`"]
    fn encode_family() {
        let fpath = "data/v2/family.json";
        let val = v2::family();

        let json = serde_json::to_string_pretty(&val).expect("Expecting no serialization issues");
        std::fs::write(fpath, &json).expect("It should be possible to write into the file");
    }

    #[test]
    fn decode_cohort() {
        let fpath = "data/v2/cohort.json";
        let bytes = std::fs::read(fpath).expect("The test file should be in the repo");

        let actual: Cohort =
            serde_json::from_reader(&bytes[..]).expect("Expecting no deserialization issues");

        let expected = v2::cohort();

        // assert_eq!(actual.id, expected.id);
        // assert_eq!(actual.description, expected.description);
        // assert_eq!(actual.members, expected.members);
        // assert_eq!(actual.files, expected.files);
        // assert_eq!(actual.meta_data, expected.meta_data);

        assert_eq!(actual, expected);
    }

    #[test]
    fn cohort_json_round_trip() {
        let expected = v2::cohort();
        let mut buf: Vec<u8> = Vec::new();

        serde_json::to_writer(&mut buf, &expected).expect("Expecting no serialization issues");

        let decoded: Cohort =
            serde_json::from_reader(&buf[..]).expect("Expecting no deserialization issues");

        assert_eq!(expected, decoded);
    }

    #[test]
    #[ignore = "run manually to generate `data/v2/cohort.json`"]
    fn encode_cohort() {
        let fpath = "data/v2/cohort.json";
        let val = v2::cohort();

        let json = serde_json::to_string_pretty(&val).expect("Expecting no serialization issues");
        std::fs::write(fpath, &json).expect("It should be possible to write into the file");
    }
}
