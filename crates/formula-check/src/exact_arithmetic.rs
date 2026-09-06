use formula_core::{
    artifacts::StructuralIdentity, canonical::CanonicalValue, digest::ArtifactDigest,
};
use num_bigint::BigInt;
use std::collections::BTreeMap;

const EXACT_ARITHMETIC_SCHEMA_V1: &str = "formula-exact-arithmetic-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegerOperation {
    Add(BigInt, BigInt),
    Sub(BigInt, BigInt),
    Mul(BigInt, BigInt),
}

impl IntegerOperation {
    pub fn structural_digest(&self) -> ArtifactDigest {
        StructuralIdentity::structural_digest(self)
    }

    fn operator_name(&self) -> &'static str {
        match self {
            Self::Add(_, _) => "ADD",
            Self::Sub(_, _) => "SUB",
            Self::Mul(_, _) => "MUL",
        }
    }

    fn operands(&self) -> (&BigInt, &BigInt) {
        match self {
            Self::Add(lhs, rhs) | Self::Sub(lhs, rhs) | Self::Mul(lhs, rhs) => (lhs, rhs),
        }
    }

    fn recompute(&self) -> BigInt {
        match self {
            Self::Add(lhs, rhs) => lhs + rhs,
            Self::Sub(lhs, rhs) => lhs - rhs,
            Self::Mul(lhs, rhs) => lhs * rhs,
        }
    }
}

impl StructuralIdentity for IntegerOperation {
    fn canonical_value(&self) -> CanonicalValue {
        let (lhs, rhs) = self.operands();
        CanonicalValue::Object(BTreeMap::from([
            (
                "kind".into(),
                CanonicalValue::String("IntegerOperation".into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(EXACT_ARITHMETIC_SCHEMA_V1.into()),
            ),
            (
                "operator".into(),
                CanonicalValue::String(self.operator_name().into()),
            ),
            ("lhs".into(), CanonicalValue::Integer(lhs.clone())),
            ("rhs".into(), CanonicalValue::Integer(rhs.clone())),
        ]))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExactArithmeticError {
    MalformedDecimal,
    IncorrectResult,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactArithmeticReceipt {
    operation_digest: ArtifactDigest,
    evidence_digest: ArtifactDigest,
    result_decimal: String,
}

impl ExactArithmeticReceipt {
    pub fn operation_digest(&self) -> ArtifactDigest {
        self.operation_digest
    }

    pub fn evidence_digest(&self) -> ArtifactDigest {
        self.evidence_digest
    }

    pub fn result_decimal(&self) -> &str {
        &self.result_decimal
    }
}

pub fn check_decimal_integer_result(
    operation: &IntegerOperation,
    producer_result: &str,
) -> Result<ExactArithmeticReceipt, ExactArithmeticError> {
    let parsed = parse_canonical_decimal(producer_result)?;
    if parsed != operation.recompute() {
        return Err(ExactArithmeticError::IncorrectResult);
    }

    let operation_digest = operation.structural_digest();
    let evidence_digest = ArtifactDigest::of_bytes(
        format!("{}\n{}", operation_digest.as_str(), producer_result).as_bytes(),
    );

    Ok(ExactArithmeticReceipt {
        operation_digest,
        evidence_digest,
        result_decimal: producer_result.to_owned(),
    })
}

fn parse_canonical_decimal(input: &str) -> Result<BigInt, ExactArithmeticError> {
    if input.is_empty() || input.starts_with('+') {
        return Err(ExactArithmeticError::MalformedDecimal);
    }

    let bytes = input.as_bytes();
    let digits = if bytes[0] == b'-' {
        if bytes.len() == 1 || input == "-0" {
            return Err(ExactArithmeticError::MalformedDecimal);
        }
        &bytes[1..]
    } else {
        bytes
    };

    if digits.is_empty()
        || digits.iter().any(|byte| !byte.is_ascii_digit())
        || (digits.len() > 1 && digits[0] == b'0')
    {
        return Err(ExactArithmeticError::MalformedDecimal);
    }

    BigInt::parse_bytes(bytes, 10).ok_or(ExactArithmeticError::MalformedDecimal)
}
