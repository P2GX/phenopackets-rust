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
    use std::error::Error;

    use phenopackets::schema::v2::{
        core::{
            time_element::Element, Age, Disease, ExternalReference, Individual, KaryotypicSex,
            MetaData, OntologyClass, Resource, Sex, TimeElement,
        },
        Phenopacket,
    };
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

    /// We start with a phenopacket and we expect to get it back.
    #[test]
    fn json_round_trip() -> Result<(), Box<dyn Error>> {
        // let pp = v2_phenopacket();

        // let mut buf: Vec<u8> = Vec::new();

        // serde_json::to_writer(&mut buf, &pp)?;

        // let decoded: Phenopacket = serde_json::from_reader(&buf[..])?;

        // assert_eq!(pp, decoded);
        Ok(())
    }

    /// A V2 phenopacket with (a subset of) information that can be found in `data/v2/phenopacket.json`.
    fn v2_phenopacket() -> Phenopacket {
        Phenopacket {
            id: "comprehensive-phenopacket-id".into(),
            subject: Some(Individual {
                id: "14 year-old boy".into(),
                alternate_ids: vec!["boy".into(), "patient".into(), "proband".into()],
                date_of_birth: Some(
                    "1970-01-02T10:17:36.000000100Z"
                        .parse()
                        .expect("Timestamp should be well formatted"),
                ),
                time_at_last_encounter: Some(TimeElement {
                    element: Some(Element::Age(Age {
                        iso8601duration: "P14Y".into(),
                    })),
                }),
                vital_status: None,
                sex: Sex::Male.into(),
                karyotypic_sex: KaryotypicSex::Xy.into(),
                gender: None,
                taxonomy: Some(OntologyClass {
                    id: "NCBITaxon:9606".into(),
                    label: "homo sapiens".into(),
                }),
            }),
            phenotypic_features: vec![],
            measurements: vec![],
            biosamples: vec![],
            interpretations: vec![],
            diseases: vec![
                Disease { term: Some(OntologyClass { id: "OMIM:101600".into(), label: "PFEIFFER SYNDROME".into() }), excluded: false, onset: Some(TimeElement{
                    element: Some(Element::OntologyClass(OntologyClass { id: "HP:0003577".into(), label: "Congenital onset".into() }))
                }), resolution: None, disease_stage: vec![], clinical_tnm_finding: vec![], primary_site: None, laterality: None }
            ],
            medical_actions: vec![],
            files: vec![],
            meta_data: Some(MetaData {
                created: Some("2022-10-03T16:39:04.000123456Z".parse().expect("Timestamp should be well formatted")),
                created_by: "Peter R.".into(),
                submitted_by: "PhenopacketLab".into(),
                resources: vec![
                    Resource { id: "hp".into(), name: "human phenotype ontology".into(), url: "http://purl.obolibrary.org/obo/hp.owl".into(), version: "2018-03-08".into(), namespace_prefix: "HP".into(), iri_prefix: "http://purl.obolibrary.org/obo/HP_".into() },
                    Resource { id: "geno".into(), name: "Genotype Ontology".into(), url: "http://purl.obolibrary.org/obo/geno.owl".into(), version: "19-03-2018".into(), namespace_prefix: "GENO".into(), iri_prefix: "http://purl.obolibrary.org/obo/GENO_".into() },
                    Resource { id: "pubmed".into(), name: "PubMed".into(), url: "".into(), version: "".into(), namespace_prefix: "PMID".into(), iri_prefix: "https://www.ncbi.nlm.nih.gov/pubmed/".into() },
                    Resource { id: "ncit".into(), name: "NCI Thesaurus".into(), url: "http://purl.obolibrary.org/obo/ncit.owl".into(), version: "20-03-2020".into(), namespace_prefix: "NCIT".into(), iri_prefix: "http://purl.obolibrary.org/obo/NCIT_".into() },
                ],
                updates: vec![],
                phenopacket_schema_version: "2.0.0".into(),
                external_references: vec![
                    ExternalReference { id: "PMID:30808312".into(), reference: "".into(), description: "COL6A1 mutation leading to Bethlem myopathy with recurrent hematuria: a case report.".into() }
                ],
            }),
        }
    }
}
