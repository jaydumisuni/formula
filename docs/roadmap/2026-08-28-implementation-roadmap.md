# Implementation Roadmap — D1–D5 to First Light and Beyond

**Date:** 2026-08-28  
**Status:** FROZEN ROADMAP  
**Repository name:** temporary only; not product identity  
**Design authority:** D1, D1A, canonical D2, D3, D4, D5, First-Light specification.

This roadmap implements the frozen architecture through proof-gated phases. It deliberately begins with the smallest local CPU system that can prove the defining growth loop, then expands mathematical breadth and scale only after First Light.

The roadmap is not a promise that every downstream backend will be implemented. Later phases remain conditional on evidence and need.

---

# 1. Global implementation laws

Every phase inherits these laws:

1. recover exact existing evidence/design before modifying architecture;
2. mathematical authority and search state remain separate;
3. no model, CAS, solver, optimizer, compiler, or external package is trusted by identity;
4. every durable authority artifact is content-addressed and generation/world scoped;
5. result authority can never be silently weakened by resource limits;
6. generated native code requires realization validation;
7. promotion is the only route from candidate mathematics to authority;
8. First Light remains local, CPU-only, model-free, network-free during canonical execution;
9. Ptah is deferred until the local proof passes;
10. broad research is closed; only design-blocking targeted spikes are allowed.

---

# 2. Dependency graph

```text
P0 Repository/Build Skeleton
  |
  v
P1 Core Identity + Authority Store (D2)
  |
  +-------> P2 Checker/Certificate Core
  |              |
  v              v
P3 Theory Packages + Capability Closure (D2)
  |              |
  +--------------+
  |
  v
P4 Query/Compiler/Campaign Core (D3)
  |
  v
P5 CandidateSpace + Discovery Subset (D3)
  |
  v
P6 First-Light Mathematical Packages/Harness
  |
  +-------> P7 Promotion + Generation Transition (D5)
  |              |
  v              v
P8 Native Realization Path (D4)
  |              |
  +--------------+
  |
  v
P9 Canonical First-Light Proof
  |
  +-------> P10 Self-Expansion Hardening (D5)
  |
  +-------> P11 Federation Breadth
  |
  +-------> P12 Self-Host/Bootstrap Trust Reduction
  |
  `-------> P13 Ptah Integration (explicitly deferred)
```

P9 is the first major implementation freeze. No distributed/GPU architecture is allowed to become a prerequisite for P9.

---

# P0 — Repository and reproducible build skeleton

## Goal

Create the smallest implementation workspace with explicit authority/checker/search separation and reproducible source/toolchain identity.

## Build scope

Recommended bounded First-Light workspace:

```text
crates/
  formula-core/
  formula-store/
  formula-check/
  formula-engine/
  formula-packages/
  formula-realize/
  formula-first-light/
  formula-cli/
```

Files/configuration include:

```text
rust-toolchain.toml
Cargo.toml
Cargo.lock
canonical schema/version constants
local .formula/ directory contract
tests/authority-boundary/
```

## Proof obligations

```text
P0-01 pinned toolchain/source dependency manifest
P0-02 formula-check cannot depend on formula-engine/search crates
P0-03 sealed First-Light fixtures cannot be imported by discovery packages
P0-04 no network dependency in canonical First-Light runtime path
P0-05 deterministic test fixture identities
```

## Gate P0

**PASS only if:** a clean local build produces exact dependency/toolchain/source metadata and architectural dependency tests prove checker/search/sealed-target separation.

## Dependencies

Frozen D1–D5 + First-Light spec only.

---

# P1 — Core structural identity and authority store

## Goal

Implement the minimum D1/D2 durable artifact system and immutable generation replay.

## Build scope

Implement schemas for:

```text
Entity
Relation
World
Judgement
EvidenceEnvelope metadata
Realization metadata
ArtifactDigest
UniverseGeneration
AuthorityContract
Observer
```

Implement:

```text
canonical encoding v1
SHA-256 structural digests
content-addressed immutable blob store
local authority index/transaction layer
generation manifest build/load/replay
```

First-Light physical choice may use SQLite plus blob directory, but APIs must not encode SQLite as semantic identity.

## Proof obligations

```text
D2-P01 deterministic structural identity replay
D2-P02 atomic generation publication
D2-P03 semantic equivalence separate from digest identity
D2-P11 historical generation replay
```

Negative tests:

```text
field-order/canonicalization variation cannot change normalized digest
non-semantic timestamp/path cannot enter structural digest
blob mutation changes digest and is rejected
partial generation transaction never becomes active
```

## Gate P1

**PASS only if:** the same semantic fixture independently serializes to the same digest across fresh local runs, historical generation roots replay, and a crash/failure injection cannot publish partial authority.

## Dependencies

P0.

---

# P2 — Independent checker and certificate-envelope core

## Goal

Establish authority production through an independent checking path before discovery exists.

## Build scope

`formula-check` implements:

```text
Evidence envelope validation
digest/dependency/world binding
exact finite/exhaustive checker framework
polynomial identity checker
GF(2)/Boolean translation+witness checker
U8 semantic equivalence checker
promotion-manifest structural checker
realization-equivalence harness interface
```

The checker accepts frozen artifacts, not engine-internal pointers.

## Proof obligations

```text
D2-P07 no silent Authority Contract downgrade
D2-P08 checker isolated from search producer
D4-P13 optimizer/compiler cannot self-admit
D5-P01 candidate frozen before certification
```

Negative tests include forged evidence, mismatched target digest, changed candidate after proof, checker-version mismatch, and stricter Authority Contract with insufficient evidence.

## Gate P2

**PASS only if:** a deliberately malicious/faulty producer cannot obtain authority without satisfying the independent checker on exact frozen artifacts.

## Dependencies

P0, P1.

---

# P3 — Theory packages, structure inference, capability closure, federation contracts

## Goal

Implement the D2 semantic package/capability machinery required by First Light.

## Build scope

Minimum packages:

```text
Integer / Rational exact arithmetic
Boolean
U8 wrapping/bitwise semantics
GF(2)
Polynomial(Integer,n)
GF2Vector / GF2Matrix
```

Implement:

```text
TheoryPackage manifest
CapabilityContract
Structure Goal IR
structure/property witness store
canonical morphism/common-parent subset
generation-scoped capability closure
FederationAdapter interface
Shared Fact polarity types
Certificate Router v1
```

No external SAT/SMT/CAS binary is required yet.

## Proof obligations

```text
D2-P04 closure scoped by generation/world
D2-P05 package activation checks composition/interference
D2-P06 shared-fact polarity enforced
D2-P07 certificate route preserves authority contract
```

## Gate P3

**PASS only if:** adding a certified structure witness changes capability availability deterministically, removing activation removes it, and an over-approximation cannot be consumed as an exact witness.

## Dependencies

P1, P2.

---

# P4 — Query, Theory Profile, Campaign IR, Obligation compiler

## Goal

Implement the D3 compiler front end and mathematical attack graph without advanced discovery algorithms.

## Build scope

Implement:

```text
QueryIR
semantic elaboration
RelevantRegion retrieval
TheoryProfile v1
Observer/Authority/Resource contracts
RepresentationNode/Edge
ReductionEdge
Decomposition
Campaign IR
Obligation IR
WorkCellPlan
Result Bundle
replay manifest
```

Campaign graph supports at least `AND` and `OR` for First Light.

## Proof obligations

```text
D3-P01 exact query semantic preservation
D3-P02 no lossy implicit morphism
D3-P03 representation preservation metadata mandatory
D3-P04 reduction preserves only certified result classes
D3-P05 decomposition reconstruction explicit
D3-P09 Work Cells cannot modify authority
D3-P11 replay manifest complete
D3-P12 unknown/refuted/resource-unknown distinct
```

## Gate P4

**PASS only if:** a query can be compiled/replayed into the same semantic campaign from exact U/World/package/policy inputs, and deliberately invalid route compositions are rejected structurally.

## Dependencies

P3.

---

# P5 — CandidateSpace and Discovery subset

## Goal

Prove symbolic search-space behavior without implementing the long-term universal substrate.

## Build scope

Three First-Light CandidateSpace backends:

```text
1. affine exact polynomial coefficient space
2. reduction-route graph space
3. typed U8/Boolean observational expression space
```

Implement backend-neutral operations needed by targets:

```text
restrict
refine
partition
empty
extract
serialize/freeze
```

Implement bounded discovery:

```text
sample/discriminator loop
CEGIS counterexample refinement
observational equivalence bucketing
minimal-cost extraction
fair fallback
```

## Proof obligations

```text
D3-P06 CandidateSpace exactness/polarity preserved
D3-P07 failure pruning remains scoped
D3-P08 heuristics cannot discharge Judgements
D3-P10 recompilation preserves unaffected identity
```

## Gate P5

**PASS only if:** a single counterexample removes/refines a whole candidate class, candidate-space serialization is deterministic, and search output has no authority until checked.

## Dependencies

P4.

---

# P6 — First-Light target harness and blindness gates

## Goal

Implement sealed FL-A, FL-B, FL-C fixtures and prove search cannot read the hidden answers.

## Build scope

Implement exactly the frozen First-Light specification:

```text
FL-A hidden polynomial identity
FL-B Boolean XOR -> GF(2) route
FL-C U8 power-of-two construction synthesis
```

Implement sealed oracle interfaces, target digests, near-miss fixtures, and dependency/lint tests preventing discovery-to-sealed imports.

## Proof obligations

```text
blindness manifest
sealed target hash binding
candidate freeze before target comparison
near-miss visibility without answer leakage
```

## Gate P6

**PASS only if:** static dependency checks and runtime harness evidence show active search receives only allowed interfaces, not hidden formulas/expected programs.

## Dependencies

P5, P2.

---

# P7 — D5 Promotion and generation transition

## Goal

Implement candidate certification-to-admission without yet requiring native compilation.

## Build scope

Implement:

```text
PromotionCandidate
CERTIFIED / ADMITTED / ACTIVATED states
promotion policy v1
atomic U_g -> U_(g+1) transaction
capability closure delta
conflict/quarantine path
supersession lineage
proof freshness/dependency cone
```

For First Light, implement semantic primitive promotion for FL-C.

## Proof obligations

```text
D5-P01 candidate frozen before cert
D5-P02 states remain distinct
D5-P08 atomic/history-preserving generation
D5-P09 closure derived from admitted inputs
D5-P14 rollback/history preservation
```

Negative transaction tests include parent-generation race, mismatched evidence, changed candidate digest, and authority-write attempts from search code.

## Gate P7

**PASS only if:** `U_0` remains byte/authority replayable after successful creation of `U_1`, and no failed/partial transaction can expose the new primitive.

## Dependencies

P1–P6.

---

# P8 — D4 native realization and validation

## Goal

Compile the newly promoted FL-C construction to a fast native CPU realization and independently validate it.

## Build scope

Implement bounded D4 pipeline:

```text
admitted semantic construction
 -> specialized forward U8->Bool plan
 -> generated standalone Rust source
 -> pinned rustc -O
 -> binary Realization manifest
 -> independent exhaustive checker
 -> admitted Realization
```

Also implement the generic D4 contracts needed to prevent this from becoming a one-off:

```text
specialization identity
realization manifest
backend/toolchain identity
fallback semantics
realization selection
```

## Proof obligations

```text
D4-P01 semantic binding
D4-P02 specialization correctness
D4-P03 lowering class explicit
D4-P12 proof erasure retains identity binding
D4-P13 compiler cannot self-admit
D4-P14 local CPU viability
```

## Gate P8

**PASS only if:** the compiled binary is independently equivalent to the admitted FL-C semantics for all 256 U8 values, and modifying the binary/source digest invalidates dispatch/admission.

## Dependencies

P7.

---

# P9 — Canonical First-Light proof and freeze

## Goal

Run the complete frozen suite from clean local state and produce one independently replayable proof manifest.

## Required run sequence

```text
create/load U_0
run FL-A blind discovery -> certify
run FL-B representation/reduction -> certify
run FL-C blind synthesis
reject mandatory near-miss
freeze final candidate
independently certify candidate
promote -> U_1
compile native realization
independently validate realization
submit second reuse query under U_1
prove no synthesis rediscovery for promoted primitive
run all negative controls
assemble proof manifest
independently replay
```

## Gate P9

All First-Light PASS markers must be emitted by the independent verifier:

```text
PASS D1_AUTHORITY_SEPARATION
PASS D2_IDENTITY_GENERATION_REPLAY
PASS D2_CERTIFICATE_ROUTING
PASS D2_SEARCH_STATE_SEPARATION
PASS D3_BLIND_SEMANTIC_ELABORATION
PASS D3_REPRESENTATION_REDUCTION
PASS D3_SYMBOLIC_CANDIDATE_SPACE
PASS D3_FALSE_NEARMISS_REJECTION
PASS D4_NATIVE_REALIZATION_EQUIVALENCE
PASS D4_CPU_LOCAL_OFFLINE
PASS D5_ATOMIC_PROMOTION
PASS D5_CAPABILITY_CLOSURE_EXPANDED
PASS D5_SECOND_QUERY_REUSE
PASS NEGATIVE_CONTROLS
PASS FIRST_LIGHT_COMPLETE
```

No partial marker set constitutes First Light.

## Freeze output

```text
proof manifest digest
U_0/U_1 digests
source commit
checker/toolchain digests
campaign/evidence/realization digests
negative-control manifest
```

## Dependencies

P0–P8.

---

# P10 — Self-expansion hardening after First Light

## Goal

Generalize D5 beyond the one bounded primitive.

## Build scope

```text
promotion class registry
structure-witness promotion
nogood/counterexample activation
reduction/morphism promotion
semantic change classification
grammar Lambda_g identity/evolution
shadow-mode metaprimitive activation
proof transport/repair framework
realization-only upgrade path
rollback/supersession tooling
```

## Proof gate

At least one non-primitive promotion must produce measurable future capability change, e.g. a new structure witness or reduction unlocks an existing generic capability without adding new solver code.

## Dependencies

P9.

---

# P11 — Federation breadth

## Goal

Prove the architecture with real specialist ecosystems while preserving the authority boundary.

Candidate package order should maximize certificate quality and architectural coverage, not popularity.

Suggested sequence:

```text
1. SAT + LRAT checker
2. SMT/Alethe checker path
3. FLINT/GMP exact arithmetic adapter
4. polynomial/Groebner package with independent certificate route
5. optimization/VIPR or exact LP certificate path
6. rigorous interval/numerical certificate package
7. proof-assistant/foundational export package
```

Each package must pass:

```text
semantic adapter proof/check
certificate routing
interference/composition gate
failure/negative fixture
version/freshness test
```

## Gate P11

At least two heterogeneous specialist packages cooperate through Shared Mathematical Facts/bridges while neither is trusted by producer identity.

## Dependencies

P9; P10 desirable but not mandatory for first adapters.

---

# P12 — Self-host/bootstrap trust reduction

## Goal

Implement D1A/D5 staged self-hosting beyond the conventional First-Light bootstrap.

## Required targeted research spike before build

**RS-BOOTSTRAP-SEED** — choose the practical Stage-0/Stage-1 seed/toolchain strategy by comparing the preserved CakeML, Mes/stage0, verified-compilation, and diverse-compilation donor patterns against the actual post-First-Light implementation.

This is targeted research because D1A freezes trust roles but deliberately does not freeze the final seed language/toolchain.

## Build scope

```text
B0 explicit seed/trust manifest
B1 minimal authority loader/checker
reproducible build manifest
independent/diverse compiler path for authority-critical components
self-hosted rebuild of selected core/checker component
successor realization promotion
rollback to prior toolchain generation
```

## Gate P12

A selected authority-critical component is rebuilt by the self-host path and accepted only after an independent path validates the successor; the successor cannot authorize itself.

## Dependencies

P9, preferably P11 foundational/checker experience.

---

# P13 — Ptah integration — EXPLICITLY DEFERRED

## Earliest entry condition

Ptah integration is forbidden as a prerequisite for P0–P9.

It may begin only after canonical First Light passes locally and these conditions are proven:

```text
Campaign IR serializes deterministically
WorkCellPlan binds exact inputs/evidence obligations
artifacts are content-addressed
work location does not affect semantic identity
promotion remains in the mathematical Core System
local consumption of U_1 remains functional without Ptah
```

## Integration scope

Ptah receives only execution/workspace contracts such as:

```text
WorkCellPlan
resource requirements
input artifact digests
checkpoint policy
expected output/evidence classes
```

Ptah returns content-addressed artifacts/results.

It does **not** receive authority to:

```text
change U_g
activate packages
weaken Authority Contract
issue mathematical verdicts beyond returned checked evidence
```

## Gate P13

The same campaign obligation executed locally and through Ptah yields semantically equivalent checked artifacts under the same input contract, while promotion/authority logic remains unchanged.

---

# 3. Targeted research spikes

Broad research is closed. The following spikes are permitted only when their trigger occurs.

## RS-1 — CandidateSpace scaling / e-graph-hypergraph substrate

**Trigger:** P5/post-P9 candidate spaces cannot scale or represent required cyclic/multi-output mathematics cleanly.

**Question:** which mature e-graph/egglog/versioned structure should implement the next backend, and when is e-hypergraph semantics mature enough to replace/augment it?

**Not required for First Light.**

## RS-2 — Canonical binary authority encoding

**Trigger:** canonical JSON becomes a measured storage/throughput bottleneck after P9.

**Question:** choose a deterministic binary format with migration/identity semantics.

**Not required for First Light.**

## RS-3 — Foundational proof interoperability

**Trigger:** domain-native certificate coverage leaves critical cross-package theorems requiring a common foundational replay/export layer.

**Question:** Lean/Rocq/Dedukti/FPC or another route for the concrete certificate families then active.

## RS-4 — Verified/validated optimizer backend

**Trigger:** D4 needs optimization beyond bounded exhaustive realization checking.

**Question:** select Alive2/LLVM validation, verified compiler, Jasmin, or domain-specific proof-producing generator based on actual KIR workload.

## RS-5 — GPU backend

**Trigger:** CPU profile after P9/P11 identifies a dominant workload with high regular parallelism and a clear semantic validation route.

**Question:** target backend/API and equivalence-checking method.

GPU research is not justified merely by availability.

## RS-6 — Ptah distributed determinism/checkpoint semantics

**Trigger:** P13 entry conditions have passed and distributed campaign scale is useful.

**Question:** map WorkCell/Campaign checkpoint contracts onto Ptah without changing authority semantics.

## RS-7 — Bootstrap seed/toolchain

**Trigger:** P12 entry.

Defined above as `RS-BOOTSTRAP-SEED`.

---

# 4. Proof-gate discipline

A phase is not complete because code exists or tests are mostly green.

Each phase freezes only when:

```text
1. exact design obligations are mapped to executable checks
2. positive proof fixtures pass
3. negative/adversarial fixtures fail closed
4. resulting artifacts are content-addressed/replayable where required
5. independent checker/reviewer boundary is preserved
6. dependency and authority inputs are recorded
7. the phase's final proof manifest is frozen
```

Runtime execution confirms the implementation; the architecture/proof obligations determine what must be true before execution is considered authoritative evidence.

---

# 5. Roadmap completion states

```text
ARCHITECTURE_FROZEN
    D1-D5 + First Light + roadmap are frozen

FIRST_LIGHT_PROVEN
    P9 complete

SELF_EXPANSION_HARDENED
    P10 complete

FEDERATION_PROVEN
    P11 complete

SELF_HOST_TRUST_REDUCED
    P12 complete

DISTRIBUTED_EXECUTION_PROVEN
    P13 complete
```

These are distinct. Passing First Light must not be misreported as completing the entire long-term system.

---

# 6. Immediate next executable action

The next implementation action after this roadmap freeze is **P0**, followed directly by P1/P2.

Do not begin Ptah, GPU, final e-hypergraph, large knowledge ingestion, public API/UI, or unsolved-math campaigns before P9.

The first implementation campaign should be judged by one question:

> **Can this repository produce the exact canonical First-Light proof manifest proving that mathematical capability grew from U_0 to U_1 and was then reused as fast native mathematics?**
