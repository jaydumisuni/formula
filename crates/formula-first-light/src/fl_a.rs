use formula_core::digest::ArtifactDigest;

const FL_A_TARGET_SCHEMA_V1: &str = "formula-p6-fl-a-sealed-target-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FlAOracleError {
    Overflow,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FlAOracle;

pub fn fl_a_oracle() -> FlAOracle {
    FlAOracle
}

pub fn fl_a_target_digest() -> ArtifactDigest {
    ArtifactDigest::of_bytes(FL_A_TARGET_SCHEMA_V1.as_bytes())
}

impl FlAOracle {
    pub fn sample(&self, n: i128) -> Result<i128, FlAOracleError> {
        let next = n.checked_add(1).ok_or(FlAOracleError::Overflow)?;
        let next_pow = next.checked_pow(7).ok_or(FlAOracleError::Overflow)?;
        let current_pow = n.checked_pow(7).ok_or(FlAOracleError::Overflow)?;
        next_pow
            .checked_sub(current_pow)
            .ok_or(FlAOracleError::Overflow)
    }
}
