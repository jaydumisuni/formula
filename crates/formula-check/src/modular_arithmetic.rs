use crate::exact_arithmetic::parse_canonical_decimal;
use formula_core::{
    artifacts::StructuralIdentity, canonical::CanonicalValue, digest::ArtifactDigest,
};
use num_bigint::BigInt;
use std::collections::BTreeMap;

const MODULAR_ARITHMETIC_SCHEMA_V1: &str = "formula-modular-arithmetic-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModularOperation {
    Mod {
        value: BigInt,
        modulus: BigInt,
    },
    MulMod {
        lhs: BigInt,
        rhs: BigInt,
        modulus: BigInt,
    },
    PowMod {
        base: BigInt,
        exponent: BigInt,
        modulus: BigInt,
    },
    Gcd {
        lhs: BigInt,
        rhs: BigInt,
    },
    ModInverse {
        value: BigInt,
        modulus: BigInt,
    },
}

impl ModularOperation {
    pub fn structural_digest(&self) -> ArtifactDigest {
        StructuralIdentity::structural_digest(self)
    }

    fn operator_name(&self) -> &'static str {
        match self {
            Self::Mod { .. } => "MOD",
            Self::MulMod { .. } => "MUL_MOD",
            Self::PowMod { .. } => "POW_MOD",
            Self::Gcd { .. } => "GCD",
            Self::ModInverse { .. } => "MOD_INVERSE",
        }
    }

    fn recompute(&self) -> Result<BigInt, ModularArithmeticError> {
        match self {
            Self::Mod { value, modulus } => {
                require_positive_modulus(modulus)?;
                Ok(floor_mod(value, modulus))
            }
            Self::MulMod { lhs, rhs, modulus } => {
                require_positive_modulus(modulus)?;
                Ok(floor_mod(&(lhs * rhs), modulus))
            }
            Self::PowMod {
                base,
                exponent,
                modulus,
            } => {
                require_positive_modulus(modulus)?;
                if exponent < &BigInt::from(0) {
                    return Err(ModularArithmeticError::NegativeExponent);
                }
                Ok(base.modpow(exponent, modulus))
            }
            Self::Gcd { lhs, rhs } => Ok(gcd_nonnegative(lhs, rhs)),
            Self::ModInverse { value, modulus } => {
                if modulus <= &BigInt::from(1) {
                    return Err(ModularArithmeticError::InvalidModulus);
                }
                value
                    .modinv(modulus)
                    .ok_or(ModularArithmeticError::NonInvertible)
            }
        }
    }
}

impl StructuralIdentity for ModularOperation {
    fn canonical_value(&self) -> CanonicalValue {
        let mut fields = BTreeMap::from([
            (
                "kind".into(),
                CanonicalValue::String("ModularOperation".into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(MODULAR_ARITHMETIC_SCHEMA_V1.into()),
            ),
            (
                "operator".into(),
                CanonicalValue::String(self.operator_name().into()),
            ),
        ]);

        match self {
            Self::Mod { value, modulus } | Self::ModInverse { value, modulus } => {
                fields.insert("value".into(), CanonicalValue::Integer(value.clone()));
                fields.insert("modulus".into(), CanonicalValue::Integer(modulus.clone()));
            }
            Self::MulMod { lhs, rhs, modulus } => {
                fields.insert("lhs".into(), CanonicalValue::Integer(lhs.clone()));
                fields.insert("rhs".into(), CanonicalValue::Integer(rhs.clone()));
                fields.insert("modulus".into(), CanonicalValue::Integer(modulus.clone()));
            }
            Self::PowMod {
                base,
                exponent,
                modulus,
            } => {
                fields.insert("base".into(), CanonicalValue::Integer(base.clone()));
                fields.insert("exponent".into(), CanonicalValue::Integer(exponent.clone()));
                fields.insert("modulus".into(), CanonicalValue::Integer(modulus.clone()));
            }
            Self::Gcd { lhs, rhs } => {
                fields.insert("lhs".into(), CanonicalValue::Integer(lhs.clone()));
                fields.insert("rhs".into(), CanonicalValue::Integer(rhs.clone()));
            }
        }

        CanonicalValue::Object(fields)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtendedGcdOperation {
    lhs: BigInt,
    rhs: BigInt,
}

impl ExtendedGcdOperation {
    pub fn new(lhs: BigInt, rhs: BigInt) -> Self {
        Self { lhs, rhs }
    }

    pub fn lhs(&self) -> &BigInt {
        &self.lhs
    }

    pub fn rhs(&self) -> &BigInt {
        &self.rhs
    }

    pub fn structural_digest(&self) -> ArtifactDigest {
        StructuralIdentity::structural_digest(self)
    }
}

impl StructuralIdentity for ExtendedGcdOperation {
    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "kind".into(),
                CanonicalValue::String("ExtendedGcdOperation".into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(MODULAR_ARITHMETIC_SCHEMA_V1.into()),
            ),
            (
                "operator".into(),
                CanonicalValue::String("EXTENDED_GCD".into()),
            ),
            ("lhs".into(), CanonicalValue::Integer(self.lhs.clone())),
            ("rhs".into(), CanonicalValue::Integer(self.rhs.clone())),
        ]))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModularArithmeticError {
    MalformedDecimal,
    IncorrectResult,
    InvalidModulus,
    NegativeExponent,
    NonInvertible,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModularArithmeticReceipt {
    operation_digest: ArtifactDigest,
    evidence_digest: ArtifactDigest,
    result_decimal: String,
}

impl ModularArithmeticReceipt {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtendedGcdReceipt {
    operation_digest: ArtifactDigest,
    evidence_digest: ArtifactDigest,
    gcd_decimal: String,
    x_decimal: String,
    y_decimal: String,
}

impl ExtendedGcdReceipt {
    pub fn operation_digest(&self) -> ArtifactDigest {
        self.operation_digest
    }

    pub fn evidence_digest(&self) -> ArtifactDigest {
        self.evidence_digest
    }

    pub fn gcd_decimal(&self) -> &str {
        &self.gcd_decimal
    }

    pub fn x_decimal(&self) -> &str {
        &self.x_decimal
    }

    pub fn y_decimal(&self) -> &str {
        &self.y_decimal
    }
}

pub fn check_modular_integer_result(
    operation: &ModularOperation,
    producer_result: &str,
) -> Result<ModularArithmeticReceipt, ModularArithmeticError> {
    let parsed = parse_decimal(producer_result)?;
    let expected = operation.recompute()?;
    if parsed != expected {
        return Err(ModularArithmeticError::IncorrectResult);
    }

    let operation_digest = operation.structural_digest();
    let evidence_digest = ArtifactDigest::of_bytes(
        format!("{}\n{}", operation_digest.as_str(), producer_result).as_bytes(),
    );

    Ok(ModularArithmeticReceipt {
        operation_digest,
        evidence_digest,
        result_decimal: producer_result.to_owned(),
    })
}

pub fn check_extended_gcd_result(
    operation: &ExtendedGcdOperation,
    gcd_result: &str,
    x_result: &str,
    y_result: &str,
) -> Result<ExtendedGcdReceipt, ModularArithmeticError> {
    let gcd = parse_decimal(gcd_result)?;
    let x = parse_decimal(x_result)?;
    let y = parse_decimal(y_result)?;

    let expected_gcd = gcd_nonnegative(&operation.lhs, &operation.rhs);
    if gcd != expected_gcd || &operation.lhs * &x + &operation.rhs * &y != gcd {
        return Err(ModularArithmeticError::IncorrectResult);
    }

    let operation_digest = operation.structural_digest();
    let evidence_digest = ArtifactDigest::of_bytes(
        format!(
            "{}\n{}\n{}\n{}",
            operation_digest.as_str(),
            gcd_result,
            x_result,
            y_result
        )
        .as_bytes(),
    );

    Ok(ExtendedGcdReceipt {
        operation_digest,
        evidence_digest,
        gcd_decimal: gcd_result.to_owned(),
        x_decimal: x_result.to_owned(),
        y_decimal: y_result.to_owned(),
    })
}

fn floor_mod(value: &BigInt, modulus: &BigInt) -> BigInt {
    let residue = value % modulus;
    if residue < BigInt::from(0) {
        residue + modulus
    } else {
        residue
    }
}

fn gcd_nonnegative(lhs: &BigInt, rhs: &BigInt) -> BigInt {
    let mut a = if lhs < &BigInt::from(0) {
        -lhs
    } else {
        lhs.clone()
    };
    let mut b = if rhs < &BigInt::from(0) {
        -rhs
    } else {
        rhs.clone()
    };
    while b != BigInt::from(0) {
        let remainder = &a % &b;
        a = b;
        b = remainder;
    }
    a
}

fn require_positive_modulus(modulus: &BigInt) -> Result<(), ModularArithmeticError> {
    if modulus <= &BigInt::from(0) {
        return Err(ModularArithmeticError::InvalidModulus);
    }
    Ok(())
}

fn parse_decimal(input: &str) -> Result<BigInt, ModularArithmeticError> {
    parse_canonical_decimal(input).map_err(|_| ModularArithmeticError::MalformedDecimal)
}
