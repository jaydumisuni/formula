# P4 Checkpoint — Query Compiler + Campaign Core

**Date:** 2026-09-02  
**Status:** PROVED SOURCE — deterministic QueryIR/compiler/campaign core with authority-inert work planning  
**Branch:** `implementation/p4-query-compiler-campaign-core`  
**P3 exact predecessor branch head:** `5c15368440ad9cc387708dae3c3d73135009f053`  
**Source-under-test commit:** `2eb1ddef2530d3a46190b4bb62dc7f98ed85dc93`  
**Canonical source proof run:** `33684127872`  
**Canonical source proof job:** `100427383372`  
**Roadmap authority:** `docs/roadmap/2026-08-28-implementation-roadmap.md`, phase P4  
**Design spec:** `docs/superpowers/specs/2026-09-02-p4-query-compiler-campaign-core-design.md`  
**Implementation plan:** `docs/superpowers/plans/2026-09-02-p4-query-compiler-campaign-core.md`

---

## 1. Scope

This checkpoint records canonical roadmap phase **P4 — Query, Theory Profile, Campaign IR, Obligation compiler** on the proved source-under-test commit above.

P4 adds the deterministic D3 compiler/campaign front end between the proved P3 package/capability substrate and the later P5 CandidateSpace/discovery layer.

Frozen P4 surfaces include:

- exact, content-addressed `QueryIR` semantic input identity;
- immutable `CompilerAuthoritySnapshot` input and deterministic `RelevantRegion`;
- `TheoryProfile` separation between certified facts and operational estimates;
- explicit representation preservation/information-loss/reconstruction contracts;
- path-wide reduction result-class preservation;
- explicit decomposition aggregation and reconstruction semantics;
- deterministic typed AND/OR `CampaignIR`;
- `ObligationIR` with distinct mathematical/resource terminal states;
- authority-inert `WorkCellPlan`;
- deterministic, complete `ReplayManifest` binding;
- structural `ResultBundle` with no certification authority;
- deterministic `CompilerV1` end-to-end campaign construction;
- adversarial rejection of semantic/authority boundary violations.

P4 does **not** implement CandidateSpace enumeration/refinement, CEGIS, solver search, theorem discovery, promotion, native realization generation, model authority, external SAT/SMT/CAS execution, or First-Light execution. Those remain later roadmap phases.

---

## 2. Exact predecessor and review boundary

P4 was reviewed against the exact final P3 documentation-bearing branch head:

```text
5c15368440ad9cc387708dae3c3d73135009f053
```

The proved P4 source boundary is:

```text
2eb1ddef2530d3a46190b4bb62dc7f98ed85dc93
```

Exact compare evidence reports:

```text
base:    5c15368440ad9cc387708dae3c3d73135009f053
head:    2eb1ddef2530d3a46190b4bb62dc7f98ed85dc93
status:  ahead
ahead:   45 commits
behind:  0 commits
```

The reviewed range is confined to intended P4 surfaces:

```text
.github/workflows/p4-canonical-proof.yml
crates/formula-engine/src/{query,region,theory_profile,representation,reduction,decomposition,campaign,obligation,work_cell,replay,result_bundle,compiler}.rs
crates/formula-engine/src/lib.rs
crates/formula-engine/tests/p4_*.rs
tests/authority-boundary/tests/p4_authority_inert.rs
docs/superpowers/specs/2026-09-02-p4-query-compiler-campaign-core-design.md
docs/superpowers/plans/2026-09-02-p4-query-compiler-campaign-core.md
```

No P1 authority-store production implementation, P2 checker implementation, P3 package/closure implementation, realization implementation, First-Light implementation, or `main` branch was modified by the P4 implementation range.

---

## 3. Canonical proof environment

The canonical source proof ran from exact commit `2eb1ddef2530d3a46190b4bb62dc7f98ed85dc93` on GitHub-hosted Ubuntu 24.04 using pinned Rust 1.98.0.

```text
workflow: P4 canonical proof
run:      33684127872
job:      100427383372
result:   success
runner:   ubuntu-24.04
```

The canonical workflow uses `permissions: contents: read` and runs the locked/offline dependency graph after cache priming.

The proof sequence includes:

```bash
git rev-parse HEAD
rustc -vV
cargo -V
cargo metadata --locked --offline --format-version 1
cargo test -p formula-archtest --locked --offline
cargo test -p formula-core --all-targets --locked --offline
cargo test -p formula-store --all-targets --locked --offline
cargo test -p formula-check --all-targets --locked --offline
cargo test -p formula-packages --all-targets --locked --offline
cargo test -p formula-engine --all-targets --locked --offline
cargo test --workspace --all-targets --locked --offline
cargo build --workspace --all-targets --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo tree --locked --offline -p formula-check --edges normal
cargo tree --locked --offline -p formula-packages --edges normal
cargo tree --locked --offline -p formula-engine --edges normal
test -z "$(git status --porcelain)"
```

Every named command completed successfully on the exact source-under-test commit.

---

## 4. Canonical-proof correction history

The first monolithic canonical P4 run correctly failed closed. Systematic debugging split the **same canonical commands in the same order** into named read-only workflow steps, proving that the first canonical-only failure was:

```text
cargo fmt --all -- --check
```

All semantic tests and workspace build gates preceding formatting were already green.

A one-shot Rust 1.98.0 formatter helper then:

1. ran `cargo fmt --all` on the exact branch;
2. enforced a strict scope guard allowing only `crates/**/*.rs` formatter changes;
3. committed the formatter output;
4. deleted itself in the same commit.

The formatter commit changed only Rust layout plus helper deletion. A human-authored comment-only workflow child then triggered exact-head canonical proof. No semantic, authority, dependency, or contract weakening was used to satisfy CI.

---

## 5. Query semantic identity

`QueryIR` structurally binds:

```text
Universe generation
World
known bindings
metavariables
target/result class
Observer
Authority Contract
resource contract
side-effect policy
activated package context
```

Set-like fields are sorted/deduplicated before canonical identity. Changing a semantic input changes QueryIR identity.

Resource limits are separate from mathematical authority and cannot rewrite the requested Authority Contract.

---

## 6. Immutable compiler authority snapshot

Compilation consumes an immutable `CompilerAuthoritySnapshot` rather than a live authority mutation handle.

`RelevantRegion::from_snapshot` fails closed on:

```text
generation mismatch
World mismatch
activated-package context mismatch
```

The region contains only the snapshot's admitted artifacts/capabilities/morphisms projected into deterministic compiler state.

The snapshot is planning input; constructing a snapshot does not publish or create authority.

---

## 7. Theory profile separation

`TheoryProfile` keeps exact/certified profile facts distinct from operational estimates.

Operational estimates may support later scheduling/ranking but do not satisfy exact semantic facts or discharge mathematical obligations.

This preserves the P3 Shared Fact strength discipline at the compiler boundary.

---

## 8. Representation, reduction, and decomposition contracts

Representation changes are explicit edges with semantic source/target bindings, World/Observer compatibility, preservation metadata, information-loss declaration, assumptions, evidence/certificate route identity, and reconstruction semantics where required.

P4 fails closed when required preservation metadata is absent or the requested result class is not preserved.

Lossy witness routes require explicit reconstruction.

Reduction validity is path-wide. The requested result class must survive every edge in the composed path; witness-preserving reductions require reconstruction semantics.

Decomposition is explicit planning data. Child obligations, aggregation semantics, reconstruction relation, and evidence are structural inputs. Missing reconstruction/aggregation semantics fail closed.

Approximate/heuristic representation classes may exist as planning metadata, but they do not rewrite the original obligation's required Authority Contract and are not themselves certification authority.

---

## 9. CampaignIR and ObligationIR

P4 produces deterministic typed campaign graphs with explicit node/edge families and AND/OR aggregation semantics.

Campaign insertion order is non-semantic; normalized graph structure, node identities, edge identities, generation, and World remain semantic.

`ObligationIR` freezes distinct terminal states including:

```text
SATISFIED
REFUTED
CERTIFIED_BOUND
SEMANTIC_UNKNOWN
RESOURCE_BOUNDED_UNKNOWN
UNDECIDABLE_GENERAL_CLASS
SUPERSEDED
BLOCKED_BY_AUTHORITY
```

Critical invariant:

```text
REFUTED != SEMANTIC_UNKNOWN != RESOURCE_BOUNDED_UNKNOWN
```

A resource ceiling or timeout cannot become mathematical refutation.

---

## 10. WorkCell authority boundary

`WorkCellPlan` is immutable planning data containing obligation identity, semantic inputs, allowed packages/capabilities, evidence requirement, required authority, resource budget, deterministic replay key, checkpoint policy, side-effect limits, and stop conditions.

Its public side-effect policies expose no authority-write-capable constructor. `deny_all` and `local_process_only` both preserve `allow_authority_write = false`.

The dedicated authority-boundary architecture test scans `formula-engine/src` for forbidden authority publication/mutation paths and checker implementation coupling.

The reviewed direct dependency graph remains:

```text
formula-engine
├── formula-core
├── formula-store
└── formula-packages

formula-engine -/-> formula-check implementation
```

P4 therefore consumes authority requirements but cannot certify, promote, publish, roll back, or mutate mathematical authority.

---

## 11. Deterministic replay and result envelopes

`ReplayManifest` binds the exact semantic/policy inputs required to reconstruct one campaign identity, including generation, World, QueryIR, activated package context, relevant region, theory profile, compiler/scheduler policy versions, resource contract, deterministic key, and campaign digest.

Identical exact inputs produce identical replay/campaign identity. Changing a semantic/policy binding changes replay identity where required.

`ResultBundle` is a structural envelope only. Referencing evidence/certificates/promotion candidates does not certify or promote them.

---

## 12. CompilerV1

`CompilerV1` performs deterministic structural compilation only:

```text
CompilerAuthoritySnapshot + QueryIR
    -> exact context validation
    -> RelevantRegion
    -> TheoryProfile
    -> validate representation/reduction/decomposition routes
    -> CampaignIR
    -> ObligationIR
    -> authority-inert WorkCellPlan
    -> ReplayManifest
```

It rejects generation, World, package, Observer, Authority, ambiguous-parent, implicit-loss, representation, reduction, decomposition, and campaign violations fail-closed.

P4 contains no CandidateSpace enumeration or adaptive discovery loop; that remains P5.

---

## 13. Adversarial gate

The P4 adversarial suite proves at least:

```text
resource exhaustion -/-> REFUTED
operational estimate -/-> exact profile fact
lossy implicit morphism -> rejection
decision-only reduction -/-> witness
missing decomposition reconstruction -> rejection
WorkCell authority write -> unavailable/rejected
replay canonical form binds frozen policy fields
```

The end-to-end compiler tests also cover semantic context mismatch and deterministic replay/campaign identity.

---

## 14. P4 proof markers

The successful canonical workflow emitted:

```text
P4-01 QueryIR exact semantics preserved                         PASS
P4-02 no lossy implicit morphism                              PASS
P4-03 representation preservation metadata enforced          PASS
P4-04 reduction result classes preserved                      PASS
P4-05 decomposition reconstruction explicit                   PASS
P4-06 CampaignIR deterministic AND/OR                         PASS
P4-07 terminal states remain distinct                         PASS
P4-08 WorkCells authority-inert                               PASS
P4-09 replay manifest complete/deterministic                  PASS
P4-10 P0-P3 gates preserved                                   PASS
```

---

## 15. P0–P3 preservation

The canonical P4 workflow reruns the predecessor architecture, core, store, checker, package, and workspace tests under the locked/offline proof environment.

P4 extends rather than replaces the proved substrate:

```text
P0 reproducible repository/build and architecture firewall
P1 deterministic identity + immutable generation authority store
P2 independent checker/certificate authority
P3 theory packages + generation/world capability closure
```

No compiler/campaign object becomes mathematical authority merely because it is structurally valid or replayable.

---

## 16. Milestone boundary

**P4 source is proved and review-clean on the isolated implementation branch for source-under-test commit `2eb1ddef2530d3a46190b4bb62dc7f98ed85dc93`.**

This checkpoint intentionally records the source proof rather than claiming its own documentation commit has been tested.

The branch has **not** been merged to `main`.

A post-checkpoint canonical proof must remain green with this checkpoint and the corresponding `CURRENT.md` update present before the documentation-bearing branch head is treated as the final frozen P4 branch candidate.

---

## 17. Next phase

The frozen roadmap names the next boundary:

```text
P5 — CandidateSpace + bounded discovery
```

P5 must consume P4 campaign/obligation/replay contracts without allowing search state or candidate-generation machinery to create or weaken mathematical authority.
