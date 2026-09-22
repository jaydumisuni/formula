use crate::exact_arithmetic::parse_canonical_decimal;
use formula_core::{
    artifacts::StructuralIdentity, canonical::CanonicalValue, digest::ArtifactDigest,
};
use num_bigint::{BigInt, Sign};
use std::collections::BTreeMap;

const VERIFIER_EQUIVALENCE_SCHEMA_V1: &str = "formula-verifier-equivalence-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerifierEquivalenceError {
    MalformedDecimal,
    MalformedHex,
    IncorrectResult,
    InvalidModulus,
    InvalidExponent,
    InvalidRadix,
    OperandOutOfRange,
    ProducerOutOfRange,
    InvalidWidth,
    InvalidType1Block,
    PayloadMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MontgomeryProductOperation {
    lhs: BigInt,
    rhs: BigInt,
    modulus: BigInt,
    radix_bits: usize,
}

impl MontgomeryProductOperation {
    pub fn new(lhs: BigInt, rhs: BigInt, modulus: BigInt, radix_bits: usize) -> Self {
        Self {
            lhs,
            rhs,
            modulus,
            radix_bits,
        }
    }

    pub fn structural_digest(&self) -> ArtifactDigest {
        StructuralIdentity::structural_digest(self)
    }

    fn expected(&self) -> Result<BigInt, VerifierEquivalenceError> {
        validate_montgomery_domain(self)?;

        let radix = BigInt::from(1u8) << self.radix_bits;
        let inverse = radix
            .modinv(&self.modulus)
            .ok_or(VerifierEquivalenceError::InvalidRadix)?;
        Ok(floor_mod(&(&self.lhs * &self.rhs * inverse), &self.modulus))
    }
}

impl StructuralIdentity for MontgomeryProductOperation {
    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "kind".into(),
                CanonicalValue::String("MontgomeryProductOperation".into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(VERIFIER_EQUIVALENCE_SCHEMA_V1.into()),
            ),
            ("lhs".into(), CanonicalValue::Integer(self.lhs.clone())),
            ("rhs".into(), CanonicalValue::Integer(self.rhs.clone())),
            (
                "modulus".into(),
                CanonicalValue::Integer(self.modulus.clone()),
            ),
            (
                "radix_bits".into(),
                CanonicalValue::Integer(BigInt::from(self.radix_bits)),
            ),
            (
                "reduction_contract".into(),
                CanonicalValue::String("SINGLE_CONDITIONAL_SUBTRACTION".into()),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MontgomeryProductReceipt {
    operation_digest: ArtifactDigest,
    evidence_digest: ArtifactDigest,
    producer_decimal: String,
    normalized_decimal: String,
    required_subtraction: bool,
}

impl MontgomeryProductReceipt {
    pub fn operation_digest(&self) -> ArtifactDigest {
        self.operation_digest
    }

    pub fn evidence_digest(&self) -> ArtifactDigest {
        self.evidence_digest
    }

    pub fn producer_decimal(&self) -> &str {
        &self.producer_decimal
    }

    pub fn normalized_decimal(&self) -> &str {
        &self.normalized_decimal
    }

    pub fn required_subtraction(&self) -> bool {
        self.required_subtraction
    }
}

pub fn check_montgomery_product_result(
    operation: &MontgomeryProductOperation,
    producer_decimal: &str,
) -> Result<MontgomeryProductReceipt, VerifierEquivalenceError> {
    let producer = parse_decimal(producer_decimal)?;
    let expected = operation.expected()?;

    if producer < BigInt::from(0) || producer >= (&operation.modulus << 1usize) {
        return Err(VerifierEquivalenceError::ProducerOutOfRange);
    }

    let required_subtraction = producer >= operation.modulus;
    let normalized = if required_subtraction {
        &producer - &operation.modulus
    } else {
        producer.clone()
    };

    if normalized != expected {
        return Err(VerifierEquivalenceError::IncorrectResult);
    }

    let operation_digest = operation.structural_digest();
    let evidence_digest = ArtifactDigest::of_bytes(
        format!(
            "{}\n{}\n{}",
            operation_digest.as_str(),
            producer_decimal,
            normalized
        )
        .as_bytes(),
    );

    Ok(MontgomeryProductReceipt {
        operation_digest,
        evidence_digest,
        producer_decimal: producer_decimal.to_owned(),
        normalized_decimal: normalized.to_string(),
        required_subtraction,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Type1PayloadOperation {
    block_hex: String,
    width_bytes: usize,
    expected_payload_hex: String,
}

impl Type1PayloadOperation {
    pub fn new(
        block_hex: impl Into<String>,
        width_bytes: usize,
        expected_payload_hex: impl Into<String>,
    ) -> Self {
        Self {
            block_hex: block_hex.into(),
            width_bytes,
            expected_payload_hex: expected_payload_hex.into(),
        }
    }

    pub fn structural_digest(&self) -> ArtifactDigest {
        StructuralIdentity::structural_digest(self)
    }
}

impl StructuralIdentity for Type1PayloadOperation {
    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "kind".into(),
                CanonicalValue::String("Type1PayloadOperation".into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(VERIFIER_EQUIVALENCE_SCHEMA_V1.into()),
            ),
            (
                "block_hex".into(),
                CanonicalValue::String(self.block_hex.clone()),
            ),
            (
                "width_bytes".into(),
                CanonicalValue::Integer(BigInt::from(self.width_bytes)),
            ),
            (
                "expected_payload_hex".into(),
                CanonicalValue::String(self.expected_payload_hex.clone()),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Type1PayloadReceipt {
    operation_digest: ArtifactDigest,
    evidence_digest: ArtifactDigest,
    payload_hex: String,
    padding_bytes: usize,
}

impl Type1PayloadReceipt {
    pub fn operation_digest(&self) -> ArtifactDigest {
        self.operation_digest
    }

    pub fn evidence_digest(&self) -> ArtifactDigest {
        self.evidence_digest
    }

    pub fn payload_hex(&self) -> &str {
        &self.payload_hex
    }

    pub fn padding_bytes(&self) -> usize {
        self.padding_bytes
    }
}

pub fn check_type1_payload(
    operation: &Type1PayloadOperation,
) -> Result<Type1PayloadReceipt, VerifierEquivalenceError> {
    let block = decode_fixed_hex(&operation.block_hex, operation.width_bytes)?;
    let payload = decode_variable_hex(&operation.expected_payload_hex)?;
    let padding_bytes = validate_type1_block(&block, &payload)?;

    let operation_digest = operation.structural_digest();
    let evidence_digest = ArtifactDigest::of_bytes(
        format!(
            "{}\n{}\n{}",
            operation_digest.as_str(),
            operation.block_hex,
            operation.expected_payload_hex
        )
        .as_bytes(),
    );

    Ok(Type1PayloadReceipt {
        operation_digest,
        evidence_digest,
        payload_hex: operation.expected_payload_hex.clone(),
        padding_bytes,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RsaType1VerifierOperation {
    representative: BigInt,
    exponent: BigInt,
    modulus: BigInt,
    expected_payload_hex: String,
}

impl RsaType1VerifierOperation {
    pub fn new(
        representative: BigInt,
        exponent: BigInt,
        modulus: BigInt,
        expected_payload_hex: impl Into<String>,
    ) -> Self {
        Self {
            representative,
            exponent,
            modulus,
            expected_payload_hex: expected_payload_hex.into(),
        }
    }

    pub fn structural_digest(&self) -> ArtifactDigest {
        StructuralIdentity::structural_digest(self)
    }
}

impl StructuralIdentity for RsaType1VerifierOperation {
    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "kind".into(),
                CanonicalValue::String("RsaType1VerifierOperation".into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(VERIFIER_EQUIVALENCE_SCHEMA_V1.into()),
            ),
            (
                "representative".into(),
                CanonicalValue::Integer(self.representative.clone()),
            ),
            (
                "exponent".into(),
                CanonicalValue::Integer(self.exponent.clone()),
            ),
            (
                "modulus".into(),
                CanonicalValue::Integer(self.modulus.clone()),
            ),
            (
                "expected_payload_hex".into(),
                CanonicalValue::String(self.expected_payload_hex.clone()),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RsaType1VerifierReceipt {
    operation_digest: ArtifactDigest,
    evidence_digest: ArtifactDigest,
    recovered_block_digest: ArtifactDigest,
    payload_hex: String,
    padding_bytes: usize,
}

impl RsaType1VerifierReceipt {
    pub fn operation_digest(&self) -> ArtifactDigest {
        self.operation_digest
    }

    pub fn evidence_digest(&self) -> ArtifactDigest {
        self.evidence_digest
    }

    pub fn recovered_block_digest(&self) -> ArtifactDigest {
        self.recovered_block_digest
    }

    pub fn payload_hex(&self) -> &str {
        &self.payload_hex
    }

    pub fn padding_bytes(&self) -> usize {
        self.padding_bytes
    }
}

pub fn check_rsa_type1_payload(
    operation: &RsaType1VerifierOperation,
) -> Result<RsaType1VerifierReceipt, VerifierEquivalenceError> {
    validate_rsa_domain(operation)?;

    let payload = decode_variable_hex(&operation.expected_payload_hex)?;
    let recovered = operation
        .representative
        .modpow(&operation.exponent, &operation.modulus);
    let width = modulus_width_bytes(&operation.modulus)?;
    let block = encode_unsigned_be(&recovered, width)?;
    let padding_bytes = validate_type1_block(&block, &payload)?;
    let operation_digest = operation.structural_digest();
    let recovered_block_digest = ArtifactDigest::of_bytes(&block);
    let evidence_digest = ArtifactDigest::of_bytes(
        format!(
            "{}\n{}\n{}",
            operation_digest.as_str(),
            recovered_block_digest.as_str(),
            operation.expected_payload_hex
        )
        .as_bytes(),
    );

    Ok(RsaType1VerifierReceipt {
        operation_digest,
        evidence_digest,
        recovered_block_digest,
        payload_hex: operation.expected_payload_hex.clone(),
        padding_bytes,
    })
}

fn validate_montgomery_domain(
    operation: &MontgomeryProductOperation,
) -> Result<(), VerifierEquivalenceError> {
    if operation.modulus <= BigInt::from(1)
        || (&operation.modulus & BigInt::from(1u8)) == BigInt::from(0)
    {
        return Err(VerifierEquivalenceError::InvalidModulus);
    }

    let radix = BigInt::from(1u8) << operation.radix_bits;
    if operation.radix_bits == 0 || radix <= operation.modulus {
        return Err(VerifierEquivalenceError::InvalidRadix);
    }

    if operation.lhs < BigInt::from(0)
        || operation.rhs < BigInt::from(0)
        || operation.lhs >= operation.modulus
        || operation.rhs >= operation.modulus
    {
        return Err(VerifierEquivalenceError::OperandOutOfRange);
    }

    Ok(())
}

fn validate_rsa_domain(
    operation: &RsaType1VerifierOperation,
) -> Result<(), VerifierEquivalenceError> {
    if operation.modulus <= BigInt::from(1) {
        return Err(VerifierEquivalenceError::InvalidModulus);
    }
    if operation.exponent <= BigInt::from(0) {
        return Err(VerifierEquivalenceError::InvalidExponent);
    }
    if operation.representative < BigInt::from(0) || operation.representative >= operation.modulus {
        return Err(VerifierEquivalenceError::OperandOutOfRange);
    }
    Ok(())
}

fn validate_type1_block(
    block: &[u8],
    expected_payload: &[u8],
) -> Result<usize, VerifierEquivalenceError> {
    if block.len() < 11 || expected_payload.len() > block.len().saturating_sub(11) {
        return Err(VerifierEquivalenceError::InvalidType1Block);
    }
    if block[0] != 0 || block[1] != 1 {
        return Err(VerifierEquivalenceError::InvalidType1Block);
    }

    let separator = block.len() - expected_payload.len() - 1;
    if separator < 10 || block[separator] != 0 {
        return Err(VerifierEquivalenceError::InvalidType1Block);
    }
    if block[2..separator].iter().any(|byte| *byte != 0xff) {
        return Err(VerifierEquivalenceError::InvalidType1Block);
    }
    if block[separator + 1..] != *expected_payload {
        return Err(VerifierEquivalenceError::PayloadMismatch);
    }

    Ok(separator - 2)
}

fn floor_mod(value: &BigInt, modulus: &BigInt) -> BigInt {
    let residue = value % modulus;
    if residue < BigInt::from(0) {
        residue + modulus
    } else {
        residue
    }
}

fn modulus_width_bytes(modulus: &BigInt) -> Result<usize, VerifierEquivalenceError> {
    let bits = modulus.bits();
    usize::try_from(bits.div_ceil(8)).map_err(|_| VerifierEquivalenceError::InvalidWidth)
}

fn encode_unsigned_be(
    value: &BigInt,
    width_bytes: usize,
) -> Result<Vec<u8>, VerifierEquivalenceError> {
    if width_bytes == 0 || value.sign() == Sign::Minus {
        return Err(VerifierEquivalenceError::InvalidWidth);
    }

    let (_, magnitude) = value.to_bytes_be();
    if magnitude.len() > width_bytes {
        return Err(VerifierEquivalenceError::InvalidWidth);
    }

    let mut bytes = vec![0u8; width_bytes - magnitude.len()];
    bytes.extend_from_slice(&magnitude);
    Ok(bytes)
}

fn decode_fixed_hex(input: &str, width_bytes: usize) -> Result<Vec<u8>, VerifierEquivalenceError> {
    if width_bytes == 0 {
        return Err(VerifierEquivalenceError::InvalidWidth);
    }
    let expected_len = width_bytes
        .checked_mul(2)
        .ok_or(VerifierEquivalenceError::InvalidWidth)?;
    if input.len() != expected_len {
        return Err(VerifierEquivalenceError::MalformedHex);
    }
    decode_hex(input)
}

fn decode_variable_hex(input: &str) -> Result<Vec<u8>, VerifierEquivalenceError> {
    if !input.len().is_multiple_of(2) {
        return Err(VerifierEquivalenceError::MalformedHex);
    }
    decode_hex(input)
}

fn decode_hex(input: &str) -> Result<Vec<u8>, VerifierEquivalenceError> {
    let raw = input.as_bytes();
    let mut out = Vec::with_capacity(raw.len() / 2);
    for pair in raw.as_chunks::<2>().0 {
        let high = decode_hex_nibble(pair[0])?;
        let low = decode_hex_nibble(pair[1])?;
        out.push((high << 4) | low);
    }
    Ok(out)
}

fn decode_hex_nibble(byte: u8) -> Result<u8, VerifierEquivalenceError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(VerifierEquivalenceError::MalformedHex),
    }
}

fn parse_decimal(input: &str) -> Result<BigInt, VerifierEquivalenceError> {
    parse_canonical_decimal(input).map_err(|_| VerifierEquivalenceError::MalformedDecimal)
}
