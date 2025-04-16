//! Rust bindings for Phenopacket Schema.
//!
//! The crate contains Rust structs and enums generated from Phenopacket Schema protobuf descriptors.
//! Phenopacket Schema versions v1 and v2 are supported.
//!
//! # Examples
//!
//! ## Create a Phenopacket Schema element programmatically
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
//! ## JSON
//!
//! Phenopacket Schema elements can be read from or written into JSON format.
//! The functionality is gated by the `serde` feature
//! which derives Serde's `Serialize` and `Deserialize` traits
//! for all Phenopacket Schema components and enables interoperability
//! with Serde data formats, such as JSON.
//!
//! The following must be added into your `Cargo.toml` file
//! to read a phenopacket from a JSON file.
//! The `serde` feature must be enabled for `phenopackets`
//! and [serde_json](https://docs.rs/serde_json/latest/serde_json/)
//! must be added:
//!
//! ```toml
//! phenopackets = { version = "*", features = ["serde"]}
//! serde_json = "1.0.140"
//! ```
//!
//! Then, a phenopacket can be read
//! from a [JSON file](https://github.com/P2GX/phenopackets-rust/blob/master/data/v2/phenopacket.json):
//!
//! ```
//! use phenopackets::schema::v2::Phenopacket;
//!
//! // An example phenopacket
//! let path = "data/v2/phenopacket.json";
//! let bytes = std::fs::read(path).expect("Reading should succeed");
//!
//! # #[cfg(feature = "serde")]
//! let pp: Phenopacket = serde_json::from_reader(&bytes[..]).expect("Expecting no decoding issues");
//!
//! # #[cfg(feature = "serde")]
//! assert_eq!(pp.id, "comprehensive-phenopacket-id");
//! ```
//!
//! ## Protobuf wire format
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
//! // Encode the component into bytes ...
//! let bytes: Vec<u8> = seizure.encode_to_vec();
//!
//! // ... and decode them back.
//! let decoded = OntologyClass::decode(&bytes[..]).unwrap();
//!
//! assert_eq!(seizure, decoded);
//! ```
//!
//! # Feature flags
//!
//! Phenopackets uses [feature flags](https://doc.rust-lang.org/cargo/reference/features.html) to enable
//! adding optional functionality.
//!
//! No features are turned on by default.
//!
//! ### Optional feature flags
//!
//! The following features customize Phenopackets' behavior:
//!
//! - `serde`: Enables interoperability with [serde](https://docs.rs/serde/latest/serde) to support encoding into or decoding from JSON format.
//!

/// The source files generated by prost crate.
pub(crate) mod generated;
#[cfg(feature = "serde")]
pub(crate) mod serde;
#[cfg(test)]
mod tests;

// Include the `phenopackets` module, which is generated from the Phenopacket Schema proto files.
// Public-facing modules.
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
