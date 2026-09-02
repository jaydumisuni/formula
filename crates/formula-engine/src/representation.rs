use crate::query::RequestedResultClass;
use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const REPRESENTATION_NODE_SCHEMA_V1: &str = "formula-representation-node-v1";
const REPRESENTATION_EDGE_SCHEMA_V1: &str = "formula-representation-edge-v1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ExactnessClass {
    Exact,
    SoundOverApproximation,
    SoundUnderApproximation,
    HeuristicProposal,
}

impl ExactnessClass {
    fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "EXACT",
            Self::SoundOverApproximation => "SOUND_OVER_APPROXIMATION",
            Self::SoundUnderApproximation => "SOUND_UNDER_APPROXIMATION",
            Self::HeuristicProposal => "HEURISTIC_PROPOSAL",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InformationLoss {
    None,
    Declared,
}

impl InformationLoss {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Declared => "DECLARED",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreservationMetadata {
    exactness: ExactnessClass,
    result_classes: Vec<RequestedResultClass>,
}

impl PreservationMetadata {
    pub fn new(exactness: ExactnessClass, mut result_classes: Vec<RequestedResultClass>) -> Self {
        result_classes.sort_unstable();
        result_classes.dedup();
        Self {
            exactness,
            result_classes,
        }
    }

    pub fn preserves(&self, requested: RequestedResultClass) -> bool {
        self.result_classes.binary_search(&requested).is_ok()
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "exactness".into(),
                CanonicalValue::String(self.exactness.as_str().into()),
            ),
            (
                "result_classes".into(),
                CanonicalValue::Array(
                    self.result_classes
                        .iter()
                        .map(|class| CanonicalValue::String(class.as_str().into()))
                        .collect(),
                ),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepresentationNode {
    semantic_target: ArtifactDigest,
    representation: ArtifactDigest,
    world: ArtifactDigest,
    exactness: ExactnessClass,
    observer: ArtifactDigest,
}

impl RepresentationNode {
    pub fn new(
        semantic_target: ArtifactDigest,
        representation: ArtifactDigest,
        world: ArtifactDigest,
        exactness: ExactnessClass,
        observer: ArtifactDigest,
    ) -> Self {
        Self {
            semantic_target,
            representation,
            world,
            exactness,
            observer,
        }
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn observer(&self) -> ArtifactDigest {
        self.observer
    }

    pub fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "exactness".into(),
                CanonicalValue::String(self.exactness.as_str().into()),
            ),
            ("observer".into(), CanonicalValue::Digest(self.observer)),
            (
                "representation".into(),
                CanonicalValue::Digest(self.representation),
            ),
            (
                "schema".into(),
                CanonicalValue::String(REPRESENTATION_NODE_SCHEMA_V1.into()),
            ),
            (
                "semantic_target".into(),
                CanonicalValue::Digest(self.semantic_target),
            ),
            ("world".into(), CanonicalValue::Digest(self.world)),
        ]))
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepresentationError {
    SourceBindingMismatch,
    TargetBindingMismatch,
    WorldMismatch,
    ObserverMismatch,
    MissingPreservationMetadata,
    RequestedResultNotPreserved,
    MissingReconstructionRoute,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepresentationEdge {
    source: ArtifactDigest,
    target: ArtifactDigest,
    transformation: ArtifactDigest,
    preservation: Option<PreservationMetadata>,
    information_loss: InformationLoss,
    reconstruction_route: Option<ArtifactDigest>,
    certificate_route: Option<ArtifactDigest>,
    assumptions: Vec<ArtifactDigest>,
}

impl RepresentationEdge {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source: ArtifactDigest,
        target: ArtifactDigest,
        transformation: ArtifactDigest,
        preservation: Option<PreservationMetadata>,
        information_loss: InformationLoss,
        reconstruction_route: Option<ArtifactDigest>,
        certificate_route: Option<ArtifactDigest>,
        mut assumptions: Vec<ArtifactDigest>,
    ) -> Self {
        assumptions.sort_unstable();
        assumptions.dedup();
        Self {
            source,
            target,
            transformation,
            preservation,
            information_loss,
            reconstruction_route,
            certificate_route,
            assumptions,
        }
    }

    pub fn validate(
        &self,
        source: &RepresentationNode,
        target: &RepresentationNode,
        requested: RequestedResultClass,
    ) -> Result<(), RepresentationError> {
        if self.source != source.digest() {
            return Err(RepresentationError::SourceBindingMismatch);
        }
        if self.target != target.digest() {
            return Err(RepresentationError::TargetBindingMismatch);
        }
        if source.world() != target.world() {
            return Err(RepresentationError::WorldMismatch);
        }
        if source.observer() != target.observer() {
            return Err(RepresentationError::ObserverMismatch);
        }
        let preservation = self
            .preservation
            .as_ref()
            .ok_or(RepresentationError::MissingPreservationMetadata)?;
        if !preservation.preserves(requested) {
            return Err(RepresentationError::RequestedResultNotPreserved);
        }
        if self.information_loss == InformationLoss::Declared
            && requested == RequestedResultClass::Witness
            && self.reconstruction_route.is_none()
        {
            return Err(RepresentationError::MissingReconstructionRoute);
        }
        Ok(())
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
                "certificate_route".into(),
                self.certificate_route
                    .map(CanonicalValue::Digest)
                    .unwrap_or(CanonicalValue::Null),
            ),
            (
                "information_loss".into(),
                CanonicalValue::String(self.information_loss.as_str().into()),
            ),
            (
                "preservation".into(),
                self.preservation
                    .as_ref()
                    .map(PreservationMetadata::canonical_value)
                    .unwrap_or(CanonicalValue::Null),
            ),
            (
                "reconstruction_route".into(),
                self.reconstruction_route
                    .map(CanonicalValue::Digest)
                    .unwrap_or(CanonicalValue::Null),
            ),
            (
                "schema".into(),
                CanonicalValue::String(REPRESENTATION_EDGE_SCHEMA_V1.into()),
            ),
            ("source".into(), CanonicalValue::Digest(self.source)),
            ("target".into(), CanonicalValue::Digest(self.target)),
            (
                "transformation".into(),
                CanonicalValue::Digest(self.transformation),
            ),
        ]))
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}
