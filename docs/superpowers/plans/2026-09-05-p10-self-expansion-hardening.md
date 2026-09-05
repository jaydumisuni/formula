# P10 Self-Expansion Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Generalize D5 self-expansion beyond the First-Light semantic primitive and prove that an independently promoted `STRUCTURE_WITNESS` unlocks an existing generic capability without adding solver/package code.

**Architecture:** Preserve the frozen P7-P9 promotion, generation, capability, and realization identities. Add a P10 class/effect wrapper and checker authorization layer, derived activation/grammar/evolution records, history-preserving rollback/selection tooling, and an independent P10 proof harness. The canonical proof uses the existing Rational package's `cap:rational:field <- goal:rational:field` contract to demonstrate non-primitive capability growth.

**Tech Stack:** Rust 1.98.0, Cargo locked workspace, SQLite/rusqlite authority store, SHA-256 `ArtifactDigest`, existing formula-core/check/store/packages/engine/realize/first-light crates, GitHub Actions Ubuntu 24.04.

**Spec:** `docs/superpowers/specs/2026-09-05-p10-self-expansion-hardening-design.md`

## Global Constraints

- Exact frozen P9 proof predecessor: `b353365fa8b20a13b658c07b3027334b69eff108`.
- P9 canonical proof remains run `33950470295`, job `101264153162`, success.
- Do not modify P7 `PromotionCandidate`, `PromotionRecord`, `PromotionAuthorization`, P8 realization identities, or P9 First-Light manifest schemas in ways that change their frozen structural digests.
- Mathematical publication remains checker-authorized `AuthorityStore::promote`; P10 class/effect authorization cannot bypass it.
- Capability closure remains generation/world scoped and derived from admitted authority.
- `Lambda_g` is derived discovery grammar identity, not mathematical authority.
- Metaprimitives cannot become automatic without strict gate authorization; `SHADOW_ONLY` never influences authoritative campaign output.
- `REALIZATION_ONLY` updates cannot create semantic admissions or change the active UniverseGeneration.
- Rollback/supersession is append-only and history preserving.
- P11 federation breadth, P12 self-hosting/bootstrap, P13 Ptah, GPU/SIMD/JIT, and a universal promotion-worth metric are excluded.
- Temporary P10 development workflow must be removed before the permanent canonical P10 proof boundary.

---

### Task 1: P10 development proof lane and core self-expansion identities

**Files:**
- Create: `.github/workflows/p10-development.yml`
- Create: `crates/formula-core/src/self_expansion.rs`
- Modify: `crates/formula-core/src/lib.rs`
- Test: `crates/formula-core/tests/p10_self_expansion_identity.rs`

**Interfaces:**
- Produces: `PromotionClass`, `ActivationMode`, `SemanticChangeClass`, `EvidenceFreshness`, `SupersessionKind`, `PromotionClassPolicy`, `PromotionClassRegistryV1`, `ClassifiedPromotionCandidate`, `ExpansionActivationRecord`, `GrammarGeneration`, `MetaprimitiveGateEvidence`, `SemanticChange`, `ProofTransportPlan`, `ProofRepairPlan`, `SupersessionRecord`, `RealizationUpgrade`.

- [ ] **Step 1: Add the branch-only development workflow**

Create `.github/workflows/p10-development.yml` with `contents: read`, `ubuntu-24.04`, Rust `1.98.0`, Cargo locked dependency priming, then:

```bash
cargo test --workspace --all-targets --locked
cargo build --workspace --all-targets --locked
cargo fmt --all -- --check
CARGO_NET_OFFLINE=true cargo clippy --workspace --all-targets --locked -- -D warnings
```

Trigger only on `implementation/p10-self-expansion-hardening` pushes and manual dispatch.

- [ ] **Step 2: Write RED identity tests**

Create tests covering deterministic canonical identity and policy completeness:

```rust
#[test]
fn every_frozen_d5_promotion_class_has_one_deterministic_policy() {
    let policies = PromotionClassRegistryV1::policies();
    assert_eq!(policies.len(), PromotionClass::ALL.len());
    for class in PromotionClass::ALL {
        assert_eq!(policies.iter().filter(|p| p.class() == class).count(), 1);
    }
    assert_eq!(
        PromotionClassRegistryV1::digest(),
        PromotionClassRegistryV1::digest()
    );
}

#[test]
fn classified_promotion_identity_is_order_independent_but_effect_sensitive() {
    let a = ClassifiedPromotionCandidate::new(
        digest("base"),
        PromotionClass::StructureWitness,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![digest("cap-b"), digest("cap-a")],
        vec![],
        vec![digest("scope-b"), digest("scope-a")],
    );
    let b = ClassifiedPromotionCandidate::new(
        digest("base"),
        PromotionClass::StructureWitness,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![digest("cap-a"), digest("cap-b")],
        vec![],
        vec![digest("scope-a"), digest("scope-b")],
    );
    assert_eq!(a.structural_digest(), b.structural_digest());
    assert_ne!(a.structural_digest(), a.with_class(PromotionClass::SemanticPrimitive).structural_digest());
}
```

Also assert the original P7 promotion identity fixture remains unchanged by importing only the new module, not modifying the old schema.

- [ ] **Step 3: Run core test and prove RED**

```bash
cargo test -p formula-core --test p10_self_expansion_identity --locked
```

Expected: compile failure because `formula_core::self_expansion` does not exist.

- [ ] **Step 4: Implement enums and canonical helpers**

Use schema constant:

```rust
const SELF_EXPANSION_SCHEMA_V1: &str = "formula-self-expansion-v1";
```

Define exact frozen enums from the spec and public `as_str()` methods. Add:

```rust
impl PromotionClass {
    pub const ALL: [Self; 14] = [ /* every frozen D5 class exactly once */ ];
}
```

All structural lists use canonical sort/dedup helpers before identity computation.

- [ ] **Step 5: Implement deterministic class policies**

`PromotionClassPolicy` fields:

```rust
class: PromotionClass,
may_change_universe: bool,
may_change_capability_closure: bool,
may_change_grammar: bool,
may_change_realization_selection: bool,
allowed_activation_modes: Vec<ActivationMode>,
requires_shadow_gate: bool,
```

`PromotionClassRegistryV1::policy(class)` returns a total static mapping. Key policies:

```text
STRUCTURE_WITNESS: U=true, capability=true, grammar=false
COUNTEREXAMPLE_NOGOOD: U=true, capability=true, grammar=true, auto requires scope
REDUCTION/MORPHISM: U=true, capability=true, grammar=true
METAPRIMITIVE: U=true, grammar=true, shadow gate=true
REALIZATION: U=false, realization=true, semantic change must be REALIZATION_ONLY
```

- [ ] **Step 6: Implement canonical record types**

Implement constructors/getters/`StructuralIdentity` for the interfaces listed above. `ClassifiedPromotionCandidate` must reference, not embed/modify, the P7 base promotion digest.

`GrammarGeneration` fields are:

```rust
universe_generation: ArtifactDigest,
parent_grammar: Option<ArtifactDigest>,
activated_constructors: Vec<ArtifactDigest>,
activated_metaprimitives: Vec<ArtifactDigest>,
shadow_metaprimitives: Vec<ArtifactDigest>,
activated_route_rules: Vec<ArtifactDigest>,
activated_theory_rules: Vec<ArtifactDigest>,
```

- [ ] **Step 7: Run Task 1 gate and commit**

```bash
cargo test -p formula-core --all-targets --locked
cargo fmt --all -- --check
cargo clippy -p formula-core --all-targets --locked -- -D warnings
```

Expected: all pass. Commit Task 1 only.

---

### Task 2: Checker-owned P10 class policy, nogood scope, route gate, and metaprimitive shadow gate

**Files:**
- Create: `crates/formula-check/src/self_expansion.rs`
- Modify: `crates/formula-check/src/lib.rs`
- Test: `crates/formula-check/tests/p10_self_expansion_policy.rs`

**Interfaces:**
- Consumes: existing opaque `PromotionAuthorization`, `ClassifiedPromotionCandidate`, exact parent `UniverseGeneration`.
- Produces: opaque `ExpansionAuthorization`, `MetaprimitiveGateAuthorization`, `authorize_expansion_v1`, `authorize_metaprimitive_gate_v1`.

- [ ] **Step 1: Write RED policy tests**

```rust
#[test]
fn classified_authorization_must_reference_exact_base_promotion() {
    // Obtain a real PromotionAuthorization through authorize_promotion_v1.
    // Wrap a different promotion digest and assert BasePromotionMismatch.
}

#[test]
fn structure_witness_may_unlock_capability_but_cannot_directly_change_grammar() {
    // Activation effects accepted; non-empty grammar effects rejected.
}

#[test]
fn automatic_nogood_requires_exact_nonempty_scope() {
    // BOUNDED_AUTOMATIC + empty scope -> NogoodScopeRequired.
}

#[test]
fn metaprimitive_default_automatic_requires_strict_gate() {
    // Ordinary expansion auth can admit SHADOW_ONLY.
    // DEFAULT_AUTOMATIC without MetaprimitiveGateAuthorization -> reject.
}
```

- [ ] **Step 2: Run and prove RED**

```bash
cargo test -p formula-check --test p10_self_expansion_policy --locked
```

- [ ] **Step 3: Implement typed policy failures and opaque authorization**

```rust
pub enum ExpansionPolicyFailure {
    BasePromotionMismatch,
    ParentGenerationMismatch,
    ActivationModeForbidden,
    CapabilityEffectForbidden,
    GrammarEffectForbidden,
    RealizationOnlySemanticAdmissionForbidden,
    NogoodScopeRequired,
    MetaprimitiveGateRequired,
    GateEvidenceNotPromotionBound(ArtifactDigest),
    GateScopeMismatch,
}

pub struct ExpansionAuthorization { /* private fields + getters */ }
pub struct MetaprimitiveGateAuthorization { /* private fields + getters */ }
```

The constructor remains private to `formula-check`.

- [ ] **Step 4: Implement `authorize_expansion_v1`**

Signature:

```rust
pub fn authorize_expansion_v1(
    base: &PromotionAuthorization,
    classified: &ClassifiedPromotionCandidate,
    parent: &UniverseGeneration,
    metaprimitive_gate: Option<&MetaprimitiveGateAuthorization>,
) -> Result<ExpansionAuthorization, ExpansionPolicyFailure>
```

Validation order:

```text
exact base promotion digest
exact parent generation
registry activation mode allowed
capability effects permitted
semantic grammar effects permitted
nogood automatic scope nonempty
REALIZATION + REALIZATION_ONLY cannot carry semantic admissions
metaprimitive automatic mode requires exact gate authorization
```

- [ ] **Step 5: Implement strict metaprimitive gate authorization**

```rust
pub fn authorize_metaprimitive_gate_v1(
    base: &PromotionAuthorization,
    classified: &ClassifiedPromotionCandidate,
    gate: &MetaprimitiveGateEvidence,
) -> Result<MetaprimitiveGateAuthorization, ExpansionPolicyFailure>
```

Require class `MetaprimitiveSearchMethod`, every gate evidence digest to be present in `base.authority_bindings()`, and requested gate scope to cover the classified scope. The gate binds all required D5 claim/evidence fields; empty required fields reject.

- [ ] **Step 6: Run formula-check + predecessor promotion tests and commit**

```bash
cargo test -p formula-check --all-targets --locked
cargo test -p formula-check --test p7_promotion_policy --locked
cargo fmt --all -- --check
cargo clippy -p formula-check --all-targets --locked -- -D warnings
```

---

### Task 3: Durable expansion activation, supersession, and generation rollback selection

**Files:**
- Create: `crates/formula-store/src/authority_store/expansion_store.rs`
- Modify: `crates/formula-store/src/authority_store.rs`
- Test: `crates/formula-store/tests/p10_expansion_store.rs`

**Interfaces:**
- Produces: `AuthorityStore::record_expansion_activation`, `resolve_expansion_activation`, `record_supersession`, `supersessions_for`, `select_active_generation`.

- [ ] **Step 1: Write RED store tests**

```rust
#[test]
fn expansion_activation_round_trips_exact_structural_identity() {
    // Persist ExpansionActivationRecord, reopen store, resolve by generation+subject,
    // assert exact structural digest and canonical fields.
}

#[test]
fn activation_rejects_unadmitted_subject_or_unbound_evidence() {
    // Both paths fail closed with typed AuthorityStoreError variants.
}

#[test]
fn rollback_reselects_historical_generation_without_deleting_newer_history() {
    // Create U0 -> U1 -> U2.
    // select_active_generation(U0), then replay U1 and U2 successfully.
    // reselect U2 and prove exact digest replay.
}
```

- [ ] **Step 2: Run and prove RED**

```bash
cargo test -p formula-store --test p10_expansion_store --locked
```

- [ ] **Step 3: Implement expansion activation tables**

Create lazily initialized tables with normalized child rows:

```text
expansion_activations(
  activation_digest PK,
  generation_digest,
  subject_digest,
  promotion_class,
  world_digest,
  activation_mode,
  UNIQUE(generation_digest, subject_digest, promotion_class)
)
expansion_activation_evidence(activation_digest, evidence_digest)
expansion_activation_scope(activation_digest, scope_digest)
```

`record_expansion_activation` validates:

```text
record.generation == active generation
subject is admitted unless class policy is realization-only derived selection
all evidence are generation authority bindings
mode != QUARANTINED for automatic consumption
```

Reconstruct and compare structural digest on resolve.

- [ ] **Step 4: Implement supersession ledger**

Persist/reconstruct `SupersessionRecord` with child evidence rows. No delete/update operation exists for historical supersession rows.

- [ ] **Step 5: Implement exact historical active-generation selection**

```rust
pub fn select_active_generation(
    &mut self,
    target: ArtifactDigest,
) -> Result<ArtifactDigest, AuthorityStoreError>
```

Before changing the pointer:

```rust
let replayed = self.replay_generation(target)?;
assert/reject if replayed.digest() != target;
```

Update only `meta.active_generation` in an immediate SQLite transaction. Do not delete or rewrite any generation/manifests/blobs.

- [ ] **Step 6: Run formula-store predecessor and full tests, then commit**

```bash
cargo test -p formula-store --all-targets --locked
cargo fmt --all -- --check
cargo clippy -p formula-store --all-targets --locked -- -D warnings
```

---

### Task 4: Non-primitive structure-witness capability unlock, scoped nogoods, promoted routes, and `Lambda_g`

**Files:**
- Create: `crates/formula-packages/src/expansion.rs`
- Create: `crates/formula-packages/src/grammar.rs`
- Modify: `crates/formula-packages/src/lib.rs`
- Test: `crates/formula-packages/tests/p10_non_primitive_unlock.rs`
- Test: `crates/formula-packages/tests/p10_activation_hardening.rs`

**Interfaces:**
- Produces: `ScopedNogoodActivation`, `PromotedRouteActivation`, `derive_grammar_generation`, `applicable_nogoods`, `active_routes`.
- Reuses: `AdmittedStructureWitness`, `derive_capabilities`, `CapabilityClosureDelta`.

- [ ] **Step 1: Write the canonical structure-witness RED test**

Use the existing built-in Rational package and exact labels:

```rust
fn id(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn promoted_structure_witness_unlocks_existing_rational_field_capability() {
    let field_goal = id("goal:rational:field");
    let field_cap = id("cap:rational:field");

    // U_g: integer+rational packages admitted and active; no field witness.
    // closure_before does not contain field_cap.
    // Produce a real checker-authorized P7 promotion for StructureWitness.
    // Produce P10 ExpansionAuthorization class=STRUCTURE_WITNESS.
    // AuthorityStore::promote -> U_(g+1).
    // AdmittedStructureWitness::new(U_(g+1), witness) succeeds.
    // Reuse unchanged package manifests.
    // closure_after contains field_cap.
    // CapabilityClosureDelta adds field_cap.
}
```

Assert before/after `rational_package(foundation).structural_digest()` is identical; no package source/config change is part of the proof.

- [ ] **Step 2: Run and prove RED for missing P10 glue**

```bash
cargo test -p formula-packages --test p10_non_primitive_unlock --locked
```

- [ ] **Step 3: Implement scoped nogood activation**

`ScopedNogoodActivation::new` validates admitted artifact, authority-bound evidence, exact world/generation, and non-empty scope for automatic modes. `applicable_nogoods(context_scope)` returns only records whose scope is fully satisfied by the requested context.

- [ ] **Step 4: Implement promoted route activation**

`PromotedRouteActivation` accepts only `PromotionClass::Reduction` or `PromotionClass::MorphismTheoryInterpretation`; automatic modes require non-empty preserved-result-class digests and evidence that is authority-bound. Wrong generation or unsupported class rejects.

- [ ] **Step 5: Implement `derive_grammar_generation`**

Signature:

```rust
pub fn derive_grammar_generation(
    generation: &UniverseGeneration,
    parent_grammar: Option<ArtifactDigest>,
    activations: &[ExpansionActivationRecord],
    routes: &[PromotedRouteActivation],
    theory_rules: &[ArtifactDigest],
) -> Result<GrammarGeneration, ExpansionError>
```

Rules:

```text
exact generation match for every record
DEFAULT/BOUNDED automatic semantic constructors -> activated_constructors
DEFAULT/BOUNDED metaprimitives -> activated_metaprimitives
SHADOW_ONLY metaprimitives -> shadow_metaprimitives only
active route records -> activated_route_rules
SUPERSEDED/QUARANTINED -> no active contribution
```

- [ ] **Step 6: Write and run hardening tests**

Tests must execute:

```text
unscoped automatic nogood reject
nogood scope mismatch no pruning
route missing preservation evidence reject
SHADOW_ONLY metaprimitive excluded from active grammar but recorded as shadow
wrong-generation activation reject
```

- [ ] **Step 7: Run formula-packages full gate and commit**

```bash
cargo test -p formula-packages --all-targets --locked
cargo fmt --all -- --check
cargo clippy -p formula-packages --all-targets --locked -- -D warnings
```

---

### Task 5: Bind CandidateSpace explicitly to `Lambda_g` and reject silent grammar reinterpretation

**Files:**
- Create: `crates/formula-engine/src/self_expansion.rs`
- Modify: `crates/formula-engine/src/lib.rs`
- Test: `crates/formula-engine/tests/p10_grammar_binding.rs`

**Interfaces:**
- Produces: `GrammarBoundCandidateContext`, `bind_candidate_context_to_grammar`, `validate_candidate_context_grammar`.
- Consumes: existing `CandidateSpaceContext::grammar_or_routes_digest()` and `GrammarGeneration`.

- [ ] **Step 1: Write RED grammar-binding tests**

```rust
#[test]
fn candidate_space_context_binds_exact_lambda_digest() {
    let bound = bind_candidate_context_to_grammar(base_context, &lambda_g);
    assert_eq!(bound.context().grammar_or_routes_digest(), lambda_g.structural_digest());
}

#[test]
fn candidate_built_under_lambda_g_cannot_be_silently_reused_under_lambda_g1() {
    let old = bind_candidate_context_to_grammar(base_context.clone(), &lambda_g);
    assert!(validate_candidate_context_grammar(old.context(), &lambda_g).is_ok());
    assert_eq!(
        validate_candidate_context_grammar(old.context(), &lambda_g1),
        Err(GrammarBindingError::GrammarDigestMismatch)
    );
    let rebuilt = bind_candidate_context_to_grammar(base_context, &lambda_g1);
    assert_ne!(old.context().digest(), rebuilt.context().digest());
}
```

- [ ] **Step 2: Run and prove RED**

```bash
cargo test -p formula-engine --test p10_grammar_binding --locked
```

- [ ] **Step 3: Implement a narrow binding adapter**

Do not change `CandidateSpaceContext` schema. Use its existing `with_grammar_or_routes` method so P5/P9 structural behavior remains unchanged. Validation compares exact structural digest.

- [ ] **Step 4: Run engine predecessor tests and commit**

```bash
cargo test -p formula-engine --all-targets --locked
cargo fmt --all -- --check
cargo clippy -p formula-engine --all-targets --locked -- -D warnings
```

---

### Task 6: Semantic-change freshness plus independently gated proof transport/repair records

**Files:**
- Modify: `crates/formula-check/src/self_expansion.rs`
- Test: `crates/formula-check/tests/p10_proof_evolution.rs`

**Interfaces:**
- Produces: `classify_freshness`, opaque `ProofEvolutionAuthorization`, `authorize_transport_v1`, `authorize_repair_v1`, `TransportedEvidenceRecord`, `RepairedEvidenceRecord`.

- [ ] **Step 1: Write RED semantic-change tests**

```rust
#[test]
fn unrelated_dependency_change_keeps_evidence_fresh() {
    let state = classify_freshness(&change, &[digest("dep-unrelated")], None);
    assert_eq!(state, EvidenceFreshness::UnchangedFresh);
}

#[test]
fn non_conservative_change_never_silently_transports() {
    let state = classify_freshness(&non_conservative, &[old], Some(relation));
    assert!(matches!(state, EvidenceFreshness::ReproveRequired | EvidenceFreshness::Quarantined));
}

#[test]
fn transport_or_repair_record_cannot_exist_without_checker_authorization() {
    // Constructors for authoritative result records are private to formula-check path.
    // Mismatched destination/plan/evidence -> reject.
}
```

- [ ] **Step 2: Implement deterministic freshness classification**

Rules:

```text
no dependency-cone intersection -> UNCHANGED_FRESH
REALIZATION_ONLY with unchanged semantic artifact -> UNCHANGED_FRESH
DEFINITIONAL_EQUIVALENT / CONSERVATIVE_EXTENSION + exact certified relation -> TRANSPORTABLE
repair-supporting class + intersecting affected slice -> REPAIRABLE
THEOREM_STRENGTHENING / ASSUMPTION_WEAKENING without certified transport -> RECHECK_REQUIRED
SIGNATURE_CHANGE -> REPROVE_REQUIRED unless exact certified transport route
NON_CONSERVATIVE_CHANGE / AUTHORITY_POLICY_CHANGE -> REPROVE_REQUIRED or QUARANTINED
```

- [ ] **Step 3: Implement opaque transport/repair authorization**

Authorization requires all plan bindings and checker/evidence digests to match exact supplied checked evidence. Result record identity includes the destination target/dependencies and checker authorization digest, guaranteeing a new identity rather than relabeling old evidence.

- [ ] **Step 4: Run formula-check full gate and commit**

---

### Task 7: Realization-only upgrade selection and rollback without semantic generation mutation

**Files:**
- Modify: `crates/formula-store/src/authority_store/realization_store.rs`
- Modify: `crates/formula-store/src/authority_store/expansion_store.rs`
- Test: `crates/formula-store/tests/p10_realization_upgrade.rs`
- Test: `crates/formula-realize/tests/p10_realization_upgrade.rs`

**Interfaces:**
- Produces: `AuthorityStore::record_realization_upgrade`, `preferred_realization`, `select_realization`.
- Consumes: existing independently admitted P8 realization records.

- [ ] **Step 1: Write RED realization-only tests**

```rust
#[test]
fn faster_admitted_realization_can_replace_selection_without_new_universe_generation() {
    let before = store.active_generation().unwrap().unwrap();
    // Admit R1 and independently admitted R2 for the same semantic artifact/context.
    // Record RealizationUpgrade(R1 -> R2, REALIZATION_ONLY).
    // preferred_realization returns R2.
    assert_eq!(store.active_generation().unwrap(), Some(before));
    // R1 remains resolvable/replayable.
}

#[test]
fn realization_upgrade_cannot_smuggle_semantic_admission() {
    // Any upgrade whose semantic artifact is not already admitted -> typed reject.
}

#[test]
fn realization_selection_can_roll_back_to_old_admitted_realization() {
    // select R1 again; U digest unchanged; R2 remains admitted/history visible.
}
```

- [ ] **Step 2: Implement realization-selection ledger**

Persist selection by exact semantic artifact/generation/world/authority/observer context. A selected realization must already be admitted and match the existing P8 dispatch context. Selection changes do not edit realization admission rows.

- [ ] **Step 3: Record `SupersessionKind::ReplacedRealizationBy`**

Every upgrade records append-only lineage from old to new realization. Rollback adds/changes selection, not historical deletion.

- [ ] **Step 4: Run formula-store + formula-realize full gates and commit**

---

### Task 8: Canonical P10 proof manifest, independent verifier, and all negative controls

**Files:**
- Create: `crates/formula-first-light/src/p10.rs`
- Modify: `crates/formula-first-light/src/lib.rs`
- Test: `crates/formula-first-light/tests/p10_self_expansion_hardening.rs`
- Modify: `crates/formula-core/src/self_expansion.rs`
- Modify: `crates/formula-check/src/self_expansion.rs`
- Test: `crates/formula-check/tests/p10_self_expansion_verifier.rs`

**Interfaces:**
- Produces: `SelfExpansionProofManifest`, `SelfExpansionNegativeControlManifest`, `verify_self_expansion_manifest`, canonical thirteen-marker transcript.

- [ ] **Step 1: Write RED manifest/verifier tests**

```rust
#[test]
fn complete_manifest_emits_exact_p10_marker_order() {
    let result = verify_self_expansion_manifest(&manifest, &evidence).unwrap();
    assert_eq!(result.markers(), P10_CANONICAL_MARKERS);
}

#[test]
fn missing_negative_control_or_changed_package_digest_rejects_complete_claim() {
    // Remove NC10-08 or mutate rational package digest; verifier fails before markers.
}
```

- [ ] **Step 2: Implement canonical negative-control identities**

Exactly once each:

```text
NC10-01 WrongBasePromotion
NC10-02 ForbiddenClassEffect
NC10-03 UnadmittedStructureWitness
NC10-04 UnboundStructureEvidence
NC10-05 UnscopedAutomaticNogood
NC10-06 RouteMissingPreservationEvidence
NC10-07 GrammarGenerationMismatch
NC10-08 UngatedAutomaticMetaprimitive
NC10-09 NonConservativeSilentTransport
NC10-10 UnauthorizedProofRepairOrTransport
NC10-11 RealizationUpgradeSemanticAdmission
NC10-12 RollbackHistoryRewrite
```

- [ ] **Step 3: Implement one clean-state P10 canonical harness**

The harness must execute, not merely name:

```text
P9 predecessor recovery identity check
U_g + active Rational package without field witness
closure_before: cap:rational:field absent
real checked StructureWitness promotion
P10 classified expansion authorization
AuthorityStore promotion -> U_(g+1)
closure_after: cap:rational:field present
unchanged Rational package digest proof
nogood scope reject/accept checks
route evidence reject/accept checks
Lambda_g -> Lambda_(g+1) identity proof
shadow metaprimitive present but non-authoritative
ungated automatic metaprimitive rejection
semantic-change freshness/transport/repair gates
realization-only selection upgrade + rollback
historical generation replay after rollback
all NC10-01..12
manifest assembly
independent verifier replay
```

- [ ] **Step 4: Emit exact marker order only from independent verifier**

```rust
pub const P10_CANONICAL_MARKERS: [&str; 13] = [
    "PASS P10_PROMOTION_CLASS_REGISTRY",
    "PASS P10_STRUCTURE_WITNESS_PROMOTION",
    "PASS P10_NON_PRIMITIVE_CAPABILITY_UNLOCK",
    "PASS P10_NOGOOD_SCOPE_ENFORCED",
    "PASS P10_ROUTE_PROMOTION_GATED",
    "PASS P10_GRAMMAR_GENERATION_BOUND",
    "PASS P10_METAPRIMITIVE_SHADOW_GATE",
    "PASS P10_SEMANTIC_CHANGE_REVALIDATION",
    "PASS P10_PROOF_TRANSPORT_REPAIR_GATED",
    "PASS P10_REALIZATION_ONLY_UPGRADE",
    "PASS P10_ROLLBACK_HISTORY_PRESERVED",
    "PASS P10_NEGATIVE_CONTROLS",
    "PASS SELF_EXPANSION_HARDENED",
];
```

- [ ] **Step 5: Run all targeted crates + workspace gate and commit**

```bash
cargo test -p formula-core --all-targets --locked
cargo test -p formula-check --all-targets --locked
cargo test -p formula-store --all-targets --locked
cargo test -p formula-packages --all-targets --locked
cargo test -p formula-engine --all-targets --locked
cargo test -p formula-realize --all-targets --locked
cargo test -p formula-first-light --all-targets --locked
cargo test --workspace --all-targets --locked
cargo build --workspace --all-targets --locked
cargo fmt --all -- --check
CARGO_NET_OFFLINE=true cargo clippy --workspace --all-targets --locked -- -D warnings
```

---

### Task 9: Permanent read-only P10 canonical workflow and source-under-test proof

**Files:**
- Delete: `.github/workflows/p10-development.yml`
- Create: `.github/workflows/p10-canonical-proof.yml`

**Interfaces:**
- Produces: immutable workflow identity and exact-head canonical run evidence.

- [ ] **Step 1: Remove temporary development workflow**

The source-under-test head must not contain `p10-development.yml`.

- [ ] **Step 2: Add permanent canonical workflow**

Requirements:

```text
name: P10 canonical proof
permissions: contents: read
runner: ubuntu-24.04
Rust: 1.98.0
locked dependency priming
proof execution locked/offline where applicable
all targeted P10/predecessor/workspace gates
rustfmt
Clippy -D warnings
dependency/authority firewall checks
clean-worktree check
```

The workflow must print exact source SHA, toolchain identity, P10 manifest digest, U_g/U_(g+1), registry digest, Lambda_g/Lambda_(g+1), unlocked capability, NC manifest digest, and all thirteen markers.

- [ ] **Step 3: Verify workflow blob identity before source proof**

Record the Git blob SHA for `.github/workflows/p10-canonical-proof.yml` and do not change the workflow after source proof begins.

- [ ] **Step 4: Run exact source-head canonical proof**

Bind exact head/run/job. Do not use a run from a neighboring commit.

- [ ] **Step 5: Review P9 frozen head -> P10 source delta**

Confirm no temporary workflow, accidental generated artifact, unrelated refactor, or P9 authority rewrite exists.

---

### Task 10: Recovery checkpoint, docs-head proof, and final freeze

**Files:**
- Create: `docs/checkpoints/2026-09-05-p10-self-expansion-hardening.md`
- Modify: `CURRENT.md`

**Interfaces:**
- Produces: cross-chat P10 recovery authority and non-recursive final freeze metadata.

- [ ] **Step 1: Write source-proof checkpoint**

Record exact P9 predecessor, P10 source head/run/job, workflow blob, identities, marker transcript, proof target, all NC10 controls, source delta, exclusions, and:

```text
P10 source proof:       PROVED
documentation proof:    PENDING
P10 final freeze:       NOT YET CLAIMED
```

- [ ] **Step 2: Update `CURRENT.md` to source-proved/docs-head-pending**

The recovery procedure must point future chats to the P10 checkpoint/design/plan and exact source proof boundary.

- [ ] **Step 3: Prove source -> docs candidate is documentation only**

Allowed changed files after source proof:

```text
CURRENT.md
docs/checkpoints/2026-09-05-p10-self-expansion-hardening.md
```

No source/test/Cargo/workflow file may differ.

- [ ] **Step 4: Run the unchanged canonical workflow on the exact docs-bearing head**

Bind exact head/run/job and verify all source-independent identities/markers remain stable. A manifest field that intentionally binds source commit may change only as designed.

- [ ] **Step 5: Record final freeze as post-proof metadata**

Update only recovery documentation to state:

```text
P10 final freeze:       FINALLY FROZEN
SELF_EXPANSION_HARDENED: FINAL RECOVERY AUTHORITY
next roadmap phase:     P11
```

These later metadata-only commits do not redefine the docs-bearing frozen proof head and do not create recursive proof requirements.

---

## Plan self-review checklist

- Every frozen roadmap P10 build-scope item is assigned to Tasks 1-8.
- The canonical proof is non-primitive and uses unchanged Rational package capability code.
- P7-P9 structural identities remain untouched; P10 wraps them.
- Automatic nogood and metaprimitive behavior fails closed without exact scope/gate evidence.
- Grammar generation is explicit and CandidateSpace remains generation-bound without schema rewrite.
- Proof transport/repair records require checker-owned authorization and receive new identities.
- Realization-only upgrade cannot change semantic generation.
- Rollback never deletes history.
- Development workflow is temporary and removed before canonical proof.
- Canonical workflow is read-only and recursively proved on the docs-bearing head.