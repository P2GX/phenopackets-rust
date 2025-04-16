/// Test de-/serialization of top-level elements of `v2` Phenopacket Schema
/// from/to JSON format.
mod v2 {

    use crate::{schema::v2::Phenopacket, tests::examples::v2};

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
}
