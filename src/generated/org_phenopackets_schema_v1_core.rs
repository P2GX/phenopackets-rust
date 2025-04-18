// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaData {
    /// ISO8601 UTC timestamp for when this phenopacket was created in ISO "2018-03-01T00:00:00Z"
    #[prost(message, optional, tag = "1")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    /// some kind of identifier for the contributor/ program
    #[prost(string, tag = "2")]
    pub created_by: ::prost::alloc::string::String,
    /// information about the person/organisation/network that has submitted this phenopacket
    #[prost(string, tag = "3")]
    pub submitted_by: ::prost::alloc::string::String,
    /// a listing of the ontologies and resources referenced in the phenopacket
    #[prost(message, repeated, tag = "4")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
    /// An OPTIONAL list of Updates to the phenopacket.
    #[prost(message, repeated, tag = "5")]
    pub updates: ::prost::alloc::vec::Vec<Update>,
    /// phenopacket-schema-version used to create this phenopacket
    #[prost(string, tag = "6")]
    pub phenopacket_schema_version: ::prost::alloc::string::String,
    /// External identifiers for this message. These are considered different representation of the same record, not
    /// records which are in some other relation with the record at hand. For example this might be a PubMed reference
    /// to a study in which the individuals are reported.
    #[prost(message, repeated, tag = "7")]
    pub external_references: ::prost::alloc::vec::Vec<ExternalReference>,
}
/// Information about when an update to a record occurred, who or what made the update and any pertinent information
/// regarding the content and/or reason for the update
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Update {
    /// ISO8601 UTC timestamps at which this record was updated, in
    /// the format YYYY-MM-DDTHH:MM:SS.SSSZ e.g. 2007-12-03T10:15:30.00Z
    /// REQUIRED
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Information about the person/organisation/network that has updated the phenopacket.
    /// OPTIONAL
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
    /// Textual comment about the changes made to the content and/or reason for the update.
    /// OPTIONAL
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
}
/// Description of an external resource used for referencing an object. For example the resource may be an ontology such
/// as the HPO or SNOMED.
/// FHIR mapping: CodeSystem (<http://www.hl7.org/fhir/codesystem.html>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// for OBO Ontologies, the value of this string MUST always be the official
    /// OBO ID, which is always equivalent to the ID prefix in lower case.
    /// Examples: hp, go, mp, mondo
    /// Consult <http://obofoundry.org> for a complete list
    /// For other ontologies (e.g. SNOMED), use the prefix in identifiers.org
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// e.g. The Human Phenotype Ontology
    /// for OBO Ontologies, the value of this string SHOULD be the same as the title
    /// field on <http://obofoundry.org>
    /// however, this field is purely for information purposes and software
    /// should not encode any assumptions
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// For OBO ontologies, this should always be the PURL, e.g.
    /// <http://purl.obolibrary.org/obo/hp.owl,> <http://purl.obolibrary.org/obo/hp.obo>
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    /// for OBO ontologies, this should be the versionIRI
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// The prefix used in the CURIE of an OntologyClass e.g. HP, MP, ECO
    /// For example an HPO term will have a CURIE like this - HP:0012828 which should be used in combination with
    /// the iri_prefix to form a fully-resolvable IRI
    #[prost(string, tag = "5")]
    pub namespace_prefix: ::prost::alloc::string::String,
    /// Full IRI prefix which can be used with the namespace_prefix and the OntologyClass::id to resolve to an IRI for a
    /// term. Tools such as the curie-util (<https://github.com/prefixcommons/curie-util>) can utilise this to produce
    /// fully-resolvable IRIs for an OntologyClass.
    /// e.g. Using the HPO term encoding the concept of 'Severe'
    ///     OntologyClass:
    ///         id: 'HP:0012828'
    ///         label: 'Severe'
    ///     Resource:
    ///         namespace_prefix: 'HP'
    ///         iri_prefix: '<http://purl.obolibrary.org/obo/HP_'>
    /// the term can be resolved to <http://purl.obolibrary.org/obo/HP_0012828>
    #[prost(string, tag = "6")]
    pub iri_prefix: ::prost::alloc::string::String,
}
/// A class (aka term, concept) in an ontology.
/// FHIR mapping: CodeableConcept (<http://www.hl7.org/fhir/datatypes.html#CodeableConcept>)
///    see also Coding (<http://www.hl7.org/fhir/datatypes.html#Coding>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OntologyClass {
    /// a CURIE-style identifier e.g. HP:0100024, MP:0001284, UBERON:0001690.
    /// This is the primary key for the ontology class
    /// REQUIRED!
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// class label, aka name. E.g. "Abnormality of cardiovascular system"
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
}
/// An individual phenotypic feature, observed as either present or absent (negated), with possible onset, modifiers and
/// frequency
/// FHIR mapping: Condition (<https://www.hl7.org/fhir/condition.html>) or Observation (<https://www.hl7.org/fhir/observation.html>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhenotypicFeature {
    /// Free-text description of the phenotype. Note this is not a acceptable place to document/describe the phenotype -
    /// the type and onset etc... fields should be used for this purpose.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// The primary ontology class which describes the phenotype. For example "HP:0001363"  "Craniosynostosis"
    /// FHIR mapping: Condition.identifier
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<OntologyClass>,
    /// Flag to indicate whether the phenotype was observed or not. Default is 'false', in other words the phenotype was
    /// observed. Therefore it is only required in cases to indicate that the phenotype was looked for, but found to be
    /// absent.
    /// More formally, this modifier indicates the logical negation of the OntologyClass used in the 'type' field.
    /// *CAUTION* It is imperative to check this field for correct interpretation of the phenotype!
    #[prost(bool, tag = "3")]
    pub negated: bool,
    /// Severity of the condition e.g. subclasses of HP:0012824-Severity or SNOMED:272141005-Severities
    /// FHIR mapping: Condition.severity
    #[prost(message, optional, tag = "4")]
    pub severity: ::core::option::Option<OntologyClass>,
    /// subclasses of HP:0012823 ! Clinical modifier apart from Severity HP:0012824 - Severity
    #[prost(message, repeated, tag = "5")]
    pub modifiers: ::prost::alloc::vec::Vec<OntologyClass>,
    /// Evidences for how the phenotype was determined.
    #[prost(message, repeated, tag = "10")]
    pub evidence: ::prost::alloc::vec::Vec<Evidence>,
    /// the values of this will come from the HPO onset hierarchy
    /// i.e. subclasses of HP:0003674
    /// FHIR mapping: Condition.onset
    #[prost(oneof = "phenotypic_feature::Onset", tags = "6, 7, 9")]
    pub onset: ::core::option::Option<phenotypic_feature::Onset>,
}
/// Nested message and enum types in `PhenotypicFeature`.
pub mod phenotypic_feature {
    /// the values of this will come from the HPO onset hierarchy
    /// i.e. subclasses of HP:0003674
    /// FHIR mapping: Condition.onset
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Onset {
        #[prost(message, tag = "6")]
        AgeOfOnset(super::Age),
        #[prost(message, tag = "7")]
        AgeRangeOfOnset(super::AgeRange),
        #[prost(message, tag = "9")]
        ClassOfOnset(super::OntologyClass),
    }
}
/// FHIR mapping: Condition.evidence (<https://www.hl7.org/fhir/condition-definitions.html#Condition.evidence>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Evidence {
    /// The encoded evidence type using, for example the Evidence & Conclusion Ontology (ECO - <http://purl.obolibrary.org/obo/eco.owl>)
    /// FHIR mapping: Condition.evidence.code
    #[prost(message, optional, tag = "1")]
    pub evidence_code: ::core::option::Option<OntologyClass>,
    /// FHIR mapping: Condition.evidence.detail
    #[prost(message, optional, tag = "2")]
    pub reference: ::core::option::Option<ExternalReference>,
}
/// FHIR mapping: Reference (<https://www.hl7.org/fhir/references.html>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalReference {
    /// e.g. ISBN, PMID:123456, DOI:...,
    /// FHIR mapping: Reference.identifier
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// FHIR mapping: Reference.display
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
/// An individual (or subject) typically corresponds to an individual human or another organism.
/// FHIR mapping: Patient (<https://www.hl7.org/fhir/patient.html>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Individual {
    /// An identifier for the individual. This must be unique within the record.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// An optional list of alternative identifiers for this individual. This field is provided
    /// for the convenience of users who may have multiple mappings to an individual which they need to track.
    #[prost(string, repeated, tag = "2")]
    pub alternate_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The date of birth of the individual as an ISO8601 UTC timestamp - rounded down to the closest known
    /// year/month/day/hour/minute
    /// e.g. "2018-03-01T00:00:00Z" for someone born on an unknown day in March 2018
    /// or "2018-01-01T00:00:00Z" for someone born on an unknown day in 2018
    /// or empty if unknown/ not stated.
    #[prost(message, optional, tag = "3")]
    pub date_of_birth: ::core::option::Option<::prost_types::Timestamp>,
    /// The phenotypic sex of the individual
    #[prost(enumeration = "Sex", tag = "6")]
    pub sex: i32,
    /// The karyotypic sex of the individual
    #[prost(enumeration = "KaryotypicSex", tag = "7")]
    pub karyotypic_sex: i32,
    /// NCBI taxonomic identifier NCBITaxon e.g. NCBITaxon:9606 or NCBITaxon:1337
    /// For resources where there may be more than one organism being studied it is advisable to indicate the taxonomic
    /// identifier of that organism, to its most specific level
    #[prost(message, optional, tag = "8")]
    pub taxonomy: ::core::option::Option<OntologyClass>,
    /// An age object describing the age of the individual at the time of collection. The Age object allows the encoding
    /// of the age either as ISO8601 duration or time interval (preferred), or as ontology term object.
    /// See <http://build.fhir.org/datatypes>
    #[prost(oneof = "individual::Age", tags = "4, 5")]
    pub age: ::core::option::Option<individual::Age>,
}
/// Nested message and enum types in `Individual`.
pub mod individual {
    /// An age object describing the age of the individual at the time of collection. The Age object allows the encoding
    /// of the age either as ISO8601 duration or time interval (preferred), or as ontology term object.
    /// See <http://build.fhir.org/datatypes>
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Age {
        #[prost(message, tag = "4")]
        AgeAtCollection(super::Age),
        #[prost(message, tag = "5")]
        AgeRangeAtCollection(super::AgeRange),
    }
}
/// A Biosample refers to a unit of biological material from which the substrate
/// molecules (e.g. genomic DNA, RNA, proteins) for molecular analyses (e.g.
/// sequencing, array hybridisation, mass-spectrometry) are extracted. Examples
/// would be a tissue biopsy, a single cell from a culture for single cell genome
/// sequencing or a protein fraction from a gradient centrifugation.
/// Several instances (e.g. technical replicates) or types of experiments (e.g.
/// genomic array as well as RNA-seq experiments) may refer to the same Biosample.
/// FHIR mapping: Specimen (<http://www.hl7.org/fhir/specimen.html>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Biosample {
    /// The Biosample id This is unique in the
    /// context of the server instance.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The id of the individual this biosample was derived from.
    #[prost(string, tag = "2")]
    pub individual_id: ::prost::alloc::string::String,
    /// The biosample's description. This attribute contains human readable text.
    /// The "description" attributes should not contain any structured data.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// UBERON class describing the tissue from which the specimen was collected.
    /// PDX-MI mapping: 'Specimen tumor tissue'
    /// FHIR mapping: Specimen.type
    #[prost(message, optional, tag = "4")]
    pub sampled_tissue: ::core::option::Option<OntologyClass>,
    /// Phenotypic characteristics of the BioSample, for example histological findings of a biopsy.
    #[prost(message, repeated, tag = "5")]
    pub phenotypic_features: ::prost::alloc::vec::Vec<PhenotypicFeature>,
    /// NCBI taxonomic identifier (NCBITaxon) of the sample e.g. NCBITaxon:9606
    #[prost(message, optional, tag = "6")]
    pub taxonomy: ::core::option::Option<OntologyClass>,
    /// This is the pathologist’s diagnosis and may often represent a refinement of the clinical diagnosis given in the
    /// Patient/Clinical module. Should use the same terminology as diagnosis, but represent the pathologist’s findings.
    /// Normal samples would be tagged with the term "NCIT:C38757", "Negative Finding"
    #[prost(message, optional, tag = "9")]
    pub histological_diagnosis: ::core::option::Option<OntologyClass>,
    /// Is the specimen tissue from the primary tumor, a metastasis or a recurrence?
    /// Most likely a child term of NCIT:C7062 (Neoplasm by Special Category)
    /// NCIT:C3677 (Benign Neoplasm)
    /// NCIT:C84509 (Primary Malignant Neoplasm)
    /// NCIT:C95606 (Second Primary Malignant Neoplasm)
    /// NCIT:C3261 (Metastatic Neoplasm)
    /// NCIT:C4813 (Recurrent Malignant Neoplasm)
    #[prost(message, optional, tag = "10")]
    pub tumor_progression: ::core::option::Option<OntologyClass>,
    /// Potentially a child term of NCIT:C28076 (Disease Grade Qualifier) or equivalent
    /// See <https://www.cancer.gov/about-cancer/diagnosis-staging/prognosis/tumor-grade-fact-sheet>
    #[prost(message, optional, tag = "11")]
    pub tumor_grade: ::core::option::Option<OntologyClass>,
    /// Clinically relevant bio markers. Most of the assays such as IHC are covered by the NCIT under the sub-hierarchy
    /// NCIT:C25294 (Laboratory Procedure).
    /// e.g. NCIT:C68748 (HER2/Neu Positive), NCIT:C131711 (Human Papillomavirus-18 Positive)
    #[prost(message, repeated, tag = "12")]
    pub diagnostic_markers: ::prost::alloc::vec::Vec<OntologyClass>,
    /// Clinical procedure performed on the subject in order to extract the biosample.
    #[prost(message, optional, tag = "13")]
    pub procedure: ::core::option::Option<Procedure>,
    /// Pointer to the relevant HTS file(s) for the biosample.
    #[prost(message, repeated, tag = "14")]
    pub hts_files: ::prost::alloc::vec::Vec<HtsFile>,
    /// Field for variants relevant to this biosample.
    #[prost(message, repeated, tag = "15")]
    pub variants: ::prost::alloc::vec::Vec<Variant>,
    /// if true, this sample is being use as a normal control, often in combination with
    /// another sample that is thought to contain a pathological finding
    /// the default value is false
    #[prost(bool, tag = "16")]
    pub is_control_sample: bool,
    /// An age object describing the age of the individual this biosample was
    /// derived from at the time of collection. The Age object allows the encoding
    /// of the age either as ISO8601 duration or time interval (preferred), or
    /// as ontology term object.
    /// See <http://build.fhir.org/datatypes>
    #[prost(oneof = "biosample::IndividualAgeAtCollection", tags = "7, 8")]
    pub individual_age_at_collection: ::core::option::Option<biosample::IndividualAgeAtCollection>,
}
/// Nested message and enum types in `Biosample`.
pub mod biosample {
    /// An age object describing the age of the individual this biosample was
    /// derived from at the time of collection. The Age object allows the encoding
    /// of the age either as ISO8601 duration or time interval (preferred), or
    /// as ontology term object.
    /// See <http://build.fhir.org/datatypes>
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum IndividualAgeAtCollection {
        #[prost(message, tag = "7")]
        AgeOfIndividualAtCollection(super::Age),
        #[prost(message, tag = "8")]
        AgeRangeOfIndividualAtCollection(super::AgeRange),
    }
}
/// A clinical procedure performed on a subject. By preference a single concept to indicate both the procedure and the
/// body site should be used. In cases where this is not possible, the body site should be indicated using a separate
/// ontology class.
/// e.g.
/// {"code":{"NCIT:C51585": "Biopsy of Soft Palate"}}
/// {"code":{"NCIT:C28743": "Punch Biopsy"}, "body_site":{"UBERON:0003403": "skin of forearm"}} - a punch biopsy of the skin from the forearm
/// FHIR mapping: Procedure (<https://www.hl7.org/fhir/procedure.html>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Procedure {
    ///
    /// FHIR mapping: Procedure.code
    #[prost(message, optional, tag = "1")]
    pub code: ::core::option::Option<OntologyClass>,
    /// FHIR mapping: Procedure.bodySite
    #[prost(message, optional, tag = "2")]
    pub body_site: ::core::option::Option<OntologyClass>,
}
/// See <http://build.fhir.org/datatypes> and <http://build.fhir.org/condition-definitions.html#Condition.onset_x_>
/// In FHIR this is represented as a UCUM measurement - <http://unitsofmeasure.org/trac/>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Age {
    /// The :ref:`ISO 8601<metadata_date_time>` age of this object as ISO8601
    /// duration or time intervals. The use of time intervals makes an additional
    /// anchor unnecessary (i.e. DOB and age can be represented as start-anchored
    /// time interval, e.g. 1967-11-21/P40Y10M05D)
    #[prost(string, tag = "1")]
    pub age: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgeRange {
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<Age>,
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<Age>,
}
/// Message to indicate a disease (diagnosis) and its recorded onset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Disease {
    /// The identifier of this disease e.g. MONDO:0007043, OMIM:101600, Orphanet:710, DOID:14705 (note these are all equivalent)
    #[prost(message, optional, tag = "1")]
    pub term: ::core::option::Option<OntologyClass>,
    /// Disease staging, the extent to which a disease has developed.
    /// For cancers, see <https://www.cancer.gov/about-cancer/diagnosis-staging/staging>
    /// Valid values include child terms of NCIT:C28108 (Disease Stage Qualifier)
    #[prost(message, repeated, tag = "5")]
    pub disease_stage: ::prost::alloc::vec::Vec<OntologyClass>,
    /// Cancer findings in the TNM system that is relevant to the diagnosis of cancer.
    /// See <https://www.cancer.gov/about-cancer/diagnosis-staging/staging>
    /// Valid values include child terms of NCIT:C48232 (Cancer TNM Finding)
    #[prost(message, repeated, tag = "6")]
    pub tnm_finding: ::prost::alloc::vec::Vec<OntologyClass>,
    /// The onset of the disease. The values of this will come from the HPO onset hierarchy
    /// i.e. subclasses of HP:0003674
    /// FHIR mapping: Condition.onset
    #[prost(oneof = "disease::Onset", tags = "2, 3, 4")]
    pub onset: ::core::option::Option<disease::Onset>,
}
/// Nested message and enum types in `Disease`.
pub mod disease {
    /// The onset of the disease. The values of this will come from the HPO onset hierarchy
    /// i.e. subclasses of HP:0003674
    /// FHIR mapping: Condition.onset
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Onset {
        #[prost(message, tag = "2")]
        AgeOfOnset(super::Age),
        #[prost(message, tag = "3")]
        AgeRangeOfOnset(super::AgeRange),
        #[prost(message, tag = "4")]
        ClassOfOnset(super::OntologyClass),
    }
}
/// <https://software.broadinstitute.org/gatk/documentation/article?id=11016>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pedigree {
    #[prost(message, repeated, tag = "1")]
    pub persons: ::prost::alloc::vec::Vec<pedigree::Person>,
}
/// Nested message and enum types in `Pedigree`.
pub mod pedigree {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Person {
        #[prost(string, tag = "1")]
        pub family_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub individual_id: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub paternal_id: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub maternal_id: ::prost::alloc::string::String,
        #[prost(enumeration = "super::Sex", tag = "5")]
        pub sex: i32,
        #[prost(enumeration = "person::AffectedStatus", tag = "6")]
        pub affected_status: i32,
    }
    /// Nested message and enum types in `Person`.
    pub mod person {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum AffectedStatus {
            Missing = 0,
            Unaffected = 1,
            Affected = 2,
        }
        impl AffectedStatus {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::Missing => "MISSING",
                    Self::Unaffected => "UNAFFECTED",
                    Self::Affected => "AFFECTED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "MISSING" => Some(Self::Missing),
                    "UNAFFECTED" => Some(Self::Unaffected),
                    "AFFECTED" => Some(Self::Affected),
                    _ => None,
                }
            }
        }
    }
}
/// A file in one of the HTS formats (<https://samtools.github.io/hts-specs>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HtsFile {
    /// URI for the file e.g. file://data/genomes/file1.vcf.gz or <https://opensnp.org/data/60.23andme-exome-vcf.231?1341012444>
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// description of the file contents
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// format of the HTS file
    #[prost(enumeration = "hts_file::HtsFormat", tag = "3")]
    pub hts_format: i32,
    /// Genome assembly the contents of this file was called against. We recommend using the Genome Reference Consortium
    /// nomenclature e.g. GRCh37, GRCh38
    #[prost(string, tag = "4")]
    pub genome_assembly: ::prost::alloc::string::String,
    /// A map of identifiers mapping an individual to a sample in the file. The key values must correspond to the
    /// Individual::id for the individuals in the message, the values must map to the samples in the file.
    #[prost(map = "string, string", tag = "5")]
    pub individual_to_sample_identifiers:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `HtsFile`.
pub mod hts_file {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HtsFormat {
        Unknown = 0,
        Sam = 1,
        Bam = 2,
        Cram = 3,
        Vcf = 4,
        Bcf = 5,
        Gvcf = 6,
        Fastq = 7,
    }
    impl HtsFormat {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unknown => "UNKNOWN",
                Self::Sam => "SAM",
                Self::Bam => "BAM",
                Self::Cram => "CRAM",
                Self::Vcf => "VCF",
                Self::Bcf => "BCF",
                Self::Gvcf => "GVCF",
                Self::Fastq => "FASTQ",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "SAM" => Some(Self::Sam),
                "BAM" => Some(Self::Bam),
                "CRAM" => Some(Self::Cram),
                "VCF" => Some(Self::Vcf),
                "BCF" => Some(Self::Bcf),
                "GVCF" => Some(Self::Gvcf),
                "FASTQ" => Some(Self::Fastq),
                _ => None,
            }
        }
    }
}
/// Gene identifier
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gene {
    /// The official gene identifier as designated by the organism gene nomenclature committee e.g. HGNC:3477 or MGI:2385071
    /// Preferentially this should be a CURIE, however it is acceptable to use another equivalent resource identifier
    /// such as ENSG00000120705, uc003ldc.6
    /// <https://www.genenames.org/data/gene-symbol-report/#!/hgnc_id/HGNC:3477>
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub alternate_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The official gene symbol as designated by the organism gene nomenclature committee e.g. ETF1 or Etf1
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
}
/// A variant allele. Alleles can be listed using HGVS, VCF, SPDI or ISCN notation.
/// SPDI format is the exchange standard used for ClinVar, dbSNP and soon the EVA
/// Tools for interconversion between SPDI, HGVS and VCF exist at <https://api.ncbi.nlm.nih.gov/variation/v0/>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Variant {
    /// Zygosity of the allele using GENO ontology
    #[prost(message, optional, tag = "6")]
    pub zygosity: ::core::option::Option<OntologyClass>,
    #[prost(oneof = "variant::Allele", tags = "2, 3, 4, 5")]
    pub allele: ::core::option::Option<variant::Allele>,
}
/// Nested message and enum types in `Variant`.
pub mod variant {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Allele {
        #[prost(message, tag = "2")]
        HgvsAllele(super::HgvsAllele),
        #[prost(message, tag = "3")]
        VcfAllele(super::VcfAllele),
        #[prost(message, tag = "4")]
        SpdiAllele(super::SpdiAllele),
        #[prost(message, tag = "5")]
        IscnAllele(super::IscnAllele),
    }
}
/// A single HGVS allele.
/// It is recommended that the string is validated using the VariantValidator - <https://variantvalidator.org/>
/// See <http://varnomen.hgvs.org/recommendations/DNA/variant/alleles/>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HgvsAllele {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// e.g., NM_000226.3:c.470T>G
    #[prost(string, tag = "2")]
    pub hgvs: ::prost::alloc::string::String,
}
/// A single VCF allele.
/// For structural variation the INFO field should contain the relevant information e.g.
/// END=43500;SVTYPE=DUP;CHR2=1;SVLEN=29000;
/// See <https://samtools.github.io/hts-specs/VCFv4.3.pdf>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VcfAllele {
    /// The value of the VCF spec fileformat field: e.g VCFv4.1, VCFv4.2, VCFv4.3
    #[prost(string, tag = "1")]
    pub vcf_version: ::prost::alloc::string::String,
    /// Genome assembly the allele was called against. We recommend using the Genome Reference Consortium
    /// nomenclature e.g. GRCh37, GRCh38
    #[prost(string, tag = "2")]
    pub genome_assembly: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub chr: ::prost::alloc::string::String,
    #[prost(int32, tag = "5")]
    pub pos: i32,
    #[prost(string, tag = "6")]
    pub r#ref: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub alt: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub info: ::prost::alloc::string::String,
}
/// A single SPDI allele.
/// The SPDI notation uses four fields and is written out as four elements delimited by colons S:P:D:I, where
/// S = SequenceId
/// P = Position , a 0-based coordinate for where the Deleted Sequence starts
/// D = DeletedSequence , sequence for the deletion, can be empty
/// I = InsertedSequence , sequence for the insertion, can be empty
/// The SPDI notation represents variation as deletion of a sequence (D) at a given position (P) in reference sequence (S)
/// followed by insertion of a replacement sequence (I) at that same position. Position 0 indicates a deletion that
/// starts immediately before the first nucleotide, and position 1 represents a deletion interval that starts between the
/// first and second residues, and so on. Either the deleted or the inserted interval can be empty, resulting a pure
/// insertion or deletion.
/// The deleted and inserted sequences in SPDI are all written on the positive strand for two-stranded molecules.
/// See <https://www.ncbi.nlm.nih.gov/variation/notation/>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpdiAllele {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub seq_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub position: i32,
    #[prost(string, tag = "4")]
    pub deleted_sequence: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub inserted_sequence: ::prost::alloc::string::String,
}
/// See <https://www.humpath.com/spip.php?article13862>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IscnAllele {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// an ICSN code such as del(6)(q23q24) , t(8;9;11)(q12;p24;p12), +15
    #[prost(string, tag = "2")]
    pub iscn: ::prost::alloc::string::String,
}
/// Sex of an individual
/// FHIR mapping: AdministrativeGender (<https://www.hl7.org/fhir/codesystem-administrative-gender.html>)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Sex {
    /// Not assessed / available.
    UnknownSex = 0,
    /// Female
    Female = 1,
    /// Male
    Male = 2,
    /// It is not possible, to accurately assess the applicability of MALE/FEMALE.
    OtherSex = 3,
}
impl Sex {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::UnknownSex => "UNKNOWN_SEX",
            Self::Female => "FEMALE",
            Self::Male => "MALE",
            Self::OtherSex => "OTHER_SEX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_SEX" => Some(Self::UnknownSex),
            "FEMALE" => Some(Self::Female),
            "MALE" => Some(Self::Male),
            "OTHER_SEX" => Some(Self::OtherSex),
            _ => None,
        }
    }
}
/// Karyotypic sex of the individual
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KaryotypicSex {
    UnknownKaryotype = 0,
    Xx = 1,
    Xy = 2,
    Xo = 3,
    Xxy = 4,
    Xxx = 5,
    Xxyy = 6,
    Xxxy = 7,
    Xxxx = 8,
    Xyy = 9,
    OtherKaryotype = 10,
}
impl KaryotypicSex {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::UnknownKaryotype => "UNKNOWN_KARYOTYPE",
            Self::Xx => "XX",
            Self::Xy => "XY",
            Self::Xo => "XO",
            Self::Xxy => "XXY",
            Self::Xxx => "XXX",
            Self::Xxyy => "XXYY",
            Self::Xxxy => "XXXY",
            Self::Xxxx => "XXXX",
            Self::Xyy => "XYY",
            Self::OtherKaryotype => "OTHER_KARYOTYPE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_KARYOTYPE" => Some(Self::UnknownKaryotype),
            "XX" => Some(Self::Xx),
            "XY" => Some(Self::Xy),
            "XO" => Some(Self::Xo),
            "XXY" => Some(Self::Xxy),
            "XXX" => Some(Self::Xxx),
            "XXYY" => Some(Self::Xxyy),
            "XXXY" => Some(Self::Xxxy),
            "XXXX" => Some(Self::Xxxx),
            "XYY" => Some(Self::Xyy),
            "OTHER_KARYOTYPE" => Some(Self::OtherKaryotype),
            _ => None,
        }
    }
}
