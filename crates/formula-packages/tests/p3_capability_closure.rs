use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    theory::{CapabilityContract, ClosureContext, StructureWitness, TheoryPackageManifest},
};
use formula_packages::closure::{AdmittedStructureWitness, derive_capabilities};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn package(goal: ArtifactDigest, capability: ArtifactDigest) -> TheoryPackageManifest {
    TheoryPackageManifest::new(
        "test.package".into(),
        d("foundation"),
        vec![],
        vec![CapabilityContract::new(capability, vec![goal])],
        vec![],
        vec![],
    )
}

#[test]
fn certified_witness_unlocks_only_the_matching_world_and_active_package() {
    let goal = d("goal:ring");
    let capability = d("cap:divide");
    let package = package(goal, capability);
    let package_digest = package.structural_digest();
    let witness = StructureWitness::new(d("world-1"), goal, d("evidence-1"));
    let wrong_world = StructureWitness::new(d("world-2"), goal, d("evidence-2"));
    let generation = UniverseGeneration::new(
        1,
        None,
        vec![witness.structural_digest(), wrong_world.structural_digest()],
        vec![witness.evidence(), wrong_world.evidence()],
    );
    let admitted = AdmittedStructureWitness::new(&generation, witness).unwrap();
    let admitted_wrong_world = AdmittedStructureWitness::new(&generation, wrong_world).unwrap();
    let context = ClosureContext::new(
        generation.digest(),
        d("world-1"),
        vec![package_digest],
        d("rules"),
        d("policy"),
    );

    let without = derive_capabilities(&context, &[], std::slice::from_ref(&package));
    let with = derive_capabilities(
        &context,
        std::slice::from_ref(&admitted),
        std::slice::from_ref(&package),
    );

    assert!(!without.contains(capability));
    assert!(with.contains(capability));

    let leaked = derive_capabilities(&context, &[admitted_wrong_world], &[package]);
    assert!(!leaked.contains(capability));
}

#[test]
fn closure_is_deterministic_and_context_identity_is_generation_scoped() {
    let goal = d("goal:ring");
    let capability = d("cap:divide");
    let package = package(goal, capability);
    let package_digest = package.structural_digest();
    let witness = StructureWitness::new(d("world-1"), goal, d("evidence"));
    let generation_a = UniverseGeneration::new(
        1,
        None,
        vec![witness.structural_digest()],
        vec![witness.evidence()],
    );
    let generation_b = UniverseGeneration::new(
        2,
        Some(generation_a.digest()),
        vec![witness.structural_digest()],
        vec![witness.evidence()],
    );
    let admitted_a = AdmittedStructureWitness::new(&generation_a, witness.clone()).unwrap();
    let admitted_b = AdmittedStructureWitness::new(&generation_b, witness).unwrap();
    let context_a = ClosureContext::new(
        generation_a.digest(),
        d("world-1"),
        vec![package_digest],
        d("rules"),
        d("policy"),
    );
    let context_b = ClosureContext::new(
        generation_b.digest(),
        d("world-1"),
        vec![package_digest],
        d("rules"),
        d("policy"),
    );

    let a1 = derive_capabilities(
        &context_a,
        std::slice::from_ref(&admitted_a),
        std::slice::from_ref(&package),
    );
    let a2 = derive_capabilities(
        &context_a,
        std::slice::from_ref(&admitted_a),
        std::slice::from_ref(&package),
    );
    let b = derive_capabilities(&context_b, &[admitted_b], &[package]);

    assert_eq!(a1, a2);
    assert_ne!(a1.context_digest(), b.context_digest());
    assert!(a1.contains(capability));
    assert!(b.contains(capability));
}
