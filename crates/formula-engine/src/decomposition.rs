use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const DECOMPOSITION_SCHEMA_V1: &str = "formula-decomposition-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AggregationSemantics {
    And,
    Or,
}

impl AggregationSemantics {
    fn as_str(self) -> &'static str {
        match self {
            Self::And => "AND",
            Self::Or => "OR",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ChildObligation {
    obligation: ArtifactDigest,
    world: ArtifactDigest,
}

impl ChildObligation {
    pub fn new(obligation: ArtifactDigest, world: ArtifactDigest) -> Self {
        Self { obligation, world }
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "obligation".into(),
                CanonicalValue::Digest(self.obligation),
            ),
            ("world".into(), CanonicalValue::Digest(self.world)),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecompositionError {
    EmptyChildren,
    MissingAggregation,
    MissingReconstruction,
    MissingEvidence,
    WorldMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decomposition {
    parent_goal: ArtifactDigest,
    world: ArtifactDigest,
    children: Vec<ChildObligation>,
    interface_or_separator: Option<ArtifactDigest>,
    aggregation: Option<AggregationSemantics>,
    reconstruction_relation: Option<ArtifactDigest>,
    evidence: Option<ArtifactDigest>,
}

impl Decomposition {
    pub fn new(
        parent_goal: ArtifactDigest,
        world: ArtifactDigest,
        mut children: Vec<ChildObligation>,
        interface_or_separator: Option<ArtifactDigest>,
        aggregation: Option<AggregationSemantics>,
        reconstruction_relation: Option<ArtifactDigest>,
        evidence: Option<ArtifactDigest>,
    ) -> Self {
        children.sort_unstable();
        children.dedup();
        Self {
            parent_goal,
            world,
            children,
            interface_or_separator,
            aggregation,
            reconstruction_relation,
            evidence,
        }
    }

    pub fn validate(&self) -> Result<(), DecompositionError> {
        if self.children.is_empty() {
            return Err(DecompositionError::EmptyChildren);
        }
        if self.aggregation.is_none() {
            return Err(DecompositionError::MissingAggregation);
        }
        if self.reconstruction_relation.is_none() {
            return Err(DecompositionError::MissingReconstruction);
        }
        if self.evidence.is_none() {
            return Err(DecompositionError::MissingEvidence);
        }
        if self.children.iter().any(|child| child.world != self.world) {
            return Err(DecompositionError::WorldMismatch);
        }
        Ok(())
    }

    pub fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "aggregation".into(),
                self.aggregation
                    .map(|value| CanonicalValue::String(value.as_str().into()))
                    .unwrap_or(CanonicalValue::Null),
            ),
            (
                "children".into(),
                CanonicalValue::Array(
                    self.children
                        .iter()
                        .map(ChildObligation::canonical_value)
                        .collect(),
                ),
            ),
            (
                "evidence".into(),
                self.evidence
                    .map(CanonicalValue::Digest)
                    .unwrap_or(CanonicalValue::Null),
            ),
            (
                "interface_or_separator".into(),
                self.interface_or_separator
                    .map(CanonicalValue::Digest)
                    .unwrap_or(CanonicalValue::Null),
            ),
            (
                "parent_goal".into(),
                CanonicalValue::Digest(self.parent_goal),
            ),
            (
                "reconstruction_relation".into(),
                self.reconstruction_relation
                    .map(CanonicalValue::Digest)
                    .unwrap_or(CanonicalValue::Null),
            ),
            (
                "schema".into(),
                CanonicalValue::String(DECOMPOSITION_SCHEMA_V1.into()),
            ),
            ("world".into(), CanonicalValue::Digest(self.world)),
        ]))
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}
