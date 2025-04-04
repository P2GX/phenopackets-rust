use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::generated::org_phenopackets_schema_v2_core::{
    Individual, KaryotypicSex, OntologyClass, Sex,
};

impl Serialize for OntologyClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut oc = serializer.serialize_struct("OntologyClass", 2)?;
        oc.serialize_field("id", self.id.as_str())?;
        oc.serialize_field("label", self.label.as_str())?;
        oc.end()
    }
}

// ExternalReference
// Evidence
// Procedure
// GestationalAge
// Age
// AgeRange
// TimeInterval
// TimeElement
// File
// Measurement
// Value
// ComplexValue
// Quantity
// TypedQuantity
// ReferenceRange
// PhenotypicFeature
// Biosample
// Disease
// Interpretation
// Diagnosis
// GenomicInterpretation
// VariantInterpretation
// AcmgPathogenicityClassification
// TherapeuticActionability

impl Serialize for Individual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut indi = serializer.serialize_struct("Individual", 2)?;
        indi.serialize_field("id", &self.id)?;
        if !self.alternate_ids.is_empty() {
            indi.serialize_field("alternate_ids", &self.alternate_ids)?;
        }
        // indi.serialize_field("date_of_birth", &self.date_of_birth)?;
        // indi.serialize_field("time_at_last_encounter", &self.time_at_last_encounter)?;
        // indi.serialize_field("vital_status", &self.vital_status)?;

        indi.serialize_field("sex", &self.sex())?;
        indi.serialize_field("karyotypic_sex", &self.karyotypic_sex())?;
        if let Some(gender) = &self.gender {
            indi.serialize_field("gender", gender)?;
        }
        if let Some(taxonomy) = &self.taxonomy {
            indi.serialize_field("taxonomy", taxonomy)?;
        }

        indi.end()
    }
}

// VitalStatus

impl Serialize for Sex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Sex::UnknownSex => serializer.serialize_str("UNKNOWN_SEX"),
            Sex::Female => serializer.serialize_str("FEMALE"),
            Sex::Male => serializer.serialize_str("MALE"),
            Sex::OtherSex => serializer.serialize_str("OTHER_SEX"),
        }
    }
}

impl Serialize for KaryotypicSex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            KaryotypicSex::UnknownKaryotype => serializer.serialize_str("UNKNOWN_KARYOTYPE"),
            KaryotypicSex::Xx => serializer.serialize_str("XX"),
            KaryotypicSex::Xy => serializer.serialize_str("XY"),
            KaryotypicSex::Xo => serializer.serialize_str("XO"),
            KaryotypicSex::Xxy => serializer.serialize_str("XXY"),
            KaryotypicSex::Xxx => serializer.serialize_str("XXX"),
            KaryotypicSex::Xxyy => serializer.serialize_str("XXYY"),
            KaryotypicSex::Xxxy => serializer.serialize_str("XXXY"),
            KaryotypicSex::Xxxx => serializer.serialize_str("XXXX"),
            KaryotypicSex::Xyy => serializer.serialize_str("XYY"),
            KaryotypicSex::OtherKaryotype => serializer.serialize_str("OTHER_KARYOTYPE"),
        }
    }
}

// MedicalAction
// Treatment
// DoseInterval
// RadiationTherapy
// TherapeuticRegimen
// DrugType
// MetaData
// Resource
// Update
// Pedigree
