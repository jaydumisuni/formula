use formula_check::exact_arithmetic::{
    ExactArithmeticError, IntegerOperation, check_decimal_integer_result,
};
use num_bigint::BigInt;
use std::str::FromStr;

fn n(value: &str) -> BigInt {
    BigInt::from_str(value).expect("valid bigint")
}

#[test]
fn independently_checks_integer_addition_beyond_u128() {
    let operation = IntegerOperation::Add(
        n("340282366920938463463374607431768211507"),
        n("18446744073709551629"),
    );
    let receipt =
        check_decimal_integer_result(&operation, "340282366920938463481821351505477763136")
            .expect("exact result must verify");

    assert_eq!(
        receipt.result_decimal(),
        "340282366920938463481821351505477763136"
    );
    assert_eq!(receipt.operation_digest(), operation.structural_digest());
    assert_ne!(receipt.evidence_digest(), operation.structural_digest());
}

#[test]
fn independently_checks_large_integer_multiplication() {
    let operation = IntegerOperation::Mul(
        n("340282366920938463463374607431768211507"),
        n("18446744073709551629"),
    );
    assert!(
        check_decimal_integer_result(
            &operation,
            "6277101735386680768259460193179866442067009288836208394903",
        )
        .is_ok()
    );
}

#[test]
fn incorrect_external_result_is_rejected() {
    let operation = IntegerOperation::Sub(n("900000000000000000000"), n("1"));
    assert_eq!(
        check_decimal_integer_result(&operation, "900000000000000000000"),
        Err(ExactArithmeticError::IncorrectResult)
    );
}

#[test]
fn malformed_or_noncanonical_decimal_is_rejected() {
    let operation = IntegerOperation::Add(n("1"), n("2"));
    for malformed in ["", "+3", "03", "-0", "3.0", " 3", "3 ", "three"] {
        assert_eq!(
            check_decimal_integer_result(&operation, malformed),
            Err(ExactArithmeticError::MalformedDecimal)
        );
    }
}

#[test]
fn canonical_negative_result_is_supported() {
    let operation = IntegerOperation::Sub(
        n("18446744073709551629"),
        n("340282366920938463463374607431768211507"),
    );
    assert!(
        check_decimal_integer_result(&operation, "-340282366920938463444927863358058659878",)
            .is_ok()
    );
}
