use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    theory::{CapabilityContract, ClosureContext, StructureWitness, TheoryPackageManifest},
};
use formula_packages::closure::{derive_capabilities, AdmittedStructureWitness};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn admitted_structure_witness_does_not_leak_across_generations() {
    let goal = d("goal:ring");
    let capability = d("cap:divide");
    let package = TheoryPackageManifest::new(
        "test.package".into(),
        d("foundation"),
        vec![],
        vec![CapabilityContract::new(capability, vec![goal])],
        vec![],
        vec![],
    );
    let package_digest = package.structural_digest();
    let generation_1 = d("generation-1");
    let generation_2 = d("generation-2");
    let world = d("world-1");
    let witness = StructureWitness::new(world, goal, d("evidence"));
    let admitted = AdmittedStructureWitness::new(generation_1, witness, d("admission-edge"));

    let context_1 = ClosureContext::new(
        generation_1,
        world,
        vec![package_digest],
        d("rules"),
        d("policy"),
    );
    let context_2 = ClosureContext::new(
        generation_2,
        world,
        vec![package_digest],
        d("rules"),
        d("policy"),
    );

    let matching = derive_capabilities(
        &context_1,
        std::slice::from_ref(&admitted),
        std::slice::from_ref(&package),
    );
    let leaked = derive_capabilities(&context_2, &[admitted], &[package]);

    assert!(matching.contains(capability));
    assert!(!leaked.contains(capability));
}
