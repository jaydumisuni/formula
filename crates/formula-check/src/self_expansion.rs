use crate::promotion::PromotionAuthorization;
use formula_core::{
    artifacts::StructuralIdentity,
    canonical::CanonicalValue,
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    self_expansion::{
        ActivationMode, ClassifiedPromotionCandidate, MetaprimitiveGateEvidence, PromotionClass,
        PromotionClassRegistryV1, SemanticChangeClass,
    },
};
use std::collections::BTreeMap;

const EXPANSION_AUTHORIZATION_SCHEMA_V1: &str = "formula-expansion-authorization-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpansionPolicyFailure {
    BasePromotionMismatch,
    ParentGenerationMismatch,
    ActivationModeForbidden,
    CapabilityEffectForbidden,
    GrammarEffectForbidden,
    RealizationOnlySemanticAdmissionForbidden,
    NogoodScopeRequired,
    MetaprimitiveGateRequired,
    MetaprimitiveClassRequired,
    GateEvidenceNotPromotionBound(ArtifactDigest),
    GateScopeMismatch,
    GateAuthorizationMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpansionAuthorization {
    base_promotion_candidate: ArtifactDigest,
    classified_candidate: ArtifactDigest,
    class: PromotionClass,
    parent_generation: ArtifactDigest,
    activation_mode: ActivationMode,
    semantic_change_class: SemanticChangeClass,
    activation_effects: Vec<ArtifactDigest>,
    grammar_effects: Vec<ArtifactDigest>,
    scope: Vec<ArtifactDigest>,
    registry_policy: ArtifactDigest,
    registry: ArtifactDigest,
    metaprimitive_gate: Option<ArtifactDigest>,
    authorization_digest: ArtifactDigest,
}

impl ExpansionAuthorization {
    pub fn base_promotion_candidate(&self) -> ArtifactDigest {
        self.base_promotion_candidate
    }

    pub fn classified_candidate(&self) -> ArtifactDigest {
        self.classified_candidate
    }

    pub fn class(&self) -> PromotionClass {
        self.class
    }

    pub fn parent_generation(&self) -> ArtifactDigest {
        self.parent_generation
    }

    pub fn activation_mode(&self) -> ActivationMode {
        self.activation_mode
    }

    pub fn semantic_change_class(&self) -> SemanticChangeClass {
        self.semantic_change_class
    }

    pub fn activation_effects(&self) -> &[ArtifactDigest] {
        &self.activation_effects
    }

    pub fn grammar_effects(&self) -> &[ArtifactDigest] {
        &self.grammar_effects
    }

    pub fn scope(&self) -> &[ArtifactDigest] {
        &self.scope
    }

    pub fn registry_policy(&self) -> ArtifactDigest {
        self.registry_policy
    }

    pub fn registry(&self) -> ArtifactDigest {
        self.registry
    }

    pub fn metaprimitive_gate(&self) -> Option<ArtifactDigest> {
        self.metaprimitive_gate
    }

    pub fn authorization_digest(&self) -> ArtifactDigest {
        self.authorization_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaprimitiveGateAuthorization {
    base_promotion_candidate: ArtifactDigest,
    classified_candidate: ArtifactDigest,
    gate_evidence: ArtifactDigest,
    scope: Vec<ArtifactDigest>,
    authorization_digest: ArtifactDigest,
}

impl MetaprimitiveGateAuthorization {
    pub fn base_promotion_candidate(&self) -> ArtifactDigest {
        self.base_promotion_candidate
    }

    pub fn classified_candidate(&self) -> ArtifactDigest {
        self.classified_candidate
    }

    pub fn gate_evidence(&self) -> ArtifactDigest {
        self.gate_evidence
    }

    pub fn scope(&self) -> &[ArtifactDigest] {
        &self.scope
    }

    pub fn authorization_digest(&self) -> ArtifactDigest {
        self.authorization_digest
    }
}

fn digest_array(values: &[ArtifactDigest]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().copied().map(CanonicalValue::Digest).collect())
}

fn optional_digest(value: Option<ArtifactDigest>) -> CanonicalValue {
    value.map_or(CanonicalValue::Null, CanonicalValue::Digest)
}

fn authorization_object(kind: &str) -> BTreeMap<String, CanonicalValue> {
    BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        (
            "schema".into(),
            CanonicalValue::String(EXPANSION_AUTHORIZATION_SCHEMA_V1.into()),
        ),
    ])
}

fn expansion_authorization_digest(
    base: &PromotionAuthorization,
    classified: &ClassifiedPromotionCandidate,
    parent_generation: ArtifactDigest,
    registry_policy: ArtifactDigest,
    metaprimitive_gate: Option<ArtifactDigest>,
) -> ArtifactDigest {
    let mut object = authorization_object("ExpansionAuthorization");
    object.insert(
        "base_promotion_candidate".into(),
        CanonicalValue::Digest(base.promotion_candidate()),
    );
    object.insert(
        "classified_candidate".into(),
        CanonicalValue::Digest(classified.structural_digest()),
    );
    object.insert(
        "class".into(),
        CanonicalValue::String(classified.class().as_str().into()),
    );
    object.insert(
        "parent_generation".into(),
        CanonicalValue::Digest(parent_generation),
    );
    object.insert(
        "activation_mode".into(),
        CanonicalValue::String(classified.requested_activation_mode().as_str().into()),
    );
    object.insert(
        "semantic_change_class".into(),
        CanonicalValue::String(classified.semantic_change_class().as_str().into()),
    );
    object.insert(
        "activation_effects".into(),
        digest_array(classified.activation_effects()),
    );
    object.insert(
        "grammar_effects".into(),
        digest_array(classified.grammar_effects()),
    );
    object.insert("scope".into(), digest_array(classified.scope()));
    object.insert(
        "registry_policy".into(),
        CanonicalValue::Digest(registry_policy),
    );
    object.insert(
        "registry".into(),
        CanonicalValue::Digest(PromotionClassRegistryV1::digest()),
    );
    object.insert(
        "metaprimitive_gate".into(),
        optional_digest(metaprimitive_gate),
    );
    CanonicalValue::Object(object).digest()
}

fn gate_authorization_digest(
    base: &PromotionAuthorization,
    classified: &ClassifiedPromotionCandidate,
    gate: &MetaprimitiveGateEvidence,
) -> ArtifactDigest {
    let mut object = authorization_object("MetaprimitiveGateAuthorization");
    object.insert(
        "base_promotion_candidate".into(),
        CanonicalValue::Digest(base.promotion_candidate()),
    );
    object.insert(
        "classified_candidate".into(),
        CanonicalValue::Digest(classified.structural_digest()),
    );
    object.insert(
        "gate_evidence".into(),
        CanonicalValue::Digest(gate.structural_digest()),
    );
    object.insert("scope".into(), digest_array(gate.scope()));
    CanonicalValue::Object(object).digest()
}

fn mode_is_automatic(mode: ActivationMode) -> bool {
    matches!(
        mode,
        ActivationMode::BoundedAutomatic | ActivationMode::DefaultAutomatic
    )
}

pub fn authorize_metaprimitive_gate_v1(
    base: &PromotionAuthorization,
    classified: &ClassifiedPromotionCandidate,
    gate: &MetaprimitiveGateEvidence,
) -> Result<MetaprimitiveGateAuthorization, ExpansionPolicyFailure> {
    if base.promotion_candidate() != classified.base_promotion_candidate() {
        return Err(ExpansionPolicyFailure::BasePromotionMismatch);
    }
    if classified.class() != PromotionClass::MetaprimitiveSearchMethod {
        return Err(ExpansionPolicyFailure::MetaprimitiveClassRequired);
    }

    for evidence in gate.required_evidence() {
        if base.authority_bindings().binary_search(&evidence).is_err() {
            return Err(ExpansionPolicyFailure::GateEvidenceNotPromotionBound(
                evidence,
            ));
        }
    }
    if classified
        .scope()
        .iter()
        .any(|scope| gate.scope().binary_search(scope).is_err())
    {
        return Err(ExpansionPolicyFailure::GateScopeMismatch);
    }

    let authorization_digest = gate_authorization_digest(base, classified, gate);
    Ok(MetaprimitiveGateAuthorization {
        base_promotion_candidate: base.promotion_candidate(),
        classified_candidate: classified.structural_digest(),
        gate_evidence: gate.structural_digest(),
        scope: gate.scope().to_vec(),
        authorization_digest,
    })
}

pub fn authorize_expansion_v1(
    base: &PromotionAuthorization,
    classified: &ClassifiedPromotionCandidate,
    parent: &UniverseGeneration,
    metaprimitive_gate: Option<&MetaprimitiveGateAuthorization>,
) -> Result<ExpansionAuthorization, ExpansionPolicyFailure> {
    if base.promotion_candidate() != classified.base_promotion_candidate() {
        return Err(ExpansionPolicyFailure::BasePromotionMismatch);
    }

    let parent_generation = parent.digest();
    if base.parent_generation() != parent_generation {
        return Err(ExpansionPolicyFailure::ParentGenerationMismatch);
    }

    let policy = PromotionClassRegistryV1::policy(classified.class());
    if !policy.allows_mode(classified.requested_activation_mode()) {
        return Err(ExpansionPolicyFailure::ActivationModeForbidden);
    }
    if !policy.may_change_capability_closure() && !classified.activation_effects().is_empty() {
        return Err(ExpansionPolicyFailure::CapabilityEffectForbidden);
    }
    if !policy.may_change_grammar() && !classified.grammar_effects().is_empty() {
        return Err(ExpansionPolicyFailure::GrammarEffectForbidden);
    }

    if classified.class() == PromotionClass::Realization
        && classified.semantic_change_class() == SemanticChangeClass::RealizationOnly
        && !base.proposed_admissions().is_empty()
    {
        return Err(ExpansionPolicyFailure::RealizationOnlySemanticAdmissionForbidden);
    }

    if classified.class() == PromotionClass::CounterexampleNogood
        && mode_is_automatic(classified.requested_activation_mode())
        && classified.scope().is_empty()
    {
        return Err(ExpansionPolicyFailure::NogoodScopeRequired);
    }

    let gate_digest = if classified.class() == PromotionClass::MetaprimitiveSearchMethod
        && mode_is_automatic(classified.requested_activation_mode())
    {
        let gate = metaprimitive_gate.ok_or(ExpansionPolicyFailure::MetaprimitiveGateRequired)?;
        if gate.base_promotion_candidate() != base.promotion_candidate()
            || gate.classified_candidate() != classified.structural_digest()
        {
            return Err(ExpansionPolicyFailure::GateAuthorizationMismatch);
        }
        Some(gate.authorization_digest())
    } else {
        None
    };

    let registry_policy = policy.structural_digest();
    let authorization_digest = expansion_authorization_digest(
        base,
        classified,
        parent_generation,
        registry_policy,
        gate_digest,
    );

    Ok(ExpansionAuthorization {
        base_promotion_candidate: base.promotion_candidate(),
        classified_candidate: classified.structural_digest(),
        class: classified.class(),
        parent_generation,
        activation_mode: classified.requested_activation_mode(),
        semantic_change_class: classified.semantic_change_class(),
        activation_effects: classified.activation_effects().to_vec(),
        grammar_effects: classified.grammar_effects().to_vec(),
        scope: classified.scope().to_vec(),
        registry_policy,
        registry: PromotionClassRegistryV1::digest(),
        metaprimitive_gate: gate_digest,
        authorization_digest,
    })
}
