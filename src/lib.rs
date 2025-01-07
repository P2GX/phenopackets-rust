//! # Rust bindings for Phenopacket Schema
//!
//! More documentation will arrive soon.

pub(crate) mod generated;

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
