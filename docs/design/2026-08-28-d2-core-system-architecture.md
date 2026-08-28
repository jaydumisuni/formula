# D2 — Core System Architecture

**Date:** 2026-08-28  
**Status:** FROZEN DESIGN MILESTONE D2  
**Repository name:** temporary only; not product identity  
**Authority:** D1 + D1A remain constitutionally superior.

This milestone defines the core system that stores, indexes, composes, checks, and exposes the mathematical authority defined by D1. It normalizes and supersedes the milestone label of the earlier `2026-08-28-d2-operational-mathematical-machine.md`, while preserving that file as design history and evidence.

D2 does **not** define the mathematical compiler/search strategy (D3), native lowering/runtime architecture (D4), self-expansion mechanics (D5), or Ptah integration.

---

## 1. Core-system identity

The Core System is the authority-bearing substrate around immutable Universe generations:

```text
                    U_g
                     |
        +------------+------------+
        |                         |
        v                         v
  durable authority        rebuildable closure
  object/evidence graph     indexes/capabilities
        |                         |
        +------------+------------+
                     |
                     v
              semantic services
                     |
        +------------+------------+
        |                         |
        v                         v
   federation/checking        compiler clients
```

Its governing laws are:

1. durable mathematical authority is immutable inside a generation;
2. semantic identity is structural/content-addressed, never defined by solving arbitrary equivalence;
3. capability closure is derived from admitted mathematics, not stored as truth;
4. packages and solvers are federated capability suppliers, not authorities by identity;
5. certificate routing is heterogeneous and checker-centered;
6. search state never shares the authority store's mutation semantics;
7. every generation transition is an explicit Promotion transaction;
8. self-hosted tooling cannot self-authorize.

---

## 2. Physical storage model vs logical mathematics

D1 defines a logical typed semantic hypergraph. D2 explicitly separates that logical view from physical storage.

The long-term logical object store consists of content-addressed immutable artifacts:

```text
Entity
Relation
World
Judgement
Evidence
Realization
Package manifests
Generation manifests
```

A physical implementation may use SQLite, RocksDB, LMDB, an object database, append-only files, remote object storage, or another backend. None becomes mathematical identity.

The storage system exposes four roles:

```text
ObjectStore
    immutable blobs by structural digest

AuthorityStore
    generation manifests and admission edges

IndexStore
    rebuildable semantic/retrieval indexes

CampaignStore
    non-authoritative campaign/search/checkpoint state
```

These roles may share one physical database in an early implementation, but their semantic mutation rules remain distinct.

---

## 3. Structural identity and canonical encoding

Every durable artifact has a versioned canonical structural encoding. Its digest covers only semantic identity inputs declared by its schema.

Identity rules:

```text
same canonical structure + same referenced semantic digests
    -> same structural digest

different structural digest
    -/-> mathematically different meaning
```

Semantic equivalence is a separately certified Judgement.

Canonical encodings must exclude accidental machine-local state such as:

- temporary filesystem paths;
- wall-clock timestamps;
- scheduler order;
- process IDs;
- non-semantic cache keys;
- hardware timing measurements.

A canonical-encoding version is itself part of generation policy. Migration to a new encoding creates explicit mapping/equivalence evidence; historical digests remain resolvable.

---

## 4. Universe Generation manifest

A Universe generation is an immutable Merkle-style authority root.

Conceptual manifest:

```text
UniverseGeneration {
    schema_version
    parent_generation_digests
    admitted_artifact_digests
    admitted_evidence_edges
    admitted_realization_edges
    package_activation_set
    authority_policy_digest
    foundation/theory roots
    closure-rule-set digest
    generation_digest
}
```

A generation may be represented as a delta over parent generations, but its effective authority is deterministic and replayable.

Generation publication is atomic. Unreachable immutable blobs may remain after a crash; a partially committed generation may never become active authority.

---

## 5. Authority Graph

The Authority Graph records why an admitted claim is usable in a generation.

Nodes include durable artifacts and certificate/checker identities. Edges include:

```text
ESTABLISHES
REFUTES
DEPENDS_ON
CHECKED_BY
TRANSPORTED_BY
REALIZES
SUPERSEDES
REPAIRS
DERIVED_FROM
```

The graph supports precise freshness and invalidation. If evidence `E2` depends on `E1`, changing or superseding `E1` affects only the reachable dependency cone; unrelated authority remains intact.

Historical evidence is never deleted merely because it is no longer active in the latest generation.

---

## 6. World and foundation separation

Worlds are immutable assumption contexts. Foundation/theory identity is explicit.

The Core System therefore resolves authority as:

```text
(U_g, World, Judgement, AuthorityContract)
```

not simply:

```text
Judgement -> true/false
```

Two apparently contradictory Judgements may both be valid when their World/foundation identities differ.

A contradiction under the same classical World/semantics is quarantined and creates a conflict obligation; it cannot silently enter ordinary closure.

---

## 7. Theory Package contract

A Theory Package is an independently versioned unit of mathematical semantics and capability.

```text
TheoryPackage {
    package_digest
    foundation/theory identity
    semantic exports
    structure/property implication rules
    capability contracts
    representations
    realizations
    certificate families/checkers
    morphisms/reductions/bridges
    dependencies
    semantic interface
    interference surface
    composition claims
    resource metadata
}
```

Lifecycle states:

```text
DISCOVERED
CANDIDATE
ADMITTED
ACTIVATED
QUARANTINED
SUPERSEDED
```

`ADMITTED` means its certified semantics may exist in a Universe generation. `ACTIVATED` additionally allows compiler/search automation to use its capabilities without explicit user selection.

Local correctness never implies composition safety.

---

## 8. Package composition and interference

Two safe packages may interact unsafely when they share new vocabulary, rewrite heads, assumptions, mutable operational resources, or semantic commitments.

Every package therefore exposes an interference boundary.

Combination can be classified as:

```text
DISJOINT_SAFE
CERTIFIED_COMBINATION
CONSERVATIVE_EXTENSION
SOUND_COOPERATION
HEURISTIC_ONLY
UNSUPPORTED
QUARANTINED
```

Where a theory morphism, interpretation, conservative extension, interface projection, or other composition proof exists, the exact certificate is bound into the activated package set.

The Core System must never infer `safe union` from two independent `safe package` badges alone.

---

## 9. Semantic interfaces and observer boundaries

A package may expose a smaller certified semantic interface rather than its entire internal theory.

```text
SemanticInterface {
    visible vocabulary
    observer family
    preserved consequences
    abstraction/forgetting map
    reconstruction/transport rules where applicable
    evidence
}
```

This supports uniform-interpolation/forgetting-style projection, black-boxing, sufficient summaries, and stable downstream dependencies.

A package implementation may change internally without invalidating consumers if the declared interface semantics remain certified equivalent.

---

## 10. Structure Goal and mathematical type resolution

Mathematical applicability is resolved through admitted structure witnesses, parents/domains, and morphisms.

Core goal forms include:

```text
HasStructure(D, S)
HasProperty(X, P)
CanonicalMorphism(A, B, f)
MorphismPreserves(f, Structure)
CommonParent(A, B, C)
Applicable(Capability, Context)
```

Results are not forced to Boolean:

```text
PROVEN_UNIQUE
PROVEN_MULTIPLE
AMBIGUOUS
REFUTED
UNKNOWN
UNDEFINED_IN_FOUNDATION
RESOURCE_BOUNDED_UNKNOWN
```

Implicit coercion is permitted only for certified canonical structure-preserving maps, or when all alternatives are proved observer-equivalent.

---

## 11. Generation-scoped Capability Closure

Within one immutable generation, accepted semantic facts form a monotone authority base. Capability closure is a derived view over that base.

```text
accepted semantic facts
       |
       +-- structure implications
       +-- package capability rules
       +-- morphism/reduction availability
       v
Capability Closure Engine
       |
       +-- eager common closure
       +-- lazy/table-goal resolution
       +-- subsumption/canonical caches
       v
Compiled Capability Graph
```

Cache identity includes at least:

```text
Universe generation digest
World digest
activated-package-set digest
closure-rule-set digest
authority-policy digest
canonical goal digest
```

`DERIVABLE` and `MATERIALIZED` are distinct. High-value capabilities may be eagerly indexed; rare expensive goals remain lazy.

A new generation creates a new closure context. Historical closure caches are discardable.

---

## 12. Theory Profile service

The Core System exposes a Theory Profile service to D3. It combines certified semantic facts with clearly separated operational estimates.

```text
TheoryProfile {
    certified {
        computability/decidability class
        structures
        finiteness
        canonical forms
        termination/confluence/coherence
        finite-basis/variant properties
        symmetry/invariants
        decomposition parameters
        abstraction/concretization contracts
        certificate families
        certified resource bounds
    }

    operational {
        sparsity estimates
        branching estimates
        empirical solver performance
        conditioning estimates
        candidate-space estimates
        measured hardware cost
    }
}
```

Operational fields may guide scheduling but can never discharge a mathematical obligation.

---

## 13. Federation fabric

External and internal specialists connect through `FederationAdapter` contracts.

```text
FederationAdapter {
    package/version digest
    semantic input forms
    semantic output forms
    translations
    assumptions
    supported directions
    result classes
    certificate output
    checker route
    side-effect contract
    determinism/randomness contract
    resource characteristics
}
```

Authority modes:

```text
CERTIFIED_TRANSLATION
CHECKED_RESULT
CANDIDATE_ONLY
```

Examples:

- a SAT engine may be untrusted while its LRAT proof is checked;
- a CAS may return a Gröbner certificate checked independently;
- GitHub code enters `CANDIDATE_ONLY` until semantic lifting and certification establish stronger status.

---

## 14. Shared Mathematical Fact Fabric

Specialists cooperate by exchanging certified/sound abstract facts, not by merging implementations.

Fact classes may include:

```text
equality/disequality
finite-domain restrictions
interval enclosures
lower/upper bounds
congruences
linear/polyhedral constraints
algebraic summaries
probability bounds
rank/spectral bounds
nogoods
```

Every shared fact declares its information polarity:

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

A sound over-approximation can never be consumed as an existence witness.

Cross-domain propagation requires a certified Bridge Contract with explicit projection directions, information order, soundness, and combination strength.

---

## 15. Certificate Router

D1 defines the universal Evidence envelope. D2 defines routing.

The router receives:

```text
Target Judgement
World
Authority Contract
Theory Profile
available certificate families
```

and produces one or more admissible verification plans.

Preference order is not globally fixed. The router generally prefers the cheapest route satisfying the requested authority, for example:

```text
exact recomputation
small domain-native certificate
finite exhaustive proof
rigorous enclosure
formal proof kernel
verifiable-computation fallback
```

but domain-specific cost and trust policy decide.

Certificate bodies remain native. The common envelope binds target, world, scope, inputs, producer, checker, trust root, replay data, and exact digests.

Interactive/statistical/cryptographic soundness classes remain distinct.

---

## 16. Checker registry and trust roots

Checkers are themselves versioned Realizations with explicit trust lineage.

```text
CheckerRegistryEntry {
    semantic checker identity
    realization digest
    certificate families
    proof/foundation lineage
    bootstrap/trust-root generation
    supported authority classes
}
```

A checker cannot approve its own successor solely by running itself. D1A's self-hosting law applies.

Critical certificate families should eventually support at least one independent/foundational validation route outside the implementation path being validated.

---

## 17. Search-state separation

The Core System enforces three separate stores/semantics:

```text
DURABLE AUTHORITY
    immutable mathematical artifacts and generation bindings

REBUILDABLE DERIVED STATE
    capability closure, indexes, e-classes, canonical views,
    premise indexes, abstract-domain projections

EPHEMERAL CAMPAIGN STATE
    frontiers, activity scores, priorities, restarts,
    speculative candidate rankings, temporary work queues
```

No API from the ephemeral store may write authority edges.

A campaign may submit a `PromotionCandidate`, but only the Promotion transaction can create the next authority generation.

---

## 18. Promotion transaction service

The Core System owns the atomic generation transition.

```text
1. freeze exact candidate/evidence/dependency digests
2. run Certification outcome checks
3. run package/composition/interference checks
4. validate promotion policy
5. stage immutable artifacts
6. open atomic generation transaction
7. recheck parent generation root
8. compute admission delta
9. compute activated-package/capability delta
10. compute new generation manifest/root
11. commit atomically
12. publish generation and rebuild derived closure lazily/eagerly
```

Failure before commit cannot partially mutate authority.

---

## 19. Change classification and proof freshness

Changes are classified semantically rather than by file diff size:

```text
REALIZATION_ONLY
DEFINITIONAL_EQUIVALENT
CONSERVATIVE_EXTENSION
THEOREM_STRENGTHENING
ASSUMPTION_WEAKENING
SIGNATURE_CHANGE
NON_CONSERVATIVE_CHANGE
AUTHORITY_POLICY_CHANGE
```

The class determines whether downstream evidence may be reused, transported, repaired, or must be re-proved.

Proof/certificate dependency cones are content-addressed so unaffected mathematics remains fresh.

---

## 20. Practical self-host/bootstrap chain

D1A requires a non-circular self-hosting path. D2 freezes the trust roles, not the final seed technology.

```text
B0 — External bootstrap root
    exact source/toolchain/binary digests
    canonical object decoding + hashing
    minimal generation/checker bootstrap manifest

B1 — Minimal authority runtime
    load generation
    verify structural identities
    run minimal certificate families
    verify promotion manifest

B2 — Rich core/checker/compiler runtime
    built from exact sources
    validated by B1 plus independent/diverse path

BN — Ordinary self-hosted generation
    may build successor, never self-authorize it
```

First Light may use a pinned conventional compiler as B0. Long-term trust minimization of B0 is a roadmap item requiring a targeted bootstrap implementation spike, not a blocker for proving D1–D5 semantics locally.

Every critical self-hosted upgrade records:

```text
source digests
toolchain semantic identity
toolchain realization digest
checker identities
bootstrap-stage identity
build policy
target architecture
output digest
independent validation evidence
```

---

## 21. Local/offline authority guarantee

A local installation containing the required generation artifacts/checkers must be able to:

- load an admitted Universe generation;
- resolve capabilities;
- evaluate admitted local realizations;
- check supported certificates;
- run bounded local campaigns;
- produce promotion candidates;
- replay locally available authority evidence.

Cloud infrastructure and Ptah may accelerate future campaigns but are not required to consume already promoted mathematics.

---

## 22. D2 proof obligations

An implementation cannot claim D2 until it proves at least:

```text
D2-P01 structural identity replay is deterministic
D2-P02 generation publication is atomic
D2-P03 semantic equivalence is not conflated with digest identity
D2-P04 capability closure is generation/world scoped
D2-P05 package activation enforces composition/interference contracts
D2-P06 shared-fact polarity is enforced
D2-P07 certificate router cannot downgrade Authority Contract silently
D2-P08 independent checker path is isolated from search producer
D2-P09 ephemeral search APIs cannot write authority
D2-P10 proof freshness invalidates only dependency-reachable authority
D2-P11 historical generations remain replayable
D2-P12 local/offline admitted capability remains usable without Ptah/network
D2-P13 self-host successor cannot authorize itself solely through its own execution
```

---

## 23. Deferred from D2

D2 deliberately does not freeze:

- final storage engine;
- final canonical binary encoding;
- final e-graph/e-hypergraph substrate;
- final proof assistant;
- final bootstrap seed technology;
- final programming language;
- distributed execution;
- Ptah integration;
- GPU stack;
- product/API/UI.

Those choices are subordinate to D1/D2 contracts.

---

## 24. D2 frozen laws

1. **Authority is content-addressed and generation-scoped.**
2. **Physical storage never defines semantic identity.**
3. **Capability closure is derived, rebuildable, and scoped by generation/world/package policy.**
4. **Theory packages are independently versioned semantic contracts with explicit interference surfaces.**
5. **Federated engines provide capability, never authority by identity.**
6. **Shared mathematical facts carry explicit information polarity.**
7. **Certificate bodies remain domain-native behind a common envelope/router.**
8. **Search state, derived views, and durable authority have different mutation laws.**
9. **Promotion is the only authority-generation transition.**
10. **Self-hosting is allowed; self-authorization is forbidden.**
11. **Local consumption of promoted mathematics must not depend on Ptah or a remote proprietary service.**

D2 is complete when these contracts are preserved by D3–D5 and demonstrated by First Light.