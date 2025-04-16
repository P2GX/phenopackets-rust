/// Test de-/serialization of top-level elements of `v2` Phenopacket Schema
/// from/to JSON format.
mod v2 {
    use std::error::Error;

    use crate::{schema::v2::Phenopacket, tests::examples::v2};

    #[test]
    fn phenopacket_json_round_trip() -> Result<(), Box<dyn Error>> {
        let pp = v2::phenopacket();
        // let mut buf: Vec<u8> = Vec::new();

        // serde_json::to_writer(&mut buf, &pp)?;

        // let decoded: Phenopacket = serde_json::from_reader(&buf[..])?;

        // assert_eq!(pp, decoded);
        Ok(())
    }

    #[test]
    #[ignore = "run manually for now"]
    fn decode_phenopacket() {
        let fpath_pp = "data/v2/phenopacket.json";
        let bytes = std::fs::read(fpath_pp).expect("The test file should be in the repo");

        let actual: Phenopacket =
            serde_json::from_reader(&bytes[..]).expect("Expecting no deserialization issues");

        let expected = v2::phenopacket();

        assert_eq!(actual.id, expected.id);
        assert_eq!(actual.subject, expected.subject);
        assert_eq!(actual.phenotypic_features, expected.phenotypic_features);
        assert_eq!(actual.measurements, expected.measurements);
        assert_eq!(actual.biosamples, expected.biosamples);
        assert_eq!(actual.interpretations, expected.interpretations);
        assert_eq!(actual.diseases, expected.diseases);
        assert_eq!(actual.medical_actions, expected.medical_actions);
        assert_eq!(actual.files, expected.files);
        assert_eq!(actual.meta_data, expected.meta_data);

        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore = "for running manually only"]
    fn encode_phenopacket() -> Result<(), Box<dyn Error>> {
        let pp = v2::phenopacket();

        let val = serde_json::to_string_pretty(&pp)?;
        std::fs::write("pp.json", &val)?;

        Ok(())
    }
}
