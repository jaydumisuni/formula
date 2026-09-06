use formula_core::{
    bootstrap::{BootstrapGenerationId, BootstrapRole, BootstrapSeedManifest},
    digest::ArtifactDigest,
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

fn environment_digest(name: &str, fallback: &[u8]) -> ArtifactDigest {
    match std::env::var(name) {
        Ok(value) => ArtifactDigest::of_bytes(value.as_bytes()),
        Err(_) => ArtifactDigest::of_bytes(fallback),
    }
}

pub fn seed_manifest() -> BootstrapSeedManifest {
    let canonical = [
        "P12_RUSTC_SHA256",
        "P12_CARGO_SHA256",
        "P12_RUST_TOOLCHAIN_SHA256",
    ]
    .iter()
    .all(|name| std::env::var(name).is_ok());

    BootstrapSeedManifest::new(
        BootstrapRole::ExternalToolchainSeed,
        "rust-1.98.0".into(),
        "88d9e12ae178fab0fb5cc050a94da85685d449ea".into(),
        "cargo-1.98.0".into(),
        "x86_64-unknown-linux-gnu".into(),
        environment_digest("P12_RUSTC_SHA256", b"LOCAL_RUSTC_SHA256"),
        environment_digest("P12_CARGO_SHA256", b"LOCAL_CARGO_SHA256"),
        environment_digest(
            "P12_RUST_TOOLCHAIN_SHA256",
            b"LOCAL_RUST_TOOLCHAIN_SHA256",
        ),
        "pinned-rust-1.98.0".into(),
        if canonical {
            "workflow-sha256".into()
        } else {
            "local-fallback".into()
        },
    )
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
