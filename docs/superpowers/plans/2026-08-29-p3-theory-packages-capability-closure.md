# P3 Theory Packages + Capability Closure Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement P3 theory packages, structure inference, generation/world-scoped capability closure, federation contracts, shared-fact polarity, and Certificate Router v1 while preserving all proved P0/P1/P2 authority boundaries.

**Architecture:** Durable semantic vocabulary stays in `formula-core`; deterministic package manifests, activation/composition validation, structure inference, common-parent/canonical-morphism resolution, capability closure, federation contracts, and shared-fact handling live in `formula-packages`; exact authority-route selection extends `formula-check`. Capability closure remains rebuildable derived state keyed by generation/world/activated packages/rules/policy, never durable authority.

**Tech Stack:** Rust 1.98.0, existing workspace crates, existing canonical encoding/SHA-256 identity, existing P2 independent checker, GitHub Actions branch proof runner.

**Spec:** `docs/design/2026-08-28-d2-core-system-architecture.md` and `docs/roadmap/2026-08-28-implementation-roadmap.md` P3.

## Global Constraints

- Preserve all P0/P1/P2 proof obligations and crate dependency firewalls.
- No external SAT/SMT/CAS binary, model, GPU, Ptah, or P4 compiler/search implementation.
- Package identity is semantic/content-addressed; no machine-local metadata in digests.
- Capability closure is generation/world scoped and rebuildable.
- Package activation fails closed on unsupported/interfering composition.
- Shared facts preserve polarity; weaker facts cannot discharge stronger goals.
- Certificate Router v1 may only select routes satisfying the exact requested Authority Contract.
- No package, federation adapter, or search component can manufacture authority.

---

### Task 1: Durable P3 semantic contracts

**Files:**
- Create: `crates/formula-core/src/theory.rs`
- Modify: `crates/formula-core/src/lib.rs`
- Test: `crates/formula-core/tests/p3_theory_contracts.rs`

**Interfaces:**
- Produces: `TheoryPackageManifest`, `CapabilityContract`, `StructureGoal`, `StructureWitness`, `CanonicalMorphism`, `CompositionClaim`, `CompositionClass`, `FederationAdapterManifest`, `SharedFact`, `FactPolarity`, `ClosureContext`.

- [ ] Write RED tests proving deterministic/set-normalized structural identity, generation/world-sensitive closure identity, and polarity identity.
- [ ] Run the P3 branch test gate; expect compile failure because `formula_core::theory` is absent.
- [ ] Implement only immutable semantic structs, constructors/accessors, canonical encodings, and `StructuralIdentity`.
- [ ] Run core + architecture tests; expect PASS.
- [ ] Commit `feat(p3): add durable theory package contracts`.

### Task 2: Minimum built-in theory packages

**Files:**
- Create: `crates/formula-packages/src/builtin.rs`
- Modify: `crates/formula-packages/src/lib.rs`
- Test: `crates/formula-packages/tests/p3_builtin_packages.rs`

**Interfaces:**
- Consumes: P3 core theory contracts.
- Produces: deterministic manifests for `Integer`, `Rational`, `Boolean`, `U8`, `GF2`, `Polynomial(Integer,n)`, `GF2Vector`, `GF2Matrix`.

- [ ] Write RED tests fixing package names, foundation bindings, dependencies, semantic exports, implication rules, and capability IDs.
- [ ] Run tests; expect failure because built-in manifests do not exist.
- [ ] Implement the minimum deterministic manifests; no solver behavior.
- [ ] Run package tests; expect PASS.
- [ ] Commit `feat(p3): add minimum theory package manifests`.

### Task 3: Structure witness inference and deterministic capability closure

**Files:**
- Create: `crates/formula-packages/src/closure.rs`
- Modify: `crates/formula-packages/src/lib.rs`
- Test: `crates/formula-packages/tests/p3_capability_closure.rs`

**Interfaces:**
- Produces: `CapabilityClosureInput`, `CapabilityClosure`, `resolve_goal`, `derive_capabilities`.

- [ ] Write RED tests: adding a certified structure witness unlocks a capability; same witness in another World does not; same inputs in another generation use a distinct closure context; repeated input is deterministic.
- [ ] Run tests; expect failure because closure engine is absent.
- [ ] Implement pure monotone implication closure over admitted witnesses and active package rules.
- [ ] Run tests; expect PASS.
- [ ] Commit `feat(p3): derive generation scoped capability closure`.

### Task 4: Package activation and interference contracts

**Files:**
- Create: `crates/formula-packages/src/activation.rs`
- Modify: `crates/formula-packages/src/lib.rs`
- Test: `crates/formula-packages/tests/p3_activation.rs`

**Interfaces:**
- Produces: `ActivationRequest`, `ActivatedPackageSet`, `ActivationError`, `validate_activation`.

- [ ] Write RED tests: deactivation removes capability after recomputation; two packages sharing interference surface without an admissible composition claim fail; `DISJOINT_SAFE`, `CERTIFIED_COMBINATION`, and `CONSERVATIVE_EXTENSION` can pass when exact bindings match; `HEURISTIC_ONLY`, `UNSUPPORTED`, `QUARANTINED` fail for exact activation.
- [ ] Run tests; expect missing activation module.
- [ ] Implement fail-closed activation validation.
- [ ] Run tests; expect PASS.
- [ ] Commit `feat(p3): enforce theory package interference contracts`.

### Task 5: Canonical morphism and common-parent subset

**Files:**
- Create: `crates/formula-packages/src/morphisms.rs`
- Modify: `crates/formula-packages/src/lib.rs`
- Test: `crates/formula-packages/tests/p3_morphisms.rs`

**Interfaces:**
- Produces: `MorphismRegistry`, `CommonParentResolution`, `resolve_common_parent`.

- [ ] Write RED tests: unique certified canonical path resolves; multiple non-equivalent paths return `AMBIGUOUS`; missing path returns `UNKNOWN`; no lossy/noncanonical map is silently selected.
- [ ] Run tests; expect missing morphism module.
- [ ] Implement bounded deterministic resolution over admitted canonical morphisms only.
- [ ] Run tests; expect PASS.
- [ ] Commit `feat(p3): resolve canonical morphisms and common parents`.

### Task 6: Shared Fact polarity enforcement

**Files:**
- Create: `crates/formula-packages/src/shared_facts.rs`
- Modify: `crates/formula-packages/src/lib.rs`
- Test: `crates/formula-packages/tests/p3_shared_fact_polarity.rs`

**Interfaces:**
- Produces: `FactRequirement`, `FactUseDecision`, `fact_satisfies`.

- [ ] Write RED tests proving `OVER_APPROXIMATION` cannot satisfy `EXACT` or existence-witness requirements; lower/upper bounds only satisfy matching bound consumers; exact facts may satisfy weaker compatible requirements.
- [ ] Run tests; expect missing polarity enforcement module.
- [ ] Implement explicit polarity lattice/compatibility table; no heuristic coercion.
- [ ] Run tests; expect PASS.
- [ ] Commit `feat(p3): enforce shared mathematical fact polarity`.

### Task 7: FederationAdapter contract validation

**Files:**
- Create: `crates/formula-packages/src/federation.rs`
- Modify: `crates/formula-packages/src/lib.rs`
- Test: `crates/formula-packages/tests/p3_federation.rs`

**Interfaces:**
- Produces: `FederationMode`, `FederationRequest`, `FederationValidation`, `validate_federation_adapter`.

- [ ] Write RED tests: `CANDIDATE_ONLY` cannot yield authority; checked/certified modes require exact checker route and translation bindings; undeclared side effects or unsupported result classes fail closed.
- [ ] Run tests; expect missing federation module.
- [ ] Implement manifest validation only—no external binary invocation.
- [ ] Run tests; expect PASS.
- [ ] Commit `feat(p3): validate federation adapter contracts`.

### Task 8: Certificate Router v1

**Files:**
- Create: `crates/formula-check/src/router.rs`
- Modify: `crates/formula-check/src/lib.rs`
- Test: `crates/formula-check/tests/p3_certificate_router.rs`

**Interfaces:**
- Produces: `CertificateRoute`, `RouteCandidate`, `RouteError`, `select_certificate_route`.

- [ ] Write RED tests: exact contract rejects probabilistic/empirical/heuristic route; exact route with correct checker/trust-root succeeds; unavailable exact route returns fail-closed error; cheaper weak route never beats stronger requirement.
- [ ] Run tests; expect missing router module.
- [ ] Implement deterministic route filtering then cost ordering only within admissible authority class.
- [ ] Run checker + architecture tests; expect PASS.
- [ ] Commit `feat(p3): add fail closed certificate router v1`.

### Task 9: P3 adversarial integration gate

**Files:**
- Create: `tests/authority-boundary/p3_adversarial.rs`
- Modify: `tests/authority-boundary/Cargo.toml` only if required by the existing test harness.

**Interfaces:**
- Consumes all P3 interfaces.

- [ ] Write integration tests covering cross-world witness leakage, cross-generation closure leakage, unsupported package union, ambiguous common parent, over-approximation-as-exact-witness, federation self-authority, and certificate-route downgrade.
- [ ] Run the branch gate; any attack reaching PASS is RED.
- [ ] Fix only the responsible P3 boundary if any adversarial case succeeds.
- [ ] Re-run until all attacks are rejected and P0/P1/P2 gates remain green.
- [ ] Commit `test(p3): prove package and capability authority boundaries`.

### Task 10: Canonical P3 proof workflow and freeze

**Files:**
- Create/replace: `.github/workflows/p3-canonical-proof.yml`
- Delete: any temporary P3 development workflow.
- Create after source proof: `docs/checkpoints/2026-08-29-p3-theory-packages-capability-closure.md`
- Modify after source proof: `CURRENT.md`

**Proof sequence:**

```bash
rustc -vV
cargo -V
cargo metadata --locked --offline --format-version 1
cargo test -p formula-architecture-tests --locked --offline
cargo test -p formula-core --all-targets --locked --offline
cargo test -p formula-store --all-targets --locked --offline
cargo test -p formula-check --all-targets --locked --offline
cargo test -p formula-packages --all-targets --locked --offline
cargo test --workspace --all-targets --locked --offline
cargo build --workspace --all-targets --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo tree -p formula-check --edges normal
cargo tree -p formula-packages --edges normal
```

- [ ] Commit canonical read-only proof workflow.
- [ ] Run it on the exact source SHA; require all P0/P1/P2 regression gates plus P3 proof markers.
- [ ] Review exact P2→P3 diff; no blocking authority finding allowed.
- [ ] Write checkpoint + `CURRENT.md` against the proved source SHA.
- [ ] Run the unchanged canonical proof on the exact documentation-bearing SHA.
- [ ] Freeze P3 only if the post-checkpoint run is fully green.
