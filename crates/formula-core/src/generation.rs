use crate::{canonical::CanonicalValue, digest::ArtifactDigest};
use num_bigint::BigInt;
use std::collections::BTreeMap;

const AUTHORITY_SCHEMA_V1: &str = "formula-authority-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UniverseGeneration {
    generation_number: u64,
    parent: Option<ArtifactDigest>,
    admitted: Vec<ArtifactDigest>,
    authority_bindings: Vec<ArtifactDigest>,
}

impl UniverseGeneration {
    pub fn new(
        generation_number: u64,
        parent: Option<ArtifactDigest>,
        mut admitted: Vec<ArtifactDigest>,
        mut authority_bindings: Vec<ArtifactDigest>,
    ) -> Self {
        admitted.sort_unstable();
        admitted.dedup();
        authority_bindings.sort_unstable();
        authority_bindings.dedup();

        Self {
            generation_number,
            parent,
            admitted,
            authority_bindings,
        }
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        self.canonical_value().to_canonical_bytes()
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "admitted".into(),
                CanonicalValue::Array(
                    self.admitted
                        .iter()
                        .copied()
                        .map(CanonicalValue::Digest)
                        .collect(),
                ),
            ),
            (
                "authority_bindings".into(),
                CanonicalValue::Array(
                    self.authority_bindings
                        .iter()
                        .copied()
                        .map(CanonicalValue::Digest)
                        .collect(),
                ),
            ),
            (
                "generation_number".into(),
                CanonicalValue::Integer(BigInt::from(self.generation_number)),
            ),
            (
                "kind".into(),
                CanonicalValue::String("UniverseGeneration".into()),
            ),
            (
                "parent".into(),
                self.parent
                    .map(CanonicalValue::Digest)
                    .unwrap_or(CanonicalValue::Null),
            ),
            (
                "schema".into(),
                CanonicalValue::String(AUTHORITY_SCHEMA_V1.into()),
            ),
        ]))
    }
}
