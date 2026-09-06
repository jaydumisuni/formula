use formula_check::bootstrap::{
    BootstrapValidationFailure, canonical_build_recipe_identity,
    canonical_normalization_rules_identity, reference_compile, semantic_evidence_identity,
    validate_bootstrap_candidate,
};
use formula_core::{
    artifacts::StructuralIdentity,
    bootstrap::{
        BootstrapBytecode, BootstrapDecision, BootstrapEquivalenceLevel, BootstrapNegativeControl,
        BootstrapNegativeControlEvidence, BootstrapNegativeControlManifest, BootstrapProofManifest,
        BootstrapProgramSource, BootstrapRebuildManifest, BootstrapValidationState,
    },
    digest::ArtifactDigest,
    generation::UniverseGeneration,
};
use formula_first_light::p12::{
    checker_identity, p11_frozen_proof_identity, seed_manifest, source_commit,
    successor_generation, verifier_identity, P11_FROZEN_PROOF_HEAD,
};
use formula_realize::bootstrap::{
    BootstrapExecutionError, canonical_generator_image, execute_bootstrap_bytecode,
    generator_image_from_bootstrap_artifact, rebuild_with_generator_image,
};
use formula_store::{authority_store::AuthorityStore, bootstrap_store::BootstrapAuthorityStore};
use tempfile::tempdir;

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn evidence(label: &str, parts: &[ArtifactDigest]) -> ArtifactDigest {
    let mut bytes = label.as_bytes().to_vec();
    for part in parts {
        bytes.push(0);
        bytes.extend_from_slice(part.as_str().as_bytes());
    }
    ArtifactDigest::of_bytes(&bytes)
}

fn rebuild(
    predecessor: formula_core::bootstrap::BootstrapGenerationId,
    successor: formula_core::bootstrap::BootstrapGenerationId,
    generator: ArtifactDigest,
    validator: ArtifactDigest,
    source: &BootstrapProgramSource,
    candidate: &BootstrapBytecode,
    seed: &formula_core::bootstrap::BootstrapSeedManifest,
) -> BootstrapRebuildManifest {
    let independent = reference_compile(source).expect("canonical source independently compiles");
    BootstrapRebuildManifest::new(
        predecessor,
        successor,
        generator,
        validator,
        source.structural_digest(),
        canonical_build_recipe_identity(),
        candidate.structural_digest(),
        independent.structural_digest(),
        canonical_normalization_rules_identity(),
        BootstrapEquivalenceLevel::ByteForByte,
        semantic_evidence_identity(source, candidate),
        seed.structural_digest(),
        BootstrapValidationState::Candidate,
    )
}

#[test]
fn p12_self_hosting_bootstrap_trust_reduction() {
    assert_eq!(
        P11_FROZEN_PROOF_HEAD,
        "6f8ce7bb6702ea1baf119aab9950aa5ba0f87283"
    );
    assert_eq!(
        p11_frozen_proof_identity(),
        ArtifactDigest::of_bytes(P11_FROZEN_PROOF_HEAD.as_bytes())
    );

    let seed = seed_manifest();
    let source = BootstrapProgramSource::identity_checker_v1();
    let validator = checker_identity();
    let dir = tempdir().unwrap();

    let mut universe = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![d("p12:math-authority-base")], vec![]);
    let u_before = universe.initialize_genesis(&u0).unwrap();

    let mut bootstrap = BootstrapAuthorityStore::open(dir.path()).unwrap();
    let t0 = bootstrap.create_bootstrap_root(&seed).unwrap();
    assert_eq!(t0.ordinal(), 0);
    assert_eq!(t0.digest(), seed.structural_digest());

    let stage0_image = canonical_generator_image();
    let stage1 = rebuild_with_generator_image(&stage0_image, &source).unwrap();
    let t1 = successor_generation(
        t0,
        stage0_image.structural_digest(),
        validator,
        source.structural_digest(),
        stage1.structural_digest(),
        seed.structural_digest(),
    );
    let stage1_rebuild = rebuild(
        t0,
        t1,
        stage0_image.structural_digest(),
        validator,
        &source,
        &stage1,
        &seed,
    );
    let auth1 = validate_bootstrap_candidate(&stage1_rebuild, &source, &stage1, &seed)
        .expect("Stage1 requires independent validation");
    assert_eq!(bootstrap.admit_bootstrap_successor(&auth1, &stage1).unwrap(), t1);

    let admitted_t1 = bootstrap.replay_bootstrap_generation(t1).unwrap();
    let admitted_stage1 = admitted_t1
        .candidate()
        .expect("T1 must replay admitted Stage1 candidate");
    let stage1_image = generator_image_from_bootstrap_artifact(admitted_stage1)
        .expect("admitted Stage1 artifact must seed Stage2 generator");
    let stage2 = rebuild_with_generator_image(&stage1_image, &source).unwrap();
    let t2 = successor_generation(
        t1,
        stage1_image.structural_digest(),
        validator,
        source.structural_digest(),
        stage2.structural_digest(),
        seed.structural_digest(),
    );
    let stage2_rebuild = rebuild(
        t1,
        t2,
        stage1_image.structural_digest(),
        validator,
        &source,
        &stage2,
        &seed,
    );
    let auth2 = validate_bootstrap_candidate(&stage2_rebuild, &source, &stage2, &seed)
        .expect("Stage2 requires independent validation");
    assert_eq!(bootstrap.admit_bootstrap_successor(&auth2, &stage2).unwrap(), t2);

    assert_eq!(stage1.bytes(), stage2.bytes());
    let same = d("p12:semantic-same");
    assert_eq!(
        execute_bootstrap_bytecode(&stage1, same, same),
        Ok(BootstrapDecision::Valid)
    );
    assert_eq!(
        execute_bootstrap_bytecode(&stage2, d("p12:a"), d("p12:b")),
        Ok(BootstrapDecision::Reject)
    );

    let mut negatives = Vec::new();

    let nc01 = BootstrapRebuildManifest::new(
        t2,
        successor_generation(
            t2,
            validator,
            validator,
            source.structural_digest(),
            stage2.structural_digest(),
            seed.structural_digest(),
        ),
        validator,
        validator,
        source.structural_digest(),
        canonical_build_recipe_identity(),
        stage2.structural_digest(),
        stage2.structural_digest(),
        canonical_normalization_rules_identity(),
        BootstrapEquivalenceLevel::ByteForByte,
        semantic_evidence_identity(&source, &stage2),
        seed.structural_digest(),
        BootstrapValidationState::Candidate,
    );
    assert_eq!(
        validate_bootstrap_candidate(&nc01, &source, &stage2, &seed),
        Err(BootstrapValidationFailure::GeneratorEqualsValidator)
    );
    negatives.push(BootstrapNegativeControlEvidence::new(
        BootstrapNegativeControl::GeneratorEqualsValidator,
        evidence("nc-bs-01", &[nc01.structural_digest()]),
    ));

    let fake_independent = BootstrapRebuildManifest::new(
        t2,
        successor_generation(
            t2,
            d("single-path-generator"),
            d("single-path-validator"),
            source.structural_digest(),
            stage2.structural_digest(),
            seed.structural_digest(),
        ),
        d("single-path-generator"),
        d("single-path-validator"),
        source.structural_digest(),
        canonical_build_recipe_identity(),
        stage2.structural_digest(),
        d("single-path-self-produced-artifact"),
        canonical_normalization_rules_identity(),
        BootstrapEquivalenceLevel::ByteForByte,
        semantic_evidence_identity(&source, &stage2),
        seed.structural_digest(),
        BootstrapValidationState::Candidate,
    );
    assert_eq!(
        validate_bootstrap_candidate(&fake_independent, &source, &stage2, &seed),
        Err(BootstrapValidationFailure::IndependentArtifactMismatch)
    );
    negatives.push(BootstrapNegativeControlEvidence::new(
        BootstrapNegativeControl::SinglePathAdmissionAttempt,
        evidence("nc-bs-02", &[fake_independent.structural_digest()]),
    ));

    let divergent = BootstrapBytecode::new(b"FBC1\x01\x02\x03\xff".to_vec());
    let divergent_manifest = rebuild(
        t2,
        successor_generation(
            t2,
            d("divergent-generator"),
            validator,
            source.structural_digest(),
            divergent.structural_digest(),
            seed.structural_digest(),
        ),
        d("divergent-generator"),
        validator,
        &source,
        &divergent,
        &seed,
    );
    assert_eq!(
        validate_bootstrap_candidate(&divergent_manifest, &source, &divergent, &seed),
        Err(BootstrapValidationFailure::CandidateReferenceMismatch)
    );
    negatives.push(BootstrapNegativeControlEvidence::new(
        BootstrapNegativeControl::UnexpectedArtifactDifference,
        evidence("nc-bs-03", &[divergent.structural_digest()]),
    ));

    let wrong_seed = BootstrapRebuildManifest::new(
        stage2_rebuild.predecessor(),
        stage2_rebuild.successor(),
        stage2_rebuild.generator_identity(),
        stage2_rebuild.validator_identity(),
        stage2_rebuild.source_digest(),
        stage2_rebuild.build_recipe_digest(),
        stage2_rebuild.candidate_artifact(),
        stage2_rebuild.independent_artifact(),
        stage2_rebuild.normalization_rules(),
        stage2_rebuild.equivalence(),
        stage2_rebuild.semantic_evidence(),
        d("wrong-seed"),
        BootstrapValidationState::Candidate,
    );
    assert_eq!(
        validate_bootstrap_candidate(&wrong_seed, &source, &stage2, &seed),
        Err(BootstrapValidationFailure::SeedProvenanceMismatch)
    );
    negatives.push(BootstrapNegativeControlEvidence::new(
        BootstrapNegativeControl::SeedProvenanceMismatch,
        evidence("nc-bs-04", &[wrong_seed.structural_digest()]),
    ));

    let wrong_source = BootstrapRebuildManifest::new(
        stage2_rebuild.predecessor(),
        stage2_rebuild.successor(),
        stage2_rebuild.generator_identity(),
        stage2_rebuild.validator_identity(),
        d("wrong-source"),
        stage2_rebuild.build_recipe_digest(),
        stage2_rebuild.candidate_artifact(),
        stage2_rebuild.independent_artifact(),
        stage2_rebuild.normalization_rules(),
        stage2_rebuild.equivalence(),
        stage2_rebuild.semantic_evidence(),
        stage2_rebuild.seed_identity(),
        BootstrapValidationState::Candidate,
    );
    assert_eq!(
        validate_bootstrap_candidate(&wrong_source, &source, &stage2, &seed),
        Err(BootstrapValidationFailure::SourceDigestMismatch)
    );
    negatives.push(BootstrapNegativeControlEvidence::new(
        BootstrapNegativeControl::SourceDigestMismatch,
        evidence("nc-bs-05", &[wrong_source.structural_digest()]),
    ));

    let wrong_recipe = BootstrapRebuildManifest::new(
        stage2_rebuild.predecessor(),
        stage2_rebuild.successor(),
        stage2_rebuild.generator_identity(),
        stage2_rebuild.validator_identity(),
        stage2_rebuild.source_digest(),
        d("wrong-build-recipe"),
        stage2_rebuild.candidate_artifact(),
        stage2_rebuild.independent_artifact(),
        stage2_rebuild.normalization_rules(),
        stage2_rebuild.equivalence(),
        stage2_rebuild.semantic_evidence(),
        stage2_rebuild.seed_identity(),
        BootstrapValidationState::Candidate,
    );
    assert_eq!(
        validate_bootstrap_candidate(&wrong_recipe, &source, &stage2, &seed),
        Err(BootstrapValidationFailure::BuildRecipeDigestMismatch)
    );
    negatives.push(BootstrapNegativeControlEvidence::new(
        BootstrapNegativeControl::BuildRecipeDigestMismatch,
        evidence("nc-bs-06", &[wrong_recipe.structural_digest()]),
    ));

    let masked = BootstrapRebuildManifest::new(
        stage2_rebuild.predecessor(),
        stage2_rebuild.successor(),
        stage2_rebuild.generator_identity(),
        stage2_rebuild.validator_identity(),
        stage2_rebuild.source_digest(),
        stage2_rebuild.build_recipe_digest(),
        stage2_rebuild.candidate_artifact(),
        stage2_rebuild.independent_artifact(),
        d("normalization:masks-semantic-difference"),
        stage2_rebuild.equivalence(),
        stage2_rebuild.semantic_evidence(),
        stage2_rebuild.seed_identity(),
        BootstrapValidationState::Candidate,
    );
    assert_eq!(
        validate_bootstrap_candidate(&masked, &source, &stage2, &seed),
        Err(BootstrapValidationFailure::NormalizationRulesMismatch)
    );
    negatives.push(BootstrapNegativeControlEvidence::new(
        BootstrapNegativeControl::NormalizationMasksSemanticDifference,
        evidence("nc-bs-07", &[masked.structural_digest()]),
    ));

    let malformed = BootstrapBytecode::new(b"FBC1\x01\x02\x03\xff".to_vec());
    assert_eq!(
        execute_bootstrap_bytecode(&malformed, same, same),
        Err(BootstrapExecutionError::MalformedOrUnsupportedBytecode)
    );
    negatives.push(BootstrapNegativeControlEvidence::new(
        BootstrapNegativeControl::MalformedOrUnsupportedBytecode,
        evidence("nc-bs-08", &[malformed.structural_digest()]),
    ));

    let weak_equivalence = BootstrapRebuildManifest::new(
        stage2_rebuild.predecessor(),
        stage2_rebuild.successor(),
        stage2_rebuild.generator_identity(),
        stage2_rebuild.validator_identity(),
        stage2_rebuild.source_digest(),
        stage2_rebuild.build_recipe_digest(),
        stage2_rebuild.candidate_artifact(),
        stage2_rebuild.independent_artifact(),
        stage2_rebuild.normalization_rules(),
        BootstrapEquivalenceLevel::SourceSemantic,
        stage2_rebuild.semantic_evidence(),
        stage2_rebuild.seed_identity(),
        BootstrapValidationState::Candidate,
    );
    assert_eq!(
        validate_bootstrap_candidate(&weak_equivalence, &source, &stage2, &seed),
        Err(BootstrapValidationFailure::EquivalenceNotByteForByte)
    );
    negatives.push(BootstrapNegativeControlEvidence::new(
        BootstrapNegativeControl::FailedEquivalencePromotionAttempt,
        evidence("nc-bs-09", &[weak_equivalence.structural_digest()]),
    ));

    bootstrap.select_bootstrap_generation(t1).unwrap();
    bootstrap.select_bootstrap_generation(t2).unwrap();
    let u_after = universe
        .active_generation()
        .unwrap()
        .expect("Universe generation must remain active");
    assert_eq!(u_before, u_after);
    negatives.push(BootstrapNegativeControlEvidence::new(
        BootstrapNegativeControl::UniverseMutationAttempt,
        evidence("nc-bs-10", &[u_before, u_after, t2.digest()]),
    ));

    let negative_controls = BootstrapNegativeControlManifest::new(negatives).unwrap();
    assert!(negative_controls.is_complete());

    let proof = BootstrapProofManifest::new(
        source_commit().into(),
        p11_frozen_proof_identity(),
        seed.clone(),
        t0,
        t1,
        t2,
        source.structural_digest(),
        stage1_rebuild,
        stage2_rebuild,
        stage1.structural_digest(),
        stage2.structural_digest(),
        negative_controls,
        u_before,
        u_after,
        checker_identity(),
        verifier_identity(),
    );

    println!("P12_SOURCE_COMMIT={}", source_commit());
    println!("P12_SEED={}", seed.structural_digest().as_str());
    println!("P12_T0={}", t0.digest().as_str());
    println!("P12_T1={}", t1.digest().as_str());
    println!("P12_T2={}", t2.digest().as_str());
    println!("P12_SOURCE={}", source.structural_digest().as_str());
    println!("P12_STAGE1={}", stage1.structural_digest().as_str());
    println!("P12_STAGE2={}", stage2.structural_digest().as_str());
    println!("P12_NC={}", proof.negative_controls().structural_digest().as_str());
    println!("P12_PROOF={}", proof.structural_digest().as_str());
}
