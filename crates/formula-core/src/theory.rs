use crate::{
    artifacts::StructuralIdentity, canonical::CanonicalValue, digest::ArtifactDigest,
};
use std::collections::BTreeMap;

const THEORY_SCHEMA_V1: &str = "formula-theory-v1";

fn canonical_object(kind: &str) -> BTreeMap<String, CanonicalValue> {
    BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        (
            "schema".into(),
            CanonicalValue::String(THEORY_SCHEMA_V1.into()),
        ),
    ])
}

fn sorted_digests(mut values: Vec<ArtifactDigest>) -> Vec<ArtifactDigest> {
    values.sort_unstable();
    values.dedup();
    values
}

fn sorted_strings(mut values: Vec<String>) -> Vec<String> {
    values.sort_unstable();
    values.dedup();
    values
}

fn digest_array(values: &[ArtifactDigest]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().copied().map(CanonicalValue::Digest).collect())
}

fn string_array(values: &[String]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().cloned().map(CanonicalValue::String).collect())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityContract {
    capability: ArtifactDigest,
    required_goals: Vec<ArtifactDigest>,
}

impl CapabilityContract {
    pub fn new(capability: ArtifactDigest, required_goals: Vec<ArtifactDigest>) -> Self {
        Self {
            capability,
            required_goals: sorted_digests(required_goals),
        }
    }

    pub fn capability(&self) -> ArtifactDigest {
        self.capability
    }

    pub fn required_goals(&self) -> &[ArtifactDigest] {
        &self.required_goals
    }
}

impl StructuralIdentity for CapabilityContract {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("CapabilityContract");
        object.insert("capability".into(), CanonicalValue::Digest(self.capability));
        object.insert("required_goals".into(), digest_array(&self.required_goals));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TheoryPackageManifest {
    package_id: String,
    foundation: ArtifactDigest,
    semantic_exports: Vec<ArtifactDigest>,
    capabilities: Vec<CapabilityContract>,
    dependencies: Vec<ArtifactDigest>,
    interference_surface: Vec<String>,
}

impl TheoryPackageManifest {
    pub fn new(
        package_id: String,
        foundation: ArtifactDigest,
        semantic_exports: Vec<ArtifactDigest>,
        mut capabilities: Vec<CapabilityContract>,
        dependencies: Vec<ArtifactDigest>,
        interference_surface: Vec<String>,
    ) -> Self {
        capabilities.sort_by_key(StructuralIdentity::structural_digest);
        capabilities.dedup_by_key(|value| value.structural_digest());
        Self {
            package_id,
            foundation,
            semantic_exports: sorted_digests(semantic_exports),
            capabilities,
            dependencies: sorted_digests(dependencies),
            interference_surface: sorted_strings(interference_surface),
        }
    }

    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    pub fn foundation(&self) -> ArtifactDigest {
        self.foundation
    }

    pub fn semantic_exports(&self) -> &[ArtifactDigest] {
        &self.semantic_exports
    }

    pub fn capabilities(&self) -> &[CapabilityContract] {
        &self.capabilities
    }

    pub fn dependencies(&self) -> &[ArtifactDigest] {
        &self.dependencies
    }

    pub fn interference_surface(&self) -> &[String] {
        &self.interference_surface
    }
}

impl StructuralIdentity for TheoryPackageManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("TheoryPackageManifest");
        object.insert("package_id".into(), CanonicalValue::String(self.package_id.clone()));
        object.insert("foundation".into(), CanonicalValue::Digest(self.foundation));
        object.insert("semantic_exports".into(), digest_array(&self.semantic_exports));
        object.insert(
            "capabilities".into(),
            CanonicalValue::Array(
                self.capabilities
                    .iter()
                    .map(StructuralIdentity::canonical_value)
                    .collect(),
            ),
        );
        object.insert("dependencies".into(), digest_array(&self.dependencies));
        object.insert(
            "interference_surface".into(),
            string_array(&self.interference_surface),
        );
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructureGoal {
    kind: String,
    arguments: Vec<ArtifactDigest>,
}

impl StructureGoal {
    pub fn new(kind: String, arguments: Vec<ArtifactDigest>) -> Self {
        Self {
            kind,
            arguments: sorted_digests(arguments),
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn arguments(&self) -> &[ArtifactDigest] {
        &self.arguments
    }
}

impl StructuralIdentity for StructureGoal {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("StructureGoal");
        object.insert("kind".into(), CanonicalValue::String(self.kind.clone()));
        object.insert("arguments".into(), digest_array(&self.arguments));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructureWitness {
    world: ArtifactDigest,
    goal: ArtifactDigest,
    evidence: ArtifactDigest,
}

impl StructureWitness {
    pub fn new(world: ArtifactDigest, goal: ArtifactDigest, evidence: ArtifactDigest) -> Self {
        Self { world, goal, evidence }
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn goal(&self) -> ArtifactDigest {
        self.goal
    }

    pub fn evidence(&self) -> ArtifactDigest {
        self.evidence
    }
}

impl StructuralIdentity for StructureWitness {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("StructureWitness");
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        object.insert("goal".into(), CanonicalValue::Digest(self.goal));
        object.insert("evidence".into(), CanonicalValue::Digest(self.evidence));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalMorphism {
    source: ArtifactDigest,
    target: ArtifactDigest,
    morphism: ArtifactDigest,
    preserves: Vec<ArtifactDigest>,
    canonical: bool,
    lossless: bool,
}

impl CanonicalMorphism {
    pub fn new(
        source: ArtifactDigest,
        target: ArtifactDigest,
        morphism: ArtifactDigest,
        preserves: Vec<ArtifactDigest>,
        canonical: bool,
        lossless: bool,
    ) -> Self {
        Self {
            source,
            target,
            morphism,
            preserves: sorted_digests(preserves),
            canonical,
            lossless,
        }
    }

    pub fn source(&self) -> ArtifactDigest { self.source }
    pub fn target(&self) -> ArtifactDigest { self.target }
    pub fn morphism(&self) -> ArtifactDigest { self.morphism }
    pub fn preserves(&self) -> &[ArtifactDigest] { &self.preserves }
    pub fn is_canonical(&self) -> bool { self.canonical }
    pub fn is_lossless(&self) -> bool { self.lossless }
}

impl StructuralIdentity for CanonicalMorphism {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("CanonicalMorphism");
        object.insert("source".into(), CanonicalValue::Digest(self.source));
        object.insert("target".into(), CanonicalValue::Digest(self.target));
        object.insert("morphism".into(), CanonicalValue::Digest(self.morphism));
        object.insert("preserves".into(), digest_array(&self.preserves));
        object.insert("canonical".into(), CanonicalValue::Bool(self.canonical));
        object.insert("lossless".into(), CanonicalValue::Bool(self.lossless));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompositionClass {
    DisjointSafe,
    CertifiedCombination,
    ConservativeExtension,
    SoundCooperation,
    HeuristicOnly,
    Unsupported,
    Quarantined,
}

impl CompositionClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DisjointSafe => "DISJOINT_SAFE",
            Self::CertifiedCombination => "CERTIFIED_COMBINATION",
            Self::ConservativeExtension => "CONSERVATIVE_EXTENSION",
            Self::SoundCooperation => "SOUND_COOPERATION",
            Self::HeuristicOnly => "HEURISTIC_ONLY",
            Self::Unsupported => "UNSUPPORTED",
            Self::Quarantined => "QUARANTINED",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionClaim {
    left_package: ArtifactDigest,
    right_package: ArtifactDigest,
    class: CompositionClass,
    evidence: ArtifactDigest,
}

impl CompositionClaim {
    pub fn new(
        left_package: ArtifactDigest,
        right_package: ArtifactDigest,
        class: CompositionClass,
        evidence: ArtifactDigest,
    ) -> Self {
        let (left_package, right_package) = if left_package <= right_package {
            (left_package, right_package)
        } else {
            (right_package, left_package)
        };
        Self { left_package, right_package, class, evidence }
    }

    pub fn left_package(&self) -> ArtifactDigest { self.left_package }
    pub fn right_package(&self) -> ArtifactDigest { self.right_package }
    pub fn class(&self) -> CompositionClass { self.class }
    pub fn evidence(&self) -> ArtifactDigest { self.evidence }
}

impl StructuralIdentity for CompositionClaim {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("CompositionClaim");
        object.insert("left_package".into(), CanonicalValue::Digest(self.left_package));
        object.insert("right_package".into(), CanonicalValue::Digest(self.right_package));
        object.insert("class".into(), CanonicalValue::String(self.class.as_str().into()));
        object.insert("evidence".into(), CanonicalValue::Digest(self.evidence));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederationAdapterManifest {
    adapter_id: String,
    package: ArtifactDigest,
    semantic_inputs: Vec<ArtifactDigest>,
    semantic_outputs: Vec<ArtifactDigest>,
    translations: Vec<ArtifactDigest>,
    checker_routes: Vec<ArtifactDigest>,
    side_effects: Vec<String>,
    result_classes: Vec<String>,
    deterministic: bool,
}

#[allow(clippy::too_many_arguments)]
impl FederationAdapterManifest {
    pub fn new(
        adapter_id: String,
        package: ArtifactDigest,
        semantic_inputs: Vec<ArtifactDigest>,
        semantic_outputs: Vec<ArtifactDigest>,
        translations: Vec<ArtifactDigest>,
        checker_routes: Vec<ArtifactDigest>,
        side_effects: Vec<String>,
        result_classes: Vec<String>,
        deterministic: bool,
    ) -> Self {
        Self {
            adapter_id,
            package,
            semantic_inputs: sorted_digests(semantic_inputs),
            semantic_outputs: sorted_digests(semantic_outputs),
            translations: sorted_digests(translations),
            checker_routes: sorted_digests(checker_routes),
            side_effects: sorted_strings(side_effects),
            result_classes: sorted_strings(result_classes),
            deterministic,
        }
    }

    pub fn adapter_id(&self) -> &str { &self.adapter_id }
    pub fn package(&self) -> ArtifactDigest { self.package }
    pub fn semantic_inputs(&self) -> &[ArtifactDigest] { &self.semantic_inputs }
    pub fn semantic_outputs(&self) -> &[ArtifactDigest] { &self.semantic_outputs }
    pub fn translations(&self) -> &[ArtifactDigest] { &self.translations }
    pub fn checker_routes(&self) -> &[ArtifactDigest] { &self.checker_routes }
    pub fn side_effects(&self) -> &[String] { &self.side_effects }
    pub fn result_classes(&self) -> &[String] { &self.result_classes }
    pub fn deterministic(&self) -> bool { self.deterministic }
}

impl StructuralIdentity for FederationAdapterManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("FederationAdapterManifest");
        object.insert("adapter_id".into(), CanonicalValue::String(self.adapter_id.clone()));
        object.insert("package".into(), CanonicalValue::Digest(self.package));
        object.insert("semantic_inputs".into(), digest_array(&self.semantic_inputs));
        object.insert("semantic_outputs".into(), digest_array(&self.semantic_outputs));
        object.insert("translations".into(), digest_array(&self.translations));
        object.insert("checker_routes".into(), digest_array(&self.checker_routes));
        object.insert("side_effects".into(), string_array(&self.side_effects));
        object.insert("result_classes".into(), string_array(&self.result_classes));
        object.insert("deterministic".into(), CanonicalValue::Bool(self.deterministic));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactPolarity {
    Exact,
    OverApproximation,
    UnderApproximation,
    LowerBound,
    UpperBound,
    NecessaryCondition,
    SufficientCondition,
    HeuristicCandidate,
}

impl FactPolarity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "EXACT",
            Self::OverApproximation => "OVER_APPROXIMATION",
            Self::UnderApproximation => "UNDER_APPROXIMATION",
            Self::LowerBound => "LOWER_BOUND",
            Self::UpperBound => "UPPER_BOUND",
            Self::NecessaryCondition => "NECESSARY_CONDITION",
            Self::SufficientCondition => "SUFFICIENT_CONDITION",
            Self::HeuristicCandidate => "HEURISTIC_CANDIDATE",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedFact {
    world: ArtifactDigest,
    subject: ArtifactDigest,
    payload: CanonicalValue,
    polarity: FactPolarity,
    evidence: ArtifactDigest,
}

impl SharedFact {
    pub fn new(
        world: ArtifactDigest,
        subject: ArtifactDigest,
        payload: CanonicalValue,
        polarity: FactPolarity,
        evidence: ArtifactDigest,
    ) -> Self {
        Self { world, subject, payload, polarity, evidence }
    }

    pub fn world(&self) -> ArtifactDigest { self.world }
    pub fn subject(&self) -> ArtifactDigest { self.subject }
    pub fn payload(&self) -> &CanonicalValue { &self.payload }
    pub fn polarity(&self) -> FactPolarity { self.polarity }
    pub fn evidence(&self) -> ArtifactDigest { self.evidence }
}

impl StructuralIdentity for SharedFact {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("SharedFact");
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        object.insert("subject".into(), CanonicalValue::Digest(self.subject));
        object.insert("payload".into(), self.payload.clone());
        object.insert("polarity".into(), CanonicalValue::String(self.polarity.as_str().into()));
        object.insert("evidence".into(), CanonicalValue::Digest(self.evidence));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClosureContext {
    generation: ArtifactDigest,
    world: ArtifactDigest,
    activated_packages: Vec<ArtifactDigest>,
    closure_rule_set: ArtifactDigest,
    authority_policy: ArtifactDigest,
}

impl ClosureContext {
    pub fn new(
        generation: ArtifactDigest,
        world: ArtifactDigest,
        activated_packages: Vec<ArtifactDigest>,
        closure_rule_set: ArtifactDigest,
        authority_policy: ArtifactDigest,
    ) -> Self {
        Self {
            generation,
            world,
            activated_packages: sorted_digests(activated_packages),
            closure_rule_set,
            authority_policy,
        }
    }

    pub fn generation(&self) -> ArtifactDigest { self.generation }
    pub fn world(&self) -> ArtifactDigest { self.world }
    pub fn activated_packages(&self) -> &[ArtifactDigest] { &self.activated_packages }
    pub fn closure_rule_set(&self) -> ArtifactDigest { self.closure_rule_set }
    pub fn authority_policy(&self) -> ArtifactDigest { self.authority_policy }
}

impl StructuralIdentity for ClosureContext {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("ClosureContext");
        object.insert("generation".into(), CanonicalValue::Digest(self.generation));
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        object.insert("activated_packages".into(), digest_array(&self.activated_packages));
        object.insert("closure_rule_set".into(), CanonicalValue::Digest(self.closure_rule_set));
        object.insert("authority_policy".into(), CanonicalValue::Digest(self.authority_policy));
        CanonicalValue::Object(object)
    }
}
