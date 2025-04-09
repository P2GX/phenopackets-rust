//! Example Phenopacket Schema elements.
pub mod v1 {
    use crate::schema::v1::Phenopacket;

    pub fn phenopacket() -> Phenopacket {
        todo!()
    }
}

pub mod v2 {

    use crate::{
        generated::{
            org_ga4gh_vrsatile_v1::{Expression, MoleculeContext, VariationDescriptor},
            org_phenopackets_schema_v2_core::{
                genomic_interpretation::{Call, InterpretationStatus},
                interpretation::ProgressStatus,
                time_element::Element,
                AcmgPathogenicityClassification, Age, Biosample, Diagnosis, Disease, Evidence,
                ExternalReference, File, GenomicInterpretation, Individual, Interpretation,
                KaryotypicSex, MetaData, OntologyClass, PhenotypicFeature, Resource, Sex,
                TherapeuticActionability, TimeElement, VariantInterpretation,
            },
        },
        schema::v2::Phenopacket,
    };

    /// A phenopacket corresponding to one at `data/v2/phenopacket.json`.
    ///
    /// The content MUST correspond to file's contents at all times.
    /// If one of them changes, the other MUST change too!
    pub fn phenopacket() -> Phenopacket {
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
                time_at_last_encounter: Some(time_element_age("P14Y")),
                vital_status: None,
                sex: Sex::Male.into(),
                karyotypic_sex: KaryotypicSex::Xy.into(),
                gender: None,
                taxonomy: Some(ontology_class("NCBITaxon:9606", "homo sapiens")),
            }),
            phenotypic_features: vec![
                PhenotypicFeature {
                    description: "".into(), 
                    r#type: Some(ontology_class("HP:0001558", "Decreased fetal movement")), 
                    excluded: false,
                    severity: None,
                    modifiers: vec![],
                    onset: Some(
                        time_element_ontology_class("HP:0011461", "Fetal onset")),
                    resolution: None,
                    evidence: vec![
                        Evidence {
                            evidence_code: Some(ontology_class("ECO:0000033", "author statement supported by traceable reference")),
                            reference: Some(
                                external_reference("PMID:30808312", "", "COL6A1 mutation leading to Bethlem myopathy with recurrent hematuria: a case report.")
                            )
                        }
                    ],
                },
                PhenotypicFeature {
                    description: "".into(),
                    r#type: Some(ontology_class("HP:0031910", "Abnormal cranial nerve physiology")),
                    excluded: true,
                    severity: None,
                    modifiers: vec![],
                    onset: None,
                    resolution: None,
                    evidence: vec![
                        Evidence {
                            evidence_code: Some(ontology_class("ECO:0000033", "author statement supported by traceable reference")),
                            reference: Some(
                                external_reference("PMID:30808312", "", "COL6A1 mutation leading to Bethlem myopathy with recurrent hematuria: a case report.")
                            )
                        }
                    ],
                },
                PhenotypicFeature {
                    description: "".into(),
                    r#type: Some(ontology_class("HP:0011463", "Macroscopic hematuria")),
                    excluded: false,
                    severity: None,
                    modifiers: vec![
                        ontology_class("HP:0031796", "Recurrent")
                    ],
                    onset: Some(time_element_age("P14Y")),
                    resolution: None,
                    evidence: vec![
                        Evidence {
                            evidence_code: Some(ontology_class("ECO:0000033", "author statement supported by traceable reference")),
                            reference: Some(
                                external_reference("PMID:30808312", "", "COL6A1 mutation leading to Bethlem myopathy with recurrent hematuria: a case report.")
                            )
                        }
                    ],
                },
                PhenotypicFeature {
                    description: "".into(),
                    r#type: Some(ontology_class("HP:0001270", "Motor delay")),
                    excluded: false,
                    severity: Some(ontology_class("HP:0012825", "Mild")),
                    modifiers: vec![],
                    onset: Some(time_element_ontology_class("HP:0011463", "Childhood onset")),
                    resolution: None,
                    evidence: vec![],
                },
            ],
            measurements: vec![],
            biosamples: vec![
                Biosample {
                    id: "biosample-id".into(),
                    individual_id: "14 year-old boy".into(),
                    derived_from_id: "".into(),
                    description: "Muscle biopsy of 14 year-old boy".into(),
                    sampled_tissue: Some(ontology_class("UBERON:0003403", "skin of forearm")),
                    sample_type: None,
                    phenotypic_features: vec![],
                    measurements: vec![],
                    taxonomy: Some(ontology_class("NCBITaxon:9606", "homo sapiens")),
                    time_of_collection: Some(time_element_age("P14Y")),
                    histological_diagnosis: Some(ontology_class("NCIT:C38757", "Negative Finding")),
                    tumor_progression: Some(ontology_class("NCIT:C3677", "Benign Neoplasm")),
                    tumor_grade: Some(ontology_class("NCIT:C28076", "Disease Grade Qualifier")),
                    pathological_stage: None,
                    pathological_tnm_finding: vec![],
                    diagnostic_markers: vec![ontology_class("NCIT:C68748", "HER2/Neu Positive")],
                    procedure: None,
                    files: vec![],
                    material_sample: Some(ontology_class("EFO:0009655", "abnormal sample")),
                    sample_processing: None,
                    sample_storage: None,
                }
            ],
            interpretations: vec![
                Interpretation {
                    id: "comprehensive-phenopacket-id".into(),
                    progress_status: ProgressStatus::Solved.into(),
                    diagnosis: Some(Diagnosis {
                        disease: Some(ontology_class("OMIM:101600", "PFEIFFER SYNDROME")),
                        genomic_interpretations: vec![
                            GenomicInterpretation {
                                subject_or_biosample_id: "14 year-old boy".into(),
                                interpretation_status: InterpretationStatus::Causative.into(),
                                call: Some(Call::VariantInterpretation(VariantInterpretation {
                                    acmg_pathogenicity_classification: AcmgPathogenicityClassification::NotProvided.into(),
                                    therapeutic_actionability: TherapeuticActionability::UnknownActionability.into(),
                                    variation_descriptor: Some(VariationDescriptor {
                                         id: "".into(),
                                         variation: None,
                                         label: "".into(),
                                         description: "".into(),
                                         gene_context: None,
                                         expressions: vec![
                                            Expression {
                                                syntax: "hgvs".into(),
                                                value: "NM_001848.2:c.877G>A".into(),
                                                version: "".into(),
                                            }
                                         ],
                                         vcf_record: None,
                                         xrefs: vec![],
                                         alternate_labels: vec![],
                                         extensions: vec![],
                                         molecule_context: MoleculeContext::UnspecifiedMoleculeContext.into(),
                                         structural_type: None,
                                         vrs_ref_allele_seq: "".into(),
                                         allelic_state: Some(ontology_class("GENO:0000135", "heterozygous")),
                                    })
                                })),
                            }
                        ],
                    }),
                    summary: "".into(),
                }
            ],
            diseases: vec![
                Disease {
                    term: Some(ontology_class("OMIM:101600", "PFEIFFER SYNDROME")),
                    excluded: false,
                    onset: Some(time_element_ontology_class("HP:0003577", "Congenital onset")),
                    resolution: None,
                    disease_stage: vec![],
                    clinical_tnm_finding: vec![],
                    primary_site: None,
                    laterality: None,
                }
            ],
            medical_actions: vec![],
            files: vec![
                File {
                    uri: "file://data/genomes/P000001C".into(),
                    individual_to_file_identifiers: [("14 year-old boy", "P000001C")].into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
                    file_attributes: [("genomeAssembly", "GRCh38.p13"), ("fileFormat", "vcf"), ("description", "Whole genome sequencing VCF output")].into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
                }
            ],
            meta_data: Some(MetaData {
                created: Some("2022-10-03T16:39:04.000123456Z".parse().expect("Timestamp should be well formatted")),
                created_by: "Peter R.".into(),
                submitted_by: "PhenopacketLab".into(),
                resources: vec![
                    Resource {
                        id: "hp".into(),
                        name: "human phenotype ontology".into(),
                        url: "http://purl.obolibrary.org/obo/hp.owl".into(),
                        version: "2018-03-08".into(),
                        namespace_prefix: "HP".into(),
                        iri_prefix: "http://purl.obolibrary.org/obo/HP_".into(),
                    },
                    Resource {
                        id: "geno".into(),
                        name: "Genotype Ontology".into(),
                        url: "http://purl.obolibrary.org/obo/geno.owl".into(),
                        version: "19-03-2018".into(),
                        namespace_prefix: "GENO".into(),
                        iri_prefix: "http://purl.obolibrary.org/obo/GENO_".into(),
                    },
                    Resource {
                        id: "pubmed".into(),
                        name: "PubMed".into(),
                        url: "".into(),
                        version: "".into(),
                        namespace_prefix: "PMID".into(),
                        iri_prefix: "https://www.ncbi.nlm.nih.gov/pubmed/".into(),
                    },
                    Resource {
                        id: "ncit".into(),
                        name: "NCI Thesaurus".into(),
                        url: "http://purl.obolibrary.org/obo/ncit.owl".into(),
                        version: "20-03-2020".into(),
                        namespace_prefix: "NCIT".into(),
                        iri_prefix: "http://purl.obolibrary.org/obo/NCIT_".into(),
                    },
                    // TODO: add
                    // - OMIM
                    // - EFO
                    // - ECO
                    // - NCBITaxon
                    // - UBERON
                    
                ],
                updates: vec![],
                phenopacket_schema_version: "2.0.0".into(),
                external_references: vec![
                    external_reference("PMID:30808312", "", "COL6A1 mutation leading to Bethlem myopathy with recurrent hematuria: a case report.")
                ],
            }),
        }
    }

    fn ontology_class(id: &str, label: &str) -> OntologyClass {
        OntologyClass {
            id: id.into(),
            label: label.into(),
        }
    }

    fn external_reference(
        id: impl ToString,
        reference: impl ToString,
        description: impl ToString,
    ) -> ExternalReference {
        ExternalReference {
            id: id.to_string(),
            reference: reference.to_string(),
            description: description.to_string(),
        }
    }

    fn time_element_age(iso8601duration: impl ToString) -> TimeElement {
        TimeElement {
            element: Some(Element::Age(Age {
                iso8601duration: iso8601duration.to_string(),
            })),
        }
    }

    fn time_element_ontology_class(id: &str, label: &str) -> TimeElement {
        TimeElement {
            element: Some(Element::OntologyClass(ontology_class(id, label))),
        }
    }
}
