use formula_check::{
    promotion::{PromotionDecision, authorize_promotion_v1},
    self_expansion::authorize_expansion_v1,
};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate, PromotionManifest},
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::PromotionCandidate,
    self_expansion::{
        ActivationMode, ClassifiedPromotionCandidate, PromotionClass, SemanticChangeClass,
    },
    theory::{ClosureContext, StructureWitness},
};
use formula_packages::{
    activation::validate_activation,
    builtin::{integer_package, rational_package},
    closure::{AdmittedStructureWitness, CapabilityClosureDelta, derive_capabilities},
};
use formula_store::authority_store::AuthorityStore;
use tempfile::tempdir;

fn id(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn promoted_structure_witness_unlocks_existing_rational_field_capability() {
    let foundation = id("p10:rational:foundation");
    let world = id("p10:rational:world");
    let field_goal = id("goal:rational:field");
    let field_cap = id("cap:rational:field");
    let evidence = id("p10:rational:field:evidence");

    let integer = integer_package(foundation);
    let rational_before = rational_package(foundation);
    let integer_digest = integer.structural_digest();
    let rational_digest = rational_before.structural_digest();
    let package_digests = vec![integer_digest, rational_digest];
    let packages = vec![integer.clone(), rational_before.clone()];

    let u0 = UniverseGeneration::new(0, None, package_digests.clone(), vec![]);
    let active0 = validate_activation(&u0, &packages, &[], &package_digests).unwrap();
    let context0 = ClosureContext::new(
        u0.digest(),
        world,
        package_digests.clone(),
        id("p10:rational:rules"),
        id("p10:rational:policy"),
    );
    let closure_before = derive_capabilities(&context0, &active0, &[], &packages).unwrap();
    assert!(!closure_before.contains(field_cap));

    let witness = StructureWitness::new(world, field_goal, evidence);
    let witness_digest = witness.structural_digest();
    let frozen = FrozenCandidate::new(
        "p10-rational-field-structure-witness".into(),
        vec![witness_digest],
        world,
        u0.digest(),
        vec![],
        vec![],
        id("p10:rational:authority-contract"),
        id("p10:rational:observer"),
    );
    let manifest = PromotionManifest::new(
        u0.digest(),
        frozen.structural_digest(),
        vec![evidence],
        vec![witness_digest],
        vec![evidence],
    );
    let promotion = PromotionCandidate::new(
        frozen.structural_digest(),
        manifest.structural_digest(),
        u0.digest(),
        u0.digest(),
        vec![],
        vec![],
    );
    let decision = authorize_promotion_v1(&manifest, &frozen, &promotion, &[evidence], &u0, &[])
        .expect("structure-witness promotion must pass P7 policy");
    let PromotionDecision::Authorized(base_authorization) = decision else {
        panic!("valid structure-witness promotion was quarantined")
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
        authorize_expansion_v1(&base_authorization, &classified, &u0, None)
            .expect("structure witness may unlock capability closure but not grammar");
    assert_eq!(expansion_authorization.class(), PromotionClass::StructureWitness);
    assert_eq!(expansion_authorization.activation_effects(), &[field_cap]);
    assert!(expansion_authorization.grammar_effects().is_empty());

    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    store.initialize_genesis(&u0).unwrap();
    let outcome = store.promote(&base_authorization).unwrap();
    let u1 = store.replay_generation(outcome.new_generation()).unwrap();

    let active1 = validate_activation(&u1, &packages, &[], &package_digests).unwrap();
    let admitted_witness = AdmittedStructureWitness::new(&u1, witness).unwrap();
    let context1 = ClosureContext::new(
        u1.digest(),
        world,
        package_digests,
        id("p10:rational:rules"),
        id("p10:rational:policy"),
    );
    let closure_after = derive_capabilities(&context1, &active1, &[admitted_witness], &packages)
        .expect("existing generic closure must consume promoted witness");
    let delta = CapabilityClosureDelta::between(&closure_before, &closure_after);

    assert!(closure_after.contains(field_cap));
    assert_eq!(delta.added().collect::<Vec<_>>(), vec![field_cap]);
    assert!(delta.removed().next().is_none());

    let rational_after = rational_package(foundation);
    assert_eq!(rational_before.structural_digest(), rational_after.structural_digest());
    assert_eq!(rational_after.structural_digest(), rational_digest);
}
