use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const CANONICAL_U8_VECTOR_SCHEMA_V1: &str = "formula-p9-canonical-u8-vector-v1";
const SECOND_QUERY_RESULT_SCHEMA_V1: &str = "formula-p9-second-query-result-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalU8Vector {
    values: Vec<u8>,
}

impl CanonicalU8Vector {
    pub fn new(values: Vec<u8>) -> Self {
        Self { values }
    }

    pub fn values(&self) -> &[u8] {
        &self.values
    }

    pub fn digest(&self) -> ArtifactDigest {
        CanonicalValue::Object(BTreeMap::from([
            (
                "schema".into(),
                CanonicalValue::String(CANONICAL_U8_VECTOR_SCHEMA_V1.into()),
            ),
            (
                "values".into(),
                CanonicalValue::Array(
                    self.values
                        .iter()
                        .map(|value| CanonicalValue::Integer((*value).into()))
                        .collect(),
                ),
            ),
        ]))
        .digest()
    }
}

pub fn canonical_second_query_vector() -> CanonicalU8Vector {
    CanonicalU8Vector::new(vec![
        0, 1, 2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128, 129, 255, 1, 3,
    ])
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecondQueryResult {
    input_digest: ArtifactDigest,
    primitive: ArtifactDigest,
    realization: ArtifactDigest,
    matching_count: u64,
}

impl SecondQueryResult {
    pub fn new(
        input: &CanonicalU8Vector,
        primitive: ArtifactDigest,
        realization: ArtifactDigest,
        matching_count: u64,
    ) -> Self {
        Self {
            input_digest: input.digest(),
            primitive,
            realization,
            matching_count,
        }
    }

    pub fn input_digest(&self) -> ArtifactDigest {
        self.input_digest
    }

    pub fn primitive(&self) -> ArtifactDigest {
        self.primitive
    }

    pub fn realization(&self) -> ArtifactDigest {
        self.realization
    }

    pub fn matching_count(&self) -> u64 {
        self.matching_count
    }

    pub fn digest(&self) -> ArtifactDigest {
        CanonicalValue::Object(BTreeMap::from([
            (
                "input_digest".into(),
                CanonicalValue::Digest(self.input_digest),
            ),
            (
                "matching_count".into(),
                CanonicalValue::String(self.matching_count.to_string()),
            ),
            ("primitive".into(), CanonicalValue::Digest(self.primitive)),
            (
                "realization".into(),
                CanonicalValue::Digest(self.realization),
            ),
            (
                "schema".into(),
                CanonicalValue::String(SECOND_QUERY_RESULT_SCHEMA_V1.into()),
            ),
        ]))
        .digest()
    }
}
