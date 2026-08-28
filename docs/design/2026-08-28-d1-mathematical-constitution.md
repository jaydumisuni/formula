# D1 — Mathematical Constitution

**Date:** 2026-08-28  
**Status:** FROZEN DESIGN MILESTONE D1  
**Repository name:** temporary only; not product identity  

This milestone freezes the mathematical constitution of the unnamed project. It does **not** freeze the implementation language, runtime, storage engine, compiler backend, proof assistant, distributed substrate, product name, or full roadmap.

The design is derived from the research authority under `docs/research/`. Research remains subordinate to design after this point: new broad research is not required unless a later design decision is unsupported, contradicted, or materially under-specified by the preserved evidence.

---

# 1. Constitutional identity

The project is a **self-expanding deterministic mathematical problem-solving architecture**.

Its defining growth loop is:

```text
U_g
  -> target
  -> represent / reduce / decompose / search / invent
  -> candidate mathematics
  -> falsify / certify
  -> generalize / compress where justified
  -> promote
  -> U_(g+1)
  -> compile / specialize / realize
  -> stronger and cheaper future problem solving
```

The machine is not constitutionally a CAS, theorem prover, AI mathematician, programming language, solver, or one proof system. Those can all participate as packages or realizations.

The central invariant is:

> **Search may propose mathematics. Only Certification + Promotion can create mathematical authority.**

And:

> **Execution may consume mathematical authority. Execution cannot manufacture mathematical authority.**

And:

> **No representation, implementation, proof language, model, or solver is the mathematics itself.**

---

# 2. Six constitutional planes

The strongest architecture has six distinct planes.

```text
                     HUMAN / AI / DOMAIN SYSTEM
                              |
                              v
                    PROBLEM COMPILER
                              |
                              v
+-------------------------------------------------------------+
| 1. MATHEMATICAL UNIVERSE                                   |
| admitted semantics, worlds, judgements, evidence,           |
| capabilities and realizations                              |
+----------------------------+--------------------------------+
                             |
          +------------------+------------------+
          |                  |                  |
          v                  v                  v
+------------------+ +------------------+ +--------------------+
| 2. DISCOVERY     | | 3. CERTIFICATION| | 4. EXECUTION /     |
| SEARCH FABRIC    | | FABRIC           | | COMPILATION        |
| representations  | | domain-native    | | specialization     |
| reductions       | | certificates     | | native CPU/SIMD    |
| synthesis        | | independent      | | optional GPU       |
| theory formation | | checkers         | | streaming/increment|
+---------+--------+ +---------+--------+ +----------+---------+
          |                    |                     |
          +--------------------+---------------------+
                               |
                               v
+-------------------------------------------------------------+
| 5. PROMOTION & EVOLUTION                                   |
| candidate -> certified semantic artifact -> admission       |
| -> optional capability activation -> new U_(g+1)            |
+-------------------------------------------------------------+

+-------------------------------------------------------------+
| 6. FEDERATION FABRIC                                       |
| CAS / SAT / SMT / MILP / proof assistants / FLINT / GAP /   |
| graph engines / numerical systems / GitHub algorithms /     |
| future mathematical packages                               |
+-------------------------------------------------------------+
```

The federation plane is a capability supply system, not mathematical authority.

---

# 3. Mathematical Universe generations

The active admitted mathematical universe is versioned by immutable generations:

```text
U_0 -> U_1 -> ... -> U_g
```

A generation is a Merkle/content-addressed authority manifest over admitted semantic artifacts and authority bindings. A promotion creates a new generation; it never mutates the old one.

Conceptually:

```text
U_(g+1) = Promote(U_g, Delta, Evidence, Policy)
```

Historical generations remain reproducible.

A generation contains durable mathematical authority. It explicitly excludes temporary solver state and heuristic search control.

Three persistence classes are constitutional:

```text
DURABLE MATHEMATICS
    entities, relations, worlds, judgements, evidence,
    reductions, morphisms, counterexamples, realizations

REBUILDABLE DERIVED VIEWS
    e-classes, canonical views, indexes, capability closure,
    solver databases, abstract-domain projections

EPHEMERAL SEARCH CONTROL
    frontier queues, activity scores, restart state,
    temporary candidate rankings, resource allocations
```

Derived canonicalization may collapse representations; it must never destroy the underlying semantic identities, provenance, or evidence.

---

# 4. Six durable semantic artifact classes

The fixed mathematical kernel knows only six durable artifact classes:

```text
Entity
Relation
World
Judgement
Evidence
Realization
```

`Generation` is the authority container, not a seventh kind of mathematics.

Everything richer — theorem, field, graph, algorithm, reduction, morphism, invariant, observer, representation, special function, primitive, solver, future unknown concept — is represented above these six.

## 4.1 Entity

An Entity is anything mathematical that can have durable structural identity.

Examples include numbers, sets, parents/domains, graphs, operators, recurrences, theories, probability laws, arithmetic circuits, newly discovered mathematical objects, and whole constructions.

Identity is structural/content-addressed:

```text
normalized exact structure
+ referenced entity digests
+ theory/foundation context
-> structural digest
```

The system never requires globally equivalent mathematics to have one hash. `x+x` and `2*x` may have different structural identities connected later by a certified equivalence Judgement.

## 4.2 Relation

A Relation is the foundational mathematical connective and is not intrinsically directional.

```text
R(x1, x2, ..., xn)
```

Functions are special cases of relations. A relation can support forward evaluation, inverse solving, partial solving, propagation, enumeration, or synthesis through different realizations without changing the semantic relation.

Relations naturally support multi-input/multi-output hypergraph-style mathematical composition.

## 4.3 World

A World is an immutable mathematical assumption context.

```text
World {
    parent_worlds,
    assumptions,
    local_definitions,
    local_equalities,
    local_disequalities,
    foundation/theory
}
```

A Judgement may be proved in one World and open or refuted in another. Assumption branching must not clone the entire universe semantically, although implementations may use versioned/shared structures.

## 4.4 Judgement

A Judgement is a proposition about Entities/Relations in a World.

Examples:

```text
Prime(17)
Equivalent(A, B)
Invertible(M)
OptimalValue(P, 17)
Terminates(C)
Preserves(Reduction, Satisfiability)
```

A Judgement does not store mutable truth such as `verified=true`. Authority is derived from admitted Evidence under a specific Universe generation.

## 4.5 Evidence

Evidence establishes, refutes, bounds, or otherwise qualifies a Judgement.

Evidence uses a universal envelope with a domain-native body:

```text
EvidenceEnvelope {
    target_judgement,
    world,
    scope,
    evidence_family,
    evidence_digest,
    producer,
    checker,
    checker_trust_root,
    verdict,
    dependency_digests,
    replay/freshness metadata
}
```

The evidence body may be LRAT, Alethe, VIPR, Farkas, ECPP, an interval certificate, a Groebner/ideal certificate, a Lean/Rocq proof, a bisimulation, an exhaustive certificate, an interactive-proof transcript, or a future domain-native certificate.

No universal proof body is required.

## 4.6 Realization

A Realization is an executable or representational embodiment of admitted mathematical semantics.

One semantic object can have many realizations:

```text
generic interpreter
native CPU kernel
SIMD kernel
GPU kernel
FLINT backend
proof-extracted program
inverse relational solver
streaming evaluator
incremental updater
rigorous interval evaluator
```

Each admitted realization must be bound to the semantics it realizes through independent realization evidence.

Mathematical correctness and realization correctness are separate proof obligations.

---

# 5. Mathematical structures and typing live above the kernel

The kernel does not hard-code `Matrix`, `Field`, `Ring`, `Graph`, or future mathematical categories.

A mathematical parent/domain is an Entity. Its structure is represented through Relations/Judgements/Evidence.

Example:

```text
Entity K
Judgement Field(K)
Evidence E_field_K
```

Generic mathematics becomes applicable whenever the required structure witnesses are admitted.

A newly proved witness can therefore unlock a large capability family without changing code or retraining a model.

Morphisms/coercions are explicit mathematical relations with certified preservation properties. Implicit transport is permitted only for canonical, unambiguous, certified structure-preserving maps. Lossy, partial, approximate, assumption-changing, or heuristic conversions remain explicit.

---

# 6. Query semantics

A query is not constitutionally an operation name such as `factor()` or `solve()`.

It is an incomplete mathematical world plus a desired condition.

Conceptually:

```text
Query {
    universe_generation,
    world,
    unknowns,
    known_bindings,
    target_condition,
    observer,
    authority_contract,
    resource_contract
}
```

Unknowns may themselves be mathematics:

```text
unknown value
unknown Entity
unknown Relation
unknown transformation
unknown representation
unknown invariant
unknown reduction
unknown proof/lemma
unknown algorithm
unknown primitive
```

The Observer specifies what information must be preserved/returned: one witness, all solutions, yes/no, count, optimum, boundary behavior, proof, sign, approximation, etc.

The Authority Contract defines what evidence classes are admissible for the result. Resource limits never silently weaken this contract.

---

# 7. Problem Compiler

The Problem Compiler elaborates the query against `U_g` and builds a relevant mathematical region.

Stages:

```text
semantic elaboration
    -> recover identities/domains/world/unknowns

relevant-universe retrieval
    -> structural/premise/capability indexes

theory profiling
    -> decidability, algebraic structure, symmetry,
       canonical forms, proof routes, complexity features

observer/sufficiency analysis
    -> determine whether the problem can be quotiented/black-boxed

representation frontier
    -> exact/sound alternate representations

reduction closure
    -> certified transformations to known problem families

decomposition
    -> expose independent/substructured obligations

campaign compilation
```

The representation supplied by the client is never assumed to be the best one.

---

# 8. Campaign IR

A mathematical campaign is an evolving AND/OR hypergraph of obligations, not a flat task list.

```text
Goal
 |
 +-- OR Route A
 |      +-- AND Obligation A1
 |      +-- AND Obligation A2
 |
 +-- OR Route B
        +-- AND Obligation B1
        +-- AND Obligation B2
```

An Obligation states mathematical meaning:

```text
Given:
    semantic context
Establish:
    Judgement / Entity / Relation / witness
Allowed:
    admissible capability families
Required:
    evidence contract
Dependencies:
    other obligations
Budget:
    resource allocation
```

Work Cells are compiled execution instances of obligations. They may be algorithms, SAT/SMT/CAS/proof engines, CPU/GPU searches, models, or nested campaigns. A Work Cell never owns truth authority.

Campaign IR is event-driven and partially recompiled when discoveries, counterexamples, new structure witnesses, reductions, or decompositions change the mathematical landscape.

---

# 9. Discovery Fabric

Discovery operates on **spaces of possible mathematics**, not merely lists of candidate formulas.

Candidate mathematics lives in a speculative campaign environment outside `U_g`.

Five discovery levels are constitutional:

```text
L0 value discovery
L1 construction discovery
L2 representation discovery
L3 theory/vocabulary discovery
L4 metamathematical/search-method discovery
```

A CandidateSpace is a semantic abstraction with interchangeable backends such as VSA, FTA, ECTA, e-graph, BDD/ZDD, decision-DNNF, automata, polyhedra, ideals, constraint systems, or future structures.

The Discovery Fabric prefers structural operations before enumeration when a domain permits them:

```text
canonicalize / complete
quotient
factor / decompose
find symmetry
find invariant
find sufficient summary
abstract / refine
invert
find reduction / theory morphism
find basis / conjugacy / dual
anti-unify / generalize
synthesize
```

These are not magical hard-coded verbs. Every metaprimitive is itself admitted mathematics with a declared applicability domain, soundness/completeness class, termination/finiteness conditions, evidence route, and cost model.

New evidence should restrict entire CandidateSpaces whenever possible. Failures should be generalized into counterexamples, nogoods, interpolants, infeasibility witnesses, or impossibility theorems capable of pruning whole candidate families.

Discovery may invent lemmas, representations, reductions, invariants, or vocabulary the user never requested when these artifacts unlock the root target.

Discovery may improve its own search machinery only through the same Certification + Promotion gate as ordinary mathematics.

---

# 10. Certification Fabric

Certification does not produce one global `verified` flag.

Authority is multi-dimensional:

```text
semantic outcome:
    proven / refuted / bounded / open / empirical / unknown

scope:
    universal / world-bound / finite / bounded / sampled

verification mode:
    foundational proof
    independently checked certificate
    exact recomputation
    exhaustive verification
    rigorous enclosure
    probabilistic certificate
    empirical observation

trust root:
    checker / proof kernel / assumption family

freshness:
    current / stale / superseded
```

These dimensions form a typed authority space, not one simplistic ranking. A rigorous interval enclosure and an exact symbolic equality are different claims, not merely different confidence levels.

## 10.1 Certification transaction

For a candidate artifact:

```text
1. Freeze exact candidate structure and dependencies.
2. Bind the target Judgement and World.
3. Select a verification route from Theory Profile + Authority Contract.
4. Produce domain-native certificate/proof/witness.
5. Check with an independent checker or admissible authority path.
6. Bind Evidence envelope to exact digests.
7. Run conflict and composition/interference checks.
8. Mark the semantic candidate eligible for Promotion.
```

Search engines, optimizers, models, CAS systems, GPU campaigns, and external libraries may produce candidates and certificates but do not receive authority by producer identity.

## 10.2 Contradictory evidence

Conflicting Judgements in different Worlds are normal.

A proof of `J` and a proof of `not J` in the **same classical World and same semantic interpretation** is an authority conflict. Such a conflict must not be silently admitted into ordinary classical closure.

The promotion transaction must:

```text
quarantine the affected authority edge
generate a conflict obligation
inspect checker/foundation/dependency/world identity
refuse ordinary closure until resolved
```

A mathematical package may deliberately use paraconsistent or other non-classical semantics, but that must be explicit in the World's foundation and must not leak classical explosion into unrelated Worlds.

---

# 11. Promotion is not one operation

Truth admission and automatic capability activation are distinct.

The promotion lifecycle has four semantic stages:

```text
CERTIFIED
    evidence establishes the candidate claim

ADMITTED
    semantic artifact is part of U_(g+1)

ACTIVATED
    compiler/search may use it automatically as capability

REALIZED
    one or more executable realizations are admitted
```

A true theorem may be ADMITTED without becoming an automatic rewrite/search primitive.

## 11.1 Semantic admission

A Judgement/Entity/Relation may be admitted when its semantics, world/scope, dependencies, and evidence meet the generation policy.

## 11.2 Capability activation

Activation changes future search behavior and therefore requires additional checks:

```text
applicability contract
interaction/interference surface
termination/completeness claims if any
composition obligations
search/regression behavior
cost/resource metadata
fallback behavior
```

Two individually certified packages may interfere when they share new symbols, rewrite heads, assumptions, state, or semantic commitments. Activation therefore performs composition checks rather than assuming local correctness implies global compatibility.

## 11.3 Primitive promotion

A new primitive requires more than repeated occurrence.

At minimum:

```text
stable semantic identity
well-defined domain/codomain/parent semantics
uniqueness or explicit solution-space semantics
branch/singularity/normalization/selector semantics where relevant
explicit dependencies
proof/certificate authority
novelty/compression/reuse value
at least one viable certified realization or evaluation route when execution is claimed
```

Human notation/name is metadata and can be added later.

## 11.4 Metaprimitive activation

A metaprimitive can alter future discovery/search and therefore has the strictest activation gate:

```text
formal/specific soundness contract
explicit applicability theory
termination/finiteness/completeness claims scoped precisely
negative/adversarial controls
transfer beyond discovery examples
interference/composition analysis
replayable fallback
no authority-write privilege
```

Even an activated metaprimitive may only propose candidate mathematics; it never bypasses Certification + Promotion.

---

# 12. Generation change and revocation

Generations are immutable; historical mathematics is never rewritten.

If a checker, dependency, assumption, realization, or semantic translation is later invalidated, a later generation changes the authority binding:

```text
CURRENT -> STALE / SUPERSEDED / WITHDRAWN
```

The affected dependency cone is rechecked or repaired where possible. Unaffected evidence remains reusable.

A change may be classified semantically as definitional/conservative, strengthening, assumption weakening, signature-changing, non-conservative, realization-only, etc. Revalidation policy follows semantic change class rather than textual diff size.

---

# 13. Execution / Realization Fabric

Execution consumes admitted mathematics and produces results under an explicit semantic contract.

The constitutional execution rule is:

> **Use the cheapest representation/realization that can certify the current obligation; escalate only when it cannot decide or satisfy the requested contract.**

The Mathematical VM is therefore a **lowering/specialization fabric**, not one giant interpreter containing every branch of mathematics.

## 13.1 Realization Contract

Every realization declares at least:

```text
semantic target digest
supported query direction / observer
world/domain/preconditions
semantic class
    total
    partial
    refining
    productive/coinductive
    relational/nondeterministic
    stochastic
numeric semantics / precision / error contract
resource profile
hardware/backend requirements
implementation digest
realization evidence
```

The execution planner selects only realizations whose contracts satisfy the query's Authority and Observer requirements.

## 13.2 Lowering path

A typical path is:

```text
admitted semantic relation
    -> query-direction specialization
    -> observer/sufficient-summary specialization
    -> representation selection
    -> algorithm/reduction selection
    -> precision/exactness plan
    -> native/library/backend lowering
    -> realization validation
    -> executable artifact
```

Possible backends include generated native code, FLINT/GMP/Arb-like specialist libraries, SAT/SMT/MILP engines, proof-extracted programs, LLVM/MLIR, Rust/C/C++, GPU kernels, streaming/out-of-core evaluators, or future verified compilers.

No backend is mathematical identity.

## 13.3 Certified escalation ladders

Approximate/native stages may execute before exact authority only when they are certified filters.

Example:

```text
static/semi-static filter
    -> native float/SIMD with rigorous error bound
    -> interval/ball arithmetic
    -> adaptive higher precision
    -> exact symbolic/arithmetic/certificate authority
```

A stage may return only if it proves decisiveness for the requested predicate/observer.

`AMBIGUOUS` is a valid typed execution outcome meaning the current representation cannot certify the distinction; it triggers escalation rather than an unqualified answer.

## 13.4 Proof erasure and hot path

Proof/certificate evidence is mandatory for admission where required, but need not remain in the hot runtime representation.

Once exact semantic and realization digests are bound to checked authority, the executable may erase proof-irrelevant material and run as stripped native code. Evidence remains content-addressed and recheckable out-of-band.

If semantic digest, assumptions, realization digest, numeric contract, compiler authority, or checker requirement changes, the cached authority binding becomes stale.

## 13.5 Hardware policy

The execution policy is not `GPU = powerful`.

The planner chooses the cheapest suitable substrate:

```text
single CPU core
SIMD
multicore
specialized exact library
out-of-core/streaming
GPU
future distributed Ptah campaign
```

Discovery cost may be enormous while execution of the promoted primitive is microseconds.

## 13.6 Runtime cannot mutate authority

Execution can produce run results, witnesses, performance measurements, or candidate evidence.

It cannot directly modify `U_g`.

Any newly discovered reusable mathematics returns through Certification + Promotion.

---

# 14. Federation boundary

External mathematical software and knowledge sources are replaceable capability providers.

They may provide:

```text
candidate facts
algorithms
solvers
proofs/certificates
realizations
benchmarks
representations
```

But external source provenance is not mathematical authority by itself.

GitHub source code may be lifted into semantic candidates; theorem libraries may provide kernel-checked facts; informal papers may generate candidate formalizations. All cross the same authority boundary.

---

# 15. Ptah boundary

The project does not own the general workspace, node scheduler, generic artifact custody, machine leasing, or distributed execution substrate.

Later, Ptah may execute mathematical Work Cells and persist campaign artifacts.

This project owns:

```text
what the mathematical obligation means
what evidence is required
which transformations are admissible
which work has root mathematical value
what may be certified/promoted
```

Ptah supplies where/how large work executes.

---

# 16. Stress tests against radically different mathematics

The constitution must express all of these without new kernel artifact classes.

## 16.1 Polynomial equation

`x^2 - 5x + 6 = 0`

- Entities: polynomial, domain, unknown `x`.
- Relation/Judgement: root relation.
- Campaign may factor, reduce, use exact algebra.
- Evidence certifies candidate roots.

No special kernel `Equation` class required.

## 16.2 Primality

`Prime(N)` is a Judgement over an Entity.

A hard producer may generate an ECPP/Pocklington certificate; a smaller checker supplies Evidence. A promoted primality primitive may use multiple realizations.

## 16.3 Nonlinear numerical root

Fast Newton/homotopy search may produce a candidate only. Interval Newton, Krawczyk, alpha-theory, or another rigorous route can establish existence/uniqueness. Approximate scouting never becomes authority by itself.

## 16.4 Graph/optimization problem

A graph is an Entity with Relations. A reduction, decomposition, dual, or sufficient summary can transform the target. Domain-native certificates establish shortest path/flow/optimization claims.

## 16.5 Recursive/infinite behavior

World + Relation + Judgement can express recursive state evolution. Least/greatest fixed-point, induction/coinduction, bisimulation, productive semantics, or certified refining approximations fit without changing the kernel.

## 16.6 New primitive unknown to the designers

A future object may be characterized by a 200-node relational structure, recurrence, branch semantics, invariant, and uniqueness proof with no conventional human formula.

It enters as Entity + Relations + Judgements + Evidence, then receives Realizations. The kernel remains unchanged.

## 16.7 Domain client such as quantitative finance/engineering

The domain system translates domain semantics into mathematical Entities/Relations/Targets. The project returns certified mathematical results and assumptions. The domain client remains authority for economic/engineering interpretation and policy.

---

# 17. D1 frozen laws

D1 freezes the following laws:

1. **Search proposes; Certification + Promotion authorize.**
2. **Execution consumes authority; it cannot create authority directly.**
3. **Mathematical identity is structural/content-addressed; semantic equivalence is separately proved.**
4. **No one representation, solver, proof language, backend, or model defines mathematics.**
5. **The fixed durable kernel consists of Entity, Relation, World, Judgement, Evidence, Realization.**
6. **Universe generations are immutable authority snapshots.**
7. **Durable mathematics, rebuildable derived views, and ephemeral search state are separate.**
8. **Unknowns may be mathematical objects, relations, representations, reductions, theories, or metaprimitives — not only values.**
9. **Campaigns are dynamic AND/OR mathematical obligation graphs; Work Cells are execution instances, not truth authorities.**
10. **Candidate spaces are first-class symbolic spaces; structural compression is preferred before enumeration when available.**
11. **Authority is typed/multi-dimensional; there is no global `verified=true`.**
12. **Truth admission and capability activation are separate.**
13. **New active primitives/metaprimitives require composition/interference analysis in addition to local correctness.**
14. **Mathematical semantics and executable realizations have independent certificates.**
15. **Proof evidence may be erased from the hot path after exact authority binding where computationally irrelevant.**
16. **The runtime chooses the cheapest realization capable of certifying the current obligation; ambiguity escalates.**
17. **Models and external systems may generate candidates but have no special mathematical authority.**
18. **Discovered mathematics can extend both the mathematical vocabulary and the discovery vocabulary only through Promotion.**
19. **Ptah is the future workspace/execution substrate, not part of mathematical truth.**
20. **The architecture must remain able to admit mathematical concepts not anticipated by the designers without kernel schema changes.**

---

# 18. What D1 deliberately does not freeze

D1 does not choose:

```text
Rust vs another implementation language
specific database/storage engine
exact hypergraph/e-graph implementation
Lean/Rocq/Isabelle as universal proof authority
specific compiler backend
specific numerical library
specific scheduler algorithm
Ptah integration details
API syntax
UI
product name
full roadmap
```

Those belong to later milestones after the next design layers are tested against D1.

---

# 19. Next milestone

The next design milestone should be **D2 — Operational Mathematical Machine**.

D2 should define, still before implementation:

1. exact capability/package contract;
2. Theory Profile and admissibility inference;
3. CandidateSpace backend contract;
4. Work Cell / Campaign IR contract;
5. scheduler/search-economy interfaces;
6. certificate envelope and authority lattice schema;
7. realization planner/lowering contract;
8. capability closure and package federation;
9. First-Light proof campaign selection;
10. targeted research spikes only where D1 evidence is insufficient.

No full implementation roadmap should be frozen until D2 demonstrates that these operational contracts can express several radically different mathematical families without special-case architecture.
