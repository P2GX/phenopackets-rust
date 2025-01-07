// README:
//
// This script uses Prost crate to generate Rust sources from proto files.
// The script should be run as a build script - this will be done automatically after renaming to `build.rs` (a little hack).
//
// After generating sources, the sources should be copied into `src/generated` and the errors ought to be resolved manually.

use prost_build;
use std::io;
use std::path::PathBuf;
use std::{collections::HashMap, fs};

// Proto files corresponding to the Phenopacket Schema hierarchy leaves.
// The dependencies are imported and compiled automatically.
const PROTOS: &[&str] = &[
    /* v1 elements */
    "phenopackets/schema/v1/interpretation.proto",
    /* v2 elements */
    "phenopackets/schema/v2/phenopackets.proto",
];

// The top-level packages (just one here).
const INCLUDES: &[&str] = &["org"];

fn main() {
    prost_build::compile_protos(PROTOS, INCLUDES).expect("Failed to compile protos");
}
