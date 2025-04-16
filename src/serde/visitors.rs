use prost_types::Timestamp;
use serde::de::{self, Visitor};

use crate::generated::{
    org_ga4gh_vrsatile_v1::MoleculeContext,
    org_phenopackets_schema_v2_core::{
        genomic_interpretation::InterpretationStatus, interpretation::ProgressStatus,
        pedigree::person::AffectedStatus, therapeutic_regimen::RegimenStatus, vital_status::Status,
        AcmgPathogenicityClassification, DrugType, KaryotypicSex, Sex, TherapeuticActionability,
    },
};

pub(super) struct TimestampOptionVisitor;
impl<'de> Visitor<'de> for TimestampOptionVisitor {
    type Value = Option<Timestamp>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("`str` with RFC3339 timestamp")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v.parse() {
            Ok(ts) => Ok(Some(ts)),
            Err(e) => Err(de::Error::custom(e.to_string())),
        }
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub(super) struct TimestampVisitor;
impl<'de> Visitor<'de> for TimestampVisitor {
    type Value = Timestamp;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("`str` with RFC3339 timestamp")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v.parse() {
            Ok(ts) => Ok(ts),
            Err(e) => Err(de::Error::custom(e.to_string())),
        }
    }
}

pub(super) struct ProgressStatusVisitor;
impl<'de> Visitor<'de> for ProgressStatusVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        const FIELDS: &[&str] = &[
            "UNKNOWN_PROGRESS",
            "IN_PROGRESS",
            "COMPLETED",
            "SOLVED",
            "UNSOLVED",
        ];
        write!(
            formatter,
            "signed int [0,{}] or one of {:?}",
            FIELDS.len() - 1,
            FIELDS
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match ProgressStatus::from_str_name(value) {
            Some(ks) => Ok(ks as i32),
            None => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if ProgressStatus::is_valid(value) {
            Ok(value)
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Signed(value as i64),
                &self,
            ))
        }
    }
}

pub(super) struct InterpretationStatusVisitor;
impl<'de> Visitor<'de> for InterpretationStatusVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        const FIELDS: &[&str] = &[
            "UNKNOWN_STATUS",
            "REJECTED",
            "CANDIDATE",
            "CONTRIBUTORY",
            "CAUSATIVE",
        ];
        write!(
            formatter,
            "signed int [0,{}] or one of {:?}",
            FIELDS.len() - 1,
            FIELDS
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match InterpretationStatus::from_str_name(value) {
            Some(ks) => Ok(ks as i32),
            None => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if InterpretationStatus::is_valid(value) {
            Ok(value)
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Signed(value as i64),
                &self,
            ))
        }
    }
}

pub(super) struct AcmgPathogenicityClassificationVisitor;
impl<'de> Visitor<'de> for AcmgPathogenicityClassificationVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        const FIELDS: &[&str] = &[
            "NOT_PROVIDED",
            "BENIGN",
            "LIKELY_BENIGN",
            "UNCERTAIN_SIGNIFICANCE",
            "LIKELY_PATHOGENIC",
            "PATHOGENIC",
        ];
        write!(
            formatter,
            "signed int [0,{}] or one of {:?}",
            FIELDS.len() - 1,
            FIELDS
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match AcmgPathogenicityClassification::from_str_name(value) {
            Some(ks) => Ok(ks as i32),
            None => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if AcmgPathogenicityClassification::is_valid(value) {
            Ok(value)
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Signed(value as i64),
                &self,
            ))
        }
    }
}

pub(super) struct TherapeuticActionabilityVisitor;
impl<'de> Visitor<'de> for TherapeuticActionabilityVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        const FIELDS: &[&str] = &["UNKNOWN_ACTIONABILITY", "NOT_ACTIONABLE", "ACTIONABLE"];
        write!(
            formatter,
            "signed int [0,{}] or one of {:?}",
            FIELDS.len() - 1,
            FIELDS
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match TherapeuticActionability::from_str_name(value) {
            Some(ks) => Ok(ks as i32),
            None => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if TherapeuticActionability::is_valid(value) {
            Ok(value)
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Signed(value as i64),
                &self,
            ))
        }
    }
}

pub(super) struct SexVisitor;
impl<'de> Visitor<'de> for SexVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        const FIELDS: &[&str] = &["UNKNOWN_SEX", "FEMALE", "MALE", "OTHER_SEX"];
        write!(
            formatter,
            "signed int [0,{}] or one of {:?}",
            FIELDS.len() - 1,
            FIELDS
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Sex::from_str_name(value) {
            Some(ks) => Ok(ks as i32),
            None => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if Sex::is_valid(value) {
            Ok(value)
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Signed(value as i64),
                &self,
            ))
        }
    }
}

pub(super) struct KaryotypicSexVisitor;
impl<'de> Visitor<'de> for KaryotypicSexVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        const FIELDS: &[&str] = &[
            "UNKNOWN_KARYOTYPE",
            "XX",
            "XY",
            "XO",
            "XXY",
            "XXX",
            "XXYY",
            "XXXY",
            "XXXX",
            "XYY",
            "OTHER_KARYOTYPE",
        ];
        write!(
            formatter,
            "signed int [0,{}] or one of {:?}",
            FIELDS.len() - 1,
            FIELDS
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match KaryotypicSex::from_str_name(value) {
            Some(ks) => Ok(ks as i32),
            None => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if KaryotypicSex::is_valid(value) {
            Ok(value)
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Signed(value as i64),
                &self,
            ))
        }
    }
}

pub(super) struct VitalStatusStatusVisitor;
impl<'de> Visitor<'de> for VitalStatusStatusVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        const FIELDS: &[&str] = &["UNKNOWN_STATUS", "ALIVE", "DECEASED"];
        write!(
            formatter,
            "signed int [0,{}] or one of {:?}",
            FIELDS.len() - 1,
            FIELDS
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Status::from_str_name(value) {
            Some(ks) => Ok(ks as i32),
            None => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if Status::is_valid(value) {
            Ok(value)
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Signed(value as i64),
                &self,
            ))
        }
    }
}

pub(super) struct RegimenStatusVisitor;
impl<'de> Visitor<'de> for RegimenStatusVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        const FIELDS: &[&str] = &["UNKNOWN_STATUS", "STARTED", "COMPLETED", "DISCONTINUED"];
        write!(
            formatter,
            "signed int [0,{}] or one of {:?}",
            FIELDS.len() - 1,
            FIELDS
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match RegimenStatus::from_str_name(value) {
            Some(ks) => Ok(ks as i32),
            None => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if RegimenStatus::is_valid(value) {
            Ok(value)
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Signed(value as i64),
                &self,
            ))
        }
    }
}

pub(super) struct DrugTypeVisitor;
impl<'de> Visitor<'de> for DrugTypeVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        const FIELDS: &[&str] = &[
            "UNKNOWN_DRUG_TYPE",
            "PRESCRIPTION",
            "EHR_MEDICATION_LIST",
            "ADMINISTRATION_RELATED_TO_PROCEDURE",
        ];
        write!(
            formatter,
            "signed int [0,{}] or one of {:?}",
            FIELDS.len() - 1,
            FIELDS
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match DrugType::from_str_name(value) {
            Some(ks) => Ok(ks as i32),
            None => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if DrugType::is_valid(value) {
            Ok(value)
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Signed(value as i64),
                &self,
            ))
        }
    }
}

pub(super) struct AffectedStatusVisitor;
impl<'de> Visitor<'de> for AffectedStatusVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        const FIELDS: &[&str] = &["MISSING", "UNAFFECTED", "AFFECTED"];
        write!(
            formatter,
            "signed int [0,{}] or one of {:?}",
            FIELDS.len() - 1,
            FIELDS
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match AffectedStatus::from_str_name(value) {
            Some(ks) => Ok(ks as i32),
            None => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if AffectedStatus::is_valid(value) {
            Ok(value)
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Signed(value as i64),
                &self,
            ))
        }
    }
}

pub(super) struct MoleculeContextVisitor;
impl<'de> Visitor<'de> for MoleculeContextVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        const FIELDS: &[&str] = &[
            "unspecified_molecule_context",
            "genomic",
            "transcript",
            "protein",
        ];
        write!(
            formatter,
            "signed int [0,{}] or one of {:?}",
            FIELDS.len() - 1,
            FIELDS
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match MoleculeContext::from_str_name(value) {
            Some(ks) => Ok(ks as i32),
            None => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if MoleculeContext::is_valid(value) {
            Ok(value)
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Signed(value as i64),
                &self,
            ))
        }
    }
}
