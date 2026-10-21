//! Independent mathematical checking for the canonical P9 counting workload.
//! Ptah's process exit or candidate bytes never grant Formula authority.

use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const VECTOR_SCHEMA: &str = "formula-p9-canonical-u8-vector-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum P13CountError {
    InputMismatch,
    CandidateDigestMismatch,
    NoncanonicalCount,
    WrongCount,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct P13CheckedCount {
    input_digest: ArtifactDigest,
    count: u64,
}
impl P13CheckedCount {
    pub fn input_digest(self) -> ArtifactDigest {
        self.input_digest
    }
    pub fn count(self) -> u64 {
        self.count
    }
}

pub fn canonical_p9_vector_digest(input: &[u8]) -> ArtifactDigest {
    CanonicalValue::Object(BTreeMap::from([
        (
            "schema".into(),
            CanonicalValue::String(VECTOR_SCHEMA.into()),
        ),
        (
            "values".into(),
            CanonicalValue::Array(
                input
                    .iter()
                    .copied()
                    .map(|b| CanonicalValue::Integer(b.into()))
                    .collect(),
            ),
        ),
    ]))
    .digest()
}

/// Check exact bytes and mathematical count independently of the executor.
/// This checker yields a local checked value, NOT U_g promotion authority.
pub fn verify_p9_count_candidate(
    input: &[u8],
    expected_input_digest: ArtifactDigest,
    candidate_bytes: &[u8],
    claimed_candidate_digest: ArtifactDigest,
) -> Result<P13CheckedCount, P13CountError> {
    let actual_input_digest = canonical_p9_vector_digest(input);
    if actual_input_digest != expected_input_digest {
        return Err(P13CountError::InputMismatch);
    }
    if ArtifactDigest::of_bytes(candidate_bytes) != claimed_candidate_digest {
        return Err(P13CountError::CandidateDigestMismatch);
    }
    let raw = candidate_bytes
        .strip_suffix(b"\n")
        .ok_or(P13CountError::NoncanonicalCount)?;
    if raw.is_empty() || (raw.len() > 1 && raw[0] == b'0') || !raw.iter().all(u8::is_ascii_digit) {
        return Err(P13CountError::NoncanonicalCount);
    }
    let actual = std::str::from_utf8(raw)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .ok_or(P13CountError::NoncanonicalCount)?;
    // Independent sealed-domain membership, not the worker's bitwise predicate.
    let expected = input
        .iter()
        .filter(|&&n| matches!(n, 1 | 2 | 4 | 8 | 16 | 32 | 64 | 128))
        .count() as u64;
    if actual != expected {
        return Err(P13CountError::WrongCount);
    }
    Ok(P13CheckedCount {
        input_digest: actual_input_digest,
        count: actual,
    })
}
