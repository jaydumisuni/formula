use crate::exact_arithmetic::parse_canonical_decimal;
use formula_core::{
    artifacts::StructuralIdentity, canonical::CanonicalValue, digest::ArtifactDigest,
};
use num_bigint::{BigInt, Sign};
use std::collections::BTreeMap;

const RSA_PUBLIC_SCHEMA_V1: &str = "formula-rsa-public-representation-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicRepresentationError {
    MalformedDecimal,
    MalformedHex,
    IncorrectResult,
    InvalidWidth,
    NegativeValue,
    WidthOverflow,
    InvalidModulus,
    InvalidExponent,
    RepresentativeOutOfRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedWidthEncodeOperation {
    value: BigInt,
    width_bytes: usize,
}

impl FixedWidthEncodeOperation {
    pub fn new(value: BigInt, width_bytes: usize) -> Self {
        Self { value, width_bytes }
    }

    pub fn value(&self) -> &BigInt {
        &self.value
    }

    pub fn width_bytes(&self) -> usize {
        self.width_bytes
    }

    pub fn structural_digest(&self) -> ArtifactDigest {
        StructuralIdentity::structural_digest(self)
    }
}

impl StructuralIdentity for FixedWidthEncodeOperation {
    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "kind".into(),
                CanonicalValue::String("FixedWidthEncodeOperation".into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(RSA_PUBLIC_SCHEMA_V1.into()),
            ),
            (
                "encoding".into(),
                CanonicalValue::String("UNSIGNED_BIG_ENDIAN".into()),
            ),
            ("value".into(), CanonicalValue::Integer(self.value.clone())),
            (
                "width_bytes".into(),
                CanonicalValue::Integer(BigInt::from(self.width_bytes)),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedWidthDecodeOperation {
    hex: String,
    width_bytes: usize,
}

impl FixedWidthDecodeOperation {
    pub fn new(hex: impl Into<String>, width_bytes: usize) -> Self {
        Self {
            hex: hex.into(),
            width_bytes,
        }
    }

    pub fn hex(&self) -> &str {
        &self.hex
    }

    pub fn width_bytes(&self) -> usize {
        self.width_bytes
    }

    pub fn structural_digest(&self) -> ArtifactDigest {
        StructuralIdentity::structural_digest(self)
    }
}

impl StructuralIdentity for FixedWidthDecodeOperation {
    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "kind".into(),
                CanonicalValue::String("FixedWidthDecodeOperation".into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(RSA_PUBLIC_SCHEMA_V1.into()),
            ),
            (
                "encoding".into(),
                CanonicalValue::String("UNSIGNED_BIG_ENDIAN".into()),
            ),
            ("hex".into(), CanonicalValue::String(self.hex.clone())),
            (
                "width_bytes".into(),
                CanonicalValue::Integer(BigInt::from(self.width_bytes)),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedWidthReceipt {
    operation_digest: ArtifactDigest,
    evidence_digest: ArtifactDigest,
    result: String,
}

impl FixedWidthReceipt {
    pub fn operation_digest(&self) -> ArtifactDigest {
        self.operation_digest
    }

    pub fn evidence_digest(&self) -> ArtifactDigest {
        self.evidence_digest
    }

    pub fn result(&self) -> &str {
        &self.result
    }
}

pub fn check_fixed_width_encode_result(
    operation: &FixedWidthEncodeOperation,
    producer_hex: &str,
) -> Result<FixedWidthReceipt, PublicRepresentationError> {
    let expected = encode_unsigned_be_hex(operation.value(), operation.width_bytes())?;
    if producer_hex != expected {
        return Err(PublicRepresentationError::IncorrectResult);
    }

    Ok(fixed_width_receipt(
        operation.structural_digest(),
        producer_hex,
    ))
}

pub fn check_fixed_width_decode_result(
    operation: &FixedWidthDecodeOperation,
    producer_decimal: &str,
) -> Result<FixedWidthReceipt, PublicRepresentationError> {
    let expected = decode_unsigned_be_hex(operation.hex(), operation.width_bytes())?;
    let parsed = parse_decimal(producer_decimal)?;
    if parsed != expected {
        return Err(PublicRepresentationError::IncorrectResult);
    }

    Ok(fixed_width_receipt(
        operation.structural_digest(),
        producer_decimal,
    ))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RsaPublicOperation {
    representative: BigInt,
    exponent: BigInt,
    modulus: BigInt,
}

impl RsaPublicOperation {
    pub fn new(representative: BigInt, exponent: BigInt, modulus: BigInt) -> Self {
        Self {
            representative,
            exponent,
            modulus,
        }
    }

    pub fn representative(&self) -> &BigInt {
        &self.representative
    }

    pub fn exponent(&self) -> &BigInt {
        &self.exponent
    }

    pub fn modulus(&self) -> &BigInt {
        &self.modulus
    }

    pub fn structural_digest(&self) -> ArtifactDigest {
        StructuralIdentity::structural_digest(self)
    }

    fn recompute(&self) -> Result<(BigInt, usize, String), PublicRepresentationError> {
        if self.modulus <= BigInt::from(1) {
            return Err(PublicRepresentationError::InvalidModulus);
        }
        if self.exponent <= BigInt::from(0) {
            return Err(PublicRepresentationError::InvalidExponent);
        }
        if self.representative < BigInt::from(0) || self.representative >= self.modulus {
            return Err(PublicRepresentationError::RepresentativeOutOfRange);
        }

        let result = self.representative.modpow(&self.exponent, &self.modulus);
        let width = modulus_width_bytes(&self.modulus)?;
        let hex = encode_unsigned_be_hex(&result, width)?;
        Ok((result, width, hex))
    }
}

impl StructuralIdentity for RsaPublicOperation {
    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "kind".into(),
                CanonicalValue::String("RsaPublicOperation".into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(RSA_PUBLIC_SCHEMA_V1.into()),
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
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RsaPublicReceipt {
    operation_digest: ArtifactDigest,
    evidence_digest: ArtifactDigest,
    result_decimal: String,
    result_hex: String,
    width_bytes: usize,
}

impl RsaPublicReceipt {
    pub fn operation_digest(&self) -> ArtifactDigest {
        self.operation_digest
    }

    pub fn evidence_digest(&self) -> ArtifactDigest {
        self.evidence_digest
    }

    pub fn result_decimal(&self) -> &str {
        &self.result_decimal
    }

    pub fn result_hex(&self) -> &str {
        &self.result_hex
    }

    pub fn width_bytes(&self) -> usize {
        self.width_bytes
    }
}

pub fn check_rsa_public_result(
    operation: &RsaPublicOperation,
    producer_decimal: &str,
    producer_hex: &str,
) -> Result<RsaPublicReceipt, PublicRepresentationError> {
    let parsed_decimal = parse_decimal(producer_decimal)?;
    let (expected_decimal, width_bytes, expected_hex) = operation.recompute()?;

    if parsed_decimal != expected_decimal || producer_hex != expected_hex {
        return Err(PublicRepresentationError::IncorrectResult);
    }

    let operation_digest = operation.structural_digest();
    let evidence_digest = ArtifactDigest::of_bytes(
        format!(
            "{}\n{}\n{}",
            operation_digest.as_str(),
            producer_decimal,
            producer_hex
        )
        .as_bytes(),
    );

    Ok(RsaPublicReceipt {
        operation_digest,
        evidence_digest,
        result_decimal: producer_decimal.to_owned(),
        result_hex: producer_hex.to_owned(),
        width_bytes,
    })
}

fn fixed_width_receipt(operation_digest: ArtifactDigest, result: &str) -> FixedWidthReceipt {
    let evidence_digest =
        ArtifactDigest::of_bytes(format!("{}\n{}", operation_digest.as_str(), result).as_bytes());
    FixedWidthReceipt {
        operation_digest,
        evidence_digest,
        result: result.to_owned(),
    }
}

fn modulus_width_bytes(modulus: &BigInt) -> Result<usize, PublicRepresentationError> {
    if modulus <= &BigInt::from(1) {
        return Err(PublicRepresentationError::InvalidModulus);
    }
    let bits = modulus.bits();
    usize::try_from(bits.div_ceil(8)).map_err(|_| PublicRepresentationError::WidthOverflow)
}

fn encode_unsigned_be_hex(
    value: &BigInt,
    width_bytes: usize,
) -> Result<String, PublicRepresentationError> {
    if width_bytes == 0 {
        return Err(PublicRepresentationError::InvalidWidth);
    }
    if value.sign() == Sign::Minus {
        return Err(PublicRepresentationError::NegativeValue);
    }

    let (_, magnitude) = value.to_bytes_be();
    if magnitude.len() > width_bytes {
        return Err(PublicRepresentationError::WidthOverflow);
    }

    let mut bytes = vec![0u8; width_bytes - magnitude.len()];
    bytes.extend_from_slice(&magnitude);
    Ok(encode_hex(&bytes))
}

fn decode_unsigned_be_hex(
    input: &str,
    width_bytes: usize,
) -> Result<BigInt, PublicRepresentationError> {
    if width_bytes == 0 {
        return Err(PublicRepresentationError::InvalidWidth);
    }
    let expected_len = width_bytes
        .checked_mul(2)
        .ok_or(PublicRepresentationError::InvalidWidth)?;
    if input.len() != expected_len {
        return Err(PublicRepresentationError::MalformedHex);
    }

    let raw = input.as_bytes();
    let mut bytes = Vec::with_capacity(width_bytes);
    for pair in raw.as_chunks::<2>().0 {
        let high = decode_hex_nibble(pair[0])?;
        let low = decode_hex_nibble(pair[1])?;
        bytes.push((high << 4) | low);
    }
    Ok(BigInt::from_bytes_be(Sign::Plus, &bytes))
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn decode_hex_nibble(byte: u8) -> Result<u8, PublicRepresentationError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(PublicRepresentationError::MalformedHex),
    }
}

fn parse_decimal(input: &str) -> Result<BigInt, PublicRepresentationError> {
    parse_canonical_decimal(input).map_err(|_| PublicRepresentationError::MalformedDecimal)
}
