use serde::Serializer;

use super::visitors::{
    AcmgPathogenicityClassificationVisitor, AffectedStatusVisitor, DrugTypeVisitor,
    InterpretationStatusVisitor, KaryotypicSexVisitor, MoleculeContextVisitor,
    ProgressStatusVisitor, RegimenStatusVisitor, SexVisitor, TherapeuticActionabilityVisitor,
    TimestampOptionVisitor, TimestampVisitor, VitalStatusStatusVisitor,
};
use prost_types::Timestamp;

use crate::generated::{
    org_ga4gh_vrsatile_v1::MoleculeContext,
    org_phenopackets_schema_v2_core::{
        genomic_interpretation::InterpretationStatus, interpretation::ProgressStatus,
        pedigree::person::AffectedStatus, therapeutic_regimen::RegimenStatus, vital_status::Status,
        AcmgPathogenicityClassification, DrugType, KaryotypicSex, Sex, TherapeuticActionability,
    },
};

pub(crate) fn serialize_interpretation_status<S>(
    value: &i32,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match InterpretationStatus::try_from(*value) {
        Ok(val) => serializer.serialize_str(val.as_str_name()),
        Err(e) => Err(serde::ser::Error::custom(e.to_string())),
    }
}

pub(crate) fn deserialize_interpretation_status<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(InterpretationStatusVisitor)
}

pub(crate) fn serialize_progress_status<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match ProgressStatus::try_from(*value) {
        Ok(val) => serializer.serialize_str(val.as_str_name()),
        Err(e) => Err(serde::ser::Error::custom(e.to_string())),
    }
}

pub(crate) fn deserialize_progress_status<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(ProgressStatusVisitor)
}

pub(crate) fn serialize_acmg_pathogenicity_classification<S>(
    value: &i32,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match AcmgPathogenicityClassification::try_from(*value) {
        Ok(val) => serializer.serialize_str(val.as_str_name()),
        Err(e) => Err(serde::ser::Error::custom(e.to_string())),
    }
}

pub(crate) fn deserialize_acmg_pathogenicity_classification<'de, D>(
    deserializer: D,
) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(AcmgPathogenicityClassificationVisitor)
}

pub(crate) fn serialize_therapeutic_actionability<S>(
    value: &i32,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match TherapeuticActionability::try_from(*value) {
        Ok(val) => serializer.serialize_str(val.as_str_name()),
        Err(e) => Err(serde::ser::Error::custom(e.to_string())),
    }
}

pub(crate) fn deserialize_therapeutic_actionability<'de, D>(
    deserializer: D,
) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(TherapeuticActionabilityVisitor)
}

pub(crate) fn serialize_vital_status_status<S>(
    value: &i32,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match Status::try_from(*value) {
        Ok(val) => serializer.serialize_str(val.as_str_name()),
        Err(e) => Err(serde::ser::Error::custom(e.to_string())),
    }
}

pub(crate) fn deserialize_vital_status_status<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(VitalStatusStatusVisitor)
}

pub(crate) fn serialize_sex<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match Sex::try_from(*value) {
        Ok(val) => serializer.serialize_str(val.as_str_name()),
        Err(e) => Err(serde::ser::Error::custom(e.to_string())),
    }
}

pub(crate) fn deserialize_sex<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(SexVisitor)
}

pub(crate) fn serialize_karyotypic_sex<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match KaryotypicSex::try_from(*value) {
        Ok(val) => serializer.serialize_str(val.as_str_name()),
        Err(e) => Err(serde::ser::Error::custom(e.to_string())),
    }
}

pub(crate) fn deserialize_karyotypic_sex<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(KaryotypicSexVisitor)
}

pub(crate) fn serialize_timestamp_option<S>(
    timestamp: &Option<Timestamp>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match timestamp {
        Some(val) => serializer.serialize_some(&val.to_string()),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn deserialize_timestamp_option<'de, D>(
    deserializer: D,
) -> Result<Option<Timestamp>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(TimestampOptionVisitor)
}

pub(crate) fn serialize_timestamp<S>(val: &Timestamp, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_some(&val.to_string())
}

pub(crate) fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(TimestampVisitor)
}

pub(crate) fn serialize_affected_status<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match AffectedStatus::try_from(*value) {
        Ok(val) => serializer.serialize_str(val.as_str_name()),
        Err(e) => Err(serde::ser::Error::custom(e.to_string())),
    }
}

pub(crate) fn deserialize_affected_status<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(AffectedStatusVisitor)
}

pub(crate) fn serialize_drug_type<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match DrugType::try_from(*value) {
        Ok(val) => serializer.serialize_str(val.as_str_name()),
        Err(e) => Err(serde::ser::Error::custom(e.to_string())),
    }
}

pub(crate) fn deserialize_drug_type<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(DrugTypeVisitor)
}

pub(crate) fn serialize_regimen_status<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match RegimenStatus::try_from(*value) {
        Ok(val) => serializer.serialize_str(val.as_str_name()),
        Err(e) => Err(serde::ser::Error::custom(e.to_string())),
    }
}

pub(crate) fn deserialize_regimen_status<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(RegimenStatusVisitor)
}

pub(crate) fn serialize_molecule_context<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match MoleculeContext::try_from(*value) {
        Ok(val) => serializer.serialize_str(val.as_str_name()),
        Err(e) => Err(serde::ser::Error::custom(e.to_string())),
    }
}

pub(crate) fn deserialize_molecule_context<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_any(MoleculeContextVisitor)
}
