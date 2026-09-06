use formula_core::digest::ArtifactDigest;

pub const P10_FROZEN_PROOF_HEAD: &str = "3aeb61daf4d575db0f018245ee271597ad475e7b";

pub fn p10_frozen_proof_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(P10_FROZEN_PROOF_HEAD.as_bytes())
}

pub fn source_commit() -> &'static str {
    option_env!("GITHUB_SHA").unwrap_or("LOCAL_UNBOUND_SOURCE")
}

pub fn checker_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(b"formula-check:p11-federation-breadth-verifier:v1")
}

pub fn verifier_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(b"formula:p11-federation-breadth-canonical-replay:v1")
}
