use formula_check::{
    u8::{BoolExpr, ByteExpr, check_u8_equivalence},
    verdict::{CheckFailure, CheckVerdict},
};

fn sub_one() -> ByteExpr {
    ByteExpr::SubWrap(Box::new(ByteExpr::X), Box::new(ByteExpr::Const(1)))
}

fn power_of_two_candidate() -> BoolExpr {
    BoolExpr::And(
        Box::new(BoolExpr::NeqZero(ByteExpr::X)),
        Box::new(BoolExpr::EqZero(ByteExpr::BitAnd(
            Box::new(ByteExpr::X),
            Box::new(sub_one()),
        ))),
    )
}

fn power_of_two_spec(x: u8) -> bool {
    x != 0 && (x & x.wrapping_sub(1)) == 0
}

#[test]
fn power_of_two_candidate_with_nonzero_guard_passes_all_256_inputs() {
    assert_eq!(
        check_u8_equivalence(&power_of_two_candidate(), power_of_two_spec),
        CheckVerdict::Pass
    );
}

#[test]
fn classic_missing_zero_guard_near_miss_fails_at_zero() {
    let near_miss = BoolExpr::EqZero(ByteExpr::BitAnd(Box::new(ByteExpr::X), Box::new(sub_one())));
    assert_eq!(
        check_u8_equivalence(&near_miss, power_of_two_spec),
        CheckVerdict::Fail(CheckFailure::U8Counterexample(0))
    );
}

#[test]
fn subtraction_is_explicit_u8_wrapping_semantics() {
    assert_eq!(sub_one().evaluate(0), 255);
    assert_eq!(sub_one().evaluate(1), 0);
}

#[test]
fn one_changed_operator_returns_exact_counterexample() {
    let changed = BoolExpr::And(
        Box::new(BoolExpr::NeqZero(ByteExpr::X)),
        Box::new(BoolExpr::EqZero(ByteExpr::BitOr(
            Box::new(ByteExpr::X),
            Box::new(sub_one()),
        ))),
    );
    assert_eq!(
        check_u8_equivalence(&changed, power_of_two_spec),
        CheckVerdict::Fail(CheckFailure::U8Counterexample(1))
    );
}
