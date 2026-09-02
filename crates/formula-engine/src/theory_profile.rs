use crate::region::RelevantRegion;
use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const PROFILE_SCHEMA_V1: &str = "formula-theory-profile-v1";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProfileFact {
    name: String,
    evidence: ArtifactDigest,
}

impl ProfileFact {
    pub fn new(name: impl Into<String>, evidence: ArtifactDigest) -> Self {
        Self {
            name: name.into(),
            evidence,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            ("evidence".into(), CanonicalValue::Digest(self.evidence)),
            ("name".into(), CanonicalValue::String(self.name.clone())),
        ]))
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OperationalEstimate {
    name: String,
    score: u64,
}

impl OperationalEstimate {
    pub fn new(name: impl Into<String>, score: u64) -> Self {
        Self {
            name: name.into(),
            score,
        }
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            ("name".into(), CanonicalValue::String(self.name.clone())),
            (
                "score".into(),
                CanonicalValue::String(self.score.to_string()),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TheoryProfile {
    region_digest: ArtifactDigest,
    exact_properties: Vec<ProfileFact>,
    operational_estimates: Vec<OperationalEstimate>,
}

impl TheoryProfile {
    pub fn compile(
        region: &RelevantRegion,
        exact_properties: &[ProfileFact],
        operational_estimates: &[OperationalEstimate],
    ) -> Self {
        let mut exact_properties = exact_properties.to_vec();
        exact_properties.sort_unstable();
        exact_properties.dedup();
        let mut operational_estimates = operational_estimates.to_vec();
        operational_estimates.sort_unstable();
        operational_estimates.dedup();
        Self {
            region_digest: region.digest(),
            exact_properties,
            operational_estimates,
        }
    }

    pub fn satisfies_exact_property(&self, name: &str) -> bool {
        self.exact_properties.iter().any(|fact| fact.name() == name)
    }

    pub fn operational_estimates(&self) -> &[OperationalEstimate] {
        &self.operational_estimates
    }

    pub fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "exact_properties".into(),
                CanonicalValue::Array(
                    self.exact_properties
                        .iter()
                        .map(ProfileFact::canonical_value)
                        .collect(),
                ),
            ),
            (
                "operational_estimates".into(),
                CanonicalValue::Array(
                    self.operational_estimates
                        .iter()
                        .map(OperationalEstimate::canonical_value)
                        .collect(),
                ),
            ),
            (
                "region_digest".into(),
                CanonicalValue::Digest(self.region_digest),
            ),
            (
                "schema".into(),
                CanonicalValue::String(PROFILE_SCHEMA_V1.into()),
            ),
        ]))
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}
