use crate::{artifacts::StructuralIdentity, canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const SELF_EXPANSION_SCHEMA_V1: &str = "formula-self-expansion-v1";

fn canonical_object(kind: &str) -> BTreeMap<String, CanonicalValue> {
    BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        (
            "schema".into(),
            CanonicalValue::String(SELF_EXPANSION_SCHEMA_V1.into()),
        ),
    ])
}

fn sorted_digests(mut values: Vec<ArtifactDigest>) -> Vec<ArtifactDigest> {
    values.sort_unstable();
    values.dedup();
    values
}

fn digest_array(values: &[ArtifactDigest]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().copied().map(CanonicalValue::Digest).collect())
}

fn optional_digest(value: Option<ArtifactDigest>) -> CanonicalValue {
    value.map_or(CanonicalValue::Null, CanonicalValue::Digest)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PromotionClass {
    TheoremJudgement,
    StructureWitness,
    CounterexampleNogood,
    InvariantCertifiedBound,
    Representation,
    Reduction,
    MorphismTheoryInterpretation,
    DecompositionSufficientSummary,
    SemanticPrimitive,
    Capability,
    MetaprimitiveSearchMethod,
    Realization,
    PackageTheoryExtension,
    ToolchainCheckerRealization,
}

impl PromotionClass {
    pub const ALL: [Self; 14] = [
        Self::TheoremJudgement,
        Self::StructureWitness,
        Self::CounterexampleNogood,
        Self::InvariantCertifiedBound,
        Self::Representation,
        Self::Reduction,
        Self::MorphismTheoryInterpretation,
        Self::DecompositionSufficientSummary,
        Self::SemanticPrimitive,
        Self::Capability,
        Self::MetaprimitiveSearchMethod,
        Self::Realization,
        Self::PackageTheoryExtension,
        Self::ToolchainCheckerRealization,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::TheoremJudgement => "THEOREM_JUDGEMENT",
            Self::StructureWitness => "STRUCTURE_WITNESS",
            Self::CounterexampleNogood => "COUNTEREXAMPLE_NOGOOD",
            Self::InvariantCertifiedBound => "INVARIANT_CERTIFIED_BOUND",
            Self::Representation => "REPRESENTATION",
            Self::Reduction => "REDUCTION",
            Self::MorphismTheoryInterpretation => "MORPHISM_THEORY_INTERPRETATION",
            Self::DecompositionSufficientSummary => "DECOMPOSITION_SUFFICIENT_SUMMARY",
            Self::SemanticPrimitive => "SEMANTIC_PRIMITIVE",
            Self::Capability => "CAPABILITY",
            Self::MetaprimitiveSearchMethod => "METAPRIMITIVE_SEARCH_METHOD",
            Self::Realization => "REALIZATION",
            Self::PackageTheoryExtension => "PACKAGE_THEORY_EXTENSION",
            Self::ToolchainCheckerRealization => "TOOLCHAIN_CHECKER_REALIZATION",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ActivationMode {
    ManualOnly,
    ShadowOnly,
    BoundedAutomatic,
    DefaultAutomatic,
    Superseded,
    Quarantined,
}

impl ActivationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ManualOnly => "MANUAL_ONLY",
            Self::ShadowOnly => "SHADOW_ONLY",
            Self::BoundedAutomatic => "BOUNDED_AUTOMATIC",
            Self::DefaultAutomatic => "DEFAULT_AUTOMATIC",
            Self::Superseded => "SUPERSEDED",
            Self::Quarantined => "QUARANTINED",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SemanticChangeClass {
    RealizationOnly,
    DefinitionalEquivalent,
    ConservativeExtension,
    TheoremStrengthening,
    AssumptionWeakening,
    SignatureChange,
    NonConservativeChange,
    AuthorityPolicyChange,
}

impl SemanticChangeClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RealizationOnly => "REALIZATION_ONLY",
            Self::DefinitionalEquivalent => "DEFINITIONAL_EQUIVALENT",
            Self::ConservativeExtension => "CONSERVATIVE_EXTENSION",
            Self::TheoremStrengthening => "THEOREM_STRENGTHENING",
            Self::AssumptionWeakening => "ASSUMPTION_WEAKENING",
            Self::SignatureChange => "SIGNATURE_CHANGE",
            Self::NonConservativeChange => "NON_CONSERVATIVE_CHANGE",
            Self::AuthorityPolicyChange => "AUTHORITY_POLICY_CHANGE",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvidenceFreshness {
    UnchangedFresh,
    Transportable,
    Repairable,
    RecheckRequired,
    ReproveRequired,
    Quarantined,
}

impl EvidenceFreshness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnchangedFresh => "UNCHANGED_FRESH",
            Self::Transportable => "TRANSPORTABLE",
            Self::Repairable => "REPAIRABLE",
            Self::RecheckRequired => "RECHECK_REQUIRED",
            Self::ReproveRequired => "REPROVE_REQUIRED",
            Self::Quarantined => "QUARANTINED",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SupersessionKind {
    SupersededBy,
    RefutedBy,
    ReplacedRealizationBy,
    WithdrawnFromDefaultActivation,
}

impl SupersessionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SupersededBy => "SUPERSEDED_BY",
            Self::RefutedBy => "REFUTED_BY",
            Self::ReplacedRealizationBy => "REPLACED_REALIZATION_BY",
            Self::WithdrawnFromDefaultActivation => "WITHDRAWN_FROM_DEFAULT_ACTIVATION",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionClassPolicy {
    class: PromotionClass,
    may_change_universe: bool,
    may_change_capability_closure: bool,
    may_change_grammar: bool,
    may_change_realization_selection: bool,
    allowed_activation_modes: Vec<ActivationMode>,
    requires_shadow_gate: bool,
}

impl PromotionClassPolicy {
    fn new(
        class: PromotionClass,
        may_change_universe: bool,
        may_change_capability_closure: bool,
        may_change_grammar: bool,
        may_change_realization_selection: bool,
        mut allowed_activation_modes: Vec<ActivationMode>,
        requires_shadow_gate: bool,
    ) -> Self {
        allowed_activation_modes.sort_unstable();
        allowed_activation_modes.dedup();
        Self {
            class,
            may_change_universe,
            may_change_capability_closure,
            may_change_grammar,
            may_change_realization_selection,
            allowed_activation_modes,
            requires_shadow_gate,
        }
    }

    pub fn class(&self) -> PromotionClass {
        self.class
    }

    pub fn may_change_universe(&self) -> bool {
        self.may_change_universe
    }

    pub fn may_change_capability_closure(&self) -> bool {
        self.may_change_capability_closure
    }

    pub fn may_change_grammar(&self) -> bool {
        self.may_change_grammar
    }

    pub fn may_change_realization_selection(&self) -> bool {
        self.may_change_realization_selection
    }

    pub fn allowed_activation_modes(&self) -> &[ActivationMode] {
        &self.allowed_activation_modes
    }

    pub fn requires_shadow_gate(&self) -> bool {
        self.requires_shadow_gate
    }

    pub fn allows_mode(&self, mode: ActivationMode) -> bool {
        self.allowed_activation_modes.binary_search(&mode).is_ok()
    }
}

impl StructuralIdentity for PromotionClassPolicy {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("PromotionClassPolicy");
        object.insert(
            "class".into(),
            CanonicalValue::String(self.class.as_str().into()),
        );
        object.insert(
            "may_change_universe".into(),
            CanonicalValue::Bool(self.may_change_universe),
        );
        object.insert(
            "may_change_capability_closure".into(),
            CanonicalValue::Bool(self.may_change_capability_closure),
        );
        object.insert(
            "may_change_grammar".into(),
            CanonicalValue::Bool(self.may_change_grammar),
        );
        object.insert(
            "may_change_realization_selection".into(),
            CanonicalValue::Bool(self.may_change_realization_selection),
        );
        object.insert(
            "allowed_activation_modes".into(),
            CanonicalValue::Array(
                self.allowed_activation_modes
                    .iter()
                    .map(|mode| CanonicalValue::String(mode.as_str().into()))
                    .collect(),
            ),
        );
        object.insert(
            "requires_shadow_gate".into(),
            CanonicalValue::Bool(self.requires_shadow_gate),
        );
        CanonicalValue::Object(object)
    }
}

pub struct PromotionClassRegistryV1;

impl PromotionClassRegistryV1 {
    pub fn policy(class: PromotionClass) -> PromotionClassPolicy {
        use ActivationMode::{
            BoundedAutomatic, DefaultAutomatic, ManualOnly, Quarantined, ShadowOnly, Superseded,
        };
        use PromotionClass::*;

        let terminal = [Superseded, Quarantined];
        let with_terminal = |mut modes: Vec<ActivationMode>| {
            modes.extend(terminal);
            modes
        };

        match class {
            TheoremJudgement => PromotionClassPolicy::new(
                class,
                true,
                false,
                false,
                false,
                with_terminal(vec![ManualOnly]),
                false,
            ),
            StructureWitness => PromotionClassPolicy::new(
                class,
                true,
                true,
                false,
                false,
                with_terminal(vec![ManualOnly, DefaultAutomatic]),
                false,
            ),
            CounterexampleNogood => PromotionClassPolicy::new(
                class,
                true,
                true,
                true,
                false,
                with_terminal(vec![ManualOnly, ShadowOnly, BoundedAutomatic]),
                false,
            ),
            InvariantCertifiedBound => PromotionClassPolicy::new(
                class,
                true,
                true,
                false,
                false,
                with_terminal(vec![ManualOnly, DefaultAutomatic]),
                false,
            ),
            Representation
            | Reduction
            | MorphismTheoryInterpretation
            | DecompositionSufficientSummary => PromotionClassPolicy::new(
                class,
                true,
                true,
                true,
                false,
                with_terminal(vec![
                    ManualOnly,
                    ShadowOnly,
                    BoundedAutomatic,
                    DefaultAutomatic,
                ]),
                false,
            ),
            SemanticPrimitive | Capability | PackageTheoryExtension => PromotionClassPolicy::new(
                class,
                true,
                true,
                true,
                false,
                with_terminal(vec![
                    ManualOnly,
                    ShadowOnly,
                    BoundedAutomatic,
                    DefaultAutomatic,
                ]),
                false,
            ),
            MetaprimitiveSearchMethod => PromotionClassPolicy::new(
                class,
                true,
                false,
                true,
                false,
                with_terminal(vec![
                    ManualOnly,
                    ShadowOnly,
                    BoundedAutomatic,
                    DefaultAutomatic,
                ]),
                true,
            ),
            Realization | ToolchainCheckerRealization => PromotionClassPolicy::new(
                class,
                false,
                false,
                false,
                true,
                with_terminal(vec![ManualOnly, DefaultAutomatic]),
                false,
            ),
        }
    }

    pub fn policies() -> Vec<PromotionClassPolicy> {
        PromotionClass::ALL.into_iter().map(Self::policy).collect()
    }

    pub fn digest() -> ArtifactDigest {
        let mut object = canonical_object("PromotionClassRegistryV1");
        object.insert(
            "policies".into(),
            CanonicalValue::Array(
                Self::policies()
                    .iter()
                    .map(StructuralIdentity::canonical_value)
                    .collect(),
            ),
        );
        CanonicalValue::Object(object).digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassifiedPromotionCandidate {
    base_promotion_candidate: ArtifactDigest,
    class: PromotionClass,
    requested_activation_mode: ActivationMode,
    semantic_change_class: SemanticChangeClass,
    activation_effects: Vec<ArtifactDigest>,
    grammar_effects: Vec<ArtifactDigest>,
    scope: Vec<ArtifactDigest>,
}

impl ClassifiedPromotionCandidate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        base_promotion_candidate: ArtifactDigest,
        class: PromotionClass,
        requested_activation_mode: ActivationMode,
        semantic_change_class: SemanticChangeClass,
        activation_effects: Vec<ArtifactDigest>,
        grammar_effects: Vec<ArtifactDigest>,
        scope: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            base_promotion_candidate,
            class,
            requested_activation_mode,
            semantic_change_class,
            activation_effects: sorted_digests(activation_effects),
            grammar_effects: sorted_digests(grammar_effects),
            scope: sorted_digests(scope),
        }
    }

    pub fn base_promotion_candidate(&self) -> ArtifactDigest {
        self.base_promotion_candidate
    }

    pub fn class(&self) -> PromotionClass {
        self.class
    }

    pub fn requested_activation_mode(&self) -> ActivationMode {
        self.requested_activation_mode
    }

    pub fn semantic_change_class(&self) -> SemanticChangeClass {
        self.semantic_change_class
    }

    pub fn activation_effects(&self) -> &[ArtifactDigest] {
        &self.activation_effects
    }

    pub fn grammar_effects(&self) -> &[ArtifactDigest] {
        &self.grammar_effects
    }

    pub fn scope(&self) -> &[ArtifactDigest] {
        &self.scope
    }
}

impl StructuralIdentity for ClassifiedPromotionCandidate {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("ClassifiedPromotionCandidate");
        object.insert(
            "base_promotion_candidate".into(),
            CanonicalValue::Digest(self.base_promotion_candidate),
        );
        object.insert(
            "class".into(),
            CanonicalValue::String(self.class.as_str().into()),
        );
        object.insert(
            "requested_activation_mode".into(),
            CanonicalValue::String(self.requested_activation_mode.as_str().into()),
        );
        object.insert(
            "semantic_change_class".into(),
            CanonicalValue::String(self.semantic_change_class.as_str().into()),
        );
        object.insert(
            "activation_effects".into(),
            digest_array(&self.activation_effects),
        );
        object.insert(
            "grammar_effects".into(),
            digest_array(&self.grammar_effects),
        );
        object.insert("scope".into(), digest_array(&self.scope));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpansionActivationRecord {
    subject: ArtifactDigest,
    promotion_class: PromotionClass,
    generation: ArtifactDigest,
    world: ArtifactDigest,
    mode: ActivationMode,
    evidence: Vec<ArtifactDigest>,
    scope: Vec<ArtifactDigest>,
}

impl ExpansionActivationRecord {
    pub fn new(
        subject: ArtifactDigest,
        promotion_class: PromotionClass,
        generation: ArtifactDigest,
        world: ArtifactDigest,
        mode: ActivationMode,
        evidence: Vec<ArtifactDigest>,
        scope: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            subject,
            promotion_class,
            generation,
            world,
            mode,
            evidence: sorted_digests(evidence),
            scope: sorted_digests(scope),
        }
    }

    pub fn subject(&self) -> ArtifactDigest {
        self.subject
    }

    pub fn promotion_class(&self) -> PromotionClass {
        self.promotion_class
    }

    pub fn generation(&self) -> ArtifactDigest {
        self.generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn mode(&self) -> ActivationMode {
        self.mode
    }

    pub fn evidence(&self) -> &[ArtifactDigest] {
        &self.evidence
    }

    pub fn scope(&self) -> &[ArtifactDigest] {
        &self.scope
    }
}

impl StructuralIdentity for ExpansionActivationRecord {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("ExpansionActivationRecord");
        object.insert("subject".into(), CanonicalValue::Digest(self.subject));
        object.insert(
            "promotion_class".into(),
            CanonicalValue::String(self.promotion_class.as_str().into()),
        );
        object.insert("generation".into(), CanonicalValue::Digest(self.generation));
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        object.insert(
            "mode".into(),
            CanonicalValue::String(self.mode.as_str().into()),
        );
        object.insert("evidence".into(), digest_array(&self.evidence));
        object.insert("scope".into(), digest_array(&self.scope));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrammarGeneration {
    universe_generation: ArtifactDigest,
    parent_grammar: Option<ArtifactDigest>,
    activated_constructors: Vec<ArtifactDigest>,
    activated_metaprimitives: Vec<ArtifactDigest>,
    shadow_metaprimitives: Vec<ArtifactDigest>,
    activated_route_rules: Vec<ArtifactDigest>,
    activated_theory_rules: Vec<ArtifactDigest>,
}

impl GrammarGeneration {
    pub fn new(
        universe_generation: ArtifactDigest,
        parent_grammar: Option<ArtifactDigest>,
        activated_constructors: Vec<ArtifactDigest>,
        activated_metaprimitives: Vec<ArtifactDigest>,
        shadow_metaprimitives: Vec<ArtifactDigest>,
        activated_route_rules: Vec<ArtifactDigest>,
        activated_theory_rules: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            universe_generation,
            parent_grammar,
            activated_constructors: sorted_digests(activated_constructors),
            activated_metaprimitives: sorted_digests(activated_metaprimitives),
            shadow_metaprimitives: sorted_digests(shadow_metaprimitives),
            activated_route_rules: sorted_digests(activated_route_rules),
            activated_theory_rules: sorted_digests(activated_theory_rules),
        }
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn parent_grammar(&self) -> Option<ArtifactDigest> {
        self.parent_grammar
    }

    pub fn activated_constructors(&self) -> &[ArtifactDigest] {
        &self.activated_constructors
    }

    pub fn activated_metaprimitives(&self) -> &[ArtifactDigest] {
        &self.activated_metaprimitives
    }

    pub fn shadow_metaprimitives(&self) -> &[ArtifactDigest] {
        &self.shadow_metaprimitives
    }

    pub fn activated_route_rules(&self) -> &[ArtifactDigest] {
        &self.activated_route_rules
    }

    pub fn activated_theory_rules(&self) -> &[ArtifactDigest] {
        &self.activated_theory_rules
    }
}

impl StructuralIdentity for GrammarGeneration {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("GrammarGeneration");
        object.insert(
            "universe_generation".into(),
            CanonicalValue::Digest(self.universe_generation),
        );
        object.insert(
            "parent_grammar".into(),
            optional_digest(self.parent_grammar),
        );
        object.insert(
            "activated_constructors".into(),
            digest_array(&self.activated_constructors),
        );
        object.insert(
            "activated_metaprimitives".into(),
            digest_array(&self.activated_metaprimitives),
        );
        object.insert(
            "shadow_metaprimitives".into(),
            digest_array(&self.shadow_metaprimitives),
        );
        object.insert(
            "activated_route_rules".into(),
            digest_array(&self.activated_route_rules),
        );
        object.insert(
            "activated_theory_rules".into(),
            digest_array(&self.activated_theory_rules),
        );
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaprimitiveGateEvidence {
    soundness: ArtifactDigest,
    applicability: ArtifactDigest,
    termination: ArtifactDigest,
    preservation: ArtifactDigest,
    adversarial: ArtifactDigest,
    transfer: ArtifactDigest,
    comparison: ArtifactDigest,
    fallback: ArtifactDigest,
    scope: Vec<ArtifactDigest>,
}

impl MetaprimitiveGateEvidence {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        soundness: ArtifactDigest,
        applicability: ArtifactDigest,
        termination: ArtifactDigest,
        preservation: ArtifactDigest,
        adversarial: ArtifactDigest,
        transfer: ArtifactDigest,
        comparison: ArtifactDigest,
        fallback: ArtifactDigest,
        scope: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            soundness,
            applicability,
            termination,
            preservation,
            adversarial,
            transfer,
            comparison,
            fallback,
            scope: sorted_digests(scope),
        }
    }

    pub fn required_evidence(&self) -> [ArtifactDigest; 8] {
        [
            self.soundness,
            self.applicability,
            self.termination,
            self.preservation,
            self.adversarial,
            self.transfer,
            self.comparison,
            self.fallback,
        ]
    }

    pub fn scope(&self) -> &[ArtifactDigest] {
        &self.scope
    }
}

impl StructuralIdentity for MetaprimitiveGateEvidence {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("MetaprimitiveGateEvidence");
        object.insert("soundness".into(), CanonicalValue::Digest(self.soundness));
        object.insert(
            "applicability".into(),
            CanonicalValue::Digest(self.applicability),
        );
        object.insert(
            "termination".into(),
            CanonicalValue::Digest(self.termination),
        );
        object.insert(
            "preservation".into(),
            CanonicalValue::Digest(self.preservation),
        );
        object.insert(
            "adversarial".into(),
            CanonicalValue::Digest(self.adversarial),
        );
        object.insert("transfer".into(), CanonicalValue::Digest(self.transfer));
        object.insert("comparison".into(), CanonicalValue::Digest(self.comparison));
        object.insert("fallback".into(), CanonicalValue::Digest(self.fallback));
        object.insert("scope".into(), digest_array(&self.scope));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticChange {
    old_artifact: ArtifactDigest,
    new_artifact: ArtifactDigest,
    class: SemanticChangeClass,
    changed_dependencies: Vec<ArtifactDigest>,
    affected_authority_cone: Vec<ArtifactDigest>,
    evidence: Vec<ArtifactDigest>,
}

impl SemanticChange {
    pub fn new(
        old_artifact: ArtifactDigest,
        new_artifact: ArtifactDigest,
        class: SemanticChangeClass,
        changed_dependencies: Vec<ArtifactDigest>,
        affected_authority_cone: Vec<ArtifactDigest>,
        evidence: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            old_artifact,
            new_artifact,
            class,
            changed_dependencies: sorted_digests(changed_dependencies),
            affected_authority_cone: sorted_digests(affected_authority_cone),
            evidence: sorted_digests(evidence),
        }
    }

    pub fn old_artifact(&self) -> ArtifactDigest {
        self.old_artifact
    }

    pub fn new_artifact(&self) -> ArtifactDigest {
        self.new_artifact
    }

    pub fn class(&self) -> SemanticChangeClass {
        self.class
    }

    pub fn changed_dependencies(&self) -> &[ArtifactDigest] {
        &self.changed_dependencies
    }

    pub fn affected_authority_cone(&self) -> &[ArtifactDigest] {
        &self.affected_authority_cone
    }

    pub fn evidence(&self) -> &[ArtifactDigest] {
        &self.evidence
    }
}

impl StructuralIdentity for SemanticChange {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("SemanticChange");
        object.insert(
            "old_artifact".into(),
            CanonicalValue::Digest(self.old_artifact),
        );
        object.insert(
            "new_artifact".into(),
            CanonicalValue::Digest(self.new_artifact),
        );
        object.insert(
            "class".into(),
            CanonicalValue::String(self.class.as_str().into()),
        );
        object.insert(
            "changed_dependencies".into(),
            digest_array(&self.changed_dependencies),
        );
        object.insert(
            "affected_authority_cone".into(),
            digest_array(&self.affected_authority_cone),
        );
        object.insert("evidence".into(), digest_array(&self.evidence));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofTransportPlan {
    source_evidence: ArtifactDigest,
    source_target: ArtifactDigest,
    destination_target: ArtifactDigest,
    certified_relation: ArtifactDigest,
    destination_dependencies: Vec<ArtifactDigest>,
    required_checker: ArtifactDigest,
}

impl ProofTransportPlan {
    pub fn new(
        source_evidence: ArtifactDigest,
        source_target: ArtifactDigest,
        destination_target: ArtifactDigest,
        certified_relation: ArtifactDigest,
        destination_dependencies: Vec<ArtifactDigest>,
        required_checker: ArtifactDigest,
    ) -> Self {
        Self {
            source_evidence,
            source_target,
            destination_target,
            certified_relation,
            destination_dependencies: sorted_digests(destination_dependencies),
            required_checker,
        }
    }

    pub fn source_evidence(&self) -> ArtifactDigest {
        self.source_evidence
    }

    pub fn source_target(&self) -> ArtifactDigest {
        self.source_target
    }

    pub fn destination_target(&self) -> ArtifactDigest {
        self.destination_target
    }

    pub fn certified_relation(&self) -> ArtifactDigest {
        self.certified_relation
    }

    pub fn destination_dependencies(&self) -> &[ArtifactDigest] {
        &self.destination_dependencies
    }

    pub fn required_checker(&self) -> ArtifactDigest {
        self.required_checker
    }
}

impl StructuralIdentity for ProofTransportPlan {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("ProofTransportPlan");
        object.insert(
            "source_evidence".into(),
            CanonicalValue::Digest(self.source_evidence),
        );
        object.insert(
            "source_target".into(),
            CanonicalValue::Digest(self.source_target),
        );
        object.insert(
            "destination_target".into(),
            CanonicalValue::Digest(self.destination_target),
        );
        object.insert(
            "certified_relation".into(),
            CanonicalValue::Digest(self.certified_relation),
        );
        object.insert(
            "destination_dependencies".into(),
            digest_array(&self.destination_dependencies),
        );
        object.insert(
            "required_checker".into(),
            CanonicalValue::Digest(self.required_checker),
        );
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofRepairPlan {
    source_evidence: ArtifactDigest,
    semantic_change: ArtifactDigest,
    affected_slice: Vec<ArtifactDigest>,
    repair_obligations: Vec<ArtifactDigest>,
    required_checker: ArtifactDigest,
}

impl ProofRepairPlan {
    pub fn new(
        source_evidence: ArtifactDigest,
        semantic_change: ArtifactDigest,
        affected_slice: Vec<ArtifactDigest>,
        repair_obligations: Vec<ArtifactDigest>,
        required_checker: ArtifactDigest,
    ) -> Self {
        Self {
            source_evidence,
            semantic_change,
            affected_slice: sorted_digests(affected_slice),
            repair_obligations: sorted_digests(repair_obligations),
            required_checker,
        }
    }

    pub fn source_evidence(&self) -> ArtifactDigest {
        self.source_evidence
    }

    pub fn semantic_change(&self) -> ArtifactDigest {
        self.semantic_change
    }

    pub fn affected_slice(&self) -> &[ArtifactDigest] {
        &self.affected_slice
    }

    pub fn repair_obligations(&self) -> &[ArtifactDigest] {
        &self.repair_obligations
    }

    pub fn required_checker(&self) -> ArtifactDigest {
        self.required_checker
    }
}

impl StructuralIdentity for ProofRepairPlan {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("ProofRepairPlan");
        object.insert(
            "source_evidence".into(),
            CanonicalValue::Digest(self.source_evidence),
        );
        object.insert(
            "semantic_change".into(),
            CanonicalValue::Digest(self.semantic_change),
        );
        object.insert("affected_slice".into(), digest_array(&self.affected_slice));
        object.insert(
            "repair_obligations".into(),
            digest_array(&self.repair_obligations),
        );
        object.insert(
            "required_checker".into(),
            CanonicalValue::Digest(self.required_checker),
        );
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupersessionRecord {
    subject: ArtifactDigest,
    successor: ArtifactDigest,
    kind: SupersessionKind,
    source_generation: ArtifactDigest,
    selection_scope: Vec<ArtifactDigest>,
    evidence: Vec<ArtifactDigest>,
}

impl SupersessionRecord {
    pub fn new(
        subject: ArtifactDigest,
        successor: ArtifactDigest,
        kind: SupersessionKind,
        source_generation: ArtifactDigest,
        selection_scope: Vec<ArtifactDigest>,
        evidence: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            subject,
            successor,
            kind,
            source_generation,
            selection_scope: sorted_digests(selection_scope),
            evidence: sorted_digests(evidence),
        }
    }

    pub fn subject(&self) -> ArtifactDigest {
        self.subject
    }

    pub fn successor(&self) -> ArtifactDigest {
        self.successor
    }

    pub fn kind(&self) -> SupersessionKind {
        self.kind
    }

    pub fn source_generation(&self) -> ArtifactDigest {
        self.source_generation
    }

    pub fn selection_scope(&self) -> &[ArtifactDigest] {
        &self.selection_scope
    }

    pub fn evidence(&self) -> &[ArtifactDigest] {
        &self.evidence
    }
}

impl StructuralIdentity for SupersessionRecord {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("SupersessionRecord");
        object.insert("subject".into(), CanonicalValue::Digest(self.subject));
        object.insert("successor".into(), CanonicalValue::Digest(self.successor));
        object.insert(
            "kind".into(),
            CanonicalValue::String(self.kind.as_str().into()),
        );
        object.insert(
            "source_generation".into(),
            CanonicalValue::Digest(self.source_generation),
        );
        object.insert(
            "selection_scope".into(),
            digest_array(&self.selection_scope),
        );
        object.insert("evidence".into(), digest_array(&self.evidence));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationUpgrade {
    semantic_artifact: ArtifactDigest,
    universe_generation: ArtifactDigest,
    old_realization: ArtifactDigest,
    new_realization: ArtifactDigest,
    semantic_change_class: SemanticChangeClass,
    validation_evidence: Vec<ArtifactDigest>,
    selection_policy: ArtifactDigest,
}

impl RealizationUpgrade {
    pub fn new(
        semantic_artifact: ArtifactDigest,
        universe_generation: ArtifactDigest,
        old_realization: ArtifactDigest,
        new_realization: ArtifactDigest,
        semantic_change_class: SemanticChangeClass,
        validation_evidence: Vec<ArtifactDigest>,
        selection_policy: ArtifactDigest,
    ) -> Self {
        Self {
            semantic_artifact,
            universe_generation,
            old_realization,
            new_realization,
            semantic_change_class,
            validation_evidence: sorted_digests(validation_evidence),
            selection_policy,
        }
    }

    pub fn semantic_artifact(&self) -> ArtifactDigest {
        self.semantic_artifact
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn old_realization(&self) -> ArtifactDigest {
        self.old_realization
    }

    pub fn new_realization(&self) -> ArtifactDigest {
        self.new_realization
    }

    pub fn semantic_change_class(&self) -> SemanticChangeClass {
        self.semantic_change_class
    }

    pub fn validation_evidence(&self) -> &[ArtifactDigest] {
        &self.validation_evidence
    }

    pub fn selection_policy(&self) -> ArtifactDigest {
        self.selection_policy
    }
}

impl StructuralIdentity for RealizationUpgrade {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("RealizationUpgrade");
        object.insert(
            "semantic_artifact".into(),
            CanonicalValue::Digest(self.semantic_artifact),
        );
        object.insert(
            "universe_generation".into(),
            CanonicalValue::Digest(self.universe_generation),
        );
        object.insert(
            "old_realization".into(),
            CanonicalValue::Digest(self.old_realization),
        );
        object.insert(
            "new_realization".into(),
            CanonicalValue::Digest(self.new_realization),
        );
        object.insert(
            "semantic_change_class".into(),
            CanonicalValue::String(self.semantic_change_class.as_str().into()),
        );
        object.insert(
            "validation_evidence".into(),
            digest_array(&self.validation_evidence),
        );
        object.insert(
            "selection_policy".into(),
            CanonicalValue::Digest(self.selection_policy),
        );
        CanonicalValue::Object(object)
    }
}
