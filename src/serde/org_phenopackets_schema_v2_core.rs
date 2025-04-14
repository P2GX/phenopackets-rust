use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::generated::org_phenopackets_schema_v2_core::{
    time_element::Element, Individual, MetaData, TimeElement, TimeInterval, Update,
};

impl Serialize for Individual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut indi = serializer.serialize_struct("Individual", 2)?;
        indi.serialize_field("id", &self.id)?;
        if !self.alternate_ids.is_empty() {
            indi.serialize_field("alternateIds", &self.alternate_ids)?;
        }
        if let Some(dob) = self.date_of_birth.as_ref() {
            indi.serialize_field("dateOfBirth", &dob.to_string())?;
        }
        if let Some(tale) = self.time_at_last_encounter.as_ref() {
            indi.serialize_field("timeAtLastEncounter", tale)?;
        }
        if let Some(vital_status) = self.vital_status.as_ref() {
            indi.serialize_field("vitalStatus", vital_status)?;
        }

        indi.serialize_field("sex", &self.sex().as_str_name())?;
        indi.serialize_field("karyotypicSex", &self.karyotypic_sex().as_str_name())?;
        if let Some(gender) = self.gender.as_ref() {
            indi.serialize_field("gender", gender)?;
        }
        if let Some(taxonomy) = self.taxonomy.as_ref() {
            indi.serialize_field("taxonomy", taxonomy)?;
        }

        indi.end()
    }
}

#[cfg(test)]
mod test_individual {
    use crate::generated::org_phenopackets_schema_v2_core::{Individual, KaryotypicSex, Sex};

    /// A minimal individual will serialize the `id`
    /// and the enum fields (`sex` and `karyotypicSex`).
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

        assert_eq!(actual, r#"{"id":"","sex":"UNKNOWN_SEX","karyotypicSex":"UNKNOWN_KARYOTYPE"}"#)
    }
}

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

impl Serialize for TimeInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ti = serializer.serialize_struct("TimeInterval", 2)?;
        if let Some(start) = self.start.as_ref() {
            ti.serialize_field("start", &start.to_string())?
        }
        if let Some(end) = self.end.as_ref() {
            ti.serialize_field("end", &end.to_string())?
        }

        ti.end()
    }
}

#[cfg(test)]
mod test_time_interval {
    use crate::generated::org_phenopackets_schema_v2_core::TimeInterval;

    #[test]
    fn serialize_time_interval() {
        let ti = TimeInterval {
            start: Some(
                "1970-01-02T10:17:36.000000100Z"
                    .parse()
                    .expect("Timestamp should be well formatted"),
            ),
            end: Some(
                "1970-01-02T10:17:36.000000200Z"
                    .parse()
                    .expect("Timestamp should be well formatted"),
            ),
        };

        let actual = serde_json::to_string(&ti).expect("Expecting no serialization issues");

        assert_eq!(
            actual.as_str(),
            r#"{"start":"1970-01-02T10:17:36.000000100Z","end":"1970-01-02T10:17:36.000000200Z"}"#
        );
    }
}

impl Serialize for MetaData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("MetaData", 7)?;
        if let Some(created) = self.created.as_ref() {
            ser.serialize_field("created", &created.to_string())?;
        } else {
            // Doesn't really matter if this is not `Timestamp`,
            // since `null` is null for `String` as well.
            ser.serialize_field("created", &None::<String>)?;
        }

        ser.serialize_field("createdBy", &self.created_by)?;

        if !self.submitted_by.is_empty() {
            ser.serialize_field("submittedBy", &self.submitted_by)?;
        }

        if !self.resources.is_empty() {
            ser.serialize_field("resources", &self.resources)?;
        }

        if !self.updates.is_empty() {
            ser.serialize_field("updates", &self.updates)?;
        }

        ser.serialize_field("phenopacketSchemaVersion", &self.phenopacket_schema_version)?;

        if !self.external_references.is_empty() {
            ser.serialize_field("externalReferences", &self.external_references)?;
        }

        ser.end()
    }
}

#[cfg(test)]
mod test_metadata {
    use crate::generated::org_phenopackets_schema_v2_core::MetaData;

    /// Expecting to see required elements even if they are empty.
    /// but not `resources`, if empty.
    #[test]
    fn serialize_metadata() {
        let meta = MetaData {
            created: Some("2019-07-21T00:25:54.662Z".parse().expect("Timestamp should be well formatted")),
            created_by: "Peter R.".into(),
            submitted_by: "".into(),
            resources: vec![],
            updates: vec![],
            phenopacket_schema_version: "".into(),
            external_references: vec![],
        };

        let actual = serde_json::to_string(&meta).expect("Expecting no serialization issues");

        assert_eq!(actual, r#"{"created":"2019-07-21T00:25:54.662Z","createdBy":"Peter R.","phenopacketSchemaVersion":""}"#)
    }
}

impl Serialize for Update {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("Update", 3)?;

        ser.serialize_field(
            "timestamp",
            &self.timestamp.as_ref().map(ToString::to_string),
        )?;

        if !self.updated_by.is_empty() {
            ser.serialize_field("updatedBy", &self.updated_by)?;
        }
        if !self.comment.is_empty() {
            ser.serialize_field("comment", &self.comment)?;
        }

        ser.end()
    }
}

#[cfg(test)]
mod test_update {
    use crate::generated::org_phenopackets_schema_v2_core::Update;

    #[test]
    fn serialize_update() {
        let u = Update {
            timestamp: Some(
                "2018-06-10T10:59:06Z"
                    .parse()
                    .expect("Timestamp should be OK"),
            ),
            updated_by: "Julius J.".into(),
            comment: "added phenotypic features to individual patient:1".into(),
        };

        let actual = serde_json::to_string(&u).expect("Expecting no serialization issues");

        assert_eq!(
            actual.as_str(),
            r#"{"timestamp":"2018-06-10T10:59:06Z","updatedBy":"Julius J.","comment":"added phenotypic features to individual patient:1"}"#
        )
    }
}
