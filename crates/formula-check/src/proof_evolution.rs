use formula_core::{
    artifacts::StructuralIdentity,
    canonical::CanonicalValue,
    digest::ArtifactDigest,
    self_expansion::{
        EvidenceFreshness, ProofRepairPlan, ProofTransportPlan, SemanticChange, SemanticChangeClass,
    },
};
use std::collections::BTreeMap;

const PROOF_EVOLUTION_SCHEMA_V1: &str = "formula-proof-evolution-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProofEvolutionKind {
    Transport,
    Repair,
}

impl ProofEvolutionKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Transport => "TRANSPORT",
            Self::Repair => "REPAIR",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProofEvolutionFailure {
    CheckerMismatch,
    SourceEvidenceMismatch,
    SourceTargetMismatch,
    DestinationTargetMismatch,
    CertifiedRelationMismatch,
    SemanticChangeMismatch,
    FreshnessNotTransportable,
    FreshnessNotRepairable,
    AffectedSliceMismatch,
    RepairObligationsRequired,
    AuthorizationMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofEvolutionAuthorization {
    kind: ProofEvolutionKind,
    plan_digest: ArtifactDigest,
    semantic_change: ArtifactDigest,
    source_evidence: ArtifactDigest,
    destination_target: ArtifactDigest,
    checker: ArtifactDigest,
    authorization_digest: ArtifactDigest,
}

impl ProofEvolutionAuthorization {
    pub fn plan_digest(&self) -> ArtifactDigest {
        self.plan_digest
    }

    pub fn semantic_change(&self) -> ArtifactDigest {
        self.semantic_change
    }

    pub fn source_evidence(&self) -> ArtifactDigest {
        self.source_evidence
    }

    pub fn destination_target(&self) -> ArtifactDigest {
        self.destination_target
    }

    pub fn checker(&self) -> ArtifactDigest {
        self.checker
    }

    pub fn authorization_digest(&self) -> ArtifactDigest {
        self.authorization_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransportedEvidenceRecord {
    source_evidence: ArtifactDigest,
    destination_target: ArtifactDigest,
    destination_dependencies: Vec<ArtifactDigest>,
    certified_relation: ArtifactDigest,
    checker: ArtifactDigest,
    authorization_digest: ArtifactDigest,
}

impl TransportedEvidenceRecord {
    pub fn source_evidence(&self) -> ArtifactDigest {
        self.source_evidence
    }

    pub fn destination_target(&self) -> ArtifactDigest {
        self.destination_target
    }

    pub fn destination_dependencies(&self) -> &[ArtifactDigest] {
        &self.destination_dependencies
    }

    pub fn certified_relation(&self) -> ArtifactDigest {
        self.certified_relation
    }

    pub fn checker(&self) -> ArtifactDigest {
        self.checker
    }

    pub fn authorization_digest(&self) -> ArtifactDigest {
        self.authorization_digest
    }
}

impl StructuralIdentity for TransportedEvidenceRecord {
    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "authorization_digest".into(),
                CanonicalValue::Digest(self.authorization_digest),
            ),
            (
                "certified_relation".into(),
                CanonicalValue::Digest(self.certified_relation),
            ),
            ("checker".into(), CanonicalValue::Digest(self.checker)),
            (
                "destination_dependencies".into(),
                digest_array(&self.destination_dependencies),
            ),
            (
                "destination_target".into(),
                CanonicalValue::Digest(self.destination_target),
            ),
            (
                "kind".into(),
                CanonicalValue::String("TransportedEvidenceRecord".into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(PROOF_EVOLUTION_SCHEMA_V1.into()),
            ),
            (
                "source_evidence".into(),
                CanonicalValue::Digest(self.source_evidence),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepairedEvidenceRecord {
    source_evidence: ArtifactDigest,
    semantic_change: ArtifactDigest,
    affected_slice: Vec<ArtifactDigest>,
    repair_obligations: Vec<ArtifactDigest>,
    checker: ArtifactDigest,
    authorization_digest: ArtifactDigest,
}

impl RepairedEvidenceRecord {
    pub fn source_evidence(&self) -> ArtifactDigest {
        self.source_evidence
    }

    pub fn semantic_change(&self) -> ArtifactDigest {
        self.semantic_change
    }

    pub fn affected_slice(&self) -> &[ArtifactDigest] {
        &self.affected_slice
    }

    pub fn repair_obligations(&self) -> &[ArtifactDigest] {
        &self.repair_obligations
    }

    pub fn checker(&self) -> ArtifactDigest {
        self.checker
    }

    pub fn authorization_digest(&self) -> ArtifactDigest {
        self.authorization_digest
    }
}

impl StructuralIdentity for RepairedEvidenceRecord {
    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            ("affected_slice".into(), digest_array(&self.affected_slice)),
            (
                "authorization_digest".into(),
                CanonicalValue::Digest(self.authorization_digest),
            ),
            ("checker".into(), CanonicalValue::Digest(self.checker)),
            (
                "kind".into(),
                CanonicalValue::String("RepairedEvidenceRecord".into()),
            ),
            (
                "repair_obligations".into(),
                digest_array(&self.repair_obligations),
            ),
            (
                "schema".into(),
                CanonicalValue::String(PROOF_EVOLUTION_SCHEMA_V1.into()),
            ),
            (
                "semantic_change".into(),
                CanonicalValue::Digest(self.semantic_change),
            ),
            (
                "source_evidence".into(),
                CanonicalValue::Digest(self.source_evidence),
            ),
        ]))
    }
}

pub fn classify_freshness(
    change: &SemanticChange,
    evidence_dependencies: &[ArtifactDigest],
    certified_relation: Option<ArtifactDigest>,
) -> EvidenceFreshness {
    let intersects = evidence_dependencies.iter().any(|dependency| {
        change
            .affected_authority_cone()
            .binary_search(dependency)
            .is_ok()
            || change
                .changed_dependencies()
                .binary_search(dependency)
                .is_ok()
    });

    if !intersects {
        return EvidenceFreshness::UnchangedFresh;
    }

    if change.class() == SemanticChangeClass::RealizationOnly
        && change.old_artifact() == change.new_artifact()
    {
        return EvidenceFreshness::UnchangedFresh;
    }

    let certified = certified_relation
        .is_some_and(|relation| change.evidence().binary_search(&relation).is_ok());

    match change.class() {
        SemanticChangeClass::RealizationOnly => EvidenceFreshness::RecheckRequired,
        SemanticChangeClass::DefinitionalEquivalent
        | SemanticChangeClass::ConservativeExtension => {
            if certified {
                EvidenceFreshness::Transportable
            } else {
                EvidenceFreshness::RecheckRequired
            }
        }
        SemanticChangeClass::TheoremStrengthening | SemanticChangeClass::AssumptionWeakening => {
            EvidenceFreshness::Repairable
        }
        SemanticChangeClass::SignatureChange => {
            if certified {
                EvidenceFreshness::Transportable
            } else {
                EvidenceFreshness::ReproveRequired
            }
        }
        SemanticChangeClass::NonConservativeChange => EvidenceFreshness::ReproveRequired,
        SemanticChangeClass::AuthorityPolicyChange => EvidenceFreshness::Quarantined,
    }
}

pub fn authorize_transport_v1(
    change: &SemanticChange,
    plan: &ProofTransportPlan,
    checker: ArtifactDigest,
    checked_source_evidence: ArtifactDigest,
    evidence_dependencies: &[ArtifactDigest],
) -> Result<ProofEvolutionAuthorization, ProofEvolutionFailure> {
    if checker != plan.required_checker() {
        return Err(ProofEvolutionFailure::CheckerMismatch);
    }
    if checked_source_evidence != plan.source_evidence() {
        return Err(ProofEvolutionFailure::SourceEvidenceMismatch);
    }
    if plan.source_target() != change.old_artifact() {
        return Err(ProofEvolutionFailure::SourceTargetMismatch);
    }
    if plan.destination_target() != change.new_artifact() {
        return Err(ProofEvolutionFailure::DestinationTargetMismatch);
    }
    if change
        .evidence()
        .binary_search(&plan.certified_relation())
        .is_err()
    {
        return Err(ProofEvolutionFailure::CertifiedRelationMismatch);
    }
    if classify_freshness(
        change,
        evidence_dependencies,
        Some(plan.certified_relation()),
    ) != EvidenceFreshness::Transportable
    {
        return Err(ProofEvolutionFailure::FreshnessNotTransportable);
    }

    Ok(build_authorization(
        ProofEvolutionKind::Transport,
        plan.structural_digest(),
        change.structural_digest(),
        checked_source_evidence,
        plan.destination_target(),
        checker,
    ))
}

pub fn authorize_repair_v1(
    change: &SemanticChange,
    plan: &ProofRepairPlan,
    checker: ArtifactDigest,
    checked_source_evidence: ArtifactDigest,
    evidence_dependencies: &[ArtifactDigest],
) -> Result<ProofEvolutionAuthorization, ProofEvolutionFailure> {
    if checker != plan.required_checker() {
        return Err(ProofEvolutionFailure::CheckerMismatch);
    }
    if checked_source_evidence != plan.source_evidence() {
        return Err(ProofEvolutionFailure::SourceEvidenceMismatch);
    }
    if plan.semantic_change() != change.structural_digest() {
        return Err(ProofEvolutionFailure::SemanticChangeMismatch);
    }
    if plan.repair_obligations().is_empty() {
        return Err(ProofEvolutionFailure::RepairObligationsRequired);
    }
    if plan.affected_slice().iter().any(|dependency| {
        change
            .affected_authority_cone()
            .binary_search(dependency)
            .is_err()
    }) {
        return Err(ProofEvolutionFailure::AffectedSliceMismatch);
    }
    if classify_freshness(change, evidence_dependencies, None) != EvidenceFreshness::Repairable {
        return Err(ProofEvolutionFailure::FreshnessNotRepairable);
    }

    Ok(build_authorization(
        ProofEvolutionKind::Repair,
        plan.structural_digest(),
        change.structural_digest(),
        checked_source_evidence,
        change.new_artifact(),
        checker,
    ))
}

pub fn transport_evidence_v1(
    authorization: &ProofEvolutionAuthorization,
    plan: &ProofTransportPlan,
) -> Result<TransportedEvidenceRecord, ProofEvolutionFailure> {
    if authorization.kind != ProofEvolutionKind::Transport
        || authorization.plan_digest != plan.structural_digest()
        || authorization.source_evidence != plan.source_evidence()
        || authorization.destination_target != plan.destination_target()
        || authorization.checker != plan.required_checker()
    {
        return Err(ProofEvolutionFailure::AuthorizationMismatch);
    }

    Ok(TransportedEvidenceRecord {
        source_evidence: plan.source_evidence(),
        destination_target: plan.destination_target(),
        destination_dependencies: plan.destination_dependencies().to_vec(),
        certified_relation: plan.certified_relation(),
        checker: plan.required_checker(),
        authorization_digest: authorization.authorization_digest,
    })
}

pub fn repair_evidence_v1(
    authorization: &ProofEvolutionAuthorization,
    plan: &ProofRepairPlan,
) -> Result<RepairedEvidenceRecord, ProofEvolutionFailure> {
    if authorization.kind != ProofEvolutionKind::Repair
        || authorization.plan_digest != plan.structural_digest()
        || authorization.source_evidence != plan.source_evidence()
        || authorization.semantic_change != plan.semantic_change()
        || authorization.checker != plan.required_checker()
    {
        return Err(ProofEvolutionFailure::AuthorizationMismatch);
    }

    Ok(RepairedEvidenceRecord {
        source_evidence: plan.source_evidence(),
        semantic_change: plan.semantic_change(),
        affected_slice: plan.affected_slice().to_vec(),
        repair_obligations: plan.repair_obligations().to_vec(),
        checker: plan.required_checker(),
        authorization_digest: authorization.authorization_digest,
    })
}

fn build_authorization(
    kind: ProofEvolutionKind,
    plan_digest: ArtifactDigest,
    semantic_change: ArtifactDigest,
    source_evidence: ArtifactDigest,
    destination_target: ArtifactDigest,
    checker: ArtifactDigest,
) -> ProofEvolutionAuthorization {
    let mut object = BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.as_str().into())),
        ("plan_digest".into(), CanonicalValue::Digest(plan_digest)),
        (
            "semantic_change".into(),
            CanonicalValue::Digest(semantic_change),
        ),
        (
            "source_evidence".into(),
            CanonicalValue::Digest(source_evidence),
        ),
        (
            "destination_target".into(),
            CanonicalValue::Digest(destination_target),
        ),
        ("checker".into(), CanonicalValue::Digest(checker)),
    ]);
    object.insert(
        "schema".into(),
        CanonicalValue::String(PROOF_EVOLUTION_SCHEMA_V1.into()),
    );
    let authorization_digest = CanonicalValue::Object(object).digest();

    ProofEvolutionAuthorization {
        kind,
        plan_digest,
        semantic_change,
        source_evidence,
        destination_target,
        checker,
        authorization_digest,
    }
}

fn digest_array(values: &[ArtifactDigest]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().copied().map(CanonicalValue::Digest).collect())
}
