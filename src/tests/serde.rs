mod v2 {
    use std::error::Error;

    use crate::{
        schema::v2::Phenopacket,
        tests::{examples::v2, read_path},
    };

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
        // let bytes = read_path(fpath_pp);

        // let actual: Phenopacket = serde_json::from_reader(&bytes[..])?;

        // let expected = v2::phenopacket();

        // assert_eq!(actual, expected);

        Ok(())
    }
}
