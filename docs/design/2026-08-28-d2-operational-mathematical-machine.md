# D2 — Operational Mathematical Machine

**Date:** 2026-08-28  
**Status:** FROZEN DESIGN MILESTONE D2  
**Repository name:** temporary only; not product identity  

D2 operationalizes D1 without changing its constitution. D1 remains the authority for mathematical identity, durable artifact classes, authority, promotion, and the separation between mathematics, derived views, and search control.

D2 freezes the machine contracts required to turn D1 into an executable problem-solving system. It does **not** freeze the implementation language, storage engine, compiler backend, proof assistant, exact solver set, distributed runtime, GPU stack, product name, or full implementation roadmap.

The research authority remains `docs/research/`. New research is required only when a later design choice is unsupported, contradicted, or materially under-specified by the preserved evidence.

---

# 1. Operational identity

D1 defines the mathematical authority cycle:

```text
U_g
  -> target
  -> search / reduce / invent
  -> candidate
  -> certify
  -> promote
  -> U_(g+1)
  -> realize
```

D2 defines the operational machine that executes that cycle:

```text
QUERY
  -> semantic elaboration
  -> capability resolution
  -> theory profile
  -> candidate-space / route construction
  -> campaign IR
  -> work-cell formations
  -> shared mathematical facts
  -> search / propagation / execution
  -> evidence
  -> result
  -> optional promotion transaction
```

The operational machine has one governing rule:

> **Control decisions may affect where compute is spent. They may never alter mathematical authority.**

---

# 2. Package Contract

The project is federated. A mathematical package is any independently versioned body of semantics, capabilities, inference rules, realizations, certificate support, or bridges that can participate in the machine.

Examples may include algebraic packages, theorem libraries, SAT/SMT backends, FLINT-style arithmetic, graph algorithms, proof assistants, numerical engines, imported GitHub algorithms, or future packages invented by the project.

A Package Contract contains at least:

```text
PackageContract {
    package_identity
    exact_version_digest
    foundation / theory identity
    semantic exports
    structure / property implications
    capabilities
    realizations
    certificate families / checkers
    morphisms / reductions / bridges
    dependencies
    interference surface
    composition claims
    resource metadata
    trust / authority status
}
```

A package may expose candidates or heuristics without those becoming authority.

Package states are operationally distinct:

```text
DISCOVERED
    known to the system, not admitted

CANDIDATE
    semantic adapter/contracts under evaluation

ADMITTED
    semantics/evidence accepted into a Universe generation

ACTIVATED
    capabilities available to the compiler automatically

QUARANTINED
    conflicts/composition obligations unresolved

SUPERSEDED
    newer generation/package relation preferred, history retained
```

A package's local correctness does not imply safe composition with every other package. Activation therefore depends on declared or proved combination/interference contracts.

---

# 3. Capability Contract

A Capability is an admitted way to establish, transform, compute, search, certify, or realize mathematics.

A Capability is not an implementation method name. It is a semantic contract.

```text
CapabilityContract {
    capability_identity
    semantic_relation / target Judgement family
    accepted query directions
    required parents / structures / properties
    required World conditions
    produced semantic artifacts / facts
    soundness class
    completeness class
    termination / finiteness conditions
    exactness / approximation semantics
    observer preservation
    evidence route
    candidate-space effects
    cost/resource envelope
    realization families
    fallback/escalation behavior
}
```

Examples:

```text
factor polynomial over a certified field
prove SAT unsatisfiability with LRAT
contract an interval relation soundly
find a graph canonical representative
search an invariant in polynomial language L
transport theorem along certified morphism f
compile relation R in forward integer specialization
```

The capability resolver answers **whether a capability is mathematically admissible**.

The dispatcher later answers **which admissible capability should be used**.

These questions must never be collapsed.

---

# 4. Structure Goal IR and Capability Resolver

Mathematical applicability is proof search over certified structure facts, not string/category matching.

The machine therefore has a small operational Structure Goal IR with goals such as:

```text
HasStructure(D, Field)
HasProperty(A, Invertible)
MorphismPreserves(f, RingStructure)
CommonParent(A, B, C)
WellFormed(Construction)
Applicable(Capability, QueryContext)
```

Goal results are richer than Boolean:

```text
PROVEN_UNIQUE
PROVEN_MULTIPLE / AMBIGUOUS
REFUTED
UNKNOWN
UNDEFINED_UNDER_CURRENT_LOGIC
RESOURCE_BOUNDED_UNKNOWN
```

Multiple mathematical structures may legitimately exist on one carrier. Implicit resolution is permitted only when the result is canonical or all valid alternatives are certified equivalent for the requested observer.

Otherwise the compiler must branch explicitly or require an explicit semantic choice.

---

# 5. Generation-Scoped Capability Closure

Within one accepted Universe generation, admitted truth is immutable and capability inference may exploit monotonicity aggressively.

Operational architecture:

```text
DURABLE ACCEPTED THEORY/PACKAGE GRAPH
        |
        +-- explicit certified structure facts
        +-- certified implication rules
        |
        v
GENERATION-SCOPED CAPABILITY ENGINE
        |
        +-- eager/semi-naive common closure
        +-- lazy tabled goal resolution
        +-- canonical/subsumptive goal caches
        |
        v
COMPILED CAPABILITY GRAPH
        |
        v
PRIMITIVE APPLICABILITY / CAMPAIGN COMPILER
```

Candidate/assumption Worlds may maintain differential/incremental overlays over the accepted base.

Cache identity binds at least to:

```text
Universe generation digest
World digest
structure-rule/package-set digest
canonical goal digest
authority policy digest
```

A new accepted generation produces a new closure context. Historical caches may be discarded without deleting mathematical evidence.

Materialization is an optimization choice:

```text
DERIVABLE != MATERIALIZED
```

Common/high-value capabilities may be eagerly indexed; rare expensive capability goals may remain lazy and tabled.

---

# 6. Theory Profile Contract

A Theory Profile is a generation- and World-specific operational description of the mathematical region relevant to a query.

It is a **derived artifact**, not a new source of truth.

The profile separates certified properties from heuristic/empirical routing features.

```text
TheoryProfile {
    universe_generation
    world
    semantic_region_digest

    certified_properties {
        computability / decidability class
        algebraic structures
        finiteness / cardinality facts
        termination / confluence / coherence
        canonical-form availability
        finite-variant / finite-basis properties
        symmetry / invariant facts
        decomposition parameters
        exact representation classes
        abstraction / concretization contracts
        available certificate families
        resource bounds where certified
    }

    operational_features {
        sparsity
        observed branching factor
        historical solver performance
        estimated treewidth/rank/dimension
        numerical conditioning estimates
        candidate-space size estimates
        expected certificate cost
    }
}
```

Heuristic profile features can influence scheduling but cannot satisfy mathematical obligations.

A Theory Profile may itself expose open structure obligations whose proof has high unlock value.

---

# 7. CandidateSpace Contract

A CandidateSpace represents a mathematical family of possible values, constructions, relations, proofs, representations, reductions, theories, or metaprimitives without enumerating every member.

Its authority is extensional: it denotes a family under declared semantics. Its concrete backend is replaceable.

```text
CandidateSpace {
    space_identity
    Universe generation
    World
    semantic family / grammar
    parents / domains
    constraints
    equivalence / disequality theory
    entanglement / shared-substructure constraints
    evidence already incorporated
    backend representation
    soundness/completeness contract
    ranking/cost annotations
}
```

Supported semantic operations include:

```text
restrict(S, constraint)
intersect(S1, S2)
union(S1, S2)
project(S, observer)
compose(S1, S2)
quotient(S, equivalence)
partition(S, discriminator)
refine(S, evidence)
remove_nogood(S, witness)
empty?(S)
extract(S, objective)
count/bound(S) when supported
generalize(S, theory-bounded contract)
```

A backend may be an e-graph, FTA, ECTA, VSA, BDD/ZDD, decision-DNNF, automaton, polyhedron, ideal/basis representation, constraint system, or future structure.

No backend is constitutional.

Every CandidateSpace operation declares whether its result is:

```text
EXACT
SOUND_OVER_APPROXIMATION
SOUND_UNDER_APPROXIMATION
HEURISTIC_PROPOSAL
```

and which conclusions that polarity supports.

Evidence should transform entire CandidateSpaces whenever possible rather than reject candidates individually.

---

# 8. Campaign IR

A Campaign is an evolving mathematical AND/OR hypergraph compiled from a Query.

The Campaign IR has these node families:

```text
Goal
Obligation
Route
World
CandidateSpace
Artifact
Fact
WorkCell
Checkpoint
Result
```

And edge families such as:

```text
REQUIRES
PRODUCES
SATISFIES
REFUTES
ALTERNATIVE_TO
DECOMPOSES_INTO
REDUCES_TO
TRANSPORTS_TO
UNLOCKS
INVALIDATES
SUPPORTS
SPECIALIZES
```

Goal composition may use:

```text
AND
OR
MIN / MAX
SUM / PRODUCT
LEAST / GREATEST FIXPOINT
other package-defined aggregation with certified semantics
```

Campaign state is not mathematical authority.

Operational obligation states include:

```text
OPEN
RUNNABLE
ACTIVE
SATISFIED
REFUTED
BLOCKED
SUPERSEDED
RESOURCE_BOUNDED_UNKNOWN
SEMANTIC_UNKNOWN
```

A discovery/proof/refutation may partially recompile the Campaign IR rather than restart the whole campaign.

---

# 9. Work Cell Contract

A Work Cell is the bounded execution unit derived from one mathematical obligation.

It is the operational analogue of a Sergeant private, but it need not be an AI agent.

```text
WorkCellContract {
    cell_identity
    campaign_identity
    Universe generation
    World
    obligation
    exact input digests
    allowed capabilities / packages
    allowed speculative operations
    required output classes
    evidence obligation
    resource budget
    checkpoint policy
    deterministic seed / replay binding
    stop conditions
}
```

A Work Cell may execute:

- a deterministic algorithm;
- SAT/SMT/CAS/MILP;
- e-graph search;
- exact enumeration;
- GPU exploration;
- theorem proving;
- model-generated candidate search;
- a nested campaign;
- any future package satisfying the contract.

A Work Cell cannot:

```text
promote mathematics
change Universe authority
weaken the root Authority Contract
silently expand its scope
issue the final mathematical verdict unless its obligation is itself the certified verdict artifact
```

Outputs are content-addressed artifacts, candidate facts, checkpoints, evidence, or explicit failure/unknown reports.

---

# 10. Shared Mathematical Fact Fabric

Specialist engines should cooperate by exchanging compact mathematical facts, not by merging implementations.

The Shared Fact Fabric may carry:

```text
exact equality / disequality
exact finite-domain restrictions
sound intervals
lower / upper bounds
congruences
linear / polyhedral constraints
algebraic summaries
probability bounds
rank / spectral bounds
candidate nogoods
other package-defined summaries
```

Every fact declares:

```text
semantic domain
World
polarity / information meaning
precision/order relation
producer
certificate/provenance
freshness
supported consumers / bridge types
```

Required polarity classes include at least:

```text
EXACT
OVER_APPROXIMATION
UNDER_APPROXIMATION
LOWER_BOUND
UPPER_BOUND
NECESSARY_CONDITION
SUFFICIENT_CONDITION
HEURISTIC_CANDIDATE
```

An over-approximation cannot become an existence witness merely because it is precise.

Cross-domain exchange requires an explicit Bridge Contract:

```text
BridgeContract(A, B) {
    projection_A_to_B
    projection_B_to_A
    soundness evidence
    fact classes
    information order
    termination/fairness metadata
    combination strength
}
```

Combination strength is explicit:

```text
COMPLETE_COMBINATION
SOUND_COOPERATIVE_REDUCTION
HEURISTIC_PROPOSAL_ONLY
UNSUPPORTED
```

Federation never assumes that two complete solvers remain complete when combined.

---

# 11. Search Economy

The search economy allocates finite compute to mathematical work. It is not authority and is allowed to be imperfect.

The scheduler combines several independent signals rather than one universal score:

```text
root-goal impact
proof/disproof work estimate
expected CandidateSpace reduction
expected information refinement
counterexample probability/value
capability unlock value
reduction/morphism multiplier value
primitive/generalization potential
certificate/checker cost
execution/resource cost
historical route performance
novelty / under-exploration
fairness / age
```

No one scalar is constitutional. Packages may contribute domain-specific estimates.

Operational scheduler layers:

```text
1. THEORY/STRUCTURE ROUTER
   choose admissible formations and representations

2. AND/OR ROOT PROPAGATOR
   compute which unresolved obligations currently gate the root

3. VALUE-OF-COMPUTATION LAYER
   prioritize mathematically valuable work relative to cost

4. DISCRIMINATION / INFORMATION LAYER
   prefer experiments/tests that divide candidate spaces strongly

5. PORTFOLIO / ADAPTATION LAYER
   learn which admissible strategies work on this problem region

6. DYNAMIC ACTIVITY LAYER
   short-lived conflict/relevance scores

7. FAIRNESS / NON-STARVATION FLOOR
   preserve bounded exploration of non-dominant valid routes
```

The scheduler operates at multiple timescales. Expensive global reprioritization must not run for every low-level operation.

Restarts may discard:

```text
frontier state
activity scores
local heuristic choices
```

while preserving:

```text
certified theorems
counterexamples
nogoods
certificates
promotable discoveries
```

Adaptive exploration may use deterministic counter-based randomness bound to campaign/cell/decision identities, preserving replayability of policy decisions without requiring byte-identical parallel timing.

---

# 12. Federation Adapter Contract

Every external specialist enters through an adapter with exact version and semantic scope.

```text
FederationAdapter {
    package/version digest
    accepted input semantic forms
    output semantic forms
    translation mappings
    assumptions / preconditions
    supported query directions
    result classes
    certificate output
    checker / verifier route
    side-effect contract
    determinism / randomness contract
    resource characteristics
}
```

There are three adapter authority modes:

```text
CERTIFIED_TRANSLATION
    semantic encode/decode bridge itself is proved/certified

CHECKED_RESULT
    translation may be untrusted but returned certificate is checked against the original Judgement

CANDIDATE_ONLY
    result may guide search but cannot enter authority until separately certified
```

Source code/GitHub algorithms enter as Candidate-only semantic artifacts unless an independent lifting/proof route establishes stronger authority.

---

# 13. Certificate Envelope Operational Contract

D1 defines Evidence. D2 freezes the operational envelope fields required for routing and replay.

```text
CertificateEnvelope {
    envelope_version
    evidence_identity
    target_judgement_digest
    Universe_generation
    World_digest
    semantic_scope
    result/outcome class
    verification_mode
    certificate_family
    certificate_family_version
    certificate_body_digest
    producer_package/version
    checker_package/version
    checker_trust_root
    checker_verdict
    input/dependency digests
    assumptions
    randomness/interaction transcript binding where relevant
    replay recipe
    resource summary
    freshness / supersession lineage
}
```

Certificate bodies remain domain-native.

Large certificates may be streamed or stored out-of-core; the envelope binds their exact content digest.

Authority Contract matching is explicit. A result is accepted for a Query only if the envelope's semantics satisfy the requested authority, scope, and observer requirements.

A probabilistic certificate does not satisfy a deterministic proof requirement unless a later deterministic/foundational route upgrades it.

---

# 14. Realization Planning

A semantic capability may have several possible executable realizations.

The Realization Planner builds a plan after mathematical applicability is established.

```text
RealizationPlan {
    semantic target
    specialization identity
    query direction
    fixed assumptions / World
    input/output representations
    required numeric semantics
    admissible realization candidates
    realization preconditions
    expected cost/resource envelope
    authority/checking route
    fallback/escalation ladder
}
```

Selection policy is CPU-first by default, not CPU-only.

The planner should prefer the cheapest sound route:

```text
single CPU primitive
SIMD
multicore
optimized native library
streaming/out-of-core
incremental updater
GPU
multi-machine campaign
```

according to the mathematical workload.

GPU power is used only when the workload structure benefits from it.

---

# 15. Certified Escalation Ladder

Exact authority does not require worst-case exact arithmetic on every input.

A RealizationPlan may contain a certified escalation ladder:

```text
cheap sound filter
    -> decisive? return
    -> ambiguous? escalate

stronger interval / mixed-precision / modular image
    -> decisive? return
    -> ambiguous? escalate

arbitrary precision / exact reconstruction
    -> exact authority path
```

The invariant is:

> **Ambiguity escalates. Only mathematically decisive evidence returns.**

The ladder policy may be performance-tuned independently of the semantic guarantee.

Examples of admissible techniques include:

```text
filtered floating-point predicates
rigorous interval arithmetic
adaptive precision
modular images + CRT/rational reconstruction
lazy exact DAGs
homomorphic image computation
```

provided the escalation contract preserves the requested semantics.

---

# 16. Specialization and Residual Primitive Generation

General relational semantics are not required to remain in the hot path.

A frequently used semantic relation may be specialized by fixing:

```text
parent/theory
World/assumption class
query direction
representation
observer
numeric semantics
```

producing a Specialization identity distinct from both semantic and realization identities.

```text
SEMANTIC IDENTITY
    what mathematics means

SPECIALIZATION IDENTITY
    fixed direction/context/assumptions

REALIZATION IDENTITY
    exact executable implementation
```

Partial evaluation, supercompilation, equality-saturation extraction, proof extraction, code generation, or future methods may produce residual implementations.

The optimizer/generator is not trusted merely because it produced fast code.

Every admitted realization requires realization evidence such as translation validation, equivalence/refinement proof, or an independently checked domain-specific certificate.

---

# 17. Proof Erasure and Hot-Path Rule

Proof is mandatory for admission where the Authority Contract requires it. Proof material is not required in ordinary execution once realization equivalence is independently established.

Therefore:

```text
CERTIFICATION PATH
    semantic proof / certificate
    realization proof / validation

HOT PATH
    stripped native realization
```

The executable keeps content-addressed lineage back to the proof but does not carry theorem-prover overhead through every invocation.

If a realization's semantic preconditions cannot be established at dispatch time, the planner must select another admissible route or return Unknown/unsupported. It may not simply run and hope.

---

# 18. Incremental Execution

Where mathematics admits a valid update operator, the compiler may derive or admit:

```text
F(x)
DeltaF(x, Delta x)
```

so small input changes reuse prior certified work rather than recomputing from zero.

Incremental realizations carry explicit validity conditions binding them to:

```text
base input/result digest
change semantics
Universe generation
World
representation
```

If those conditions fail, incremental execution is invalid and the planner falls back to full recomputation.

Incremental proof/certificate repair is similarly preferred in this order:

```text
reuse unchanged evidence nodes
transport across certified equivalence/morphism
repair affected proof cone
fresh proof search
```

---

# 19. Out-of-Core and Succinct Execution

The runtime must not equate mathematical-set cardinality with RAM usage.

A capability may operate on:

```text
streamed symbolic terms
external sorted runs
BDD/ZDD/automata
short generating functions
polyhedral/oracle representations
arithmetic circuits
factorized representations
```

without materializing the represented set.

Capability/representation contracts declare which operations are streamable, local, closed under the representation, or require materialization.

Ptah may later provide workspace and distributed execution, but D2 keeps all mathematical semantics independent of Ptah.

---

# 20. Result Bundle

A campaign does not return a naked answer.

```text
ResultBundle {
    query_identity
    Universe_generation
    World
    observer_result
    authority/certificate envelopes
    exact assumptions
    unresolved obligations
    certified partial bounds / progress
    counterexamples / nogoods
    discovered candidate mathematics
    promotable artifacts
    execution / campaign provenance
    resource summary
}
```

For ordinary clients the presentation layer may collapse this to a number, formula, yes/no, or concise explanation.

The full result remains available to mathematical clients and future campaigns.

---

# 21. Operational Promotion / Generation Build

Promotion remains outside active campaign execution.

A Generation Builder takes a set of certified eligible artifacts and constructs a candidate next generation.

```text
1. Freeze promotion delta.
2. Verify semantic identity/dependency closure.
3. Recheck certificate envelopes.
4. Detect same-World authority conflicts.
5. Evaluate package/composition/interference obligations.
6. Compute capability-closure delta.
7. Evaluate automatic activation policy.
8. Build/rebuild derived indexes required by the generation.
9. Run negative/adversarial/regression controls for activated capabilities.
10. Bind generation policy + package digests + closure rules.
11. Compute immutable generation root.
12. Publish U_(g+1) atomically.
```

Partial generation publication is forbidden.

A semantic theorem may be admitted without activation.

A metaprimitive or rewrite that changes future search has a stricter activation threshold than an inert theorem.

A new realization may be attached in a later generation without changing the semantic identity it realizes.

---

# 22. First-Light Proof Campaign

First Light must prove the machine's **growth loop**, not attempt an unsolved research theorem.

The campaign is intentionally bounded and blind.

## 22.1 Target family

Use several exact, known mathematical targets whose authoritative definitions/proofs are hidden from the active Discovery Fabric but retained in a sealed evaluation set.

At least three classes are required:

```text
A. exact recurrence / identity discovery
B. representation or reduction discovery
C. synthesized executable construction
```

The examples must be nontrivial enough that simple memorized syntax does not satisfy the campaign, but small enough to run on ordinary CPU hardware.

## 22.2 Blindness rule

The active candidate grammar/package set may contain permitted primitives and general mathematics but must not expose the hidden target relation/primitive itself or an alias trivially equivalent by lookup.

Campaign manifests record:

```text
Universe generation
available package digests
search grammar digest
hidden-target digest held outside active search
```

so rediscovery can later be audited.

## 22.3 Required cycle

For at least one target the machine must demonstrate:

```text
problem/specification
    -> symbolic CandidateSpace
    -> discriminating tests / structural search
    -> candidate construction
    -> deliberate false near-misses rejected
    -> independent certification
    -> semantic admission
    -> capability activation
    -> specialization/native realization
    -> realization validation
    -> second related query solved using promoted primitive
```

The second query must measurably avoid the original discovery campaign.

This is the proof of self-expansion.

## 22.4 Required negative controls

First Light must include:

```text
false formula matching early samples
formula valid only under missing assumption
numerical coincidence
candidate with wrong branch/domain semantics
candidate whose optimized realization disagrees with semantics
ambiguous structure/coercion case
resource exhaustion without authority downgrade
```

The system must refuse promotion or return the correct qualified result in each case.

## 22.5 Required proof markers

First Light is successful only if the run produces machine-checkable evidence for all of:

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

## 22.6 Hardware rule

The canonical First-Light proof must run locally on ordinary CPU hardware.

GPU/distributed acceleration may be demonstrated separately but is not required for the canonical proof.

This ensures the project begins with a strong local capability substrate rather than making large hardware a dependency of ordinary use.

---

# 23. D2 Constitutional Operational Laws

D2 freezes the following laws:

1. **Capability proof and capability dispatch are separate.**
2. **Candidate spaces are semantic families with replaceable compact backends.**
3. **Accepted-generation capability closure is generation-scoped and immutable; candidate worlds may use incremental overlays.**
4. **A Theory Profile separates certified mathematical properties from heuristic routing features.**
5. **Work Cells execute bounded obligations and have no promotion authority.**
6. **Specialist engines cooperate through typed facts and certified bridges, not by assuming shared semantics.**
7. **Search economy is disposable control state and cannot create truth.**
8. **Certificate envelopes bind exact claim, World, dependencies, checker, and replay lineage; certificate bodies stay domain-native.**
9. **Semantic identity, specialization identity, and realization identity are distinct.**
10. **Performance policy may change without changing mathematical authority.**
11. **Ambiguity escalates; it never becomes a guessed exact answer.**
12. **Proof may be erased from the hot path only after realization correctness is independently established.**
13. **Generation publication is atomic.**
14. **First Light must prove discovery -> certification -> promotion -> faster reuse on ordinary CPU hardware.**

---

# 24. What D2 deliberately does not freeze

D2 does not choose:

```text
Rust vs another core implementation language
specific database/storage technology
specific e-graph/FTA/logic engine
specific proof assistant
specific SAT/SMT/MILP solver
LLVM vs MLIR vs other compiler path
specific exact-arithmetic libraries
specific GPU framework
Ptah integration mechanics
public API syntax
human mathematical notation
product name
```

Those are implementation and later-design decisions.

The operational contracts above must survive those choices.

---

# 25. D2 Exit State

With D2 frozen, the project now has:

```text
D1
    mathematical constitution
    identity / truth / worlds / promotion / realization law

D2
    operational contracts
    package/capability model
    theory profiling
    capability closure
    candidate spaces
    Campaign/Work Cell IR
    fact federation
    search economy
    certificate routing
    realization planning
    First-Light proof campaign
```

The next milestone is **D3 — First-Light Build Architecture**.

D3 should select the smallest concrete implementation stack capable of proving D1+D2 without prematurely building the full future system. It should define the exact First-Light target suite, concrete package/checker choices, local persistence format, executable IR subset, build/test/proof layout, and technology decisions necessary for implementation.

A full long-term roadmap should still wait until D3 establishes what the first executable proof actually requires.
