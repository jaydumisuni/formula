use formula_check::modular_arithmetic::{
    ExtendedGcdOperation, ModularArithmeticError, ModularOperation, check_extended_gcd_result,
    check_modular_integer_result,
};
use num_bigint::BigInt;
use std::str::FromStr;

fn n(value: &str) -> BigInt {
    BigInt::from_str(value).expect("valid bigint")
}

#[test]
fn floor_mod_normalizes_negative_values() {
    let operation = ModularOperation::Mod {
        value: n("-8"),
        modulus: n("3"),
    };
    let receipt = check_modular_integer_result(&operation, "1").expect("exact mod");
    assert_eq!(receipt.result_decimal(), "1");
    assert_eq!(receipt.operation_digest(), operation.structural_digest());
}

#[test]
fn independently_checks_large_mul_mod() {
    let operation = ModularOperation::MulMod {
        lhs: n("340282366920938463463374607431768211507"),
        rhs: n("18446744073709551629"),
        modulus: n("6277101735386680763835789423207666416102355444464034512897"),
    };
    assert!(
        check_modular_integer_result(&operation, "4423670769972200025964653844372173882006",)
            .is_ok()
    );
}

#[test]
fn independently_checks_pow_mod_with_65537() {
    let operation = ModularOperation::PowMod {
        base: n("12345678901234567890123456789012345678901234567890"),
        exponent: n("65537"),
        modulus: n(
            "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031",
        ),
    };
    assert!(
        check_modular_integer_result(
            &operation,
            "13085734294248901071547503726829129848486365758767053187574898664226634276573482161831868354933113252323151",
        )
        .is_ok()
    );
}

#[test]
fn pow_mod_zero_exponent_and_modulus_one_are_exact() {
    let operation = ModularOperation::PowMod {
        base: n("-999999999999999999999999"),
        exponent: n("0"),
        modulus: n("1"),
    };
    assert!(check_modular_integer_result(&operation, "0").is_ok());
}

#[test]
fn invalid_modulus_and_negative_exponent_fail_closed() {
    for modulus in ["0", "-7"] {
        let operation = ModularOperation::Mod {
            value: n("5"),
            modulus: n(modulus),
        };
        assert_eq!(
            check_modular_integer_result(&operation, "0"),
            Err(ModularArithmeticError::InvalidModulus)
        );
    }

    let operation = ModularOperation::PowMod {
        base: n("2"),
        exponent: n("-1"),
        modulus: n("17"),
    };
    assert_eq!(
        check_modular_integer_result(&operation, "9"),
        Err(ModularArithmeticError::NegativeExponent)
    );
}

#[test]
fn gcd_is_non_negative_and_exact() {
    let operation = ModularOperation::Gcd {
        lhs: n("-240"),
        rhs: n("46"),
    };
    assert!(check_modular_integer_result(&operation, "2").is_ok());

    let zero = ModularOperation::Gcd {
        lhs: n("0"),
        rhs: n("0"),
    };
    assert!(check_modular_integer_result(&zero, "0").is_ok());
}

#[test]
fn extended_gcd_checks_bezout_not_one_specific_coefficient_pair() {
    let operation = ExtendedGcdOperation::new(n("240"), n("46"));

    assert!(check_extended_gcd_result(&operation, "2", "-9", "47").is_ok());
    assert!(check_extended_gcd_result(&operation, "2", "14", "-73").is_ok());
    assert_eq!(
        check_extended_gcd_result(&operation, "2", "-9", "46"),
        Err(ModularArithmeticError::IncorrectResult)
    );
}

#[test]
fn independently_checks_large_modular_inverse() {
    let operation = ModularOperation::ModInverse {
        value: n("123456789012345678901234567890123456789"),
        modulus: n("100000000000000000000000000000000000000000000000151"),
    };
    assert!(
        check_modular_integer_result(
            &operation,
            "67754844758800268780391605624395299367238732644747",
        )
        .is_ok()
    );
}

#[test]
fn inverse_is_normalized_for_negative_values() {
    let operation = ModularOperation::ModInverse {
        value: n("-3"),
        modulus: n("11"),
    };
    let receipt = check_modular_integer_result(&operation, "7").expect("normalized inverse");
    assert_eq!(receipt.result_decimal(), "7");
}

#[test]
fn noninvertible_or_degenerate_inverse_fails_closed() {
    let noninvertible = ModularOperation::ModInverse {
        value: n("12"),
        modulus: n("18"),
    };
    assert_eq!(
        check_modular_integer_result(&noninvertible, "0"),
        Err(ModularArithmeticError::NonInvertible)
    );

    let degenerate = ModularOperation::ModInverse {
        value: n("1"),
        modulus: n("1"),
    };
    assert_eq!(
        check_modular_integer_result(&degenerate, "0"),
        Err(ModularArithmeticError::InvalidModulus)
    );
}

#[test]
fn incorrect_and_malformed_results_are_rejected() {
    let operation = ModularOperation::Mod {
        value: n("10"),
        modulus: n("7"),
    };
    assert_eq!(
        check_modular_integer_result(&operation, "4"),
        Err(ModularArithmeticError::IncorrectResult)
    );

    for malformed in ["", "+3", "03", "-0", "3.0", " 3", "3 "] {
        assert_eq!(
            check_modular_integer_result(&operation, malformed),
            Err(ModularArithmeticError::MalformedDecimal)
        );
    }
}

#[test]
fn structural_identity_binds_modulus_and_operator() {
    let a = ModularOperation::Mod {
        value: n("9"),
        modulus: n("7"),
    };
    let b = ModularOperation::Mod {
        value: n("9"),
        modulus: n("5"),
    };
    let c = ModularOperation::Gcd {
        lhs: n("9"),
        rhs: n("7"),
    };

    assert_ne!(a.structural_digest(), b.structural_digest());
    assert_ne!(a.structural_digest(), c.structural_digest());
}
