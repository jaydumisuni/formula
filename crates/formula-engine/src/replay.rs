use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

use crate::query::ResourceContract;

const REPLAY_SCHEMA_V1: &str = "formula-replay-manifest-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayManifest {
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    query_digest: ArtifactDigest,
    activated_package_set: ArtifactDigest,
    relevant_region_digest: ArtifactDigest,
    theory_profile_digest: ArtifactDigest,
    compiler_policy_version: String,
    scheduler_policy_version: String,
    resource_contract: ResourceContract,
    random_key: ArtifactDigest,
    campaign_digest: ArtifactDigest,
}

impl ReplayManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        universe_generation: ArtifactDigest,
        world: ArtifactDigest,
        query_digest: ArtifactDigest,
        activated_package_set: ArtifactDigest,
        relevant_region_digest: ArtifactDigest,
        theory_profile_digest: ArtifactDigest,
        compiler_policy_version: impl Into<String>,
        scheduler_policy_version: impl Into<String>,
        resource_contract: ResourceContract,
        random_key: ArtifactDigest,
        campaign_digest: ArtifactDigest,
    ) -> Self {
        Self {
            universe_generation,
            world,
            query_digest,
            activated_package_set,
            relevant_region_digest,
            theory_profile_digest,
            compiler_policy_version: compiler_policy_version.into(),
            scheduler_policy_version: scheduler_policy_version.into(),
            resource_contract,
            random_key,
            campaign_digest,
        }
    }

    pub fn with_universe_generation(mut self, value: ArtifactDigest) -> Self {
        self.universe_generation = value;
        self
    }

    pub fn with_world(mut self, value: ArtifactDigest) -> Self {
        self.world = value;
        self
    }

    pub fn with_query_digest(mut self, value: ArtifactDigest) -> Self {
        self.query_digest = value;
        self
    }

    pub fn with_activated_package_set(mut self, value: ArtifactDigest) -> Self {
        self.activated_package_set = value;
        self
    }

    pub fn with_relevant_region_digest(mut self, value: ArtifactDigest) -> Self {
        self.relevant_region_digest = value;
        self
    }

    pub fn with_theory_profile_digest(mut self, value: ArtifactDigest) -> Self {
        self.theory_profile_digest = value;
        self
    }

    pub fn with_compiler_policy_version(mut self, value: impl Into<String>) -> Self {
        self.compiler_policy_version = value.into();
        self
    }

    pub fn with_scheduler_policy_version(mut self, value: impl Into<String>) -> Self {
        self.scheduler_policy_version = value.into();
        self
    }

    pub fn with_resource_contract(mut self, value: ResourceContract) -> Self {
        self.resource_contract = value;
        self
    }

    pub fn with_random_key(mut self, value: ArtifactDigest) -> Self {
        self.random_key = value;
        self
    }

    pub fn with_campaign_digest(mut self, value: ArtifactDigest) -> Self {
        self.campaign_digest = value;
        self
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "activated_package_set".into(),
                CanonicalValue::Digest(self.activated_package_set),
            ),
            (
                "campaign_digest".into(),
                CanonicalValue::Digest(self.campaign_digest),
            ),
            (
                "compiler_policy_version".into(),
                CanonicalValue::String(self.compiler_policy_version.clone()),
            ),
            (
                "query_digest".into(),
                CanonicalValue::Digest(self.query_digest),
            ),
            ("random_key".into(), CanonicalValue::Digest(self.random_key)),
            (
                "relevant_region_digest".into(),
                CanonicalValue::Digest(self.relevant_region_digest),
            ),
            (
                "resource_contract".into(),
                CanonicalValue::String(format!("{:?}", self.resource_contract)),
            ),
            (
                "scheduler_policy_version".into(),
                CanonicalValue::String(self.scheduler_policy_version.clone()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(REPLAY_SCHEMA_V1.into()),
            ),
            (
                "theory_profile_digest".into(),
                CanonicalValue::Digest(self.theory_profile_digest),
            ),
            (
                "universe_generation".into(),
                CanonicalValue::Digest(self.universe_generation),
            ),
            ("world".into(), CanonicalValue::Digest(self.world)),
        ]))
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        self.canonical_value().to_canonical_bytes()
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}
