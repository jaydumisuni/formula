use crate::{
    artifacts::StructuralIdentity,
    canonical::CanonicalValue,
    digest::ArtifactDigest,
};
use std::collections::BTreeMap;

const REALIZATION_SCHEMA_V1: &str = "formula-p8-realization-v1";
const QUERY_DIRECTION: &str = "u8_to_bool_forward";
const INPUT_DOMAIN: &str = "u8:0..=255";
const OUTPUT_DOMAIN: &str = "bool";
const LOWERING_CLASS: &str = "EXACT_EQUIVALENCE";
const COMPILER: &str = "rustc";
const OPTIMIZATION: &str = "-O";
const BACKEND_FAMILY: &str = "standalone-rust-native";
const INPUT_REPRESENTATION: &str = "u8";
const OUTPUT_REPRESENTATION: &str = "bool";
const FALLBACK_SEMANTICS: &str = "semantic_execution";

fn canonical_object(kind: &str) -> BTreeMap<String, CanonicalValue> {
    BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        (
            "schema".into(),
            CanonicalValue::String(REALIZATION_SCHEMA_V1.into()),
        ),
    ])
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpecializationIdentity {
    semantic_target: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
}

impl SpecializationIdentity {
    pub fn new(
        semantic_target: ArtifactDigest,
        universe_generation: ArtifactDigest,
        world: ArtifactDigest,
        authority_contract: ArtifactDigest,
        observer: ArtifactDigest,
    ) -> Self {
        Self {
            semantic_target,
            universe_generation,
            world,
            authority_contract,
            observer,
        }
    }

    pub fn semantic_target(&self) -> ArtifactDigest {
        self.semantic_target
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn authority_contract(&self) -> ArtifactDigest {
        self.authority_contract
    }

    pub fn observer(&self) -> ArtifactDigest {
        self.observer
    }

    pub fn query_direction(&self) -> &'static str {
        QUERY_DIRECTION
    }

    pub fn input_domain(&self) -> &'static str {
        INPUT_DOMAIN
    }

    pub fn output_domain(&self) -> &'static str {
        OUTPUT_DOMAIN
    }

    pub fn lowering_class(&self) -> &'static str {
        LOWERING_CLASS
    }
}

impl StructuralIdentity for SpecializationIdentity {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("SpecializationIdentity");
        object.insert(
            "authority_contract".into(),
            CanonicalValue::Digest(self.authority_contract),
        );
        object.insert(
            "input_domain".into(),
            CanonicalValue::String(INPUT_DOMAIN.into()),
        );
        object.insert(
            "lowering_class".into(),
            CanonicalValue::String(LOWERING_CLASS.into()),
        );
        object.insert("observer".into(), CanonicalValue::Digest(self.observer));
        object.insert(
            "output_domain".into(),
            CanonicalValue::String(OUTPUT_DOMAIN.into()),
        );
        object.insert(
            "query_direction".into(),
            CanonicalValue::String(QUERY_DIRECTION.into()),
        );
        object.insert(
            "semantic_target".into(),
            CanonicalValue::Digest(self.semantic_target),
        );
        object.insert(
            "universe_generation".into(),
            CanonicalValue::Digest(self.universe_generation),
        );
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeToolchainIdentity {
    rust_release: String,
    host_target: String,
}

impl NativeToolchainIdentity {
    pub fn new(rust_release: String, host_target: String) -> Self {
        Self {
            rust_release,
            host_target,
        }
    }

    pub fn compiler(&self) -> &'static str {
        COMPILER
    }

    pub fn rust_release(&self) -> &str {
        &self.rust_release
    }

    pub fn optimization(&self) -> &'static str {
        OPTIMIZATION
    }

    pub fn host_target(&self) -> &str {
        &self.host_target
    }

    pub fn backend_family(&self) -> &'static str {
        BACKEND_FAMILY
    }
}

impl StructuralIdentity for NativeToolchainIdentity {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("NativeToolchainIdentity");
        object.insert(
            "backend_family".into(),
            CanonicalValue::String(BACKEND_FAMILY.into()),
        );
        object.insert("compiler".into(), CanonicalValue::String(COMPILER.into()));
        object.insert(
            "host_target".into(),
            CanonicalValue::String(self.host_target.clone()),
        );
        object.insert(
            "optimization".into(),
            CanonicalValue::String(OPTIMIZATION.into()),
        );
        object.insert(
            "rust_release".into(),
            CanonicalValue::String(self.rust_release.clone()),
        );
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeRealizationManifest {
    semantic_target: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
    specialization_digest: ArtifactDigest,
    source_digest: ArtifactDigest,
    toolchain_digest: ArtifactDigest,
    binary_digest: ArtifactDigest,
}

impl NativeRealizationManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        semantic_target: ArtifactDigest,
        universe_generation: ArtifactDigest,
        world: ArtifactDigest,
        authority_contract: ArtifactDigest,
        observer: ArtifactDigest,
        specialization_digest: ArtifactDigest,
        source_digest: ArtifactDigest,
        toolchain_digest: ArtifactDigest,
        binary_digest: ArtifactDigest,
    ) -> Self {
        Self {
            semantic_target,
            universe_generation,
            world,
            authority_contract,
            observer,
            specialization_digest,
            source_digest,
            toolchain_digest,
            binary_digest,
        }
    }

    pub fn semantic_target(&self) -> ArtifactDigest {
        self.semantic_target
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn authority_contract(&self) -> ArtifactDigest {
        self.authority_contract
    }

    pub fn observer(&self) -> ArtifactDigest {
        self.observer
    }

    pub fn specialization_digest(&self) -> ArtifactDigest {
        self.specialization_digest
    }

    pub fn source_digest(&self) -> ArtifactDigest {
        self.source_digest
    }

    pub fn toolchain_digest(&self) -> ArtifactDigest {
        self.toolchain_digest
    }

    pub fn binary_digest(&self) -> ArtifactDigest {
        self.binary_digest
    }

    pub fn lowering_class(&self) -> &'static str {
        LOWERING_CLASS
    }

    pub fn input_representation(&self) -> &'static str {
        INPUT_REPRESENTATION
    }

    pub fn output_representation(&self) -> &'static str {
        OUTPUT_REPRESENTATION
    }

    pub fn fallback_semantics(&self) -> &'static str {
        FALLBACK_SEMANTICS
    }
}

impl StructuralIdentity for NativeRealizationManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("NativeRealizationManifest");
        object.insert(
            "authority_contract".into(),
            CanonicalValue::Digest(self.authority_contract),
        );
        object.insert(
            "binary_digest".into(),
            CanonicalValue::Digest(self.binary_digest),
        );
        object.insert(
            "fallback_semantics".into(),
            CanonicalValue::String(FALLBACK_SEMANTICS.into()),
        );
        object.insert(
            "input_representation".into(),
            CanonicalValue::String(INPUT_REPRESENTATION.into()),
        );
        object.insert(
            "lowering_class".into(),
            CanonicalValue::String(LOWERING_CLASS.into()),
        );
        object.insert("observer".into(), CanonicalValue::Digest(self.observer));
        object.insert(
            "output_representation".into(),
            CanonicalValue::String(OUTPUT_REPRESENTATION.into()),
        );
        object.insert(
            "semantic_target".into(),
            CanonicalValue::Digest(self.semantic_target),
        );
        object.insert(
            "source_digest".into(),
            CanonicalValue::Digest(self.source_digest),
        );
        object.insert(
            "specialization_digest".into(),
            CanonicalValue::Digest(self.specialization_digest),
        );
        object.insert(
            "toolchain_digest".into(),
            CanonicalValue::Digest(self.toolchain_digest),
        );
        object.insert(
            "universe_generation".into(),
            CanonicalValue::Digest(self.universe_generation),
        );
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationDispatchContext {
    semantic_target: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
}

impl RealizationDispatchContext {
    pub fn new(
        semantic_target: ArtifactDigest,
        universe_generation: ArtifactDigest,
        world: ArtifactDigest,
        authority_contract: ArtifactDigest,
        observer: ArtifactDigest,
    ) -> Self {
        Self {
            semantic_target,
            universe_generation,
            world,
            authority_contract,
            observer,
        }
    }

    pub fn semantic_target(&self) -> ArtifactDigest {
        self.semantic_target
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn authority_contract(&self) -> ArtifactDigest {
        self.authority_contract
    }

    pub fn observer(&self) -> ArtifactDigest {
        self.observer
    }
}

impl StructuralIdentity for RealizationDispatchContext {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("RealizationDispatchContext");
        object.insert(
            "authority_contract".into(),
            CanonicalValue::Digest(self.authority_contract),
        );
        object.insert("observer".into(), CanonicalValue::Digest(self.observer));
        object.insert(
            "semantic_target".into(),
            CanonicalValue::Digest(self.semantic_target),
        );
        object.insert(
            "universe_generation".into(),
            CanonicalValue::Digest(self.universe_generation),
        );
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        CanonicalValue::Object(object)
    }
}
