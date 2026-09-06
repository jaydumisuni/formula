use formula_core::digest::{ArtifactDigest, DigestError};
use formula_first_light::p12::seed_manifest_from_workflow_sha256;

fn hex(byte: &str) -> String {
    byte.repeat(32)
}

#[test]
fn workflow_sha256_values_are_bound_as_actual_digests_not_rehashed_text() {
    let rustc = hex("01");
    let cargo = hex("23");
    let toolchain = hex("ab");

    let seed = seed_manifest_from_workflow_sha256(&rustc, &cargo, &toolchain)
        .expect("canonical lowercase SHA-256 values must parse");

    let expected_rustc = ArtifactDigest::parse(&format!("sha256:{rustc}")).unwrap();
    let expected_cargo = ArtifactDigest::parse(&format!("sha256:{cargo}")).unwrap();
    let expected_toolchain = ArtifactDigest::parse(&format!("sha256:{toolchain}")).unwrap();

    assert_eq!(seed.rustc_executable(), expected_rustc);
    assert_eq!(seed.cargo_executable(), expected_cargo);
    assert_eq!(seed.rust_toolchain_file(), expected_toolchain);

    assert_ne!(seed.rustc_executable(), ArtifactDigest::of_bytes(rustc.as_bytes()));
    assert_ne!(seed.cargo_executable(), ArtifactDigest::of_bytes(cargo.as_bytes()));
    assert_ne!(
        seed.rust_toolchain_file(),
        ArtifactDigest::of_bytes(toolchain.as_bytes())
    );
    assert_eq!(seed.provenance(), "workflow-sha256");
}

#[test]
fn malformed_or_noncanonical_workflow_sha256_fails_closed() {
    let valid = hex("01");
    let uppercase = "AB".repeat(32);

    assert_eq!(
        seed_manifest_from_workflow_sha256("not-a-digest", &valid, &valid),
        Err(DigestError::InvalidLength)
    );
    assert_eq!(
        seed_manifest_from_workflow_sha256(&uppercase, &valid, &valid),
        Err(DigestError::NonCanonicalHex)
    );
}
