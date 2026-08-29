use formula_core::{
    artifacts::StructuralIdentity,
    canonical::CanonicalValue,
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    theory::{
        CanonicalMorphism, CapabilityContract, ClosureContext, CompositionClaim, CompositionClass,
        FactPolarity, FederationAdapterManifest, SharedFact, StructureWitness,
        TheoryPackageManifest,
    },
};
use formula_packages::{
    activation::{ActivationError, validate_activation},
    closure::{AdmittedStructureWitness, derive_capabilities},
    federation::{FederationError, FederationMode, FederationRequest, validate_federation_adapter},
    morphisms::{CommonParentResolution, MorphismRegistry, resolve_common_parent},
    shared_facts::{FactRequirement, FactUseDecision, fact_satisfies},
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn package(
    name: &str,
    goal: ArtifactDigest,
    capability: ArtifactDigest,
    interference_surface: Vec<String>,
) -> TheoryPackageManifest {
    TheoryPackageManifest::new(
        name.into(),
        d("foundation"),
        vec![],
        vec![CapabilityContract::new(capability, vec![goal])],
        vec![],
        interference_surface,
    )
}

#[test]
fn world_generation_and_activation_boundaries_do_not_leak_capability() {
    let goal = d("goal");
    let capability = d("capability");
    let package = package("pkg", goal, capability, vec![]);
    let package_digest = package.structural_digest();
    let witness = StructureWitness::new(d("world-1"), goal, d("evidence"));
    let generation = UniverseGeneration::new(
        1,
        None,
        vec![package_digest, witness.structural_digest()],
        vec![witness.evidence()],
    );
    let admitted = AdmittedStructureWitness::new(&generation, witness).unwrap();
    let active_set = validate_activation(
        &generation,
        std::slice::from_ref(&package),
        &[],
        &[package_digest],
    )
    .unwrap();
    let deactivated_set =
        validate_activation(&generation, std::slice::from_ref(&package), &[], &[]).unwrap();
    let active = ClosureContext::new(
        generation.digest(),
        d("world-1"),
        vec![package_digest],
        d("rules"),
        d("policy"),
    );
    let deactivated = ClosureContext::new(
        generation.digest(),
        d("world-1"),
        vec![],
        d("rules"),
        d("policy"),
    );
    let another_generation =
        UniverseGeneration::new(2, Some(generation.digest()), vec![package_digest], vec![]);
    let another_generation_set = validate_activation(
        &another_generation,
        std::slice::from_ref(&package),
        &[],
        &[package_digest],
    )
    .unwrap();
    let another_generation_context = ClosureContext::new(
        another_generation.digest(),
        d("world-1"),
        vec![package_digest],
        d("rules"),
        d("policy"),
    );
    let another_world = ClosureContext::new(
        generation.digest(),
        d("world-2"),
        vec![package_digest],
        d("rules"),
        d("policy"),
    );

    let active_closure = derive_capabilities(
        &active,
        &active_set,
        std::slice::from_ref(&admitted),
        std::slice::from_ref(&package),
    )
    .unwrap();
    assert!(active_closure.contains(capability));

    let deactivated_closure = derive_capabilities(
        &deactivated,
        &deactivated_set,
        std::slice::from_ref(&admitted),
        std::slice::from_ref(&package),
    )
    .unwrap();
    assert!(!deactivated_closure.contains(capability));

    let wrong_world = derive_capabilities(
        &another_world,
        &active_set,
        std::slice::from_ref(&admitted),
        std::slice::from_ref(&package),
    )
    .unwrap();
    assert!(!wrong_world.contains(capability));

    let another_generation_closure = derive_capabilities(
        &another_generation_context,
        &another_generation_set,
        &[admitted],
        &[package],
    )
    .unwrap();
    assert!(!another_generation_closure.contains(capability));
    assert_ne!(
        active_closure.context_digest(),
        another_generation_closure.context_digest()
    );
}

#[test]
fn package_interference_and_common_parent_ambiguity_fail_closed() {
    let a = package("a", d("ga"), d("ca"), vec!["shared".into()]);
    let b = package("b", d("gb"), d("cb"), vec!["shared".into()]);
    let requested = vec![a.structural_digest(), b.structural_digest()];
    let packages_only = UniverseGeneration::new(1, None, requested.clone(), vec![]);

    assert_eq!(
        validate_activation(&packages_only, &[a.clone(), b.clone()], &[], &requested),
        Err(ActivationError::InterferenceUnproven)
    );
    let unsupported = CompositionClaim::new(
        a.structural_digest(),
        b.structural_digest(),
        CompositionClass::Unsupported,
        d("evidence"),
    );
    let unsupported_generation = UniverseGeneration::new(
        1,
        None,
        vec![
            a.structural_digest(),
            b.structural_digest(),
            unsupported.structural_digest(),
        ],
        vec![unsupported.evidence()],
    );
    assert_eq!(
        validate_activation(
            &unsupported_generation,
            &[a, b],
            std::slice::from_ref(&unsupported),
            &requested,
        ),
        Err(ActivationError::CompositionNotAdmissible)
    );

    let morphisms = vec![
        CanonicalMorphism::new(d("left"), d("p1"), d("l-p1"), vec![], true, true),
        CanonicalMorphism::new(d("right"), d("p1"), d("r-p1"), vec![], true, true),
        CanonicalMorphism::new(d("left"), d("p2"), d("l-p2"), vec![], true, true),
        CanonicalMorphism::new(d("right"), d("p2"), d("r-p2"), vec![], true, true),
    ];
    let morphism_generation = UniverseGeneration::new(
        1,
        None,
        morphisms
            .iter()
            .map(StructuralIdentity::structural_digest)
            .collect(),
        vec![],
    );
    let registry = MorphismRegistry::new(&morphism_generation, morphisms).unwrap();
    assert_eq!(
        resolve_common_parent(&registry, d("left"), d("right")),
        CommonParentResolution::Ambiguous
    );
}

#[test]
fn weak_shared_fact_and_candidate_only_federation_cannot_manufacture_authority() {
    let over = SharedFact::new(
        d("world"),
        d("subject"),
        CanonicalValue::String("possible region".into()),
        FactPolarity::OverApproximation,
        d("evidence"),
    );
    assert_eq!(
        fact_satisfies(&over, FactRequirement::Exact),
        FactUseDecision::Rejected
    );
    assert_eq!(
        fact_satisfies(&over, FactRequirement::ExistenceWitness),
        FactUseDecision::Rejected
    );

    let adapter = FederationAdapterManifest::new(
        "adapter".into(),
        d("package"),
        vec![d("input")],
        vec![d("output")],
        vec![d("translation")],
        vec![d("checker")],
        vec![],
        vec!["EXACT_WITNESS".into()],
        true,
    );
    let request = FederationRequest::new(
        "EXACT_WITNESS".into(),
        true,
        Some(d("checker")),
        Some(d("translation")),
        false,
    );
    assert_eq!(
        validate_federation_adapter(&adapter, FederationMode::CandidateOnly, &request),
        Err(FederationError::CandidateOnlyCannotAuthorize)
    );
}
