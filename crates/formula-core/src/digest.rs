use sha2::{Digest, Sha256};
use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ArtifactDigest([u8; 32]);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DigestError {
    InvalidPrefix,
    InvalidLength,
    InvalidHex,
    NonCanonicalHex,
}

impl fmt::Display for DigestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for DigestError {}

impl ArtifactDigest {
    pub fn of_bytes(bytes: &[u8]) -> Self {
        let hash = Sha256::digest(bytes);
        let mut out = [0u8; 32];
        out.copy_from_slice(&hash);
        Self(out)
    }

    pub fn parse(value: &str) -> Result<Self, DigestError> {
        let hex = value
            .strip_prefix("sha256:")
            .ok_or(DigestError::InvalidPrefix)?;
        if hex.len() != 64 {
            return Err(DigestError::InvalidLength);
        }
        if hex.bytes().any(|byte| matches!(byte, b'A'..=b'F')) {
            return Err(DigestError::NonCanonicalHex);
        }

        let mut bytes = [0u8; 32];
        for (i, chunk) in hex.as_bytes().chunks_exact(2).enumerate() {
            let high = decode_nibble(chunk[0])?;
            let low = decode_nibble(chunk[1])?;
            bytes[i] = (high << 4) | low;
        }
        Ok(Self(bytes))
    }

    pub fn hex(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out = String::with_capacity(64);
        for byte in self.0 {
            out.push(HEX[(byte >> 4) as usize] as char);
            out.push(HEX[(byte & 0x0f) as usize] as char);
        }
        out
    }

    pub fn as_str(&self) -> String {
        format!("sha256:{}", self.hex())
    }
}

fn decode_nibble(byte: u8) -> Result<u8, DigestError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(DigestError::InvalidHex),
    }
}
