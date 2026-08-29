use crate::{artifacts::StructuralIdentity, canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const CERTIFICATION_SCHEMA_V1: &str = "formula-certification-v1";

fn canonical_object(kind: &str) -> BTreeMap<String, CanonicalValue> {
    BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        (
            "schema".into(),
            CanonicalValue::String(CERTIFICATION_SCHEMA_V1.into()),
        ),
    ])
}

fn sorted_digests(mut values: Vec<ArtifactDigest>) -> Vec<ArtifactDigest> {
    values.sort_unstable();
    values.dedup();
    values
}

fn digest_array(values: &[ArtifactDigest]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().copied().map(CanonicalValue::Digest).collect())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenCandidate {
    candidate_class: String,
    semantic_artifacts: Vec<ArtifactDigest>,
    world: ArtifactDigest,
    universe_generation: ArtifactDigest,
    dependencies: Vec<ArtifactDigest>,
    proposed_judgements: Vec<ArtifactDigest>,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
}

impl FrozenCandidate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        candidate_class: String,
        semantic_artifacts: Vec<ArtifactDigest>,
        world: ArtifactDigest,
        universe_generation: ArtifactDigest,
        dependencies: Vec<ArtifactDigest>,
        proposed_judgements: Vec<ArtifactDigest>,
        authority_contract: ArtifactDigest,
        observer: ArtifactDigest,
    ) -> Self {
        Self {
            candidate_class,
            semantic_artifacts: sorted_digests(semantic_artifacts),
            world,
            universe_generation,
            dependencies: sorted_digests(dependencies),
            proposed_judgements: sorted_digests(proposed_judgements),
            authority_contract,
            observer,
        }
    }

    pub fn candidate_class(&self) -> &str {
        &self.candidate_class
    }

    pub fn semantic_artifacts(&self) -> &[ArtifactDigest] {
        &self.semantic_artifacts
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn dependencies(&self) -> &[ArtifactDigest] {
        &self.dependencies
    }

    pub fn proposed_judgements(&self) -> &[ArtifactDigest] {
        &self.proposed_judgements
    }

    pub fn authority_contract(&self) -> ArtifactDigest {
        self.authority_contract
    }

    pub fn observer(&self) -> ArtifactDigest {
        self.observer
    }
}

impl StructuralIdentity for FrozenCandidate {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("FrozenCandidate");
        object.insert(
            "authority_contract".into(),
            CanonicalValue::Digest(self.authority_contract),
        );
        object.insert(
            "candidate_class".into(),
            CanonicalValue::String(self.candidate_class.clone()),
        );
        object.insert("dependencies".into(), digest_array(&self.dependencies));
        object.insert("observer".into(), CanonicalValue::Digest(self.observer));
        object.insert(
            "proposed_judgements".into(),
            digest_array(&self.proposed_judgements),
        );
        object.insert(
            "semantic_artifacts".into(),
            digest_array(&self.semantic_artifacts),
        );
        object.insert(
            "universe_generation".into(),
            CanonicalValue::Digest(self.universe_generation),
        );
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertificateEnvelope {
    frozen_candidate: ArtifactDigest,
    target_judgement: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    semantic_scope: CanonicalValue,
    outcome_class: String,
    verification_mode: String,
    certificate_family: String,
    certificate_family_version: String,
    certificate_body_digest: ArtifactDigest,
    producer: ArtifactDigest,
    checker: ArtifactDigest,
    checker_trust_root: ArtifactDigest,
    dependencies: Vec<ArtifactDigest>,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
    replay_binding: CanonicalValue,
}

impl CertificateEnvelope {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        frozen_candidate: ArtifactDigest,
        target_judgement: ArtifactDigest,
        universe_generation: ArtifactDigest,
        world: ArtifactDigest,
        semantic_scope: CanonicalValue,
        outcome_class: String,
        verification_mode: String,
        certificate_family: String,
        certificate_family_version: String,
        certificate_body_digest: ArtifactDigest,
        producer: ArtifactDigest,
        checker: ArtifactDigest,
        checker_trust_root: ArtifactDigest,
        dependencies: Vec<ArtifactDigest>,
        authority_contract: ArtifactDigest,
        observer: ArtifactDigest,
        replay_binding: CanonicalValue,
    ) -> Self {
        Self {
            frozen_candidate,
            target_judgement,
            universe_generation,
            world,
            semantic_scope,
            outcome_class,
            verification_mode,
            certificate_family,
            certificate_family_version,
            certificate_body_digest,
            producer,
            checker,
            checker_trust_root,
            dependencies: sorted_digests(dependencies),
            authority_contract,
            observer,
            replay_binding,
        }
    }

    pub fn frozen_candidate(&self) -> ArtifactDigest {
        self.frozen_candidate
    }

    pub fn target_judgement(&self) -> ArtifactDigest {
        self.target_judgement
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn semantic_scope(&self) -> &CanonicalValue {
        &self.semantic_scope
    }

    pub fn outcome_class(&self) -> &str {
        &self.outcome_class
    }

    pub fn verification_mode(&self) -> &str {
        &self.verification_mode
    }

    pub fn certificate_family(&self) -> &str {
        &self.certificate_family
    }

    pub fn certificate_family_version(&self) -> &str {
        &self.certificate_family_version
    }

    pub fn certificate_body_digest(&self) -> ArtifactDigest {
        self.certificate_body_digest
    }

    pub fn producer(&self) -> ArtifactDigest {
        self.producer
    }

    pub fn checker(&self) -> ArtifactDigest {
        self.checker
    }

    pub fn checker_trust_root(&self) -> ArtifactDigest {
        self.checker_trust_root
    }

    pub fn dependencies(&self) -> &[ArtifactDigest] {
        &self.dependencies
    }

    pub fn authority_contract(&self) -> ArtifactDigest {
        self.authority_contract
    }

    pub fn observer(&self) -> ArtifactDigest {
        self.observer
    }

    pub fn replay_binding(&self) -> &CanonicalValue {
        &self.replay_binding
    }
}

impl StructuralIdentity for CertificateEnvelope {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("CertificateEnvelope");
        object.insert(
            "authority_contract".into(),
            CanonicalValue::Digest(self.authority_contract),
        );
        object.insert("checker".into(), CanonicalValue::Digest(self.checker));
        object.insert(
            "checker_trust_root".into(),
            CanonicalValue::Digest(self.checker_trust_root),
        );
        object.insert(
            "certificate_body_digest".into(),
            CanonicalValue::Digest(self.certificate_body_digest),
        );
        object.insert(
            "certificate_family".into(),
            CanonicalValue::String(self.certificate_family.clone()),
        );
        object.insert(
            "certificate_family_version".into(),
            CanonicalValue::String(self.certificate_family_version.clone()),
        );
        object.insert("dependencies".into(), digest_array(&self.dependencies));
        object.insert(
            "frozen_candidate".into(),
            CanonicalValue::Digest(self.frozen_candidate),
        );
        object.insert("observer".into(), CanonicalValue::Digest(self.observer));
        object.insert(
            "outcome_class".into(),
            CanonicalValue::String(self.outcome_class.clone()),
        );
        object.insert("producer".into(), CanonicalValue::Digest(self.producer));
        object.insert("replay_binding".into(), self.replay_binding.clone());
        object.insert("semantic_scope".into(), self.semantic_scope.clone());
        object.insert(
            "target_judgement".into(),
            CanonicalValue::Digest(self.target_judgement),
        );
        object.insert(
            "universe_generation".into(),
            CanonicalValue::Digest(self.universe_generation),
        );
        object.insert(
            "verification_mode".into(),
            CanonicalValue::String(self.verification_mode.clone()),
        );
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionManifest {
    parent_generation: ArtifactDigest,
    frozen_candidate: ArtifactDigest,
    evidence_envelopes: Vec<ArtifactDigest>,
    proposed_admissions: Vec<ArtifactDigest>,
    proposed_authority_bindings: Vec<ArtifactDigest>,
}

impl PromotionManifest {
    pub fn new(
        parent_generation: ArtifactDigest,
        frozen_candidate: ArtifactDigest,
        evidence_envelopes: Vec<ArtifactDigest>,
        proposed_admissions: Vec<ArtifactDigest>,
        proposed_authority_bindings: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            parent_generation,
            frozen_candidate,
            evidence_envelopes: sorted_digests(evidence_envelopes),
            proposed_admissions: sorted_digests(proposed_admissions),
            proposed_authority_bindings: sorted_digests(proposed_authority_bindings),
        }
    }

    pub fn parent_generation(&self) -> ArtifactDigest {
        self.parent_generation
    }

    pub fn frozen_candidate(&self) -> ArtifactDigest {
        self.frozen_candidate
    }

    pub fn evidence_envelopes(&self) -> &[ArtifactDigest] {
        &self.evidence_envelopes
    }

    pub fn proposed_admissions(&self) -> &[ArtifactDigest] {
        &self.proposed_admissions
    }

    pub fn proposed_authority_bindings(&self) -> &[ArtifactDigest] {
        &self.proposed_authority_bindings
    }
}

impl StructuralIdentity for PromotionManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("PromotionManifest");
        object.insert(
            "evidence_envelopes".into(),
            digest_array(&self.evidence_envelopes),
        );
        object.insert(
            "frozen_candidate".into(),
            CanonicalValue::Digest(self.frozen_candidate),
        );
        object.insert(
            "parent_generation".into(),
            CanonicalValue::Digest(self.parent_generation),
        );
        object.insert(
            "proposed_admissions".into(),
            digest_array(&self.proposed_admissions),
        );
        object.insert(
            "proposed_authority_bindings".into(),
            digest_array(&self.proposed_authority_bindings),
        );
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationCheckManifest {
    semantic_target: ArtifactDigest,
    realization: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
    realization_artifact_digest: ArtifactDigest,
}

impl RealizationCheckManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        semantic_target: ArtifactDigest,
        realization: ArtifactDigest,
        universe_generation: ArtifactDigest,
        world: ArtifactDigest,
        authority_contract: ArtifactDigest,
        observer: ArtifactDigest,
        realization_artifact_digest: ArtifactDigest,
    ) -> Self {
        Self {
            semantic_target,
            realization,
            universe_generation,
            world,
            authority_contract,
            observer,
            realization_artifact_digest,
        }
    }

    pub fn semantic_target(&self) -> ArtifactDigest {
        self.semantic_target
    }

    pub fn realization(&self) -> ArtifactDigest {
        self.realization
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn authority_contract(&self) -> ArtifactDigest {
        self.authority_contract
    }

    pub fn observer(&self) -> ArtifactDigest {
        self.observer
    }

    pub fn realization_artifact_digest(&self) -> ArtifactDigest {
        self.realization_artifact_digest
    }
}

impl StructuralIdentity for RealizationCheckManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("RealizationCheckManifest");
        object.insert(
            "authority_contract".into(),
            CanonicalValue::Digest(self.authority_contract),
        );
        object.insert("observer".into(), CanonicalValue::Digest(self.observer));
        object.insert(
            "realization".into(),
            CanonicalValue::Digest(self.realization),
        );
        object.insert(
            "realization_artifact_digest".into(),
            CanonicalValue::Digest(self.realization_artifact_digest),
        );
        object.insert(
            "semantic_target".into(),
            CanonicalValue::Digest(self.semantic_target),
        );
        object.insert(
            "universe_generation".into(),
            CanonicalValue::Digest(self.universe_generation),
        );
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        CanonicalValue::Object(object)
    }
}
