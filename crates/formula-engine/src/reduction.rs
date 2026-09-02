use crate::query::RequestedResultClass;
use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const REDUCTION_EDGE_SCHEMA_V1: &str = "formula-reduction-edge-v1";
const COMPOSED_REDUCTION_SCHEMA_V1: &str = "formula-composed-reduction-v1";

pub type ResultClass = RequestedResultClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReductionEdge {
    source_class: String,
    target_class: String,
    preserved_result_classes: Vec<ResultClass>,
    encode_relation: ArtifactDigest,
    reconstruction_relation: Option<ArtifactDigest>,
    assumptions: Vec<ArtifactDigest>,
    evidence: Option<ArtifactDigest>,
}

impl ReductionEdge {
    pub fn new(
        source_class: impl Into<String>,
        target_class: impl Into<String>,
        mut preserved_result_classes: Vec<ResultClass>,
        encode_relation: ArtifactDigest,
        reconstruction_relation: Option<ArtifactDigest>,
        mut assumptions: Vec<ArtifactDigest>,
        evidence: Option<ArtifactDigest>,
    ) -> Self {
        preserved_result_classes.sort_unstable();
        preserved_result_classes.dedup();
        assumptions.sort_unstable();
        assumptions.dedup();
        Self {
            source_class: source_class.into(),
            target_class: target_class.into(),
            preserved_result_classes,
            encode_relation,
            reconstruction_relation,
            assumptions,
            evidence,
        }
    }

    pub fn preserves(&self, requested: ResultClass) -> bool {
        self.preserved_result_classes.binary_search(&requested).is_ok()
    }

    pub fn reconstruction_relation(&self) -> Option<ArtifactDigest> {
        self.reconstruction_relation
    }

    pub fn evidence(&self) -> Option<ArtifactDigest> {
        self.evidence
    }

    pub fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "assumptions".into(),
                CanonicalValue::Array(
                    self.assumptions
                        .iter()
                        .copied()
                        .map(CanonicalValue::Digest)
                        .collect(),
                ),
            ),
            (
                "encode_relation".into(),
                CanonicalValue::Digest(self.encode_relation),
            ),
            (
                "evidence".into(),
                self.evidence
                    .map(CanonicalValue::Digest)
                    .unwrap_or(CanonicalValue::Null),
            ),
            (
                "preserved_result_classes".into(),
                CanonicalValue::Array(
                    self.preserved_result_classes
                        .iter()
                        .map(|class| CanonicalValue::String(class.as_str().into()))
                        .collect(),
                ),
            ),
            (
                "reconstruction_relation".into(),
                self.reconstruction_relation
                    .map(CanonicalValue::Digest)
                    .unwrap_or(CanonicalValue::Null),
            ),
            (
                "schema".into(),
                CanonicalValue::String(REDUCTION_EDGE_SCHEMA_V1.into()),
            ),
            (
                "source_class".into(),
                CanonicalValue::String(self.source_class.clone()),
            ),
            (
                "target_class".into(),
                CanonicalValue::String(self.target_class.clone()),
            ),
        ]))
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReductionError {
    EmptyPath,
    MissingEvidence,
    RequestedResultNotPreserved,
    MissingReconstruction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComposedReduction {
    requested: ResultClass,
    edge_digests: Vec<ArtifactDigest>,
}

impl ComposedReduction {
    pub fn requested(&self) -> ResultClass {
        self.requested
    }

    pub fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "edge_digests".into(),
                CanonicalValue::Array(
                    self.edge_digests
                        .iter()
                        .copied()
                        .map(CanonicalValue::Digest)
                        .collect(),
                ),
            ),
            (
                "requested".into(),
                CanonicalValue::String(self.requested.as_str().into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(COMPOSED_REDUCTION_SCHEMA_V1.into()),
            ),
        ]))
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}

pub fn compose_reduction_path(
    path: &[ReductionEdge],
    requested: ResultClass,
) -> Result<ComposedReduction, ReductionError> {
    if path.is_empty() {
        return Err(ReductionError::EmptyPath);
    }

    for edge in path {
        if edge.evidence().is_none() {
            return Err(ReductionError::MissingEvidence);
        }
        if !edge.preserves(requested) {
            return Err(ReductionError::RequestedResultNotPreserved);
        }
        if requested == ResultClass::Witness && edge.reconstruction_relation().is_none() {
            return Err(ReductionError::MissingReconstruction);
        }
    }

    Ok(ComposedReduction {
        requested,
        edge_digests: path.iter().map(ReductionEdge::digest).collect(),
    })
}
