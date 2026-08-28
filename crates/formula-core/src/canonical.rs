use crate::digest::ArtifactDigest;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, Zero};
use std::{collections::BTreeMap, error::Error, fmt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalRational {
    numerator: BigInt,
    denominator: BigInt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalError {
    ZeroDenominator,
}

impl fmt::Display for CanonicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroDenominator => f.write_str("canonical rational denominator must be non-zero"),
        }
    }
}

impl Error for CanonicalError {}

impl CanonicalRational {
    pub fn new(mut numerator: BigInt, mut denominator: BigInt) -> Result<Self, CanonicalError> {
        if denominator.is_zero() {
            return Err(CanonicalError::ZeroDenominator);
        }

        if denominator.is_negative() {
            numerator = -numerator;
            denominator = -denominator;
        }

        let gcd = numerator.gcd(&denominator);
        numerator /= &gcd;
        denominator /= gcd;

        Ok(Self {
            numerator,
            denominator,
        })
    }

    pub fn numerator(&self) -> &BigInt {
        &self.numerator
    }

    pub fn denominator(&self) -> &BigInt {
        &self.denominator
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalValue {
    Null,
    Bool(bool),
    Integer(BigInt),
    Rational(CanonicalRational),
    String(String),
    Array(Vec<CanonicalValue>),
    Object(BTreeMap<String, CanonicalValue>),
    Digest(ArtifactDigest),
}

impl CanonicalValue {
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        let mut out = String::new();
        self.write_to(&mut out);
        out.into_bytes()
    }

    pub fn digest(&self) -> ArtifactDigest {
        ArtifactDigest::of_bytes(&self.to_canonical_bytes())
    }

    fn write_to(&self, out: &mut String) {
        match self {
            Self::Null => out.push_str("null"),
            Self::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
            Self::Integer(value) => out.push_str(&value.to_string()),
            Self::Rational(value) => {
                out.push_str("{\"denominator\":");
                out.push_str(&value.denominator.to_string());
                out.push_str(",\"numerator\":");
                out.push_str(&value.numerator.to_string());
                out.push('}');
            }
            Self::String(value) => write_json_string(out, value),
            Self::Array(values) => {
                out.push('[');
                for (index, value) in values.iter().enumerate() {
                    if index != 0 {
                        out.push(',');
                    }
                    value.write_to(out);
                }
                out.push(']');
            }
            Self::Object(values) => {
                out.push('{');
                for (index, (key, value)) in values.iter().enumerate() {
                    if index != 0 {
                        out.push(',');
                    }
                    write_json_string(out, key);
                    out.push(':');
                    value.write_to(out);
                }
                out.push('}');
            }
            Self::Digest(value) => write_json_string(out, &value.as_str()),
        }
    }
}

fn write_json_string(out: &mut String, value: &str) {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{0000}'..='\u{001f}' => {
                let code = ch as u8;
                out.push_str("\\u00");
                out.push(HEX[(code >> 4) as usize] as char);
                out.push(HEX[(code & 0x0f) as usize] as char);
            }
            _ => out.push(ch),
        }
    }
    out.push('"');
}
