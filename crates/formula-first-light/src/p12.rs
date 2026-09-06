use formula_core::{
    bootstrap::{BootstrapGenerationId, BootstrapRole, BootstrapSeedManifest},
    digest::{ArtifactDigest, DigestError},
};

pub const P11_FROZEN_PROOF_HEAD: &str = "6f8ce7bb6702ea1baf119aab9950aa5ba0f87283";

pub fn p11_frozen_proof_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(P11_FROZEN_PROOF_HEAD.as_bytes())
}

pub fn source_commit() -> &'static str {
    option_env!("GITHUB_SHA").unwrap_or("LOCAL_UNBOUND_SOURCE")
}

pub fn checker_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(b"formula-check:p12-bootstrap-independent-validator:v1")
}

pub fn verifier_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(b"formula-check:p12-bootstrap-final-replay:v1")
}

fn bootstrap_seed_manifest(
    rustc_executable: ArtifactDigest,
    cargo_executable: ArtifactDigest,
    rust_toolchain_file: ArtifactDigest,
    provenance: &str,
) -> BootstrapSeedManifest {
    BootstrapSeedManifest::new(
        BootstrapRole::ExternalToolchainSeed,
        "rust-1.98.0".into(),
        "88d9e12ae178fab0fb5cc050a94da85685d449ea".into(),
        "cargo-1.98.0".into(),
        "x86_64-unknown-linux-gnu".into(),
        rustc_executable,
        cargo_executable,
        rust_toolchain_file,
        "pinned-rust-1.98.0".into(),
        provenance.into(),
    )
}

fn parse_workflow_sha256(value: &str) -> Result<ArtifactDigest, DigestError> {
    ArtifactDigest::parse(&format!("sha256:{value}"))
}

pub fn seed_manifest_from_workflow_sha256(
    rustc_sha256: &str,
    cargo_sha256: &str,
    rust_toolchain_sha256: &str,
) -> Result<BootstrapSeedManifest, DigestError> {
    Ok(bootstrap_seed_manifest(
        parse_workflow_sha256(rustc_sha256)?,
        parse_workflow_sha256(cargo_sha256)?,
        parse_workflow_sha256(rust_toolchain_sha256)?,
        "workflow-sha256",
    ))
}

fn local_seed_manifest() -> BootstrapSeedManifest {
    bootstrap_seed_manifest(
        ArtifactDigest::of_bytes(b"LOCAL_RUSTC_SHA256"),
        ArtifactDigest::of_bytes(b"LOCAL_CARGO_SHA256"),
        ArtifactDigest::of_bytes(b"LOCAL_RUST_TOOLCHAIN_SHA256"),
        "local-fallback",
    )
}

pub fn seed_manifest() -> BootstrapSeedManifest {
    let rustc = std::env::var("P12_RUSTC_SHA256");
    let cargo = std::env::var("P12_CARGO_SHA256");
    let toolchain = std::env::var("P12_RUST_TOOLCHAIN_SHA256");

    match (rustc, cargo, toolchain) {
        (Ok(rustc), Ok(cargo), Ok(toolchain)) => {
            seed_manifest_from_workflow_sha256(&rustc, &cargo, &toolchain)
                .expect("P12 workflow SHA-256 provenance must be canonical lowercase hex")
        }
        (
            Err(std::env::VarError::NotPresent),
            Err(std::env::VarError::NotPresent),
            Err(std::env::VarError::NotPresent),
        ) => local_seed_manifest(),
        _ => panic!(
            "P12 workflow provenance must provide all three SHA-256 values together or none"
        ),
    }
}

pub fn successor_generation(
    predecessor: BootstrapGenerationId,
    generator: ArtifactDigest,
    validator: ArtifactDigest,
    source: ArtifactDigest,
    candidate: ArtifactDigest,
    seed: ArtifactDigest,
) -> BootstrapGenerationId {
    let ordinal = predecessor
        .ordinal()
        .checked_add(1)
        .expect("bootstrap generation ordinal overflow");
    let mut bytes = b"formula-bootstrap-successor:v1".to_vec();
    bytes.extend_from_slice(&predecessor.ordinal().to_be_bytes());
    for digest in [
        predecessor.digest(),
        generator,
        validator,
        source,
        candidate,
        seed,
    ] {
        bytes.push(0);
        bytes.extend_from_slice(digest.as_str().as_bytes());
    }
    BootstrapGenerationId::new(ordinal, ArtifactDigest::of_bytes(&bytes))
}
