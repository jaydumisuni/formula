use formula_check::{
    promotion::{PromotionDecision, authorize_promotion_v1},
    realization::authorize_native_u8_realization_v1,
    u8::{BoolExpr as CheckedBoolExpr, ByteExpr as CheckedByteExpr},
    verdict::CheckFailure,
};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{
        FrozenCandidate as CertifiedFrozenCandidate, PromotionManifest, RealizationCheckManifest,
    },
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::PromotionCandidate,
    realization::{
        NativeRealizationManifest, NativeToolchainIdentity, RealizationDispatchContext,
        SpecializationIdentity,
    },
};
use formula_engine::{
    candidate_space::CandidateSpaceContext,
    observational::{
        BoolExpr as EngineBoolExpr, ByteExpr as EngineByteExpr, ObservationalExprSpace,
        U8BoolGrammar,
    },
};
use formula_first_light::fl_c::{fl_c_grammar_digest, fl_c_oracle, fl_c_target_digest};
use formula_realize::rust_native::generate_u8_bool_rust_source;
use formula_store::authority_store::AuthorityStore;
use std::{fs, path::Path, process::Command};
use tempfile::tempdir;

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
    assert!(output.status.success(), "rustc -vV must succeed");
    assert!(output.stderr.is_empty(), "rustc -vV must not emit stderr");
    let version = String::from_utf8(output.stdout).expect("rustc -vV is UTF-8");
    let release = version
        .lines()
        .find_map(|line| line.strip_prefix("release: "))
        .expect("rustc release evidence");
    let host = version
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .expect("rustc host evidence");
    assert_eq!(release, "1.98.0", "P8 canonical Rust release changed");
    NativeToolchainIdentity::new(release.into(), host.into())
}

fn execute_native(binary: &Path, input: u8) -> bool {
    let output = Command::new(binary)
        .arg(input.to_string())
        .output()
        .expect("compiled native realization must execute");
    assert!(
        output.status.success(),
        "native realization failed for {input}"
    );
    assert!(
        output.stderr.is_empty(),
        "native realization emitted stderr for {input}"
    );
    match output.stdout.as_slice() {
        b"0\n" => false,
        b"1\n" => true,
        other => panic!("native realization emitted non-canonical output for {input}: {other:?}"),
    }
}

#[test]
fn fl_c_promoted_primitive_is_compiled_checked_admitted_and_dispatched_on_cpu() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);
    let u0_digest = store.initialize_genesis(&u0).unwrap();
    let u0_bytes = u0.canonical_bytes();
    let world = d("p7-fl-c-world");
    let authority_contract = d("p7-fl-c-authority-contract");
    let observer = d("p7-fl-c-observer");

    let context = CandidateSpaceContext::new(
        u0_digest,
        world,
        d("p7-fl-c-query"),
        d("p7-fl-c-obligation"),
        fl_c_grammar_digest(),
        d("p7-fl-c-search-policy"),
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
        .expect("bounded FL-C discovery converges within U8 counterexample bound");
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
        panic!("valid FL-C semantic primitive was quarantined")
    };
    let promotion_outcome = store.promote(&promotion_authorization).unwrap();
    let u1_digest = promotion_outcome.new_generation();
    let u1_before = store.replay_generation(u1_digest).unwrap();
    let u1_bytes = u1_before.canonical_bytes();
    assert!(u1_before.admitted().contains(&primitive));

    let specialization =
        SpecializationIdentity::new(primitive, u1_digest, world, authority_contract, observer);
    assert_eq!(specialization.semantic_target(), primitive);
    assert_eq!(specialization.universe_generation(), u1_digest);
    assert_eq!(specialization.lowering_class(), "EXACT_EQUIVALENCE");

    let generated = generate_u8_bool_rust_source(&engine_expression, &specialization).unwrap();
    let toolchain = canonical_rust_toolchain();
    let build_dir = tempdir().unwrap();
    let source_path = build_dir.path().join("p8_fl_c_native.rs");
    let binary_path = build_dir.path().join("p8_fl_c_native");
    fs::write(&source_path, generated.source().as_bytes()).unwrap();
    let compile_status = Command::new(toolchain.compiler())
        .arg(&source_path)
        .arg(toolchain.optimization())
        .arg("-o")
        .arg(&binary_path)
        .status()
        .expect("pinned rustc must compile generated source");
    assert!(
        compile_status.success(),
        "generated FL-C source must compile"
    );
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
    assert_eq!(native_manifest.lowering_class(), "EXACT_EQUIVALENCE");
    assert_eq!(native_manifest.source_digest(), generated.source_digest());
    assert_eq!(native_manifest.binary_digest(), binary_digest);

    let dispatch =
        RealizationDispatchContext::new(primitive, u1_digest, world, authority_contract, observer);
    assert!(store.resolve_realization(&dispatch).unwrap().is_none());

    let outputs: Vec<bool> = (0u16..=255)
        .map(|raw| execute_native(&binary_path, raw as u8))
        .collect();
    assert_eq!(outputs.len(), 256);
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

    let authorization = authorize_native_u8_realization_v1(
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

    let mut mutated_source = generated.source().as_bytes().to_vec();
    mutated_source.extend_from_slice(b"// source mutation\n");
    let source_error = authorize_native_u8_realization_v1(
        &native_manifest,
        &specialization,
        &toolchain,
        &check_manifest,
        &mutated_source,
        &binary_bytes,
        &semantic,
        &outputs,
    )
    .unwrap_err();
    assert_eq!(source_error, CheckFailure::RealizationSourceDigestMismatch);

    let mut mutated_binary = binary_bytes.clone();
    mutated_binary.push(0);
    let binary_error = authorize_native_u8_realization_v1(
        &native_manifest,
        &specialization,
        &toolchain,
        &check_manifest,
        generated.source().as_bytes(),
        &mutated_binary,
        &semantic,
        &outputs,
    )
    .unwrap_err();
    assert_eq!(
        binary_error,
        CheckFailure::RealizationArtifactDigestMismatch
    );

    let admitted = store
        .admit_realization(&authorization, &binary_bytes)
        .unwrap();
    assert_eq!(admitted.binary_digest(), binary_digest);
    let resolved = store.resolve_realization(&dispatch).unwrap().unwrap();
    assert_eq!(resolved.binary_digest(), binary_digest);
    assert_eq!(resolved.binary_bytes(), binary_bytes);

    let wrong_dispatch = RealizationDispatchContext::new(
        primitive,
        u1_digest,
        world,
        authority_contract,
        d("wrong-p8-observer"),
    );
    assert!(
        store
            .resolve_realization(&wrong_dispatch)
            .unwrap()
            .is_none()
    );

    let replayed_u0 = store.replay_generation(u0_digest).unwrap();
    assert_eq!(replayed_u0.digest(), u0_digest);
    assert_eq!(replayed_u0.canonical_bytes(), u0_bytes);
    let replayed_u1 = store.replay_generation(u1_digest).unwrap();
    assert_eq!(replayed_u1.digest(), u1_digest);
    assert_eq!(replayed_u1.canonical_bytes(), u1_bytes);
}
