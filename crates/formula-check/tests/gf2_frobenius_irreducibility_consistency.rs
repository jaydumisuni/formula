//! P2 source-only cross-family evidence: a Frobenius/GCD oracle checks
//! frozen GF(2)[x] irreducibility claims independently of divisor enumeration.
//! This regression grants no certificate, Judgement, or promotion authority.

use formula_check::gf2_polynomial_irreducibility::{
    Gf2PolynomialIrreducibilityError, check_frozen_gf2_polynomial_irreducibility,
};

// Polynomial long division over GF(2), used only by the test oracle.
// The production irreducibility checker instead enumerates possible divisors.
fn remainder(mut dividend: u32, divisor: u16) -> u16 {
    let divisor_degree = 15 - divisor.leading_zeros();
    while dividend != 0 {
        let dividend_degree = 31 - dividend.leading_zeros();
        if dividend_degree < divisor_degree {
            break;
        }
        dividend ^= u32::from(divisor) << (dividend_degree - divisor_degree);
    }
    dividend as u16
}

fn gcd(mut lhs: u16, mut rhs: u16) -> u16 {
    while rhs != 0 {
        let next = remainder(u32::from(lhs), rhs);
        lhs = rhs;
        rhs = next;
    }
    lhs
}

// In characteristic two, squaring doubles coefficient exponents.
// Every u16 residue has degree <= 15; its unreduced square fits in u32.
fn square_mod(value: u16, modulus: u16) -> u16 {
    let mut square = 0_u32;
    for bit in 0..=15 {
        if value & (1_u16 << bit) != 0 {
            square ^= 1_u32 << (2 * bit);
        }
    }
    remainder(square, modulus)
}

// Rabin's criterion: x^(2^n) == x (mod f), and
// gcd(f, x^(2^(n/q)) - x) == 1 for every prime q dividing n.
// This uses repeated Frobenius squaring and Euclidean GCD rather than
// enumerating nontrivial factors or trusting a supplied factor witness.
fn frobenius_irreducible(polynomial: u16) -> bool {
    let degree = 15 - polynomial.leading_zeros();
    let x_mod = remainder(2, polynomial);
    let mut powers = vec![x_mod];
    for _ in 0..degree {
        let next = square_mod(*powers.last().expect("initial x residue"), polynomial);
        powers.push(next);
    }
    if powers[degree as usize] != x_mod {
        return false;
    }
    for prime in [2_u32, 3, 5, 7, 11, 13] {
        if degree % prime == 0 && gcd(polynomial, powers[(degree / prime) as usize] ^ x_mod) != 1 {
            return false;
        }
    }
    true
}

#[test]
fn frobenius_gcd_oracle_cross_checks_full_u16_nonconstant_domain() {
    // All 65,534 nonconstant u16 coefficient bitsets, including degree-one
    // inputs where x must first be reduced modulo the polynomial.
    let mut irreducibles_by_degree = [0_usize; 16];
    for polynomial in 2_u16..=u16::MAX {
        let expected = frobenius_irreducible(polynomial);
        let degree = (15 - polynomial.leading_zeros()) as usize;
        if expected {
            irreducibles_by_degree[degree] += 1;
        }
        let [high, low] = polynomial.to_be_bytes();
        assert_eq!(
            check_frozen_gf2_polynomial_irreducibility(&[high, low, u8::from(expected)]),
            Ok(()),
            "Frobenius/GCD classification polynomial={polynomial:#x}"
        );
        assert_eq!(
            check_frozen_gf2_polynomial_irreducibility(&[high, low, u8::from(!expected)]),
            Err(Gf2PolynomialIrreducibilityError::IrreducibilityMismatch {
                expected,
                actual: !expected,
            }),
            "forged Frobenius/GCD claim polynomial={polynomial:#x}"
        );
    }
    // Known monic irreducible counts over GF(2), degrees 1 through 15.
    assert_eq!(
        irreducibles_by_degree,
        [0, 2, 1, 2, 3, 6, 9, 18, 30, 56, 99, 186, 335, 630, 1161, 2182]
    );
    assert_eq!(irreducibles_by_degree.iter().sum::<usize>(), 4720);
}
