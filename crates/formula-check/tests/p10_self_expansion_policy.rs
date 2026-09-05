use formula_check::{
    promotion::{PromotionAuthorization, PromotionDecision, authorize_promotion_v1},
    self_expansion::{
        ExpansionPolicyFailure, authorize_expansion_v1, authorize_metaprimitive_gate_v1,
    },
};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate, PromotionManifest},
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::PromotionCandidate,
    self_expansion::{
        ActivationMode, ClassifiedPromotionCandidate, MetaprimitiveGateEvidence, PromotionClass,
        SemanticChangeClass,
    },
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

struct Fixture {
    parent: UniverseGeneration,
    frozen: FrozenCandidate,
    manifest: PromotionManifest,
    candidate: PromotionCandidate,
    evidence: Vec<ArtifactDigest>,
}

fn fixture_with_evidence(evidence: Vec<ArtifactDigest>) -> Fixture {
    let dependency = d("p10-admitted-dependency");
    let superseded = d("p10-older-artifact");
    let promoted = d("p10-promoted-artifact");
    let parent = UniverseGeneration::new(0, None, vec![dependency, superseded], vec![]);
    let parent_digest = parent.digest();
    let frozen = FrozenCandidate::new(
        "p10-candidate".into(),
        vec![promoted],
        d("p10-world"),
        parent_digest,
        vec![dependency],
        vec![],
        d("p10-authority-contract"),
        d("p10-observer"),
    );
    let manifest = PromotionManifest::new(
        parent_digest,
        frozen.structural_digest(),
        evidence.clone(),
        vec![promoted],
        evidence.clone(),
    );
    let candidate = PromotionCandidate::new(
        frozen.structural_digest(),
        manifest.structural_digest(),
        parent_digest,
        parent_digest,
        vec![dependency],
        vec![superseded],
    );
    Fixture {
        parent,
        frozen,
        manifest,
        candidate,
        evidence,
    }
}

fn fixture() -> Fixture {
    fixture_with_evidence(vec![d("p10-checked-certificate")])
}

fn base_authorization(f: &Fixture) -> PromotionAuthorization {
    let decision = authorize_promotion_v1(
        &f.manifest,
        &f.frozen,
        &f.candidate,
        &f.evidence,
        &f.parent,
        &[],
    )
    .unwrap();
    let PromotionDecision::Authorized(authorization) = decision else {
        panic!("valid P10 fixture was quarantined")
    };
    authorization
}

#[test]
fn classified_authorization_must_reference_exact_base_promotion() {
    let f = fixture();
    let base = base_authorization(&f);
    let classified = ClassifiedPromotionCandidate::new(
        d("wrong-base-promotion"),
        PromotionClass::StructureWitness,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![d("capability-effect")],
        vec![],
        vec![],
    );

    assert_eq!(
        authorize_expansion_v1(&base, &classified, &f.parent, None),
        Err(ExpansionPolicyFailure::BasePromotionMismatch)
    );
}

#[test]
fn structure_witness_may_unlock_capability_but_cannot_directly_change_grammar() {
    let f = fixture();
    let base = base_authorization(&f);
    let accepted = ClassifiedPromotionCandidate::new(
        f.candidate.structural_digest(),
        PromotionClass::StructureWitness,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![d("cap:rational:field")],
        vec![],
        vec![],
    );
    let authorization = authorize_expansion_v1(&base, &accepted, &f.parent, None).unwrap();
    assert_eq!(authorization.class(), PromotionClass::StructureWitness);
    assert_eq!(authorization.parent_generation(), f.parent.digest());
    assert_eq!(
        authorization.classified_candidate(),
        accepted.structural_digest()
    );

    let forbidden = ClassifiedPromotionCandidate::new(
        f.candidate.structural_digest(),
        PromotionClass::StructureWitness,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![d("cap:rational:field")],
        vec![d("grammar-effect")],
        vec![],
    );
    assert_eq!(
        authorize_expansion_v1(&base, &forbidden, &f.parent, None),
        Err(ExpansionPolicyFailure::GrammarEffectForbidden)
    );
}

#[test]
fn automatic_nogood_requires_exact_nonempty_scope() {
    let f = fixture();
    let base = base_authorization(&f);
    let classified = ClassifiedPromotionCandidate::new(
        f.candidate.structural_digest(),
        PromotionClass::CounterexampleNogood,
        ActivationMode::BoundedAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![d("pruning-effect")],
        vec![d("nogood-rule")],
        vec![],
    );

    assert_eq!(
        authorize_expansion_v1(&base, &classified, &f.parent, None),
        Err(ExpansionPolicyFailure::NogoodScopeRequired)
    );
}

#[test]
fn metaprimitive_shadow_is_admissible_but_automatic_requires_strict_gate() {
    let f = fixture();
    let base = base_authorization(&f);
    let shadow = ClassifiedPromotionCandidate::new(
        f.candidate.structural_digest(),
        PromotionClass::MetaprimitiveSearchMethod,
        ActivationMode::ShadowOnly,
        SemanticChangeClass::ConservativeExtension,
        vec![],
        vec![d("metaprimitive-rule")],
        vec![d("bounded-domain")],
    );
    assert!(authorize_expansion_v1(&base, &shadow, &f.parent, None).is_ok());

    let automatic = ClassifiedPromotionCandidate::new(
        f.candidate.structural_digest(),
        PromotionClass::MetaprimitiveSearchMethod,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![],
        vec![d("metaprimitive-rule")],
        vec![d("bounded-domain")],
    );
    assert_eq!(
        authorize_expansion_v1(&base, &automatic, &f.parent, None),
        Err(ExpansionPolicyFailure::MetaprimitiveGateRequired)
    );
}

#[test]
fn strict_metaprimitive_gate_binds_all_checked_evidence_and_scope() {
    let evidence = (0..8)
        .map(|index| d(&format!("gate-evidence-{index}")))
        .collect::<Vec<_>>();
    let f = fixture_with_evidence(evidence.clone());
    let base = base_authorization(&f);
    let scope = d("bounded-domain");
    let automatic = ClassifiedPromotionCandidate::new(
        f.candidate.structural_digest(),
        PromotionClass::MetaprimitiveSearchMethod,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![],
        vec![d("metaprimitive-rule")],
        vec![scope],
    );
    let gate = MetaprimitiveGateEvidence::new(
        evidence[0],
        evidence[1],
        evidence[2],
        evidence[3],
        evidence[4],
        evidence[5],
        evidence[6],
        evidence[7],
        vec![scope],
    );
    let gate_authorization = authorize_metaprimitive_gate_v1(&base, &automatic, &gate).unwrap();
    assert_eq!(gate_authorization.scope(), &[scope]);
    assert_eq!(gate_authorization.gate_evidence(), gate.structural_digest());
    assert!(
        authorize_expansion_v1(&base, &automatic, &f.parent, Some(&gate_authorization)).is_ok()
    );
}
