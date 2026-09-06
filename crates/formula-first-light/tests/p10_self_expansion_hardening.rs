use formula_check::{
    proof_evolution::{
        ProofEvolutionFailure, authorize_repair_v1, authorize_transport_v1, classify_freshness,
        repair_evidence_v1, transport_evidence_v1,
    },
    promotion::{PromotionDecision, authorize_promotion_v1},
    realization::authorize_native_u8_realization_v1,
    self_expansion::{ExpansionPolicyFailure, authorize_expansion_v1},
    self_expansion_verifier::{
        P10_CANONICAL_MARKERS, SelfExpansionReplayClaims, SelfExpansionReplayEvidence,
        verify_self_expansion_manifest,
    },
    u8::{BoolExpr, ByteExpr},
};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate, PromotionManifest, RealizationCheckManifest},
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::PromotionCandidate,
    realization::{
        NativeRealizationManifest, NativeToolchainIdentity, RealizationDispatchContext,
        SpecializationIdentity,
    },
    self_expansion::{
        ActivationMode, ClassifiedPromotionCandidate, EvidenceFreshness, ExpansionActivationRecord,
        PromotionClass, PromotionClassRegistryV1, ProofRepairPlan, ProofTransportPlan,
        RealizationUpgrade, SemanticChange, SemanticChangeClass,
    },
    self_expansion_proof::{
        SelfExpansionNegativeControl, SelfExpansionNegativeControlEvidence,
        SelfExpansionNegativeControlManifest, SelfExpansionProofManifest,
    },
    theory::{ClosureContext, StructureWitness},
};
use formula_engine::{
    candidate_space::CandidateSpaceContext,
    self_expansion::{
        GrammarBindingError, bind_candidate_context_to_grammar, validate_candidate_context_grammar,
    },
};
use formula_first_light::p10::{
    P9_FROZEN_PROOF_HEAD, checker_identity, p9_frozen_proof_identity, source_commit,
    verifier_identity,
};
use formula_packages::{
    activation::validate_activation,
    builtin::{integer_package, rational_package},
    closure::{AdmittedStructureWitness, CapabilityClosure, CapabilityClosureDelta, derive_capabilities},
    expansion::{
        ExpansionError, PromotedRouteActivation, ScopedNogoodActivation, active_routes,
        applicable_nogoods,
    },
    grammar::derive_grammar_generation,
};
use formula_store::authority_store::{AuthorityStore, RealizationUpgradeError};
use tempfile::tempdir;

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn digest_parts(label: &str, parts: &[ArtifactDigest]) -> ArtifactDigest {
    let mut bytes = label.as_bytes().to_vec();
    for part in parts {
        bytes.push(0);
        bytes.extend_from_slice(part.as_str().as_bytes());
    }
    ArtifactDigest::of_bytes(&bytes)
}

fn closure_digest(label: &str, closure: &CapabilityClosure) -> ArtifactDigest {
    let mut parts = vec![closure.context_digest()];
    parts.extend(closure.capabilities());
    digest_parts(label, &parts)
}

fn closure_delta_digest(delta: &CapabilityClosureDelta) -> ArtifactDigest {
    let mut parts = vec![delta.before_context_digest(), delta.after_context_digest()];
    parts.extend(delta.added());
    parts.extend(delta.removed());
    digest_parts("p10:closure-delta", &parts)
}

fn nc(
    control: SelfExpansionNegativeControl,
    label: &str,
    parts: &[ArtifactDigest],
) -> SelfExpansionNegativeControlEvidence {
    SelfExpansionNegativeControlEvidence::new(control, digest_parts(label, parts))
}

fn semantic() -> BoolExpr {
    BoolExpr::And(
        Box::new(BoolExpr::NeqZero(ByteExpr::X)),
        Box::new(BoolExpr::EqZero(ByteExpr::BitAnd(
            Box::new(ByteExpr::X),
            Box::new(ByteExpr::SubWrap(
                Box::new(ByteExpr::X),
                Box::new(ByteExpr::Const(1)),
            )),
        ))),
    )
}

fn realization_authorization(
    semantic_target: ArtifactDigest,
    generation: ArtifactDigest,
    world: ArtifactDigest,
    authority: ArtifactDigest,
    observer: ArtifactDigest,
    binary: &[u8],
) -> formula_check::realization::RealizationAuthorization {
    let source = b"fn main() { /* canonical P10 realization variant */ }\n";
    let specialization =
        SpecializationIdentity::new(semantic_target, generation, world, authority, observer);
    let toolchain =
        NativeToolchainIdentity::new("1.98.0".into(), "x86_64-unknown-linux-gnu".into());
    let native_manifest = NativeRealizationManifest::new(
        semantic_target,
        generation,
        world,
        authority,
        observer,
        specialization.structural_digest(),
        ArtifactDigest::of_bytes(source),
        toolchain.structural_digest(),
        ArtifactDigest::of_bytes(binary),
    );
    let check_manifest = RealizationCheckManifest::new(
        semantic_target,
        native_manifest.structural_digest(),
        generation,
        world,
        authority,
        observer,
        ArtifactDigest::of_bytes(binary),
    );
    let semantic = semantic();
    let outputs = (0u16..=255)
        .map(|raw| semantic.evaluate(raw as u8))
        .collect::<Vec<_>>();

    authorize_native_u8_realization_v1(
        &native_manifest,
        &specialization,
        &toolchain,
        &check_manifest,
        source,
        binary,
        &semantic,
        &outputs,
    )
    .unwrap()
}

fn dispatch_context(
    target: ArtifactDigest,
    generation: ArtifactDigest,
    world: ArtifactDigest,
    authority: ArtifactDigest,
    observer: ArtifactDigest,
) -> RealizationDispatchContext {
    RealizationDispatchContext::new(target, generation, world, authority, observer)
}

#[test]
fn p10_self_expansion_hardening() {
    assert_eq!(
        p9_frozen_proof_identity(),
        ArtifactDigest::of_bytes(P9_FROZEN_PROOF_HEAD.as_bytes())
    );
    assert_eq!(
        P9_FROZEN_PROOF_HEAD,
        "b353365fa8b20a13b658c07b3027334b69eff108"
    );
    assert_eq!(
        PromotionClassRegistryV1::policies().len(),
        PromotionClass::ALL.len()
    );

    let foundation = d("p10:rational:foundation");
    let world = d("p10:canonical:world");
    let field_goal = d("goal:rational:field");
    let field_cap = d("cap:rational:field");
    let witness_evidence = d("p10:rational:field:evidence");
    let nogood_subject = d("p10:canonical:nogood");
    let nogood_evidence = d("p10:canonical:nogood:evidence");
    let nogood_scope = d("p10:canonical:nogood:scope");
    let route_subject = d("p10:canonical:route");
    let route_evidence = d("p10:canonical:route:evidence");
    let route_scope = d("p10:canonical:route:scope");
    let route_result_class = d("p10:canonical:result-class:exact-witness");
    let metaprimitive = d("p10:canonical:metaprimitive");
    let metaprimitive_evidence = d("p10:canonical:metaprimitive:evidence");
    let metaprimitive_scope = d("p10:canonical:metaprimitive:scope");
    let realization_target = d("p10:canonical:realization-target");
    let realization_evidence = d("p10:canonical:realization:evidence");
    let realization_authority = d("p10:canonical:realization:authority");
    let realization_observer = d("p10:canonical:realization:observer");

    let integer = integer_package(foundation);
    let rational_before = rational_package(foundation);
    let integer_digest = integer.structural_digest();
    let rational_digest = rational_before.structural_digest();
    let packages = vec![integer.clone(), rational_before.clone()];
    let package_digests = vec![integer_digest, rational_digest];

    let u_g = UniverseGeneration::new(
        0,
        None,
        vec![
            integer_digest,
            rational_digest,
            nogood_subject,
            route_subject,
            metaprimitive,
            realization_target,
        ],
        vec![
            nogood_evidence,
            route_evidence,
            metaprimitive_evidence,
            realization_evidence,
        ],
    );
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u_g_digest = store.initialize_genesis(&u_g).unwrap();
    assert_eq!(u_g_digest, u_g.digest());

    let active_g = validate_activation(&u_g, &packages, &[], &package_digests).unwrap();
    let context_g = ClosureContext::new(
        u_g_digest,
        world,
        package_digests.clone(),
        d("p10:canonical:rules"),
        d("p10:canonical:policy"),
    );
    let closure_before = derive_capabilities(&context_g, &active_g, &[], &packages).unwrap();
    assert!(!closure_before.contains(field_cap));
    let closure_before_id = closure_digest("p10:closure-before", &closure_before);

    let witness = StructureWitness::new(world, field_goal, witness_evidence);
    let witness_digest = witness.structural_digest();
    let frozen = FrozenCandidate::new(
        "p10-rational-field-structure-witness".into(),
        vec![witness_digest],
        world,
        u_g_digest,
        vec![],
        vec![],
        d("p10:canonical:authority-contract"),
        d("p10:canonical:observer"),
    );
    let promotion_manifest = PromotionManifest::new(
        u_g_digest,
        frozen.structural_digest(),
        vec![witness_evidence],
        vec![witness_digest],
        vec![witness_evidence],
    );
    let promotion = PromotionCandidate::new(
        frozen.structural_digest(),
        promotion_manifest.structural_digest(),
        u_g_digest,
        u_g_digest,
        vec![],
        vec![],
    );
    let decision = authorize_promotion_v1(
        &promotion_manifest,
        &frozen,
        &promotion,
        &[witness_evidence],
        &u_g,
        &[],
    )
    .unwrap();
    let PromotionDecision::Authorized(base_authorization) = decision else {
        panic!("canonical P10 structure witness was quarantined")
    };

    let classified = ClassifiedPromotionCandidate::new(
        promotion.structural_digest(),
        PromotionClass::StructureWitness,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![field_cap],
        vec![],
        vec![field_goal],
    );
    let expansion_authorization =
        authorize_expansion_v1(&base_authorization, &classified, &u_g, None).unwrap();
    assert_eq!(
        expansion_authorization.class(),
        PromotionClass::StructureWitness
    );
    assert_eq!(expansion_authorization.activation_effects(), &[field_cap]);
    assert!(expansion_authorization.grammar_effects().is_empty());

    let outcome = store.promote(&base_authorization).unwrap();
    let u_g1 = store.replay_generation(outcome.new_generation()).unwrap();
    let u_g1_digest = u_g1.digest();
    assert_eq!(u_g1.parent(), Some(u_g_digest));
    assert_ne!(u_g_digest, u_g1_digest);

    let active_g1 = validate_activation(&u_g1, &packages, &[], &package_digests).unwrap();
    let admitted_witness = AdmittedStructureWitness::new(&u_g1, witness.clone()).unwrap();
    let context_g1 = ClosureContext::new(
        u_g1_digest,
        world,
        package_digests.clone(),
        d("p10:canonical:rules"),
        d("p10:canonical:policy"),
    );
    let closure_after = derive_capabilities(
        &context_g1,
        &active_g1,
        std::slice::from_ref(&admitted_witness),
        &packages,
    )
    .unwrap();
    let closure_after_id = closure_digest("p10:closure-after", &closure_after);
    let closure_delta = CapabilityClosureDelta::between(&closure_before, &closure_after);
    let closure_delta_id = closure_delta_digest(&closure_delta);
    assert!(closure_after.contains(field_cap));
    assert_eq!(closure_delta.added().collect::<Vec<_>>(), vec![field_cap]);
    assert!(closure_delta.removed().next().is_none());

    let rational_after = rational_package(foundation);
    assert_eq!(
        rational_before.structural_digest(),
        rational_after.structural_digest()
    );
    assert_eq!(rational_after.structural_digest(), rational_digest);

    let lambda_g = derive_grammar_generation(
        &u_g,
        None,
        &[],
        &[],
        &[d("p10:canonical:theory-rule")],
    )
    .unwrap();
    let nogood_record = ExpansionActivationRecord::new(
        nogood_subject,
        PromotionClass::CounterexampleNogood,
        u_g1_digest,
        world,
        ActivationMode::BoundedAutomatic,
        vec![nogood_evidence],
        vec![nogood_scope],
    );
    let nogood_activation = ScopedNogoodActivation::new(&u_g1, &nogood_record).unwrap();
    assert!(applicable_nogoods(&[], std::slice::from_ref(&nogood_activation)).is_empty());
    assert_eq!(
        applicable_nogoods(&[nogood_scope], std::slice::from_ref(&nogood_activation)),
        vec![nogood_subject]
    );
    let nogood_proof = digest_parts(
        "p10:nogood-proof",
        &[nogood_record.structural_digest(), nogood_scope],
    );

    let route_record = ExpansionActivationRecord::new(
        route_subject,
        PromotionClass::Reduction,
        u_g1_digest,
        world,
        ActivationMode::DefaultAutomatic,
        vec![route_evidence],
        vec![route_scope],
    );
    let route_activation = PromotedRouteActivation::new(
        &u_g1,
        &route_record,
        vec![route_result_class],
    )
    .unwrap();
    assert_eq!(active_routes(std::slice::from_ref(&route_activation)), vec![route_subject]);
    let route_proof = digest_parts(
        "p10:route-proof",
        &[
            route_record.structural_digest(),
            route_result_class,
            route_evidence,
        ],
    );

    let shadow_record = ExpansionActivationRecord::new(
        metaprimitive,
        PromotionClass::MetaprimitiveSearchMethod,
        u_g1_digest,
        world,
        ActivationMode::ShadowOnly,
        vec![metaprimitive_evidence],
        vec![metaprimitive_scope],
    );
    let lambda_g1 = derive_grammar_generation(
        &u_g1,
        Some(lambda_g.structural_digest()),
        std::slice::from_ref(&shadow_record),
        std::slice::from_ref(&route_activation),
        &[d("p10:canonical:theory-rule")],
    )
    .unwrap();
    assert!(lambda_g1.activated_metaprimitives().is_empty());
    assert_eq!(lambda_g1.shadow_metaprimitives(), &[metaprimitive]);
    assert_eq!(lambda_g1.activated_route_rules(), &[route_subject]);
    assert_ne!(lambda_g.structural_digest(), lambda_g1.structural_digest());

    let base_context_g = CandidateSpaceContext::new(
        u_g_digest,
        world,
        d("p10:canonical:query:g"),
        d("p10:canonical:obligation"),
        d("p10:legacy-grammar"),
        d("p10:canonical:search-policy"),
    );
    let bound_g = bind_candidate_context_to_grammar(base_context_g, &lambda_g).unwrap();
    validate_candidate_context_grammar(bound_g.context(), &lambda_g).unwrap();
    let base_context_g1 = CandidateSpaceContext::new(
        u_g1_digest,
        world,
        d("p10:canonical:query:g1"),
        d("p10:canonical:obligation"),
        d("p10:legacy-grammar"),
        d("p10:canonical:search-policy"),
    );
    let bound_g1 = bind_candidate_context_to_grammar(base_context_g1, &lambda_g1).unwrap();
    validate_candidate_context_grammar(bound_g1.context(), &lambda_g1).unwrap();
    assert_ne!(bound_g.context().digest(), bound_g1.context().digest());

    let proof_checker = checker_identity();
    let transport_source = d("p10:proof:source-evidence");
    let transport_old = d("p10:proof:old-target");
    let transport_new = d("p10:proof:new-target");
    let transport_dep = d("p10:proof:dependency");
    let transport_relation = d("p10:proof:definitional-relation");
    let semantic_change = SemanticChange::new(
        transport_old,
        transport_new,
        SemanticChangeClass::DefinitionalEquivalent,
        vec![transport_dep],
        vec![transport_dep],
        vec![transport_relation],
    );
    assert_eq!(
        classify_freshness(
            &semantic_change,
            &[transport_dep],
            Some(transport_relation)
        ),
        EvidenceFreshness::Transportable
    );
    let transport_plan = ProofTransportPlan::new(
        transport_source,
        transport_old,
        transport_new,
        transport_relation,
        vec![transport_dep],
        proof_checker,
    );
    let transport_auth = authorize_transport_v1(
        &semantic_change,
        &transport_plan,
        proof_checker,
        transport_source,
        &[transport_dep],
    )
    .unwrap();
    let transported = transport_evidence_v1(&transport_auth, &transport_plan).unwrap();

    let repair_source = d("p10:proof:repair-source");
    let repair_dep = d("p10:proof:repair-dependency");
    let repair_obligation = d("p10:proof:repair-obligation");
    let repair_change = SemanticChange::new(
        d("p10:proof:repair-old"),
        d("p10:proof:repair-new"),
        SemanticChangeClass::TheoremStrengthening,
        vec![repair_dep],
        vec![repair_dep],
        vec![],
    );
    assert_eq!(
        classify_freshness(&repair_change, &[repair_dep], None),
        EvidenceFreshness::Repairable
    );
    let repair_plan = ProofRepairPlan::new(
        repair_source,
        repair_change.structural_digest(),
        vec![repair_dep],
        vec![repair_obligation],
        proof_checker,
    );
    let repair_auth = authorize_repair_v1(
        &repair_change,
        &repair_plan,
        proof_checker,
        repair_source,
        &[repair_dep],
    )
    .unwrap();
    let repaired = repair_evidence_v1(&repair_auth, &repair_plan).unwrap();
    let proof_evolution = digest_parts(
        "p10:proof-evolution",
        &[transported.structural_digest(), repaired.structural_digest()],
    );

    let r1_auth = realization_authorization(
        realization_target,
        u_g1_digest,
        world,
        realization_authority,
        realization_observer,
        b"p10-canonical-native-r1",
    );
    let r2_auth = realization_authorization(
        realization_target,
        u_g1_digest,
        world,
        realization_authority,
        realization_observer,
        b"p10-canonical-native-r2-faster",
    );
    let r1 = store
        .admit_realization(&r1_auth, b"p10-canonical-native-r1")
        .unwrap();
    let r2 = store
        .admit_realization(&r2_auth, b"p10-canonical-native-r2-faster")
        .unwrap();
    let dispatch = dispatch_context(
        realization_target,
        u_g1_digest,
        world,
        realization_authority,
        realization_observer,
    );
    let semantic_generation_before_upgrade = store.active_generation().unwrap().unwrap();
    let upgrade = RealizationUpgrade::new(
        realization_target,
        u_g1_digest,
        r1.manifest_digest(),
        r2.manifest_digest(),
        SemanticChangeClass::RealizationOnly,
        vec![realization_evidence],
        d("p10:canonical:realization-selection-policy"),
    );
    store.record_realization_upgrade(&upgrade).unwrap();
    assert_eq!(
        store
            .preferred_realization(&dispatch)
            .unwrap()
            .unwrap()
            .manifest_digest(),
        r2.manifest_digest()
    );
    assert_eq!(
        store.active_generation().unwrap(),
        Some(semantic_generation_before_upgrade)
    );

    store
        .select_realization(&dispatch, r1.manifest_digest())
        .unwrap();
    assert_eq!(
        store
            .preferred_realization(&dispatch)
            .unwrap()
            .unwrap()
            .manifest_digest(),
        r1.manifest_digest()
    );
    assert!(
        store
            .realization_by_manifest(r2.manifest_digest())
            .unwrap()
            .is_some()
    );
    assert_eq!(
        store.active_generation().unwrap(),
        Some(semantic_generation_before_upgrade)
    );
    let realization_rollback = digest_parts(
        "p10:realization-rollback",
        &[r1.manifest_digest(), r2.manifest_digest(), u_g1_digest],
    );

    let wrong_base = ClassifiedPromotionCandidate::new(
        d("p10:wrong-base-promotion"),
        PromotionClass::StructureWitness,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![field_cap],
        vec![],
        vec![field_goal],
    );
    assert_eq!(
        authorize_expansion_v1(&base_authorization, &wrong_base, &u_g, None),
        Err(ExpansionPolicyFailure::BasePromotionMismatch)
    );
    let nc01 = nc(
        SelfExpansionNegativeControl::WrongBasePromotion,
        "p10:nc01:base-mismatch",
        &[wrong_base.structural_digest(), promotion.structural_digest()],
    );

    let forbidden_effect = ClassifiedPromotionCandidate::new(
        promotion.structural_digest(),
        PromotionClass::StructureWitness,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![field_cap],
        vec![d("p10:forbidden-grammar-effect")],
        vec![field_goal],
    );
    assert_eq!(
        authorize_expansion_v1(&base_authorization, &forbidden_effect, &u_g, None),
        Err(ExpansionPolicyFailure::GrammarEffectForbidden)
    );
    let nc02 = nc(
        SelfExpansionNegativeControl::ForbiddenClassEffect,
        "p10:nc02:forbidden-class-effect",
        &[forbidden_effect.structural_digest()],
    );

    let unadmitted_witness = StructureWitness::new(world, d("p10:unadmitted-goal"), d("p10:unadmitted-evidence"));
    assert!(AdmittedStructureWitness::new(&u_g1, unadmitted_witness.clone()).is_err());
    let nc03 = nc(
        SelfExpansionNegativeControl::UnadmittedStructureWitness,
        "p10:nc03:unadmitted-witness",
        &[unadmitted_witness.structural_digest(), u_g1_digest],
    );

    let unbound_witness = StructureWitness::new(world, d("p10:unbound-goal"), d("p10:unbound-evidence"));
    let unbound_generation = UniverseGeneration::new(
        99,
        Some(u_g1_digest),
        vec![unbound_witness.structural_digest()],
        vec![],
    );
    assert!(AdmittedStructureWitness::new(&unbound_generation, unbound_witness.clone()).is_err());
    let nc04 = nc(
        SelfExpansionNegativeControl::UnboundStructureEvidence,
        "p10:nc04:unbound-witness-evidence",
        &[unbound_witness.structural_digest(), unbound_generation.digest()],
    );

    let unscoped_nogood = ExpansionActivationRecord::new(
        nogood_subject,
        PromotionClass::CounterexampleNogood,
        u_g1_digest,
        world,
        ActivationMode::BoundedAutomatic,
        vec![nogood_evidence],
        vec![],
    );
    assert_eq!(
        ScopedNogoodActivation::new(&u_g1, &unscoped_nogood),
        Err(ExpansionError::ScopeRequired)
    );
    let nc05 = nc(
        SelfExpansionNegativeControl::UnscopedAutomaticNogood,
        "p10:nc05:unscoped-nogood",
        &[unscoped_nogood.structural_digest()],
    );

    assert_eq!(
        PromotedRouteActivation::new(&u_g1, &route_record, vec![]),
        Err(ExpansionError::PreservationEvidenceRequired)
    );
    let nc06 = nc(
        SelfExpansionNegativeControl::RouteMissingPreservationEvidence,
        "p10:nc06:route-preservation-required",
        &[route_record.structural_digest()],
    );

    let mismatch_context = CandidateSpaceContext::new(
        u_g1_digest,
        world,
        d("p10:nc07:query"),
        d("p10:nc07:obligation"),
        d("p10:nc07:legacy-grammar"),
        d("p10:nc07:policy"),
    );
    assert_eq!(
        bind_candidate_context_to_grammar(mismatch_context, &lambda_g),
        Err(GrammarBindingError::GenerationMismatch)
    );
    let nc07 = nc(
        SelfExpansionNegativeControl::GrammarGenerationMismatch,
        "p10:nc07:grammar-generation-mismatch",
        &[u_g1_digest, lambda_g.structural_digest()],
    );

    let ungated_metaprimitive = ClassifiedPromotionCandidate::new(
        promotion.structural_digest(),
        PromotionClass::MetaprimitiveSearchMethod,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![],
        vec![metaprimitive],
        vec![metaprimitive_scope],
    );
    assert_eq!(
        authorize_expansion_v1(
            &base_authorization,
            &ungated_metaprimitive,
            &u_g,
            None
        ),
        Err(ExpansionPolicyFailure::MetaprimitiveGateRequired)
    );
    let nc08 = nc(
        SelfExpansionNegativeControl::UngatedAutomaticMetaprimitive,
        "p10:nc08:ungated-metaprimitive",
        &[ungated_metaprimitive.structural_digest()],
    );

    let nonconservative_relation = d("p10:nc09:relation");
    let nonconservative_dep = d("p10:nc09:dependency");
    let nonconservative = SemanticChange::new(
        d("p10:nc09:old"),
        d("p10:nc09:new"),
        SemanticChangeClass::NonConservativeChange,
        vec![nonconservative_dep],
        vec![nonconservative_dep],
        vec![nonconservative_relation],
    );
    assert_eq!(
        classify_freshness(
            &nonconservative,
            &[nonconservative_dep],
            Some(nonconservative_relation)
        ),
        EvidenceFreshness::ReproveRequired
    );
    let nc09 = nc(
        SelfExpansionNegativeControl::NonConservativeSilentTransport,
        "p10:nc09:nonconservative-reprove",
        &[nonconservative.structural_digest()],
    );

    assert_eq!(
        authorize_transport_v1(
            &semantic_change,
            &transport_plan,
            d("p10:nc10:wrong-checker"),
            transport_source,
            &[transport_dep]
        ),
        Err(ProofEvolutionFailure::CheckerMismatch)
    );
    let wrong_repair_plan = ProofRepairPlan::new(
        repair_source,
        d("p10:nc10:wrong-change"),
        vec![repair_dep],
        vec![repair_obligation],
        proof_checker,
    );
    assert_eq!(
        repair_evidence_v1(&repair_auth, &wrong_repair_plan),
        Err(ProofEvolutionFailure::AuthorizationMismatch)
    );
    let nc10 = nc(
        SelfExpansionNegativeControl::UnauthorizedProofRepairOrTransport,
        "p10:nc10:checker-authorization-required",
        &[
            transport_plan.structural_digest(),
            wrong_repair_plan.structural_digest(),
        ],
    );

    let semantic_smuggle = RealizationUpgrade::new(
        d("p10:nc11:not-admitted-semantic"),
        u_g1_digest,
        r1.manifest_digest(),
        r2.manifest_digest(),
        SemanticChangeClass::RealizationOnly,
        vec![realization_evidence],
        d("p10:canonical:realization-selection-policy"),
    );
    assert!(matches!(
        store.record_realization_upgrade(&semantic_smuggle),
        Err(RealizationUpgradeError::SemanticNotAdmitted(_))
    ));
    let nc11 = nc(
        SelfExpansionNegativeControl::RealizationUpgradeSemanticAdmission,
        "p10:nc11:realization-semantic-smuggle",
        &[semantic_smuggle.structural_digest()],
    );

    let u_g1_bytes = u_g1.canonical_bytes();
    store.select_active_generation(u_g_digest).unwrap();
    let replayed_after_rollback = store.replay_generation(u_g1_digest).unwrap();
    assert_eq!(replayed_after_rollback.digest(), u_g1_digest);
    assert_eq!(replayed_after_rollback.canonical_bytes(), u_g1_bytes);
    store.select_active_generation(u_g1_digest).unwrap();
    assert_eq!(store.active_generation().unwrap(), Some(u_g1_digest));
    let nc12 = nc(
        SelfExpansionNegativeControl::RollbackHistoryRewrite,
        "p10:nc12:rollback-history-preserved",
        &[u_g_digest, u_g1_digest, replayed_after_rollback.digest()],
    );

    let negative_controls = SelfExpansionNegativeControlManifest::new(vec![
        nc01, nc02, nc03, nc04, nc05, nc06, nc07, nc08, nc09, nc10, nc11, nc12,
    ])
    .unwrap();
    assert!(negative_controls.is_complete());

    let manifest = SelfExpansionProofManifest::new(
        source_commit().to_owned(),
        p9_frozen_proof_identity(),
        u_g_digest,
        u_g1_digest,
        world,
        PromotionClassRegistryV1::digest(),
        rational_before.structural_digest(),
        rational_after.structural_digest(),
        closure_before_id,
        closure_after_id,
        closure_delta_id,
        field_cap,
        witness_digest,
        promotion.structural_digest(),
        expansion_authorization.authorization_digest(),
        lambda_g.structural_digest(),
        lambda_g1.structural_digest(),
        nogood_proof,
        route_proof,
        shadow_record.structural_digest(),
        semantic_change.structural_digest(),
        proof_evolution,
        upgrade.structural_digest(),
        realization_rollback,
        negative_controls.clone(),
        checker_identity(),
        verifier_identity(),
    );
    let replay = SelfExpansionReplayEvidence::new(
        manifest.structural_digest(),
        u_g_digest,
        u_g1_digest,
        u_g_digest,
        PromotionClassRegistryV1::digest(),
        rational_before.structural_digest(),
        rational_after.structural_digest(),
        closure_before_id,
        closure_after_id,
        closure_delta_id,
        field_cap,
        witness_digest,
        promotion.structural_digest(),
        expansion_authorization.authorization_digest(),
        lambda_g.structural_digest(),
        lambda_g1.structural_digest(),
        nogood_proof,
        route_proof,
        shadow_record.structural_digest(),
        semantic_change.structural_digest(),
        proof_evolution,
        upgrade.structural_digest(),
        realization_rollback,
        negative_controls.clone(),
        checker_identity(),
        verifier_identity(),
        SelfExpansionReplayClaims::all_proved(),
    );
    let verified = verify_self_expansion_manifest(&manifest, &replay).unwrap();
    assert_eq!(verified.markers(), &P10_CANONICAL_MARKERS);

    println!("P10_SOURCE_SHA={}", source_commit());
    println!("P10_P9_PREDECESSOR={}", P9_FROZEN_PROOF_HEAD);
    println!("P10_MANIFEST={}", manifest.structural_digest().as_str());
    println!("P10_U_G={}", u_g_digest.as_str());
    println!("P10_U_G1={}", u_g1_digest.as_str());
    println!(
        "P10_REGISTRY={}",
        PromotionClassRegistryV1::digest().as_str()
    );
    println!("P10_LAMBDA_G={}", lambda_g.structural_digest().as_str());
    println!(
        "P10_LAMBDA_G1={}",
        lambda_g1.structural_digest().as_str()
    );
    println!("P10_UNLOCKED_CAPABILITY={}", field_cap.as_str());
    println!(
        "P10_NEGATIVE_CONTROLS={}",
        negative_controls.structural_digest().as_str()
    );
    for marker in verified.markers() {
        println!("{marker}");
    }
}
