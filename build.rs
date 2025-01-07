use prost_build;

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
