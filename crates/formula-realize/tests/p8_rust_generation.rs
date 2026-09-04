use formula_core::{digest::ArtifactDigest, realization::SpecializationIdentity};
use formula_engine::observational::{BoolExpr, ByteExpr};
use formula_realize::rust_native::{generate_u8_bool_rust_source, NativeGenerationError};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn specialization(semantic_target: ArtifactDigest) -> SpecializationIdentity {
    SpecializationIdentity::new(
        semantic_target,
        d("u1"),
        d("world"),
        d("authority"),
        d("observer"),
    )
}

fn power_of_two_expr() -> BoolExpr {
    BoolExpr::and(
        BoolExpr::neq_zero(ByteExpr::x()),
        BoolExpr::eq_zero(ByteExpr::bit_and(
            ByteExpr::x(),
            ByteExpr::sub_wrap(ByteExpr::x(), ByteExpr::one()),
        )),
    )
}

#[test]
fn native_rust_generation_is_deterministic_and_covers_bounded_ast() {
    let expr = power_of_two_expr();
    let specialization = specialization(expr.digest());

    let a = generate_u8_bool_rust_source(&expr, &specialization).unwrap();
    let b = generate_u8_bool_rust_source(&expr, &specialization).unwrap();

    assert_eq!(a.source(), b.source());
    assert_eq!(a.source_digest(), b.source_digest());
    assert_eq!(a.source_digest(), ArtifactDigest::of_bytes(a.source().as_bytes()));
    assert!(a.source().contains("wrapping_sub"));
    assert!(a.source().contains("&"));
    assert!(a.source().contains("== 0"));
    assert!(a.source().contains("!= 0"));
    assert!(a.source().contains("&&"));
    assert!(a.source().contains("std::process::exit(2);"));
    assert!(
        a.source()
            .contains("println!(\"{}\", if result { 1 } else { 0 });")
    );
}

#[test]
fn generator_handles_zero_leaf_without_environment_or_fixture_inputs() {
    let expr = BoolExpr::eq_zero(ByteExpr::zero());
    let specialization = specialization(expr.digest());

    let generated = generate_u8_bool_rust_source(&expr, &specialization).unwrap();

    assert!(generated.source().contains("0u8"));
    assert!(!generated.source().contains("env::var"));
    assert!(!generated.source().contains("formula_first_light"));
    assert!(!generated.source().contains("std::net"));
}

#[test]
fn semantic_target_mismatch_fails_before_source_generation() {
    let expr = power_of_two_expr();
    let wrong_specialization = specialization(d("different-semantic-target"));

    let error = generate_u8_bool_rust_source(&expr, &wrong_specialization).unwrap_err();

    assert_eq!(error, NativeGenerationError::SemanticTargetMismatch);
}
