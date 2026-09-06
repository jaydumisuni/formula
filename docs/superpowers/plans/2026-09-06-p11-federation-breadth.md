# P11 Federation Breadth Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Prove two heterogeneous specialist federation routes—SAT/LRAT and exact integer arithmetic—can cooperate through checked Shared Facts without trusting producer identity.

**Architecture:** Extend the existing D2 federation, Shared Fact, composition, and certificate-routing contracts. Add independent SAT/LRAT-RUP and arbitrary-precision integer-result checkers, provenance-bound federation facts, directional bridge validation, one canonical P11 integration manifest, and a permanent read-only proof workflow.

**Tech Stack:** Rust 1.98.0, existing workspace crates, `num-bigint`, GitHub Actions on Ubuntu 24.04.

**Spec:** `docs/superpowers/specs/2026-09-06-p11-federation-breadth-design.md`

## Global Constraints

- P10 frozen proof authority must remain unchanged.
- External producer identity never creates mathematical authority.
- Unsupported LRAT proof behavior fails closed.
- Cross-package propagation requires an explicit directional bridge and safe composition class.
- Shared Fact polarity may not be strengthened by a bridge.
- Canonical P11 proof runs offline after locked dependency fetch.
- Permanent canonical workflow uses `contents: read` and Rust `1.98.0`.

---

### Task 1: Structural federation provenance and bridge identities

**Files:**
- Modify: `crates/formula-core/src/theory.rs`
- Test: `crates/formula-core/tests/p11_federation_identity.rs`

**Interfaces:**
- Produces: `CertifiedFederationFact`, `BridgeContract`, structural digest accessors and polarity/package/route bindings.

- [ ] Write failing tests proving field order does not affect canonical list inputs, changing adapter/checker/translation/input/evidence changes federation-fact digest, and reversing a bridge changes bridge identity.
- [ ] Run `cargo test -p formula-core --test p11_federation_identity --locked` and confirm compile/test failure because the new types do not exist.
- [ ] Implement the minimal structural types using existing `CanonicalValue`, `ArtifactDigest`, `FactPolarity`, and `StructuralIdentity` patterns.
- [ ] Re-run the targeted test and then `cargo test -p formula-core --all-targets --locked`.
- [ ] Commit the task.

### Task 2: SAT DIMACS + LRAT-RUP independent checker

**Files:**
- Create: `crates/formula-check/src/sat_lrat.rs`
- Modify: `crates/formula-check/src/lib.rs`
- Test: `crates/formula-check/tests/p11_sat_lrat.rs`

**Interfaces:**
- Produces: `SatCnf`, `LratProof`, `LratCheckError`, `check_lrat_rup_unsat(&SatCnf, &str) -> Result<LratProof, LratCheckError>`.

- [ ] Write failing tests for a minimal valid UNSAT LRAT-RUP proof and NC11-02/03/04: forged hint, no empty clause, and unsupported RAT/deletion behavior.
- [ ] Run the targeted test and verify RED because the module/API is absent.
- [ ] Implement deterministic DIMACS semantic representation plus LRAT-RUP parsing, proof-id uniqueness, referenced-clause lookup, unit propagation, and explicit unsupported-form rejection.
- [ ] Run targeted tests, then all `formula-check` tests.
- [ ] Commit the task.

### Task 3: Exact arbitrary-precision arithmetic checker

**Files:**
- Create: `crates/formula-check/src/exact_arithmetic.rs`
- Modify: `crates/formula-check/src/lib.rs`
- Test: `crates/formula-check/tests/p11_exact_arithmetic.rs`

**Interfaces:**
- Produces: `IntegerOperation::{Add,Sub,Mul}`, `ExactArithmeticReceipt`, `ExactArithmeticError`, `check_decimal_integer_result`.

- [ ] Write failing tests using values larger than `u128`, plus malformed decimal and incorrect-result negative cases.
- [ ] Run the targeted test and verify RED because the module/API is absent.
- [ ] Implement strict signed-decimal parsing with `num_bigint::BigInt`, independently recompute the operation, and return a structural receipt only on exact equality.
- [ ] Run targeted and full `formula-check` tests.
- [ ] Commit the task.

### Task 4: Federation fact admission and bridge enforcement

**Files:**
- Create: `crates/formula-packages/src/cooperation.rs`
- Modify: `crates/formula-packages/src/lib.rs`
- Test: `crates/formula-packages/tests/p11_cooperation.rs`

**Interfaces:**
- Consumes: existing `FederationAdapterManifest`, `FederationRequest`, `validate_federation_adapter`, `CompositionClaim`, `SharedFact`, `fact_satisfies`.
- Produces: `certify_federation_fact`, `apply_bridge`, `CooperationError`.

- [ ] Write failing tests for valid checked admission and NC11-01/05/08/09/10/11/12/13/14.
- [ ] Run targeted test and verify RED because cooperation APIs do not exist.
- [ ] Implement exact package/adapter/translation/checker/input/evidence binding, safe composition-class check, directional bridge match, and non-strengthening polarity check.
- [ ] Run targeted and full `formula-packages` tests.
- [ ] Commit the task.

### Task 5: Canonical heterogeneous cooperation proof

**Files:**
- Create: `crates/formula-first-light/tests/p11_federation_breadth.rs`
- Create: `crates/formula-check/src/federation_verifier.rs`
- Modify: `crates/formula-check/src/lib.rs`
- Create: `crates/formula-check/tests/p11_federation_verifier.rs`

**Interfaces:**
- Produces: `FederationBreadthProofManifest`, `verify_federation_breadth_manifest`, ordered P11 markers.

- [ ] Write a failing integration proof that constructs a branch-choice CNF, verifies its LRAT-RUP evidence, creates a checked SAT fact, bridges it into the arithmetic branch domain, independently verifies a large-integer arithmetic result, and requires both facts for the final target.
- [ ] Add executed NC11-01...NC11-14 paths and construct the negative-control manifest only after each expected rejection occurs.
- [ ] Add a failing independent-verifier test that rejects a mutated manifest and expects the exact ordered markers from a valid manifest.
- [ ] Implement the minimal proof manifest and independent verifier.
- [ ] Run the P11 integration proof, independent verifier test, all affected crate tests, and full workspace tests.
- [ ] Commit the task.

### Task 6: Permanent P11 canonical workflow and architecture firewalls

**Files:**
- Create: `.github/workflows/p11-canonical-proof.yml`

**Interfaces:**
- Produces: read-only canonical proof for `implementation/p11-federation-breadth`.

- [ ] Add a permanent workflow pinned to Ubuntu 24.04/Rust 1.98.0 with `contents: read`.
- [ ] Gate exact source identity, locked/offline metadata, P11 canonical integration, P11 independent verifier, frozen P10 predecessor proof, architecture tests, every crate/workspace test, build, rustfmt, Clippy `-D warnings`, dependency trees, authority/source firewalls, and clean worktree.
- [ ] Push and inspect the exact workflow run to confirm any failure is attributable to source rather than runner setup.
- [ ] Fix source only through normal TDD commits; do not use a write-capable proof workflow.
- [ ] Require the final exact source candidate to pass every gate.

### Task 7: Freeze and recovery authority

**Files:**
- Create: `docs/checkpoints/2026-09-06-p11-federation-breadth.md`
- Modify: `CURRENT.md`

**Interfaces:**
- Produces: exact P11 source SHA/run/job identities, proof-manifest digest, package/adapter/bridge identities, ordered markers, and next roadmap boundary.

- [ ] Record the successful source-under-test P11 proof without moving the proof boundary.
- [ ] Create the documentation-bearing candidate changing only the checkpoint and `CURRENT.md` while preserving the canonical P11 workflow blob.
- [ ] Prove that exact documentation-bearing candidate with the unchanged workflow.
- [ ] Record docs-head SHA/run/job as post-proof recovery metadata without recursive proof movement.
- [ ] Re-run the branch-head canonical workflow for final green-state assurance.
- [ ] Only then mark `FEDERATION_BREADTH_PROVED` final recovery authority and move to the next actionable roadmap phase.
