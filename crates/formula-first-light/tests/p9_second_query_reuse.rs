use formula_check::{
    promotion::{PromotionDecision, authorize_promotion_v1},
    realization::authorize_native_u8_realization_v1,
    u8::{BoolExpr as CheckedBoolExpr, ByteExpr as CheckedByteExpr},
};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{
        FrozenCandidate as CertifiedFrozenCandidate, PromotionManifest, RealizationCheckManifest,
    },
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::{PromotionCandidate, PromotionRecord, PromotionState},
    realization::{
        NativeRealizationManifest, NativeToolchainIdentity, RealizationDispatchContext,
        SpecializationIdentity,
    },
    theory::ClosureContext,
};
use formula_engine::{
    candidate_space::CandidateSpaceContext,
    compiler::{CompilerError, CompilerInputs, CompilerV1},
    observational::{
        BoolExpr as EngineBoolExpr, ByteExpr as EngineByteExpr, ObservationalExprSpace,
        U8BoolGrammar,
    },
    query::{
        ActivatedPackageBinding, KnownBinding, QueryIR, RequestedResultClass, ResourceContract,
        SideEffectPolicy, TargetRequest,
    },
    region::CompilerAuthoritySnapshot,
    reuse::ReuseRequest,
};
use formula_first_light::{
    fl_c::{fl_c_grammar_digest, fl_c_oracle, fl_c_target_digest},
    reuse::{SecondQueryResult, canonical_second_query_vector},
};
use formula_packages::{
    activation::validate_activation,
    closure::derive_capabilities_with_semantic_activations,
};
use formula_realize::rust_native::generate_u8_bool_rust_source;
use formula_store::authority_store::AuthorityStore;
use std::{fs, path::Path, process::Command};
use tempfile::tempdir;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn checked_byte(expression: &EngineByteExpr) -> CheckedByteExpr {
    match expression {
        EngineByteExpr::X => CheckedByteExpr::X,
        EngineByteExpr::Zero => CheckedByteExpr::Const(0),
        EngineByteExpr::One => CheckedByteExpr::Const(1),
        EngineByteExpr::SubWrap(left, right) => {
            CheckedByteExpr::SubWrap(Box::new(checked_byte(left)), Box::new(checked_byte(right)))
        }
        EngineByteExpr::BitAnd(left, right) => {
            CheckedByteExpr::BitAnd(Box::new(checked_byte(left)), Box::new(checked_byte(right)))
        }
    }
}

fn checked_bool(expression: &EngineBoolExpr) -> CheckedBoolExpr {
    match expression {
        EngineBoolExpr::EqZero(value) => CheckedBoolExpr::EqZero(checked_byte(value)),
        EngineBoolExpr::NeqZero(value) => CheckedBoolExpr::NeqZero(checked_byte(value)),
        EngineBoolExpr::And(left, right) => {
            CheckedBoolExpr::And(Box::new(checked_bool(left)), Box::new(checked_bool(right)))
        }
    }
}

fn canonical_rust_toolchain() -> NativeToolchainIdentity {
    let output = Command::new("rustc")
        .arg("-vV")
        .output()
        .expect("pinned rustc must be executable");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let version = String::from_utf8(output.stdout).unwrap();
    let release = version
        .lines()
        .find_map(|line| line.strip_prefix("release: "))
        .unwrap();
    let host = version
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .unwrap();
    assert_eq!(release, "1.98.0");
    NativeToolchainIdentity::new(release.into(), host.into())
}

fn execute_native(binary: &Path, input: u8) -> bool {
    let output = Command::new(binary)
        .arg(input.to_string())
        .output()
        .expect("admitted native realization must execute");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    match output.stdout.as_slice() {
        b"0\n" => false,
        b"1\n" => true,
        other => panic!("non-canonical admitted realization output: {other:?}"),
    }
}

#[test]
fn second_count_query_uses_already_admitted_p8_binary_without_rediscovery_or_recompile() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);
    let u0_digest = store.initialize_genesis(&u0).unwrap();
    let world = d("p7-fl-c-world");
    let authority_contract = d("p7-fl-c-authority-contract");
    let observer = d("p7-fl-c-observer");

    let context = CandidateSpaceContext::new(
        u0_digest,
        world,
        d("p9-fl-c-query"),
        d("p9-fl-c-obligation"),
        fl_c_grammar_digest(),
        d("p9-fl-c-search-policy"),
    );
    let mut space = ObservationalExprSpace::new(context, U8BoolGrammar::minimal(), 9);
    let final_candidate = (0..=u8::MAX)
        .find_map(|_| {
            let candidate = space.extract_min_cost().expect("bounded FL-C candidate");
            match fl_c_oracle().first_counterexample(&candidate) {
                Some((input, expected)) => {
                    space.refine_counterexample(input, expected);
                    None
                }
                None => Some(candidate),
            }
        })
        .expect("bounded FL-C discovery converges");
    assert_eq!(fl_c_oracle().first_counterexample(&final_candidate), None);

    let engine_expression = final_candidate.expression().clone();
    let primitive = engine_expression.digest();
    let evidence = d("p7-fl-c-exhaustive-equivalence-evidence");
    let frozen = CertifiedFrozenCandidate::new(
        "first-light-fl-c-semantic-primitive".into(),
        vec![primitive],
        world,
        u0_digest,
        vec![],
        vec![fl_c_target_digest()],
        authority_contract,
        observer,
    );
    let promotion_manifest = PromotionManifest::new(
        u0_digest,
        frozen.structural_digest(),
        vec![evidence],
        vec![primitive],
        vec![evidence],
    );
    let promotion = PromotionCandidate::new(
        frozen.structural_digest(),
        promotion_manifest.structural_digest(),
        u0_digest,
        u0_digest,
        vec![],
        vec![],
    );
    let decision = authorize_promotion_v1(
        &promotion_manifest,
        &frozen,
        &promotion,
        &[evidence],
        &u0,
        &[],
    )
    .unwrap();
    let PromotionDecision::Authorized(promotion_authorization) = decision else {
        panic!("valid FL-C primitive must authorize")
    };
    let promotion_outcome = store.promote(&promotion_authorization).unwrap();
    let u1_digest = promotion_outcome.new_generation();
    let u1 = store.replay_generation(u1_digest).unwrap();

    // P8 realization setup happens before the second query begins.
    let specialization =
        SpecializationIdentity::new(primitive, u1_digest, world, authority_contract, observer);
    let generated = generate_u8_bool_rust_source(&engine_expression, &specialization).unwrap();
    let toolchain = canonical_rust_toolchain();
    let build_dir = tempdir().unwrap();
    let source_path = build_dir.path().join("p9_p8_native.rs");
    let binary_path = build_dir.path().join("p9_p8_native");
    fs::write(&source_path, generated.source().as_bytes()).unwrap();
    let compile_status = Command::new(toolchain.compiler())
        .arg(&source_path)
        .arg(toolchain.optimization())
        .arg("-o")
        .arg(&binary_path)
        .status()
        .unwrap();
    assert!(compile_status.success());
    let binary_bytes = fs::read(&binary_path).unwrap();
    let binary_digest = ArtifactDigest::of_bytes(&binary_bytes);
    let native_manifest = NativeRealizationManifest::new(
        primitive,
        u1_digest,
        world,
        authority_contract,
        observer,
        specialization.structural_digest(),
        generated.source_digest(),
        toolchain.structural_digest(),
        binary_digest,
    );
    let outputs: Vec<bool> = (0u16..=255)
        .map(|raw| execute_native(&binary_path, raw as u8))
        .collect();
    let semantic = checked_bool(&engine_expression);
    let check_manifest = RealizationCheckManifest::new(
        primitive,
        native_manifest.structural_digest(),
        u1_digest,
        world,
        authority_contract,
        observer,
        binary_digest,
    );
    let realization_authorization = authorize_native_u8_realization_v1(
        &native_manifest,
        &specialization,
        &toolchain,
        &check_manifest,
        generated.source().as_bytes(),
        &binary_bytes,
        &semantic,
        &outputs,
    )
    .unwrap();
    let admitted = store
        .admit_realization(&realization_authorization, &binary_bytes)
        .unwrap();

    let activated = PromotionRecord::new(
        promotion.structural_digest(),
        PromotionState::Activated,
        u1_digest,
        promotion_authorization.policy_digest(),
        vec![evidence],
        vec![primitive],
    );
    store
        .admit_semantic_activation(&activated, primitive)
        .unwrap();
    let persisted_activation = store
        .resolve_semantic_activation(u1_digest, primitive)
        .unwrap()
        .unwrap();

    // Canonical second query begins here. No source generation, rustc invocation,
    // realization authorization, or realization admission occurs after this point.
    let vector = canonical_second_query_vector();
    let packages = validate_activation(&u1, &[], &[], &[]).unwrap();
    let closure_context = ClosureContext::new(
        u1_digest,
        world,
        packages.digests().to_vec(),
        d("p9-second-query-closure-rules"),
        authority_contract,
    );
    let closure_without_activation = derive_capabilities_with_semantic_activations(
        &closure_context,
        &packages,
        &[],
        &[],
        &u1,
        &[],
    )
    .unwrap();
    let closure = derive_capabilities_with_semantic_activations(
        &closure_context,
        &packages,
        &[],
        &[],
        &u1,
        &[persisted_activation],
    )
    .unwrap();
    assert!(!closure_without_activation.contains(primitive));
    assert!(closure.contains(primitive));

    let package_binding = ActivatedPackageBinding::new(
        u1_digest,
        packages.digests().to_vec(),
        packages.composition_claims().to_vec(),
    );
    let query = QueryIR::new(
        u1_digest,
        world,
        vec![KnownBinding::new("values", vector.digest())],
        vec![],
        vec![TargetRequest::new(
            d("p9-count-power-of-two-u8"),
            RequestedResultClass::Count,
        )],
        observer,
        authority_contract,
        ResourceContract::new(100, 1024, 50),
        SideEffectPolicy::deny_all(),
        package_binding.clone(),
    );
    let compiler_inputs = CompilerInputs::new(
        observer,
        authority_contract,
        d("p9-second-query-evidence-requirement"),
        d("p9-second-query-random-key"),
    );
    let missing_snapshot = CompilerAuthoritySnapshot::new(
        u1_digest,
        world,
        package_binding.clone(),
        vec![vector.digest()],
        closure_without_activation.capabilities().collect(),
        vec![],
    );
    let request = ReuseRequest::new(&query, primitive);
    assert_eq!(
        CompilerV1::compile_reuse(
            &query,
            &missing_snapshot,
            compiler_inputs.clone(),
            &request,
        )
        .unwrap_err(),
        CompilerError::RequiredCapabilityUnavailable
    );

    let snapshot = CompilerAuthoritySnapshot::new(
        u1_digest,
        world,
        package_binding,
        vec![vector.digest(), primitive],
        closure.capabilities().collect(),
        vec![],
    );
    let compiled = CompilerV1::compile_reuse(&query, &snapshot, compiler_inputs, &request).unwrap();
    assert!(compiled.work_cells().is_empty());
    assert_eq!(compiled.metrics().primitive_discovery_candidate_spaces(), 0);
    assert_eq!(compiled.metrics().primitive_discovery_work_cells(), 0);
    assert_eq!(compiled.metrics().resolved_capability_count(), 1);
    assert_eq!(compiled.metrics().execution_work_items(), 1);

    let dispatch =
        RealizationDispatchContext::new(primitive, u1_digest, world, authority_contract, observer);
    let resolved = store
        .resolve_realization(&dispatch)
        .unwrap()
        .expect("P8 admitted realization must resolve for second query");
    assert_eq!(resolved.manifest_digest(), admitted.manifest_digest());
    assert_eq!(resolved.binary_digest(), admitted.binary_digest());

    let execution_dir = tempdir().unwrap();
    let admitted_binary_path = execution_dir.path().join("p9_admitted_native");
    fs::write(&admitted_binary_path, resolved.binary_bytes()).unwrap();
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(&admitted_binary_path).unwrap().permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&admitted_binary_path, permissions).unwrap();
    }

    let native_count = vector
        .values()
        .iter()
        .copied()
        .filter(|value| execute_native(&admitted_binary_path, *value))
        .count() as u64;
    let semantic_count = vector
        .values()
        .iter()
        .copied()
        .filter(|value| engine_expression.eval(*value))
        .count() as u64;
    assert_eq!(native_count, semantic_count);

    let result = SecondQueryResult::new(
        &vector,
        primitive,
        resolved.manifest_digest(),
        native_count,
    );
    assert_eq!(result.input_digest(), vector.digest());
    assert_eq!(result.primitive(), primitive);
    assert_eq!(result.realization(), resolved.manifest_digest());
    assert_eq!(result.matching_count(), native_count);
    assert_ne!(result.digest(), ArtifactDigest::of_bytes(&[]));
}
