//! Rust bindings for Phenopacket Schema.
//!
//! The crate contains Rust structs and enums generated from Phenopacket Schema protobuf descriptors.
//! Phenopacket Schema versions v1 and v2 are supported.
//!
//! ## Examples
//!
//! ### Create a Phenopacket Schema element programmatically
//!
//! Any struct or enum of the schema can be created by invoking its initializer.
//! For instance, an [`schema::v2::core::OntologyClass`]:
//!
//! ```rust
//! use phenopackets::schema::v2::core::OntologyClass;
//!
//! let seizure = OntologyClass {
//!   id: "HP:0001250".into(),
//!   label: "Seizure".into(),
//! };
//!
//! assert_eq!(&seizure.id, "HP:0001250");
//! assert_eq!(&seizure.label, "Seizure");
//! ```
//!
//! ### Encode/decode into protobuf binary format
//!
//! The schema elements can be encoded into or decoded from bytes:
//!
//! ```rust
//! use phenopackets::schema::v2::core::OntologyClass;
//! use prost::Message;
//!
//! let seizure = OntologyClass {
//!   id: "HP:0001250".into(),
//!   label: "Seizure".into(),
//! };
//!
//! // Dump to bytes ...
//! let bytes: Vec<u8> = seizure.encode_to_vec();
//!
//! // ... and read back.
//! let other = OntologyClass::decode(&bytes[..]).unwrap();
//!
//! assert_eq!(seizure, other);
//! ```
//!
//! Note, only the protobuf binary format is implemented (no JSON, YAML, ...).

pub(crate) mod generated;
#[cfg(feature = "serde")]
mod serde;

// Include the `phenopackets` module, which is generated from the Phenopacket Schema proto files.

pub mod schema {
    pub mod v1 {
        pub mod core {
            use crate::generated;
            pub use generated::org_phenopackets_schema_v1_core::*;
        }
        use crate::generated;
        pub use generated::org_phenopackets_schema_v1::*;
    }
    pub mod v2 {
        pub mod core {
            use crate::generated;
            pub use generated::org_phenopackets_schema_v2_core::*;
        }
        use crate::generated;
        pub use generated::org_phenopackets_schema_v2::*;
    }
}

pub mod ga4gh {
    pub mod vrsatile {
        pub mod v1 {
            use crate::generated;
            pub use generated::org_ga4gh_vrsatile_v1::*;
        }
    }
    pub mod vrs {
        pub mod v1 {
            use crate::generated;
            pub use generated::org_ga4gh_vrs_v1::*;
        }
    }
}
