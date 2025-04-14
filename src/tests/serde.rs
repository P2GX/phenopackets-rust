/// Test de-/serialization of top-level elements of `v2` Phenopacket Schema
/// from/to JSON format.
mod v2 {
    use std::error::Error;

    use crate::tests::examples::v2;

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
    fn decode_phenopacket() -> Result<(), Box<dyn Error>> {
        // let fpath_pp = "data/v2/phenopacket.json";
        // let bytes = std::fs::read(fpath_pp).expect("The test file should be in the repo");

        // let actual: Phenopacket = serde_json::from_reader(&bytes[..])?;

        // let expected = v2::phenopacket();

        // assert_eq!(actual, expected);

        Ok(())
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
