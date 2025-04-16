use prost_types::Timestamp;
use serde::{
    de::{self, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::generated::{
    org_ga4gh_vrsatile_v1::MoleculeContext,
    org_phenopackets_schema_v2_core::{
        genomic_interpretation::InterpretationStatus, interpretation::ProgressStatus,
        pedigree::person::AffectedStatus, therapeutic_regimen::RegimenStatus,
        time_element::Element, vital_status::Status, AcmgPathogenicityClassification, DrugType,
        KaryotypicSex, Sex, TherapeuticActionability, TimeElement,
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

// #[cfg(test)]
// mod test_time_interval {
//     use prost_types::Timestamp;

//     use crate::generated::org_phenopackets_schema_v2_core::TimeInterval;

//     #[test]
//     fn serialize_time_interval() {
//         let ti = TimeInterval {
//             start: Some(
//                 "1970-01-02T10:17:36.000000100Z"
//                     .parse()
//                     .expect("Timestamp should be well formatted"),
//             ),
//             end: Some(
//                 "1970-01-02T10:17:36.000000200Z"
//                     .parse()
//                     .expect("Timestamp should be well formatted"),
//             ),
//         };

//         let actual = serde_json::to_string(&ti).expect("Expecting no serialization issues");

//         assert_eq!(
//             actual.as_str(),
//             r#"{"start":"1970-01-02T10:17:36.000000100Z","end":"1970-01-02T10:17:36.000000200Z"}"#
//         );
//     }

//     #[test]
//     fn deserialize_time_interval() {
//         let val =
//             r#"{"start":"1970-01-02T10:17:36.000000100Z","end":"1970-01-02T10:17:36.000000200Z"}"#;
//         let ti: TimeInterval = serde_json::from_str(val).expect("No deserialization issues");

//         assert_eq!(
//             ti.start,
//             Some(
//                 "1970-01-02T10:17:36.000000100Z"
//                     .parse::<Timestamp>()
//                     .unwrap()
//             )
//         );
//         assert_eq!(
//             ti.end,
//             Some(
//                 "1970-01-02T10:17:36.000000200Z"
//                     .parse::<Timestamp>()
//                     .unwrap()
//             )
//         );
//     }
// }

impl Serialize for TimeElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut te = serializer.serialize_struct("TimeElement", 1)?;
        if let Some(e) = self.element.as_ref() {
            match e {
                Element::GestationalAge(ga) => te.serialize_field("gestationalAge", ga)?,
                Element::Age(age) => te.serialize_field("age", age)?,
                Element::AgeRange(ar) => te.serialize_field("ageRange", ar)?,
                Element::OntologyClass(oc) => te.serialize_field("ontologyClass", oc)?,
                Element::Timestamp(timestamp) => {
                    te.serialize_field("timestamp", &timestamp.to_string())?
                }
                Element::Interval(ti) => te.serialize_field("timeInterval", ti)?,
            }
        }
        te.end()
    }
}

impl<'de> Deserialize<'de> for TimeElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gestationalAge",
            "age",
            "ageRange",
            "ontologyClass",
            "timestamp",
            "timeInterval",
        ];
        enum Field {
            GestationalAge,
            Age,
            AgeRange,
            OntologyClass,
            Timestamp,
            TimeInterval,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(
                            "gestationalAge, age, ageRange, ontologyClass, timestamp, timeInterval",
                        )
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "gestationalAge" => Ok(Field::GestationalAge),
                            "age" => Ok(Field::Age),
                            "ageRange" => Ok(Field::AgeRange),
                            "ontologyClass" => Ok(Field::OntologyClass),
                            "timestamp" => Ok(Field::Timestamp),
                            "timeInterval" => Ok(Field::TimeInterval),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct TimeElementVisitor;

        impl<'de> Visitor<'de> for TimeElementVisitor {
            type Value = TimeElement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct TimeElement")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                if let Some(key) = map.next_key()? {
                    match key {
                        Field::GestationalAge => Ok(TimeElement {
                            element: Some(Element::GestationalAge(map.next_value()?)),
                        }),
                        Field::Age => Ok(TimeElement {
                            element: Some(Element::Age(map.next_value()?)),
                        }),
                        Field::AgeRange => Ok(TimeElement {
                            element: Some(Element::AgeRange(map.next_value()?)),
                        }),
                        Field::OntologyClass => Ok(TimeElement {
                            element: Some(Element::OntologyClass(map.next_value()?)),
                        }),
                        Field::Timestamp => match map.next_value::<&str>()?.parse() {
                            Ok(ts) => Ok(TimeElement {
                                element: Some(Element::Timestamp(ts)),
                            }),
                            Err(err) => return Err(de::Error::custom(err)),
                        },
                        Field::TimeInterval => Ok(TimeElement {
                            element: Some(Element::Interval(map.next_value()?)),
                        }),
                    }
                } else {
                    Err(de::Error::custom("Unknown value"))
                }
            }
        }

        deserializer.deserialize_struct("TimeElement", FIELDS, TimeElementVisitor)
    }
}

#[cfg(test)]
mod test_time_element {
    use crate::generated::org_phenopackets_schema_v2_core::{
        time_element::Element, Age, TimeElement,
    };

    #[test]
    fn serialize_time_element_with_age() {
        let te = TimeElement {
            element: Some(Element::Age(Age {
                iso8601duration: "P14Y".to_string(),
            })),
        };

        let actual = serde_json::to_string(&te).expect("Expecting no serialization issues");

        assert_eq!(actual, r#"{"age":{"iso8601duration":"P14Y"}}"#);
    }

    /// We produce an empty object: `{}`.
    ///
    /// This should not happen in real life, since the oneof field should always be set.
    /// However, Prost wraps the oneof field in an `Option`, so we must test the behavior (here).
    #[test]
    fn serialize_time_element_with_nothing() {
        let te = TimeElement { element: None };

        let actual = serde_json::to_string(&te).expect("Expecting no serialization issues");

        assert_eq!(actual, r#"{}"#);
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

#[cfg(test)]
mod test_individual {
    use crate::generated::org_phenopackets_schema_v2_core::{
        time_element::Element, Age, Individual, KaryotypicSex, OntologyClass, Sex, TimeElement,
    };

    /// A minimal individual serializes only the enum fields (`sex` and `karyotypicSex`).
    #[test]
    fn serialize_individual_minimal_fields() {
        let individual = Individual {
            id: "".into(),
            alternate_ids: vec![],
            date_of_birth: None,
            time_at_last_encounter: None,
            vital_status: None,
            sex: Sex::UnknownSex.into(),
            karyotypic_sex: KaryotypicSex::UnknownKaryotype.into(),
            gender: None,
            taxonomy: None,
        };

        let actual = serde_json::to_string(&individual).expect("Expecting no serialization issues");

        assert_eq!(
            actual,
            r#"{"sex":"UNKNOWN_SEX","karyotypicSex":"UNKNOWN_KARYOTYPE"}"#
        )
    }

    #[test]
    fn deserialize_individual() {
        let value = r#"{
        "id": "14 year-old boy",
        "alternateIds": ["boy", "patient", "proband"],
        "dateOfBirth": "1970-01-02T10:17:36.000000100Z",
        "timeAtLastEncounter": {
            "age": {
                "iso8601duration": "P14Y"
            }
        },
        "sex": "MALE",
        "karyotypicSex": "XY",
        "taxonomy": {
            "id": "NCBITaxon:9606",
            "label": "homo sapiens"
        }
        }"#;

        let individual: Individual =
            serde_json::from_str(value).expect("No deserialization issues");

        assert_eq!(&individual.id, "14 year-old boy");
        assert_eq!(&individual.alternate_ids, &["boy", "patient", "proband"]);
        assert_eq!(
            individual.date_of_birth,
            Some(
                "1970-01-02T10:17:36.000000100Z"
                    .parse()
                    .expect("No issues here")
            )
        );
        assert_eq!(
            individual.time_at_last_encounter,
            Some(TimeElement {
                element: Some(Element::Age(Age {
                    iso8601duration: "P14Y".into()
                }))
            })
        );
        assert_eq!(individual.vital_status, None);
        assert_eq!(individual.sex(), Sex::Male);
        assert_eq!(individual.karyotypic_sex(), KaryotypicSex::Xy);
        assert_eq!(individual.gender, None);
        assert_eq!(
            individual.taxonomy,
            Some(OntologyClass {
                id: "NCBITaxon:9606".into(),
                label: "homo sapiens".into()
            })
        );
    }

    #[test]
    fn deserialize_individual_minimal_example() {
        let value = "{}";

        let individual: Individual =
            serde_json::from_str(value).expect("No deserialization issues");

        assert_eq!(&individual.id, "");
        assert!(individual.alternate_ids.is_empty());
        assert_eq!(individual.date_of_birth, None);
        assert_eq!(individual.time_at_last_encounter, None);
        assert_eq!(individual.vital_status, None);
        assert_eq!(individual.sex, 0);
        assert_eq!(individual.karyotypic_sex, 0);
        assert_eq!(individual.gender, None);
        assert_eq!(individual.taxonomy, None);
    }
}
