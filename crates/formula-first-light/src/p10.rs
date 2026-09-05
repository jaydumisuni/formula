use formula_core::digest::ArtifactDigest;

pub const P9_FROZEN_PROOF_HEAD: &str = "b353365fa8b20a13b658c07b3027334b69eff108";

pub fn p9_frozen_proof_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(P9_FROZEN_PROOF_HEAD.as_bytes())
}

pub fn source_commit() -> &'static str {
    option_env!("GITHUB_SHA").unwrap_or("LOCAL_UNBOUND_SOURCE")
}

pub fn checker_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(b"formula-check:p10-self-expansion-verifier:v1")
}

pub fn verifier_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(b"formula:p10-self-expansion-canonical-replay:v1")
}
