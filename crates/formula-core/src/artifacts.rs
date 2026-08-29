use crate::{canonical::CanonicalValue, digest::ArtifactDigest};
use num_bigint::BigInt;
use std::collections::BTreeMap;

const AUTHORITY_SCHEMA_V1: &str = "formula-authority-v1";

pub trait StructuralIdentity {
    fn canonical_value(&self) -> CanonicalValue;

    fn structural_digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}

fn canonical_object(kind: &str) -> BTreeMap<String, CanonicalValue> {
    BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        (
            "schema".into(),
            CanonicalValue::String(AUTHORITY_SCHEMA_V1.into()),
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

fn sorted_strings(mut values: Vec<String>) -> Vec<String> {
    values.sort_unstable();
    values.dedup();
    values
}

fn string_array(values: &[String]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().cloned().map(CanonicalValue::String).collect())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entity {
    foundation: ArtifactDigest,
    structure: CanonicalValue,
    references: Vec<ArtifactDigest>,
}

impl Entity {
    pub fn new(
        foundation: ArtifactDigest,
        structure: CanonicalValue,
        references: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            foundation,
            structure,
            references: sorted_digests(references),
        }
    }
}

impl StructuralIdentity for Entity {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("Entity");
        object.insert("foundation".into(), CanonicalValue::Digest(self.foundation));
        object.insert("references".into(), digest_array(&self.references));
        object.insert("structure".into(), self.structure.clone());
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Relation {
    foundation: ArtifactDigest,
    arity: u32,
    definition: CanonicalValue,
    references: Vec<ArtifactDigest>,
}

impl Relation {
    pub fn new(
        foundation: ArtifactDigest,
        arity: u32,
        definition: CanonicalValue,
        references: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            foundation,
            arity,
            definition,
            references: sorted_digests(references),
        }
    }
}

impl StructuralIdentity for Relation {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("Relation");
        object.insert("arity".into(), CanonicalValue::Integer(BigInt::from(self.arity)));
        object.insert("definition".into(), self.definition.clone());
        object.insert("foundation".into(), CanonicalValue::Digest(self.foundation));
        object.insert("references".into(), digest_array(&self.references));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct World {
    parent_worlds: Vec<ArtifactDigest>,
    assumptions: Vec<ArtifactDigest>,
    local_definitions: Vec<ArtifactDigest>,
    local_equalities: Vec<ArtifactDigest>,
    local_disequalities: Vec<ArtifactDigest>,
    foundation: ArtifactDigest,
}

impl World {
    pub fn new(
        parent_worlds: Vec<ArtifactDigest>,
        assumptions: Vec<ArtifactDigest>,
        local_definitions: Vec<ArtifactDigest>,
        local_equalities: Vec<ArtifactDigest>,
        local_disequalities: Vec<ArtifactDigest>,
        foundation: ArtifactDigest,
    ) -> Self {
        Self {
            parent_worlds: sorted_digests(parent_worlds),
            assumptions: sorted_digests(assumptions),
            local_definitions: sorted_digests(local_definitions),
            local_equalities: sorted_digests(local_equalities),
            local_disequalities: sorted_digests(local_disequalities),
            foundation,
        }
    }
}

impl StructuralIdentity for World {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("World");
        object.insert("assumptions".into(), digest_array(&self.assumptions));
        object.insert("foundation".into(), CanonicalValue::Digest(self.foundation));
        object.insert(
            "local_definitions".into(),
            digest_array(&self.local_definitions),
        );
        object.insert(
            "local_disequalities".into(),
            digest_array(&self.local_disequalities),
        );
        object.insert(
            "local_equalities".into(),
            digest_array(&self.local_equalities),
        );
        object.insert("parent_worlds".into(), digest_array(&self.parent_worlds));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Judgement {
    world: ArtifactDigest,
    proposition: CanonicalValue,
    references: Vec<ArtifactDigest>,
}

impl Judgement {
    pub fn new(
        world: ArtifactDigest,
        proposition: CanonicalValue,
        references: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            world,
            proposition,
            references: sorted_digests(references),
        }
    }
}

impl StructuralIdentity for Judgement {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("Judgement");
        object.insert("proposition".into(), self.proposition.clone());
        object.insert("references".into(), digest_array(&self.references));
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceEnvelope {
    target_judgement: ArtifactDigest,
    world: ArtifactDigest,
    scope: CanonicalValue,
    evidence_family: String,
    evidence_body_digest: ArtifactDigest,
    producer: ArtifactDigest,
    checker: ArtifactDigest,
    checker_trust_root: ArtifactDigest,
    verdict: String,
    dependency_digests: Vec<ArtifactDigest>,
    replay_binding: CanonicalValue,
}

impl EvidenceEnvelope {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        target_judgement: ArtifactDigest,
        world: ArtifactDigest,
        scope: CanonicalValue,
        evidence_family: String,
        evidence_body_digest: ArtifactDigest,
        producer: ArtifactDigest,
        checker: ArtifactDigest,
        checker_trust_root: ArtifactDigest,
        verdict: String,
        dependency_digests: Vec<ArtifactDigest>,
        replay_binding: CanonicalValue,
    ) -> Self {
        Self {
            target_judgement,
            world,
            scope,
            evidence_family,
            evidence_body_digest,
            producer,
            checker,
            checker_trust_root,
            verdict,
            dependency_digests: sorted_digests(dependency_digests),
            replay_binding,
        }
    }
}

impl StructuralIdentity for EvidenceEnvelope {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("EvidenceEnvelope");
        object.insert("checker".into(), CanonicalValue::Digest(self.checker));
        object.insert(
            "checker_trust_root".into(),
            CanonicalValue::Digest(self.checker_trust_root),
        );
        object.insert(
            "dependency_digests".into(),
            digest_array(&self.dependency_digests),
        );
        object.insert(
            "evidence_body_digest".into(),
            CanonicalValue::Digest(self.evidence_body_digest),
        );
        object.insert(
            "evidence_family".into(),
            CanonicalValue::String(self.evidence_family.clone()),
        );
        object.insert("producer".into(), CanonicalValue::Digest(self.producer));
        object.insert("replay_binding".into(), self.replay_binding.clone());
        object.insert("scope".into(), self.scope.clone());
        object.insert(
            "target_judgement".into(),
            CanonicalValue::Digest(self.target_judgement),
        );
        object.insert("verdict".into(), CanonicalValue::String(self.verdict.clone()));
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationMetadata {
    semantic_target: ArtifactDigest,
    realization_kind: String,
    source_digest: ArtifactDigest,
    binary_digest: ArtifactDigest,
    input_output_semantics: CanonicalValue,
    validation_evidence_digest: ArtifactDigest,
}

impl RealizationMetadata {
    pub fn new(
        semantic_target: ArtifactDigest,
        realization_kind: String,
        source_digest: ArtifactDigest,
        binary_digest: ArtifactDigest,
        input_output_semantics: CanonicalValue,
        validation_evidence_digest: ArtifactDigest,
    ) -> Self {
        Self {
            semantic_target,
            realization_kind,
            source_digest,
            binary_digest,
            input_output_semantics,
            validation_evidence_digest,
        }
    }
}

impl StructuralIdentity for RealizationMetadata {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("RealizationMetadata");
        object.insert("binary_digest".into(), CanonicalValue::Digest(self.binary_digest));
        object.insert(
            "input_output_semantics".into(),
            self.input_output_semantics.clone(),
        );
        object.insert(
            "realization_kind".into(),
            CanonicalValue::String(self.realization_kind.clone()),
        );
        object.insert(
            "semantic_target".into(),
            CanonicalValue::Digest(self.semantic_target),
        );
        object.insert("source_digest".into(), CanonicalValue::Digest(self.source_digest));
        object.insert(
            "validation_evidence_digest".into(),
            CanonicalValue::Digest(self.validation_evidence_digest),
        );
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityContract {
    requested_authority_class: String,
    allowed_evidence_families: Vec<String>,
    exactness_requirement: String,
}

impl AuthorityContract {
    pub fn new(
        requested_authority_class: String,
        allowed_evidence_families: Vec<String>,
        exactness_requirement: String,
    ) -> Self {
        Self {
            requested_authority_class,
            allowed_evidence_families: sorted_strings(allowed_evidence_families),
            exactness_requirement,
        }
    }
}

impl StructuralIdentity for AuthorityContract {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("AuthorityContract");
        object.insert(
            "allowed_evidence_families".into(),
            string_array(&self.allowed_evidence_families),
        );
        object.insert(
            "exactness_requirement".into(),
            CanonicalValue::String(self.exactness_requirement.clone()),
        );
        object.insert(
            "requested_authority_class".into(),
            CanonicalValue::String(self.requested_authority_class.clone()),
        );
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Observer {
    observer_family: String,
    semantics: CanonicalValue,
}

impl Observer {
    pub fn new(observer_family: String, semantics: CanonicalValue) -> Self {
        Self {
            observer_family,
            semantics,
        }
    }
}

impl StructuralIdentity for Observer {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("Observer");
        object.insert(
            "observer_family".into(),
            CanonicalValue::String(self.observer_family.clone()),
        );
        object.insert("semantics".into(), self.semantics.clone());
        CanonicalValue::Object(object)
    }
}
