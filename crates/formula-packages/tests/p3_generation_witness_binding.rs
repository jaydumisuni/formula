use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    theory::{CapabilityContract, ClosureContext, StructureWitness, TheoryPackageManifest},
};
use formula_packages::closure::{
    AdmittedStructureWitness, WitnessAdmissionError, derive_capabilities,
};

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
    let witness = StructureWitness::new(d("world-1"), goal, d("evidence"));
    let generation_1 = UniverseGeneration::new(
        1,
        None,
        vec![witness.structural_digest()],
        vec![witness.evidence()],
    );
    let generation_2 = UniverseGeneration::new(2, Some(generation_1.digest()), vec![], vec![]);
    let admitted = AdmittedStructureWitness::new(&generation_1, witness).unwrap();

    let context_1 = ClosureContext::new(
        generation_1.digest(),
        d("world-1"),
        vec![package_digest],
        d("rules"),
        d("policy"),
    );
    let context_2 = ClosureContext::new(
        generation_2.digest(),
        d("world-1"),
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

#[test]
fn witness_admission_requires_semantic_and_authority_membership() {
    let witness = StructureWitness::new(d("world"), d("goal"), d("evidence"));

    let missing_witness = UniverseGeneration::new(1, None, vec![], vec![witness.evidence()]);
    assert_eq!(
        AdmittedStructureWitness::new(&missing_witness, witness.clone()),
        Err(WitnessAdmissionError::WitnessNotAdmitted)
    );

    let missing_authority = UniverseGeneration::new(
        1,
        None,
        vec![witness.structural_digest()],
        vec![],
    );
    assert_eq!(
        AdmittedStructureWitness::new(&missing_authority, witness),
        Err(WitnessAdmissionError::EvidenceNotAuthorityBound)
    );
}
