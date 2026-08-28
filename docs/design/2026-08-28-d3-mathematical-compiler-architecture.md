# D3 — Mathematical Compiler Architecture

**Date:** 2026-08-28  
**Status:** FROZEN DESIGN MILESTONE D3  
**Repository name:** temporary only; not product identity  
**Authority:** D1/D1A define mathematics and authority; D2 defines the Core System.

D3 defines how a mathematical target becomes an executable, adaptive mathematical campaign. It supersedes the milestone label of the earlier `2026-08-28-d3-first-light-build-architecture.md`; that earlier file is preserved as a First-Light precursor and is subsumed by the canonical First-Light specification.

D3 does not define machine-code lowering (D4) or promotion/evolution mechanics beyond the D1/D2 interfaces (D5).

---

## 1. Compiler identity

The Mathematical Compiler compiles:

```text
incomplete mathematical world + target + observer + authority contract + resource contract
```

into:

```text
an evolving mathematical attack graph whose obligations can be executed,
refined, certified, recompiled, and eventually closed.
```

It is not a conventional syntax-to-code compiler and not a solver dispatcher.

The core question is:

> **Which mathematically valid changes of representation, reduction, decomposition, abstraction, proof strategy, or newly invented structure can establish the requested observer condition most economically under the authority contract?**

---

## 2. Query IR

Canonical conceptual query:

```text
QueryIR {
    universe_generation
    world
    known_entities / bindings
    metavariables / unknown artifact classes
    target_judgements / target relation
    observer
    authority_contract
    resource_contract
    admissible side-effect policy
}
```

Unknowns may be:

```text
value
Entity
Relation
representation
transformation
reduction
morphism
invariant
lemma/theorem
algorithm
proof/certificate
primitive
metaprimitive
```

The compiler therefore searches over mathematics as well as values.

---

## 3. Semantic elaboration

The front end resolves query notation into D1 semantic artifacts and open obligations.

Responsibilities:

```text
resolve structural identities
resolve parents/domains
resolve or branch canonical morphisms
recover admitted structure witnesses
bind World/foundation
normalize observer semantics
normalize Authority Contract
create explicit open structure obligations
```

Example:

```text
human/request form:
    solve A x = b

elaborated semantics:
    A : Matrix(K,m,n)
    x : Vector(K,n)
    b : Vector(K,m)
    target Relation MatrixVectorProduct(A,x,b)

possible open obligations:
    Field(K)
    rank/invertibility properties
```

The front end does not assume inversion is the algorithm.

---

## 4. Relevant-Universe Retrieval

D3 queries D2 indexes to materialize only the mathematical region relevant to the target.

Retrieval sources include:

- structural/term indexes;
- structure capability closure;
- theorem/premise indexes;
- theory morphisms/reductions;
- observer-equivalent summaries;
- representation relationships;
- prior certified campaigns;
- package activation graph.

Output:

```text
RelevantRegion {
    exact Universe/World binding
    selected semantic artifacts
    candidate capabilities
    bridge/reduction graph
    retrieval provenance
}
```

Retrieval is expandable. A newly proved structure witness or morphism may make previously irrelevant mathematics reachable, causing a partial compiler re-run.

---

## 5. Theory Profile compilation

The compiler requests a D2 Theory Profile for the relevant region.

The profile determines which transformations are mathematically admissible and which search families are likely useful.

Examples:

```text
finite + exhaustive feasible
Presburger-decidable
real-closed-field fragment
polynomial ideal / Groebner route
finite-variant rewrite theory
convex optimization
bounded-treewidth decomposition
symmetry group available
holonomic representation
streamable expression
sound abstraction available
native certificate family available
```

Certified properties constrain correctness. Operational estimates only influence routing.

The compiler may emit a high-value structure obligation when proving one property would unlock substantial capability.

---

## 6. Observer/Sufficiency Compiler

Before solving the entire mathematical object, D3 determines what the requested observer can distinguish.

It searches for a certified summary/quotient:

```text
Summary : X -> S
```

such that:

```text
Observer(X) = ObserverFromSummary(Summary(X))
```

or a sound one-way preservation appropriate to the requested Authority Contract.

This stage can invoke:

- behavioral quotienting/bisimulation;
- minimal-state construction;
- black-box boundary extraction;
- sufficient-statistic/summary discovery;
- symmetry quotienting;
- interface forgetting/projection;
- abstract interpretation.

The goal is not always to preserve every detail. It is to preserve exactly the semantics required by the declared observer.

---

## 7. Representation Frontier

D3 maintains a graph of alternate representations of the target problem.

```text
RepresentationNode {
    semantic target/observer relation
    representation Entity
    World
    exactness/soundness class
    size/structure metadata
}

RepresentationEdge {
    transformation/morphism
    preservation contract
    information loss
    inverse/reconstruction route
    certificate route
    cost envelope
}
```

Possible nodes may be symbolic expressions, ideals, automata, matrices, factor graphs, SAT/SMT, decision diagrams, arithmetic circuits, recurrences, tensor networks, interval domains, spectral coordinates, or future structures.

No representation is preferred constitutionally.

Representation invention is allowed as a D1 Discovery obligation.

---

## 8. Reduction Closure

A `ReductionEdge` transforms one problem family into another while declaring exactly which semantics are preserved.

```text
ReductionEdge {
    source semantic class
    target semantic class
    encode relation
    decode/reconstruct relation
    preserves {
        decision?
        witness?
        count?
        optimum?
        approximation?
        parameter?
    }
    assumptions
    evidence
    cost/resource bounds
}
```

Reduction paths compose only when preservation contracts compose.

A decision-preserving reduction cannot automatically serve a counting query. A lossy abstraction cannot reconstruct a full witness unless an independent reconstruction theorem exists.

The compiler searches reduction paths because one certified edge can unlock an entire mature target-theory capability family.

---

## 9. Decomposition Compiler

D3 searches for decompositions that expose independent or weakly coupled structure.

Examples:

- connected components;
- graph separators/tree decompositions;
- factor/tensor decompositions;
- algebraic factorization;
- block diagonalization;
- conditional independence;
- symmetry orbits;
- recursive substructure;
- variable elimination orders;
- independent modular images.

A decomposition produces explicit assembly semantics.

```text
Decomposition {
    parent goal
    child obligations
    separator/interface
    aggregation semantics
    reconstruction relation
    evidence
}
```

Decomposition is a first-class discovery target because exposing independence may change complexity exponentially.

---

## 10. Campaign IR

The compiled campaign is an evolving typed AND/OR hypergraph.

Canonical node families:

```text
Goal
Route
Obligation
WorldRef
CandidateSpaceRef
FactRef
ArtifactRef
WorkCellPlan
CertificatePlan
Checkpoint
Result
```

Core edges:

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
SPECIALIZES
SUPPORTS
```

Core aggregation semantics:

```text
AND
OR
MIN
MAX
SUM/PRODUCT where mathematically declared
LEAST_FIXPOINT
GREATEST_FIXPOINT
package-defined certified aggregation
```

Campaign IR is not authority. It is a compiled search/execution plan bound to exact authority inputs.

---

## 11. Obligation IR

The fundamental unit of mathematical work is an Obligation, not a process/agent.

```text
Obligation {
    obligation_digest
    universe_generation
    world
    semantic prerequisites
    target artifact/judgement family
    observer
    required authority
    admissible capabilities
    candidate-space contract
    dependencies
    budget
    stop/terminal states
}
```

Terminal states include:

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

The distinction prevents a timeout from masquerading as a mathematical negative result.

---

## 12. CandidateSpace ABI

Discovery/search operates on symbolic mathematical families through a backend-neutral ABI.

Required conceptual operations:

```text
restrict
intersect
union
project
compose
quotient
partition
refine
subtract_nogood
empty
extract
count_or_bound_when_supported
generalize_when_supported
serialize_checkpoint
```

Each operation declares:

```text
EXACT
SOUND_OVER_APPROXIMATION
SOUND_UNDER_APPROXIMATION
HEURISTIC_PROPOSAL
```

and, where applicable, completeness/termination conditions.

Possible backends include affine coefficient spaces, e-graphs, VSA/FTA/ECTA, BDD/ZDD, decision-DNNF, automata, polyhedra, ideals, constraint systems, symbolic grammars, and future structures.

A campaign can switch CandidateSpace backend through certified representation edges rather than treating one structure as universal.

---

## 13. Discovery Compilation

The Discovery Compiler emits obligations at five semantic levels:

```text
L0 value
L1 construction/algorithm/relation
L2 representation/reduction/invariant/decomposition
L3 theory/generalization/new vocabulary
L4 metamathematical/search-method
```

Before enumeration it asks whether the domain admits stronger structural operations:

```text
finite basis?
completion/canonicalization?
symmetry quotient?
exact invariant ideal?
finite-variant inversion?
sound abstraction/refinement?
behavioral quotient?
parameterized kernel?
homomorphic-image computation?
```

Where available, those operations transform the candidate space itself.

---

## 14. Bidirectional/propagation compilation

A semantic Relation may expose multiple directional realizations:

```text
forward evaluator
inverse solver
partial solver
constraint propagator
interval contractor
modular propagator
enumerator
synthesizer
```

D3 chooses/query-specializes direction based on known/unknown ports and Theory Profile.

The semantic relation remains authoritative; direction-specific execution is a realization/planning decision.

D3 never assumes every relation is efficiently invertible or that a general relational interpreter will terminate.

---

## 15. Shared-fact propagation

D3 compiles fact-exchange routes among cooperating specialist obligations.

Example:

```text
interval engine -> x in [2,3]
congruence engine -> x = 1 mod 2
linear engine -> x+y <= 7
```

Bridge contracts determine whether a fact may be consumed and with what polarity.

Propagation iterates to a fixed point where appropriate, but the compiler distinguishes monotone derived fact closure from contextual canonicalization that may collapse representations.

---

## 16. Failure Compilation

A failed candidate is not enough. D3 attempts to compile failure into reusable restrictions.

Possible artifacts:

```text
counterexample
unsat core
MUS/MCS
Craig interpolant
Farkas certificate support
Nullstellensatz/Positivstellensatz witness
conflict clause/nogood
impossibility theorem
failed representation precondition
```

The failure artifact is applied to whole CandidateSpaces when its semantics permit.

A certified failure may also become a promotion candidate for future pruning knowledge under D5.

---

## 17. Inductive/recursive proof compilation

When direct proof fails on recursive/infinite structure, D3 may generate obligations for:

```text
induction-scheme selection
statement strengthening/generalization
auxiliary lemma invention
invariant synthesis
ranking/progress witness
coinductive bisimulation
least/greatest fixed-point reasoning
```

Proof search may therefore invent stronger mathematics than the user requested if it is necessary to establish the target.

---

## 18. Search Economy

Search scheduling is mathematically informed but non-authoritative.

D3 combines several signals:

```text
root-goal gating impact
proof/disproof work estimates
candidate-space elimination/refinement value
information/discrimination gain
structure/capability unlock value
reduction/morphism multiplier value
primitive/generalization value
certificate cost
execution cost
historical solver/route performance
novelty/under-exploration
fairness/age
```

No universal scalar objective is frozen.

Scheduler layers:

```text
Theory/Structure Router
AND/OR Root Propagator
Value-of-Computation allocator
Discrimination/Experiment selector
Portfolio adaptation
Ephemeral activity/relevance
Fairness/non-starvation floor
```

Durable mathematical progress and ephemeral search-control score are never conflated.

---

## 19. Certified progress

Where a domain defines a mathematical information order or certified bounds, D3 may use domain-native progress evidence.

Examples:

```text
shrinking exact interval enclosure
shrinking certified optimality gap
stronger abstract invariant
fixed-point progress measure
smaller exact CandidateSpace
proved branch closure
```

There is no universal `progress = 73%` field.

Progress semantics are Theory Profile capabilities.

---

## 20. Work Cell compilation

D3 converts runnable obligations into WorkCellPlans.

```text
WorkCellPlan {
    obligation digest
    exact semantic inputs
    allowed packages/capabilities
    candidate-space slice
    evidence obligation
    resource budget
    deterministic seed/replay key
    checkpoint policy
    side-effect limits
    stop conditions
}
```

A Work Cell may be a native algorithm, solver, theorem prover, model, GPU search, enumerator, or nested campaign.

It cannot change Universe authority or weaken the root Authority Contract.

Ptah may eventually execute these plans but is not part of D3 semantics.

---

## 21. Event-driven partial recompilation

The compiler is incremental.

Events that may trigger affected-region recompilation include:

```text
new structure witness
new theorem/lemma
counterexample/nogood
candidate-space split
new representation
new reduction/morphism
new decomposition
new certified bound
resource exhaustion
package quarantine
```

Unchanged campaign subgraphs retain identity where their semantic inputs are unchanged.

A restart can discard heuristic/frontier state without losing certified mathematical discoveries.

---

## 22. Replay contract

Every Campaign manifest binds:

```text
Universe generation
World
QueryIR digest
activated package set
Theory Profile semantic inputs
candidate grammar/space identities
compiler policy/version
scheduler policy/version
random seed/key derivation
resource contract
```

Adaptive/random exploration may be reproducible through counter-based deterministic randomness. Mathematical verdict reproducibility is mandatory; byte-identical parallel timing is not.

---

## 23. Compiler output

D3 can terminate with a Result Bundle containing:

```text
observer result
Evidence/certificate references
World/assumptions
open/unresolved obligations
certified bounds/partial information
counterexamples/nogoods
new candidate mathematics
promotion candidates
execution/campaign provenance
```

A hard problem may therefore return a rigorous bound or impossibility result rather than a fabricated exact answer.

---

## 24. D3 proof obligations

```text
D3-P01 QueryIR preserves exact World/Authority/Observer semantics
D3-P02 semantic elaboration never inserts lossy implicit morphisms
D3-P03 representation edges declare preservation/information-loss semantics
D3-P04 reduction composition preserves only explicitly proved result classes
D3-P05 decomposition includes reconstruction/aggregation semantics
D3-P06 CandidateSpace polarity/completeness is preserved across operations
D3-P07 failures can prune only within certified applicability scope
D3-P08 heuristic profile/search scores cannot discharge Judgements
D3-P09 Work Cells cannot modify authority or weaken Authority Contract
D3-P10 event recompilation preserves unaffected semantic identities
D3-P11 replay manifest binds all semantic and policy inputs needed for verdict reproduction
D3-P12 `UNKNOWN`, `REFUTED`, and `RESOURCE_BOUNDED_UNKNOWN` remain distinct
D3-P13 Ptah absence does not change campaign meaning
```

---

## 25. Deferred from D3

D3 does not freeze:

- one CandidateSpace backend;
- one e-graph/hypergraph implementation;
- one scheduler heuristic;
- one solver portfolio;
- final distributed scheduler;
- final GPU strategy;
- final natural-language formalizer;
- final proof system.

Those remain implementation/package choices subject to D1–D3 contracts.

---

## 26. D3 frozen laws

1. **Queries are incomplete mathematical worlds plus targets, not operation names.**
2. **Unknowns may be mathematics themselves.**
3. **Observer/authority/resource semantics are explicit before route selection.**
4. **Representation, reduction, sufficiency, and decomposition are first-class compilation stages.**
5. **Campaigns are evolving typed AND/OR mathematical hypergraphs.**
6. **Obligations carry mathematical meaning; Work Cells merely execute them.**
7. **Discovery operates over symbolic candidate spaces before enumeration whenever possible.**
8. **Failure should become reusable mathematical restriction when certifiable.**
9. **Search control is adaptive but non-authoritative.**
10. **Discoveries can trigger partial recompilation and may escalate to theory/metatheory under bounded policy.**
11. **Search never promotes its own output.**

D3 is complete when D4 can consume its realization plans and First Light can prove its query-to-campaign behavior locally.