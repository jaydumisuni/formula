use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const QUERY_SCHEMA_V1: &str = "formula-query-ir-v1";

fn sorted_digests(mut values: Vec<ArtifactDigest>) -> Vec<ArtifactDigest> {
    values.sort_unstable();
    values.dedup();
    values
}

fn digest_array(values: &[ArtifactDigest]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().copied().map(CanonicalValue::Digest).collect())
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct KnownBinding {
    name: String,
    artifact: ArtifactDigest,
}

impl KnownBinding {
    pub fn new(name: impl Into<String>, artifact: ArtifactDigest) -> Self {
        Self {
            name: name.into(),
            artifact,
        }
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            ("artifact".into(), CanonicalValue::Digest(self.artifact)),
            ("name".into(), CanonicalValue::String(self.name.clone())),
        ]))
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Metavariable {
    name: String,
    artifact_class: String,
}

impl Metavariable {
    pub fn new(name: impl Into<String>, artifact_class: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            artifact_class: artifact_class.into(),
        }
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "artifact_class".into(),
                CanonicalValue::String(self.artifact_class.clone()),
            ),
            ("name".into(), CanonicalValue::String(self.name.clone())),
        ]))
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RequestedResultClass {
    Decision,
    Witness,
    Count,
    Optimum,
    Bound,
}

impl RequestedResultClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Decision => "DECISION",
            Self::Witness => "WITNESS",
            Self::Count => "COUNT",
            Self::Optimum => "OPTIMUM",
            Self::Bound => "BOUND",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TargetRequest {
    target: ArtifactDigest,
    result_class: RequestedResultClass,
}

impl TargetRequest {
    pub fn new(target: ArtifactDigest, result_class: RequestedResultClass) -> Self {
        Self {
            target,
            result_class,
        }
    }

    pub fn target(&self) -> ArtifactDigest {
        self.target
    }

    pub fn result_class(&self) -> RequestedResultClass {
        self.result_class
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "result_class".into(),
                CanonicalValue::String(self.result_class.as_str().into()),
            ),
            ("target".into(), CanonicalValue::Digest(self.target)),
        ]))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceContract {
    max_work_units: u64,
    max_memory_bytes: u64,
    wall_clock_millis: u64,
}

impl ResourceContract {
    pub fn new(max_work_units: u64, max_memory_bytes: u64, wall_clock_millis: u64) -> Self {
        Self {
            max_work_units,
            max_memory_bytes,
            wall_clock_millis,
        }
    }

    fn canonical_value(self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "max_memory_bytes".into(),
                CanonicalValue::String(self.max_memory_bytes.to_string()),
            ),
            (
                "max_work_units".into(),
                CanonicalValue::String(self.max_work_units.to_string()),
            ),
            (
                "wall_clock_millis".into(),
                CanonicalValue::String(self.wall_clock_millis.to_string()),
            ),
        ]))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SideEffectPolicy {
    allow_local_process: bool,
    allow_network: bool,
    allow_authority_write: bool,
}

impl SideEffectPolicy {
    pub fn deny_all() -> Self {
        Self {
            allow_local_process: false,
            allow_network: false,
            allow_authority_write: false,
        }
    }

    pub fn local_process_only() -> Self {
        Self {
            allow_local_process: true,
            allow_network: false,
            allow_authority_write: false,
        }
    }

    pub fn allows_authority_write(self) -> bool {
        self.allow_authority_write
    }

    fn canonical_value(self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "allow_authority_write".into(),
                CanonicalValue::Bool(self.allow_authority_write),
            ),
            (
                "allow_local_process".into(),
                CanonicalValue::Bool(self.allow_local_process),
            ),
            (
                "allow_network".into(),
                CanonicalValue::Bool(self.allow_network),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivatedPackageBinding {
    generation: ArtifactDigest,
    package_digests: Vec<ArtifactDigest>,
    composition_claims: Vec<ArtifactDigest>,
}

impl ActivatedPackageBinding {
    pub fn new(
        generation: ArtifactDigest,
        package_digests: Vec<ArtifactDigest>,
        composition_claims: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            generation,
            package_digests: sorted_digests(package_digests),
            composition_claims: sorted_digests(composition_claims),
        }
    }

    pub fn generation(&self) -> ArtifactDigest {
        self.generation
    }

    pub fn package_digests(&self) -> &[ArtifactDigest] {
        &self.package_digests
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "composition_claims".into(),
                digest_array(&self.composition_claims),
            ),
            ("generation".into(), CanonicalValue::Digest(self.generation)),
            (
                "package_digests".into(),
                digest_array(&self.package_digests),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryIR {
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    known_bindings: Vec<KnownBinding>,
    metavariables: Vec<Metavariable>,
    targets: Vec<TargetRequest>,
    observer: ArtifactDigest,
    authority_contract: ArtifactDigest,
    resource_contract: ResourceContract,
    side_effect_policy: SideEffectPolicy,
    activated_packages: ActivatedPackageBinding,
}

impl QueryIR {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        universe_generation: ArtifactDigest,
        world: ArtifactDigest,
        mut known_bindings: Vec<KnownBinding>,
        mut metavariables: Vec<Metavariable>,
        mut targets: Vec<TargetRequest>,
        observer: ArtifactDigest,
        authority_contract: ArtifactDigest,
        resource_contract: ResourceContract,
        side_effect_policy: SideEffectPolicy,
        activated_packages: ActivatedPackageBinding,
    ) -> Self {
        known_bindings.sort_unstable();
        known_bindings.dedup();
        metavariables.sort_unstable();
        metavariables.dedup();
        targets.sort_unstable();
        targets.dedup();
        Self {
            universe_generation,
            world,
            known_bindings,
            metavariables,
            targets,
            observer,
            authority_contract,
            resource_contract,
            side_effect_policy,
            activated_packages,
        }
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn observer(&self) -> ArtifactDigest {
        self.observer
    }

    pub fn authority_contract(&self) -> ArtifactDigest {
        self.authority_contract
    }

    pub fn requested_result_class(&self) -> RequestedResultClass {
        self.targets
            .first()
            .map(TargetRequest::result_class)
            .unwrap_or(RequestedResultClass::Decision)
    }

    pub fn resource_contract(&self) -> ResourceContract {
        self.resource_contract
    }

    pub fn activated_packages(&self) -> &ActivatedPackageBinding {
        &self.activated_packages
    }

    pub fn with_universe_generation(mut self, value: ArtifactDigest) -> Self {
        self.universe_generation = value;
        self
    }

    pub fn with_world(mut self, value: ArtifactDigest) -> Self {
        self.world = value;
        self
    }

    pub fn with_observer(mut self, value: ArtifactDigest) -> Self {
        self.observer = value;
        self
    }

    pub fn with_authority_contract(mut self, value: ArtifactDigest) -> Self {
        self.authority_contract = value;
        self
    }

    pub fn with_known_bindings(mut self, mut values: Vec<KnownBinding>) -> Self {
        values.sort_unstable();
        values.dedup();
        self.known_bindings = values;
        self
    }

    pub fn with_targets(mut self, mut values: Vec<TargetRequest>) -> Self {
        values.sort_unstable();
        values.dedup();
        self.targets = values;
        self
    }

    pub fn with_resource_contract(mut self, value: ResourceContract) -> Self {
        self.resource_contract = value;
        self
    }

    pub fn with_side_effect_policy(mut self, value: SideEffectPolicy) -> Self {
        self.side_effect_policy = value;
        self
    }

    pub fn with_activated_packages(mut self, value: ActivatedPackageBinding) -> Self {
        self.activated_packages = value;
        self
    }

    pub fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "activated_packages".into(),
                self.activated_packages.canonical_value(),
            ),
            (
                "authority_contract".into(),
                CanonicalValue::Digest(self.authority_contract),
            ),
            (
                "known_bindings".into(),
                CanonicalValue::Array(
                    self.known_bindings
                        .iter()
                        .map(KnownBinding::canonical_value)
                        .collect(),
                ),
            ),
            (
                "metavariables".into(),
                CanonicalValue::Array(
                    self.metavariables
                        .iter()
                        .map(Metavariable::canonical_value)
                        .collect(),
                ),
            ),
            ("observer".into(), CanonicalValue::Digest(self.observer)),
            (
                "resource_contract".into(),
                self.resource_contract.canonical_value(),
            ),
            (
                "schema".into(),
                CanonicalValue::String(QUERY_SCHEMA_V1.into()),
            ),
            (
                "side_effect_policy".into(),
                self.side_effect_policy.canonical_value(),
            ),
            (
                "targets".into(),
                CanonicalValue::Array(
                    self.targets
                        .iter()
                        .map(TargetRequest::canonical_value)
                        .collect(),
                ),
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
