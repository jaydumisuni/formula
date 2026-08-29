# P3 Checkpoint — Theory Packages + Capability Closure

**Date:** 2026-08-29  
**Status:** PROVED SOURCE — theory packages, generation/world-scoped capability closure, federation contracts, shared-fact polarity, and exact certificate routing  
**Branch:** `implementation/p3-theory-packages-capability-closure`  
**P2 exact predecessor branch head:** `05d2c433f89c02ebe5187151284d1442c65bfe8e`  
**Source-under-test commit:** `296902f4dac1d3e0f2b4e6e2e64cfb3c7306c4f7`  
**Canonical source proof run:** `33263907506`  
**Canonical source proof job:** `99130422586`  
**Roadmap authority:** `docs/roadmap/2026-08-28-implementation-roadmap.md`, phase P3  
**Implementation plan:** `docs/superpowers/plans/2026-08-29-p3-theory-packages-capability-closure.md`

---

## 1. Scope

This checkpoint records canonical roadmap phase **P3 — Theory packages, structure inference, capability closure, federation contracts** on the proved source-under-test commit above.

P3 adds the D2 semantic package/capability machinery needed before the P4 compiler/campaign layer can exist. It preserves the P0 repository/architecture boundary, the P1 immutable authority-store substrate, and the P2 independent-checker/certificate authority boundary.

Frozen P3 surfaces include:

- immutable, content-addressed theory/package semantic contracts;
- deterministic minimum built-in theory package manifests;
- structure goals and independently admitted structure witnesses;
- generation/world-scoped capability-closure identity;
- deterministic capability derivation from active packages plus admitted witnesses;
- package activation/deactivation and fail-closed interference/composition contracts;
- generation-admitted canonical morphisms and bounded common-parent resolution;
- explicit Shared Fact polarity compatibility;
- FederationAdapter manifest validation with no producer self-authority;
- Certificate Router v1 preserving the exact requested Authority Contract;
- adversarial tests for cross-generation/world leakage, unsupported composition, ambiguous morphisms, weak-fact escalation, federation self-authority, and route downgrade.

P3 does **not** implement P4 QueryIR/TheoryProfile/Campaign/Obligation compilation, P5 discovery/CandidateSpace, P7 promotion orchestration, P8 native realization generation, First Light, external SAT/SMT/CAS execution, model authority, GPU execution, or Ptah/distributed execution.

---

## 2. Exact predecessor and review boundary

P3 was reviewed against the exact final P2 branch head:

```text
05d2c433f89c02ebe5187151284d1442c65bfe8e
```

The proved P3 source boundary is:

```text
296902f4dac1d3e0f2b4e6e2e64cfb3c7306c4f7
```

Exact repository compare evidence reports:

```text
base:    05d2c433f89c02ebe5187151284d1442c65bfe8e
head:    296902f4dac1d3e0f2b4e6e2e64cfb3c7306c4f7
status:  ahead
ahead:   46 commits
behind:  0 commits
```

The compare is confined to the intended P3 surfaces:

```text
.github/workflows/p3-canonical-proof.yml
crates/formula-core/src/theory.rs
crates/formula-core/tests/p3_theory_contracts.rs
crates/formula-packages/src/{activation,builtin,closure,federation,morphisms,shared_facts}.rs
crates/formula-packages/tests/p3_*.rs
crates/formula-check/src/router.rs
crates/formula-check/tests/p3_*.rs
docs/superpowers/plans/2026-08-29-p3-theory-packages-capability-closure.md
minimal module-export changes in formula-core/formula-packages/formula-check
```

No P1 store production implementation, realization implementation, First-Light implementation, or pre-existing P2 checker family was modified in the reviewed range.

---

## 3. Canonical proof environment

The canonical source proof ran from exact commit `296902f4dac1d3e0f2b4e6e2e64cfb3c7306c4f7` on GitHub-hosted Ubuntu 24.04 using the pinned Rust 1.98.0 toolchain.

```text
workflow: P3 canonical proof
run:      33263907506
job:      99130422586
result:   success
runner:   ubuntu-24.04
```

Pinned toolchain:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
commit-hash: 88d9e12ae178fab0fb5cc050a94da85685d449ea
host: x86_64-unknown-linux-gnu
release: 1.98.0

cargo 1.98.0 (797e8a9bc 2026-08-05)
```

The workflow token has `contents: read`. Network access is used only for checkout/toolchain provisioning and the locked dependency-cache fetch. Canonical Cargo metadata, tests, build, Clippy, and dependency-tree checks execute with the locked dependency graph and offline mode where applicable.

---

## 4. Durable P3 semantic contracts

`formula-core::theory` introduces immutable semantic schemas whose structural identity uses the existing canonical encoding and digest substrate.

The frozen vocabulary includes:

```text
TheoryPackageManifest
CapabilityContract
StructureGoal
StructureWitness
CanonicalMorphism
CompositionClaim
CompositionClass
FederationAdapterManifest
SharedFact
FactPolarity
ClosureContext
```

Set-like fields are sorted and deduplicated before canonical structural projection. Package identity therefore depends on semantic package ID, foundation, exports, capability contracts, dependencies, and interference surface rather than insertion order or machine-local metadata.

`ClosureContext` structurally binds capability derivation to the exact Universe generation, World, active package set, rules identity, and policy identity. A closure result from one generation/world is not interchangeable with another.

---

## 5. Minimum deterministic theory packages

P3 provides the bounded built-in package manifests required by the frozen First-Light roadmap:

```text
Integer
Rational
Boolean
U8
GF2
Polynomial(Integer,n)
GF2Vector
GF2Matrix
```

These are deterministic semantic manifests, not solver implementations. Their foundation/dependency/export/capability bindings are structural inputs. The package layer does not gain authority by existing or by being selected.

---

## 6. Generation-bound activation authority

A package can become active only when the exact package digest is already admitted by the supplied `UniverseGeneration`.

For overlapping/interfering packages, an admissible exact composition claim is also required. The claim itself must be admitted by the generation, and its evidence digest must be present in the generation's authority set.

The regression gate proves all three rejection boundaries:

```text
package digest not admitted
    -> PackageNotAdmitted

composition claim not admitted
    -> CompositionClaimNotAdmitted

composition evidence not authority-bound
    -> CompositionEvidenceNotAuthorityBound
```

Only after those bindings hold does activation produce an `ActivatedPackageSet` bound to the exact generation digest.

Exact activation accepts only supported exact composition classes with matching bindings. Heuristic, unsupported, or quarantined composition cannot be silently upgraded into exact activation.

---

## 7. Structure witnesses and capability closure

P3 capability closure is **derived state**, not durable mathematical authority.

A structure witness must be admitted against the exact Universe generation before it participates in capability derivation. Witness authority binds both the semantic witness identity and its evidence authority membership.

Capability derivation then requires a matching tuple:

```text
Universe generation
World
ActivatedPackageSet for that generation
AdmittedStructureWitness set
TheoryPackage manifests
Rules identity
Policy identity
```

The canonical tests prove:

- a certified/admitted witness unlocks only matching capabilities;
- the same witness does not leak into another World;
- an activated package set from another generation is rejected;
- an admitted witness from another generation does not transfer authority;
- deactivation removes capability after deterministic recomputation;
- repeated identical inputs produce deterministic closure identity and output.

Resource state or producer-local claims do not appear as authority inputs to closure.

---

## 8. Package interference and composition fail closed

Package interference surfaces are explicit. If active packages overlap on a declared interference surface, activation requires an exact admissible composition claim for the relevant pair.

Supported exact classes are bounded and explicit. Weak/non-authoritative classes do not satisfy exact activation.

The adversarial P3 gate proves that unsupported package unions do not activate and cannot create capabilities through mere coexistence.

---

## 9. Canonical morphisms and common-parent resolution

`MorphismRegistry` accepts only morphisms whose structural digests are admitted by the exact supplied generation.

Common-parent resolution considers only bounded admitted morphisms that are both canonical and lossless. It does not silently select lossy or noncanonical coercions.

Frozen result behavior:

```text
one admissible canonical common parent -> resolved
no admissible path                  -> UNKNOWN
multiple non-equivalent parents     -> AMBIGUOUS
```

Ambiguity therefore remains explicit instead of becoming an implicit representation choice.

---

## 10. Shared Mathematical Fact polarity

P3 makes fact polarity a structural part of Shared Fact identity and applies an explicit compatibility relation when a consumer asks to use a fact.

Proved non-escalation rules include:

```text
OVER_APPROXIMATION -/-> EXACT
OVER_APPROXIMATION -/-> existence witness
LOWER_BOUND        -/-> upper-bound consumer
UPPER_BOUND        -/-> lower-bound consumer
heuristic fact     -/-> authoritative exact requirement
```

Exact facts may satisfy weaker semantically compatible consumers, but weaker facts cannot discharge stronger proof obligations.

---

## 11. Federation contracts cannot manufacture authority

P3 introduces FederationAdapter manifest/request validation only. It does not invoke external binaries.

A federation manifest structurally declares semantic inputs/outputs, translations, checker routes, side effects, result classes, and determinism properties.

Frozen rules include:

- `CANDIDATE_ONLY` output is non-authoritative;
- checked/certified modes require the exact declared checker route and translation binding;
- undeclared side effects fail closed;
- unsupported result classes fail closed;
- adapter/producer identity alone can never create authority.

This preserves the constitutional rule that external specialist systems may propose or produce evidence, but only the independent authority path can make that evidence authoritative.

---

## 12. Certificate Router v1 preserves the Authority Contract

The P3 Certificate Router extends `formula-check` with deterministic route selection.

Selection is two-stage:

1. reject every route that does not satisfy the exact requested authority/checker/trust-root contract;
2. compare cost only among routes that already satisfy that contract.

The canonical tests prove:

- a cheaper weak route never beats an exact requirement;
- unavailable exact authority returns a fail-closed error;
- checker identity and trust root are exact route requirements;
- probabilistic/empirical/heuristic alternatives cannot satisfy an exact request.

Resource/cost pressure therefore cannot weaken authority.

---

## 13. Adversarial integration gate

The P3 adversarial suite exercises cross-boundary attacks instead of only happy-path package behavior.

Proved rejection classes include:

```text
cross-World witness reuse
cross-generation witness/closure reuse
activation from another generation
package not admitted by generation
composition claim not admitted by generation
composition evidence not in generation authority
morphism not admitted by generation
unsupported/interfering package union
ambiguous common parent
lossy/noncanonical implicit morphism
weak Shared Fact consumed as exact
candidate-only federation producing authority
undeclared federation side effects
weak certificate route selected for exact request
```

No tested attack reaches authoritative PASS.

---

## 14. Dependency and architecture freeze

The canonical dependency trees show:

```text
formula-packages
└── formula-core

formula-check
├── formula-core
└── num-bigint 0.4.8
```

P3 adds no external runtime dependency to `formula-packages`. The P2 checker dependency closure remains unchanged, and the canonical architecture test still proves:

```text
formula-check -/-> formula-engine
formula-check -/-> formula-realize
formula-check -/-> formula-first-light
formula-check -/-> formula-store

formula-engine -/-> formula-check implementation
```

The P0 sealed-fixture and runtime-network architecture gates also remain green.

---

## 15. Canonical source proof sequence

The successful P3 workflow executed:

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
cargo test --workspace --all-targets --locked --offline
cargo build --workspace --all-targets --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo tree --locked --offline -p formula-check --edges normal
cargo tree --locked --offline -p formula-packages --edges normal
test -z "$(git status --porcelain)"
```

Result: **PASS** on exact source-under-test commit `296902f4dac1d3e0f2b4e6e2e64cfb3c7306c4f7`.

---

## 16. P3 proof markers

```text
P3-01 theory package structural identity deterministic          PASS
P3-02 minimum builtin package manifests dependency-bound      PASS
P3-03 capability closure scoped by generation/world           PASS
P3-04 witness activation/deactivation deterministic           PASS
P3-05 package composition/interference fails closed           PASS
P3-06 canonical/common-parent ambiguity fails closed          PASS
P3-07 shared mathematical fact polarity enforced              PASS
P3-08 federation adapter cannot manufacture authority         PASS
P3-09 certificate router preserves Authority Contract         PASS
P3-10 P0/P1/P2 architecture and authority gates preserved    PASS
```

All ten markers were emitted by canonical job `99130422586`, which concluded successfully.

---

## 17. P0/P1/P2 preservation

P3 extends rather than replaces predecessor authority.

The canonical P3 workflow reruns and preserves:

```text
P0 repository/build architecture firewall
P0 sealed First-Light fixture identity/boundary
P0 canonical runtime-network allowlist boundary
P1 deterministic structural identity
P1 immutable content-addressed backing
P1 atomic generation publication and rollback
P1 historical replay/corruption rejection
P2 certificate-envelope binding
P2 exact Authority Contract matching
P2 malicious-producer rejection
P2 exact polynomial/GF2/U8 checking
P2 promotion-manifest preflight
P2 independent realization comparison harness
P2 checker/producer dependency firewall
```

P3 package/closure/federation logic does not acquire certification or store-publication authority.

---

## 18. Milestone boundary

**P3 source is proved and review-clean on the isolated implementation branch for the source-under-test commit recorded above.**

This checkpoint intentionally records the source proof rather than claiming its own documentation commit was tested.

The branch has **not** been merged to `main`.

A post-checkpoint canonical proof must remain green with this checkpoint and `CURRENT.md` present before the documentation-bearing branch head is treated as the final frozen P3 branch candidate.

---

## 19. Next phase

The frozen roadmap names the next boundary:

```text
P4 — Query, Theory Profile, Campaign IR, Obligation compiler
```

P4 is not started by this checkpoint. It must build on the proved P3 package/capability substrate without allowing compiler/campaign/search state to create or weaken mathematical authority.
