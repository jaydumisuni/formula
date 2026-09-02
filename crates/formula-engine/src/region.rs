use crate::query::{ActivatedPackageBinding, QueryIR};
use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const REGION_SCHEMA_V1: &str = "formula-relevant-region-v1";

fn sorted_digests(mut values: Vec<ArtifactDigest>) -> Vec<ArtifactDigest> {
    values.sort_unstable();
    values.dedup();
    values
}

fn digest_array(values: &[ArtifactDigest]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().copied().map(CanonicalValue::Digest).collect())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerAuthoritySnapshot {
    generation: ArtifactDigest,
    world: ArtifactDigest,
    package_context: ActivatedPackageBinding,
    admitted_artifacts: Vec<ArtifactDigest>,
    admitted_capabilities: Vec<ArtifactDigest>,
    admitted_morphisms: Vec<ArtifactDigest>,
}

impl CompilerAuthoritySnapshot {
    pub fn new(
        generation: ArtifactDigest,
        world: ArtifactDigest,
        package_context: ActivatedPackageBinding,
        admitted_artifacts: Vec<ArtifactDigest>,
        admitted_capabilities: Vec<ArtifactDigest>,
        admitted_morphisms: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            generation,
            world,
            package_context,
            admitted_artifacts: sorted_digests(admitted_artifacts),
            admitted_capabilities: sorted_digests(admitted_capabilities),
            admitted_morphisms: sorted_digests(admitted_morphisms),
        }
    }

    pub fn generation(&self) -> ArtifactDigest {
        self.generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn package_context(&self) -> &ActivatedPackageBinding {
        &self.package_context
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegionError {
    GenerationMismatch,
    WorldMismatch,
    PackageContextMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelevantRegion {
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    query_digest: ArtifactDigest,
    semantic_artifacts: Vec<ArtifactDigest>,
    admitted_capabilities: Vec<ArtifactDigest>,
    admitted_morphisms: Vec<ArtifactDigest>,
}

impl RelevantRegion {
    pub fn from_snapshot(
        query: &QueryIR,
        snapshot: &CompilerAuthoritySnapshot,
    ) -> Result<Self, RegionError> {
        if query.universe_generation() != snapshot.generation {
            return Err(RegionError::GenerationMismatch);
        }
        if query.world() != snapshot.world {
            return Err(RegionError::WorldMismatch);
        }
        if query.activated_packages() != &snapshot.package_context {
            return Err(RegionError::PackageContextMismatch);
        }
        Ok(Self {
            universe_generation: snapshot.generation,
            world: snapshot.world,
            query_digest: query.digest(),
            semantic_artifacts: snapshot.admitted_artifacts.clone(),
            admitted_capabilities: snapshot.admitted_capabilities.clone(),
            admitted_morphisms: snapshot.admitted_morphisms.clone(),
        })
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn query_digest(&self) -> ArtifactDigest {
        self.query_digest
    }

    pub fn admitted_capabilities(&self) -> &[ArtifactDigest] {
        &self.admitted_capabilities
    }

    pub fn admitted_morphisms(&self) -> &[ArtifactDigest] {
        &self.admitted_morphisms
    }

    pub fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "admitted_capabilities".into(),
                digest_array(&self.admitted_capabilities),
            ),
            (
                "admitted_morphisms".into(),
                digest_array(&self.admitted_morphisms),
            ),
            (
                "query_digest".into(),
                CanonicalValue::Digest(self.query_digest),
            ),
            (
                "schema".into(),
                CanonicalValue::String(REGION_SCHEMA_V1.into()),
            ),
            (
                "semantic_artifacts".into(),
                digest_array(&self.semantic_artifacts),
            ),
            (
                "universe_generation".into(),
                CanonicalValue::Digest(self.universe_generation),
            ),
            ("world".into(), CanonicalValue::Digest(self.world)),
        ]))
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}
