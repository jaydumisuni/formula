# D3 — First-Light Build Architecture

**Date:** 2026-08-28  
**Status:** FROZEN DESIGN MILESTONE D3  
**Repository name:** temporary only; not product identity  

D3 selects the smallest concrete implementation architecture capable of proving D1 + D2 on ordinary local CPU hardware without prematurely implementing the long-term mathematical machine.

D1 remains authority for the mathematical constitution. D2 remains authority for the operational contracts. D3 freezes only the concrete First-Light build boundary, target suite, local persistence format, executable subset, checker separation, realization path, and proof markers required to implement the first executable proof.

A full product roadmap remains downstream of First-Light evidence.

---

# 1. D3 objective

First Light must prove the complete growth loop:

```text
U_0
  -> blind target
  -> symbolic/structured discovery
  -> false candidate rejection
  -> independent certification
  -> semantic admission
  -> capability activation
  -> U_1
  -> native realization
  -> realization validation
  -> second related query
  -> reuse promoted primitive without rediscovery
```

D3 explicitly does **not** attempt to prove:

- universal mathematical coverage;
- unsolved research mathematics;
- distributed execution;
- GPU acceleration;
- a final mathematical IR;
- a final proof assistant;
- a final storage engine;
- a final e-graph/hypergraph implementation;
- the final programming language/runtime architecture;
- model-assisted discovery.

The canonical First-Light proof is local, deterministic/replay-bound, model-free, network-free during execution, and CPU-only.

---

# 2. Concrete technology boundary

## 2.1 Core language

Use **stable Rust** as the First-Light implementation language.

The exact compiler release is pinned by `rust-toolchain.toml` and `Cargo.lock` when implementation begins. D3 freezes Rust as the First-Light build language, not as the permanent constitutional implementation language.

Reasons:

- native CPU performance;
- deterministic ownership/memory behavior;
- strong enum/type modeling for D1 semantic classes;
- straightforward exact-integer libraries;
- safe process/file boundaries for independent checkers;
- easy generation of stripped native realizations;
- compatibility with later e-graph/egglog and native-library integration if required.

## 2.2 Exact arithmetic

First Light uses pure/exact integer and rational arithmetic in authority paths:

```text
num-bigint
num-rational
num-traits
```

No IEEE floating-point value may participate in semantic identity or exact certification in First Light.

## 2.3 Persistence

Use:

```text
SQLite (rusqlite, bundled)
+
immutable content-addressed blob directory
```

SQLite stores indexes, relations, generation manifests, campaign metadata, and transactional promotion state.

Large/immutable artifacts are stored by digest outside SQLite.

This is a First-Light persistence choice only. D1/D2 do not require SQLite long-term.

## 2.4 Canonical authority encoding

Authority-bearing manifests use versioned canonical JSON under a restricted schema:

- UTF-8;
- deterministic field ordering;
- no native floating-point numbers;
- integers encoded canonically;
- rationals encoded as reduced signed numerator/positive denominator pairs;
- referenced durable artifacts represented by exact digest strings;
- no timestamps or machine-local paths inside structural identity.

Authority digests use **SHA-256**.

A future canonical binary encoding may replace this in a later generation through an explicit migration relation; First-Light needs inspectability and cross-tool reproducibility more than storage density.

## 2.5 Runtime dependencies deliberately excluded

Canonical First Light does not require:

```text
Lean / Rocq / Isabelle
SMT / SAT binaries
FLINT / GMP C FFI
egg / egglog
LLVM API embedding
GPU libraries
Ptah
network access
AI models
```

These remain future packages. The purpose is to prove the architecture before proving integration breadth.

---

# 3. Cargo workspace boundary

Use a small workspace with strict authority separation:

```text
crates/
  formula-core/
  formula-check/
  formula-engine/
  formula-packages/
  formula-realize/
  formula-first-light/
  formula-cli/
```

## 3.1 `formula-core`

Owns only cross-layer immutable schema and identity machinery:

```text
Entity
Relation
World
Judgement
EvidenceEnvelope metadata
Realization metadata
Generation manifest
Query
AuthorityContract
ArtifactDigest
canonical encoding / SHA-256
```

It contains no discovery algorithm and no promotion policy implementation.

## 3.2 `formula-check`

Independent certification binary/library.

May depend on `formula-core` and exact arithmetic only.

It must **not** depend on:

```text
formula-engine
formula-realize
candidate search implementations
search heuristics
```

This separation is required so a search bug cannot silently share code with its own checker.

First-Light checker families:

```text
polynomial identity checker
GF(2) witness / reduction checker
U8 exhaustive semantic checker
promotion-manifest checker
realization-output equivalence checker
```

## 3.3 `formula-engine`

Owns:

```text
Universe loading
World/query elaboration
Package/Capability resolver
TheoryProfile derivation
Campaign IR
Work Cell execution contracts
CandidateSpace orchestration
search economy subset
promotion transaction orchestration
```

It cannot produce an authoritative PASS by itself; authoritative proof markers are issued by the independent First-Light verifier after artifact replay.

## 3.4 `formula-packages`

Contains the minimal First-Light mathematical packages behind D2 Package/Capability contracts:

```text
core exact arithmetic / Z / Q
GF(2)
polynomial expressions
Boolean / U8 bitvector semantics
linear systems over GF(2)
First-Light CandidateSpace backends
```

These packages are replaceable implementations of admitted semantics, not constitutional kernel types.

## 3.5 `formula-realize`

Owns specialization and native realization generation for the bounded First-Light primitive.

It may generate Rust source and invoke the pinned `rustc` as an **untrusted optimizer/compiler path**.

Its output becomes usable only after `formula-check` independently validates realization equivalence.

## 3.6 `formula-first-light`

Owns:

```text
sealed target harness
blindness enforcement
canonical target suite
negative-control suite
proof-manifest assembly
PASS marker production after independent replay
```

The active Discovery Fabric does not import the sealed target definitions.

## 3.7 `formula-cli`

Thin local CLI only:

```text
formula first-light run
formula first-light verify <manifest>
formula universe inspect <digest>
formula campaign inspect <digest>
```

No UI/server is required.

---

# 4. Local repository/runtime layout

Implementation should create this concrete layout:

```text
.formula/
  authority.sqlite
  objects/
    sha256/
      aa/
        <remaining-digest>
  generations/
    <generation-digest>.json
  campaigns/
    <campaign-digest>/
      manifest.json
      events.jsonl
      checkpoints/
  proofs/
    <proof-digest>/
  realizations/
    <realization-digest>/
  tmp/
```

The repository test tree contains:

```text
tests/first-light/
  public/
  sealed/
  negative/
```

`sealed/` is consumed only by the First-Light harness/checker process, never imported into the active discovery package set.

---

# 5. Atomic generation/promotion protocol

First-Light promotion must prove D1 generation semantics concretely.

Protocol:

```text
1. Candidate artifacts are already content-addressed and immutable.
2. Independent evidence checker validates the candidate Judgement.
3. Promotion policy validates semantic admission + activation obligations.
4. New immutable blobs/manifests are written first.
5. SQLite transaction opens.
6. Parent generation digest is rechecked.
7. Delta/admission rows are inserted.
8. New generation manifest/root digest is computed.
9. Capability closure delta is computed against the new generation.
10. Generation root is atomically committed.
11. Transaction commits.
```

A crash before step 10/11 may leave unreachable immutable blobs, but it may not expose a partially authoritative Universe generation.

No candidate can mutate `U_0` in place.

---

# 6. Minimal executable semantic subset

First Light does not implement arbitrary future mathematics.

The executable semantic subset contains only what the three targets require.

## 6.1 Parents/domains

```text
Integer
Rational
Boolean
U8
GF2
Polynomial(Integer, variable n)
GF2Vector(n)
GF2Matrix(m,n)
```

## 6.2 Relation/operator subset

```text
integer/rational add/sub/mul/equality
polynomial construction/evaluation/normalization
Boolean not/and/or/xor/equality
U8 wrapping subtraction
U8 bitwise and/or/xor/not
U8 equality/inequality-to-zero
GF2 addition/multiplication
GF2 linear equation
```

All operator semantics are versioned package exports.

## 6.3 Campaign subset

Required Campaign nodes:

```text
Goal
Route
Obligation
CandidateSpace
WorkCell
Artifact
Evidence
Result
```

Required aggregation:

```text
AND
OR
```

D3 does not require implementing general least/greatest fixed-point Campaign aggregation yet.

## 6.4 Search-economy subset

Only enough scheduling to prove the D2 boundary:

```text
root-goal gating
static/certified route cost where available
candidate-space reduction count
fair FIFO fallback
```

Bandits, advanced metareasoning, proof-number search, activity decay, and distributed scheduling remain downstream.

---

# 7. Three concrete CandidateSpace backends

First Light deliberately uses three different backends so `CandidateSpace` is proven as a semantic interface rather than accidentally identified with one data structure.

## 7.1 Polynomial coefficient space

Represents:

```text
P(n) = c_0 + c_1 n + ... + c_d n^d
```

with exact linear constraints over coefficients.

Operations:

```text
add_sample(n,y)
solve_exact()
restrict_degree(d)
extract_min_degree()
```

This is a compact affine coefficient space; it does not enumerate coefficient tuples.

## 7.2 Reduction-route space

Represents candidate semantic routes between problem representations:

```text
Boolean XOR constraints
    -> direct Boolean enumeration
    -> GF(2) affine system
```

Routes are graph paths with preconditions, preservation claims, and cost metadata.

The candidate space is restricted by TheoryProfile/structure facts before execution.

## 7.3 Observational expression space

Typed U8/Boolean expression synthesis uses a bottom-up grammar with observational quotienting.

Expressions having the same current sample behavior are stored in one equivalence bucket with a lowest-cost representative.

The search therefore reasons over behavior classes rather than retaining every syntactic expression.

New counterexamples split/refine the surviving CandidateSpace.

This backend is intentionally small and bounded. It proves symbolic/equivalence-space search without requiring the long-term e-graph implementation.

---

# 8. Blind First-Light target suite

The suite follows D2 Section 22 and contains three classes.

## FL-A — Exact identity discovery

### Hidden semantic target

For integer `n`:

```text
F(n) = (n + 1)^7 - n^7
```

The active Discovery Fabric does **not** receive the expanded polynomial.

It may obtain exact oracle samples through a narrow sample relation.

### Required discovery

Recover the exact minimal-degree polynomial:

```text
7 n^6 + 21 n^5 + 35 n^4 + 35 n^3 + 21 n^2 + 7 n + 1
```

using the polynomial coefficient CandidateSpace.

### Certification

After the candidate structure is frozen, the independent checker expands the sealed semantic definition and canonicalizes both polynomials over exact integers.

Equality of the normalized coefficient vectors establishes a universal polynomial identity over integers.

### Negative near-miss

Inject/allow a candidate of the form:

```text
P(n) + k * product(n-i, i=0..6)
```

which matches the early sample set but fails at a discriminating unseen input.

The candidate must be rejected before/at certification.

### What FL-A proves

```text
exact relation discovery
symbolic CandidateSpace refinement
blind target binding
false numerical coincidence rejection
independent exact certification
```

---

## FL-B — Representation/reduction discovery

### Public semantic problem

A system of Boolean XOR constraints over **24 variables**.

The query requests **one exact satisfying witness**, or exact infeasibility if none exists.

The hidden fixture contains the expected outcome but is unavailable to the active route search.

### Candidate routes

At minimum:

```text
Route 1: direct Boolean assignment search
Route 2: certified Boolean-XOR -> GF(2) affine-system representation
```

The GF(2) route maps Boolean XOR to field addition modulo 2 and solves by exact Gaussian elimination.

### Required discovery

TheoryProfile and route search must discover/select the GF(2) representation under the canonical local resource budget.

The target is intentionally sized so exhaustive Boolean search is unattractive under the canonical budget while GF(2) elimination is trivial on ordinary CPU hardware.

### Certification

The independent checker verifies:

```text
translation of every XOR row to GF(2)
returned Boolean/GF2 witness satisfies every original XOR constraint
all decoded values lie in the declared Boolean domain
```

If an infeasible fixture is included, a separate exact contradiction certificate may be added later; the canonical D3 witness target requires a satisfiable instance only.

### What FL-B proves

```text
representation frontier
certified reduction path
cross-parent semantics
capability routing
solver federation without monolith
observer-preserving witness reconstruction
```

---

## FL-C — Synthesized executable construction and self-expansion

FL-C is the canonical full growth-loop target.

### Target semantics

For `x : U8`, synthesize a Boolean construction implementing:

```text
IsPowerOfTwoU8(x)
```

whose specification is:

```text
true exactly for x in {1,2,4,8,16,32,64,128}
```

The known compact formula is hidden from the Discovery Fabric.

### Allowed grammar

Typed grammar contains a bounded subset sufficient to discover the classic construction, for example:

```text
ByteExpr:
    x
    0
    1
    sub_wrap(ByteExpr, ByteExpr)
    bit_and(ByteExpr, ByteExpr)

BoolExpr:
    eq_zero(ByteExpr)
    neq_zero(ByteExpr)
    and(BoolExpr, BoolExpr)
```

The exact grammar is content-addressed in the campaign manifest.

### Discovery strategy

Use CEGIS-style observational synthesis:

```text
small initial discriminating sample
    -> observational expression classes
    -> cheapest candidate
    -> evaluate against specification oracle
    -> counterexample if wrong
    -> refine behavior space
    -> repeat
```

A deliberate early near-miss must appear or be injected:

```text
(x & (x - 1)) == 0
```

which incorrectly accepts `0`.

The `x != 0` obligation must be learned/retained for the final construction.

### Independent semantic certification

Because the declared domain is finite U8, `formula-check` exhaustively compares the frozen candidate against the independent specification evaluator for all 256 inputs.

This produces an exact finite exhaustive certificate, not a statistical test.

### Promotion

The certified relation/construction is admitted into `U_1` as:

```text
Entity/Relation semantic artifact
Judgement establishing specification equivalence
Evidence envelope from exhaustive checker
activated CapabilityContract
```

The hidden human-known expression/name is not used as authority.

### Native realization

`formula-realize` generates a small standalone Rust implementation of the promoted construction and invokes the pinned `rustc -O`.

The resulting executable is **not trusted** merely because compilation succeeded.

### Realization validation

A separate checker executes the native realization over all 256 U8 inputs and compares outputs with the admitted semantic evaluator.

Only then is the native Realization admitted.

### Reuse query

A second query, under `U_1`, asks the engine to classify/filter a canonical set of U8 values using `IsPowerOfTwoU8`.

Required evidence:

```text
capability resolver selects the promoted primitive
no synthesis CandidateSpace is created
no discovery Work Cell for the primitive is launched
native or admitted semantic realization is used
result is exact
```

The second query must measurably avoid the original synthesis campaign.

This is the canonical proof of self-expansion.

---

# 9. Canonical negative-control suite

D2 requires the architecture to reject misleading or malformed mathematics.

D3 freezes these concrete controls.

## N1 — sample-fitting false identity

FL-A near-miss matches the initial sample points but fails a hidden/discriminating point.

Expected:

```text
REFUTED / candidate removed
no semantic admission
```

## N2 — missing assumption

Candidate claim:

```text
sqrt(x*x) = x
```

with integer/real-style semantics but without the required `x >= 0` assumption.

The minimal First-Light package may encode this as a prebuilt semantic fixture rather than implement a general square-root engine.

Expected:

```text
refuse universal Judgement
identify missing/insufficient World condition
```

## N3 — numerical coincidence

A polynomial candidate that agrees on an insufficient finite sample set but fails exact identity certification.

Expected:

```text
candidate status only
no authority upgrade
```

## N4 — wrong domain/branch semantics

A candidate structurally identical under integer arithmetic but evaluated under U8 wrapping arithmetic, or vice versa.

Expected:

```text
structural/parent mismatch
no silent coercion
```

## N5 — bad optimized realization

Mutate the generated FL-C realization, e.g. remove the `x != 0` guard.

Expected:

```text
semantic object remains admitted
mutated Realization rejected
Universe mathematical authority unchanged
```

## N6 — ambiguous coercion

Provide an Entity with more than one non-equivalent admissible route into a requested parent and no canonical witness selecting one.

Expected:

```text
AMBIGUOUS
compiler branches or refuses implicit resolution
```

## N7 — resource exhaustion

Run a synthesis query under an intentionally insufficient Work Cell budget.

Expected:

```text
RESOURCE_BOUNDED_UNKNOWN
no fallback to empirical truth
no promotion
Authority Contract unchanged
```

---

# 10. Blindness enforcement

Blindness is an execution property, not a claim about humans being unable to inspect the repository.

Canonical campaign harness must enforce:

```text
active package allowlist
active grammar digest
Universe generation digest
no sealed target package in engine dependency graph
separate sealed evaluator process/module boundary
hidden-target digest recorded before run
candidate frozen before hidden-definition equivalence check
```

The engine runs from a temporary campaign root containing only public/allowed packages and generated artifacts.

The sealed harness may expose narrow oracle operations required by a target, e.g.:

```text
sample(n) -> exact integer
check_candidate(candidate_digest) -> counterexample/acceptance for CEGIS
```

It does not expose the hidden target implementation/expanded expression.

Every oracle call is logged and content-addressed for replay.

---

# 11. Independent proof/replay boundary

The canonical run has two distinct commands:

```text
formula first-light run
formula first-light verify <proof-manifest>
```

`run` may search, schedule, synthesize, compile, and generate candidate certificates.

`verify` starts from the proof manifest and independently checks:

```text
Universe/root digests
blindness manifest
candidate structures
certificate bodies
promotion transaction
U_0 -> U_1 delta
capability closure delta
native realization outputs
reuse-query campaign trace
negative controls
```

`verify` must not invoke discovery algorithms.

A First-Light PASS is valid only if emitted by the verifier.

---

# 12. First-Light proof manifest

Canonical artifact:

```text
FirstLightProofManifest {
    schema_version
    source_commit_digest
    rust_toolchain_digest/version
    cargo_lock_digest

    U0_digest
    U1_digest
    package_set_digest

    target_A_campaign
    target_B_campaign
    target_C_campaign
    negative_control_campaigns

    sealed_target_digests
    grammar_digests

    evidence_envelope_digests
    promotion_transaction_digest
    capability_delta_digest
    realization_digest
    realization_validation_digest
    reuse_query_digest

    replay_policy_digest
}
```

Every referenced artifact is immutable/content-addressed.

---

# 13. Canonical PASS markers

The independent verifier is successful only if it emits all D2 markers:

```text
PASS_UNIVERSE_BINDING
PASS_BLIND_DISCOVERY
PASS_FALSE_CANDIDATE_REJECTION
PASS_CERTIFICATION
PASS_PROMOTION_ATOMICITY
PASS_CAPABILITY_CLOSURE_DELTA
PASS_REALIZATION_EQUIVALENCE
PASS_REUSE_WITHOUT_REDISCOVERY
PASS_AUTHORITY_NOT_DOWNGRADED
PASS_REPLAY_BINDING
```

D3 adds three diagnostic markers which do not replace the D2 constitutional set:

```text
PASS_TARGET_A_EXACT_IDENTITY
PASS_TARGET_B_REPRESENTATION_REDUCTION
PASS_TARGET_C_SYNTHESIZED_PRIMITIVE
```

The canonical milestone is accepted only if all constitutional and target markers pass in one proof manifest.

---

# 14. Test architecture

The implementation cycle must separate four proof classes.

## 14.1 Schema/identity tests

Prove deterministic canonical encoding/digests for every D1 artifact class implemented by First Light.

Required tests include:

```text
same semantic structure -> same structural digest under canonical encoding
field/order serialization variation -> same canonical bytes
changed dependency -> changed digest
World change -> changed World-bound evidence identity
```

## 14.2 Unit/property tests

For exact arithmetic, CandidateSpace operations, route composition, and campaign transitions.

Property tests may discover engineering bugs but do not replace mathematical First-Light evidence.

## 14.3 Checker negative tests

Every checker must include malformed/false certificates that must fail.

Search code and checker code may not share the same implementation of the mathematical predicate being certified where independence is required.

## 14.4 End-to-end proof test

One canonical command produces the First-Light manifest; a fresh verifier process replays it and emits the full PASS set.

---

# 15. First-Light build sequence

Implementation should proceed in this dependency order:

```text
B01 canonical identity + blob store
B02 D1 durable schemas + U_0 generation
B03 Evidence envelope + independent checker process
B04 Package/Capability contracts + closure resolver
B05 Query + minimal Campaign IR + Work Cell runner
B06 FL-A polynomial CandidateSpace + checker
B07 FL-B route/reduction CandidateSpace + GF2 package/checker
B08 FL-C observational synthesis CandidateSpace
B09 promotion transaction U_0 -> U_1
B10 generated native realization + independent finite-domain validation
B11 reuse query / no-rediscovery proof
B12 negative controls
B13 canonical verifier + full PASS manifest
```

A later build phase may optimize or parallelize only after B13 passes.

---

# 16. Freeze gates for implementation

No stage may be called complete merely because its code executes.

Each build stage freezes only when:

```text
Understand
    design authority + exact obligation recovered

Build
    implementation exists

Review
    semantics/dependencies/trust boundary inspected

Freeze
    exact candidate commit/artifact identified

Prove
    stage-specific proof/tests/evidence pass

Ship
    checkpoint committed without weakening earlier gates
```

This mirrors the engineering proof discipline of the surrounding ecosystem while keeping mathematical authority inside D1/D2 contracts.

---

# 17. What First Light deliberately does not optimize

Do not add these before the canonical proof requires them:

```text
Ptah distributed scheduling
GPU execution
web service/API
GUI
models
advanced e-graphs
formal theorem prover integration
full graph/hypergraph storage
large knowledge ingestion
FLINT/Sage/GAP federation
JIT/LLVM embedding
complex search-learning policies
out-of-core execution
```

These are valuable future capabilities but would make First Light harder to falsify and harder to understand.

The first executable proof must remain small enough that every authority edge can be inspected.

---

# 18. D3 success condition

D3 is a design milestone, not an implementation claim.

D3 is complete when the repository freezes:

```text
concrete First-Light language/runtime boundary
persistence/content identity layout
workspace/crate trust boundaries
minimal semantic and Campaign subset
three exact target classes
negative controls
blindness protocol
independent checker/replay architecture
promotion atomicity protocol
native realization/validation route
canonical proof manifest and PASS markers
build order
```

This document freezes those decisions.

---

# 19. Post-D3 next milestone

The next milestone is **F0 — First-Light implementation campaign**.

F0 should implement B01-B13 exactly against D1/D2/D3, producing the first canonical proof manifest.

Only after F0 passes should the project decide the broader implementation roadmap from executable evidence.

Questions such as long-term graph substrate, proof-assistant integration, FLINT/GAP/CAS federation, GPU kernels, Ptah work-cell execution, large mathematical ingestion, and generalized compiler/language architecture remain downstream and should be selected using what First Light actually exposes.

---

# 20. D3 frozen laws

1. **First Light proves self-expansion, not mathematical fame.**
2. **The canonical proof runs locally on ordinary CPU hardware with no model/GPU/network dependency.**
3. **Search and independent checking are separate code/process boundaries.**
4. **Three different CandidateSpace styles are used so no data structure becomes constitutional by accident.**
5. **The full growth loop is proven on FL-C: discover -> certify -> promote -> realize -> validate -> reuse without rediscovery.**
6. **Blindness is bound by package/grammar/Universe digests and sealed evaluation, not by informal claims.**
7. **Authority artifacts use exact canonical content identity; search/control state does not define truth.**
8. **A failed native realization cannot invalidate admitted mathematics.**
9. **Resource exhaustion never downgrades the Authority Contract.**
10. **No advanced infrastructure is admitted into First Light unless a D1/D2 obligation cannot be proven without it.**
