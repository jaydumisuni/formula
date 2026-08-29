use crate::verdict::{CheckFailure, CheckVerdict};
use num_bigint::BigInt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegerPolynomial {
    coefficients: Vec<BigInt>,
}

impl IntegerPolynomial {
    pub fn new(mut coefficients: Vec<BigInt>) -> Self {
        while coefficients.len() > 1 && coefficients.last() == Some(&BigInt::from(0)) {
            coefficients.pop();
        }
        if coefficients.is_empty() {
            coefficients.push(BigInt::from(0));
        }
        Self { coefficients }
    }

    pub fn from_i64(coefficients: &[i64]) -> Self {
        Self::new(coefficients.iter().copied().map(BigInt::from).collect())
    }

    pub fn coefficients(&self) -> &[BigInt] {
        &self.coefficients
    }
}

pub fn check_polynomial_identity(
    expected: &IntegerPolynomial,
    candidate: &IntegerPolynomial,
) -> CheckVerdict {
    if expected.coefficients == candidate.coefficients {
        CheckVerdict::Pass
    } else {
        CheckVerdict::Fail(CheckFailure::SemanticMismatch)
    }
}
