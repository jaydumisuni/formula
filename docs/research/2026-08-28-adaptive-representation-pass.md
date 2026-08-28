# Research Pass — Adaptive Representation, Abstract Domains, and Representation Switching

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how the unnamed mathematical project can move among several representations of the same semantic mathematical region based on precision, closure, proof strength, cost, and expected representation growth.

The central result is:

> **Semantic identity should remain stable while active mathematical representations may be selected, refined, combined, reordered, or abandoned dynamically.**

---

## 1. Abstract interpretation provides a formal semantics for sound representation change

Abstract Interpretation relates a concrete semantic domain and an abstract domain through abstraction/concretization maps, classically expressed by a Galois connection.

Representative discussion:

https://link.springer.com/article/10.1007/s10270-023-01145-x

### Architectural implication

Representation change need not be an informal approximation step.

A representation adapter can declare:

```text
concrete semantic family C
abstract representation A
abstraction map alpha
concretization map gamma
soundness relation
precision/completeness properties
```

This gives the project a principled way to distinguish:

```text
exact equivalent representation
sound overapproximation
sound underapproximation
heuristic/non-certified representation
```

The type of representation change should become part of mathematical truth metadata.

---

## 2. APRON proves multiple mathematical abstractions can share one semantic API

APRON exposes several numerical abstract domains under a common interface, including:

- intervals;
- octagons;
- convex polyhedra;
- zonotopes;
- optional grids/congruences;
- reduced products.

Sources:

https://antoinemine.github.io/Apron/doc/

https://github.com/antoinemine/apron

Different domains make different precision/complexity trade-offs while exposing semantic operations such as assignment, tests, conjunction, entailment, projection, and dimension manipulation.

### Architectural implication

The unnamed project should distinguish:

```text
semantic operation
```

from:

```text
backend algorithm/representation implementing that operation
```

A work cell may begin in a cheap abstract domain and move to a stronger one only when needed.

---

## 3. Exactness can vary even inside one representation family

APRON's octagon implementation explicitly documents which predicates are exact under particular coefficient/domain/settings and which become only sound semi-tests under other settings.

Source:

https://antoinemine.github.io/Apron/doc/api/c/oct_doc.html

### Architectural implication

The project must not assign truth authority simply by backend type/name.

A representation operation should expose its actual contract for the current configuration:

```text
operation: inclusion_test
result_class: exact
```

or:

```text
operation: saturation_test
result_class: sound_but_incomplete
```

This fits the existing multidimensional truth/certification model.

---

## 4. Reduced products let different representations exchange precision

Abstract Interpretation uses reduced products and related constructions to combine domains so information discovered in one representation can refine another.

APRON and Crab include examples such as polyhedra + congruences and arbitrary-domain reduced products.

Sources:

https://antoinemine.github.io/Apron/doc/api/c/Managers-and-Abstract-Domains.html

https://github.com/seahorn/crab

Recent synthesis work even constructs cooperative abstract transformers for reduced-product domains automatically.

Source:

https://arxiv.org/abs/2408.04040

### Architectural implication

Representation choice does not have to be exclusive:

```text
Representation A
Representation B
    <-> exchange certified/sound information
```

For example:

```text
interval bounds
+ modular congruence information
+ sparse equality information
```

may jointly prune a problem more cheaply than moving immediately to one maximally expressive representation.

A future representation fabric should therefore support **cooperating views**, not only switching.

---

## 5. Strong preservation/completeness gives a language-relative criterion for representation adequacy

Abstract-interpretation research characterizes when an abstraction preserves exactly the properties expressible in a particular specification language and how to minimally refine an abstraction to obtain stronger preservation.

Source:

https://arxiv.org/abs/cs/0401016

### Architectural implication

The correct question is not always:

> Is this representation exact for everything?

but:

> Is it exact enough for the mathematical queries/operations currently required?

Possible Theory Profile field:

```text
strongly_preserves:
    query fragment Q
```

A cheap representation can remain active as long as it preserves every property relevant to the current obligation.

---

## 6. Refinement can be demand-driven

CEGAR and trace-partitioning traditions refine abstractions only after a coarse representation is shown insufficient or produces spurious behavior.

Trace partitioning deliberately delays merging some paths/states because premature merging loses precision, but unlimited partitioning can explode representation size.

Representative source:

https://link.springer.com/chapter/10.1007/978-3-319-89963-3_15

### Architectural implication

Representation complexity can be budgeted dynamically:

```text
coarse view
    -> try solve/check
    -> ambiguity/spurious result
    -> refine the relevant dimensions/world distinctions only
```

The system should avoid eagerly paying for maximal precision everywhere.

---

## 7. Representation parameters can themselves be adapted automatically

Parf (2024) adaptively tunes parameters of abstract-interpretation analyses under a time budget, using accumulated analysis outcomes to refine future parameter choices.

Source:

https://arxiv.org/abs/2409.05794

### Architectural implication

A representation backend may expose tunable policy dimensions:

```text
partition granularity
widening thresholds
precision level
integer/rational/float coefficient mode
solver budget
refinement depth
```

These can participate in the mathematical search economy as **representation-policy choices**, while soundness remains guaranteed by the backend contract.

---

## 8. Decision diagrams demonstrate dynamic reorganization of an exact representation

BDD/ZDD semantics are independent of variable order, but physical representation size can change exponentially with ordering. Mature BDD packages support dynamic variable reordering such as sifting, window permutation, annealing, genetic algorithms, and other heuristics.

Sources:

https://github.com/sjtusonic/cudd

https://github.com/tulip-control/dd

### Architectural implication

Representation optimization need not change the representation family at all.

There are at least three different operations:

```text
backend switch
    BDD -> automaton / polyhedron / relation backend

representation refinement
    interval -> octagon -> polyhedron

internal reorganization
    same BDD semantics, different variable order
```

All can be search-economy decisions.

---

## 9. Representation optimization can itself be hard

Optimal BDD variable ordering is NP-complete, so mature systems use heuristics and dynamic search rather than expecting one canonical best ordering.

### Architectural implication

The project must avoid assuming:

```text
canonical mathematical representation
==
cheapest executable representation
```

A semantic object may have one stable canonical identity while many physical representations compete for performance.

The optimizer can remain untrusted if representation equivalence/conversion is independently checkable.

---

## 10. Blackboard analysis architectures show sound components can cooperate without one monolithic domain

Recent work on modular soundness for blackboard-style static analysis investigates multiple analysis components communicating facts while preserving soundness compositionally.

Source:

https://link.springer.com/chapter/10.1007/978-3-031-57267-8_14

### Architectural implication

This is relevant to the project's envisioned mathematical work formation:

```text
relation backend
polyhedral backend
equality backend
finite model backend
numerical enclosure backend
```

can share certified/sound deductions through a controlled mathematical blackboard without forcing one representation to absorb all others.

The difficult part is defining the inter-domain contracts and avoiding unsound strengthening when information crosses representation boundaries.

---

## 11. Representation conversion itself should be proof-bearing or translation-validated

Current project research already separates mathematical semantics from executable realization.

The same principle applies to representation changes:

```text
semantic object S
    -> converter
    -> representation R2
    -> conversion certificate / independent semantic check
```

For an exact conversion, semantic equivalence should be established.

For a sound abstraction, inclusion/simulation-style obligations should be established.

For an underapproximation, the direction reverses.

---

## 12. Current adaptive-representation hypothesis

A semantic mathematical region may simultaneously own several active views:

```text
Semantic Region
    |
    +-- exact relation representation
    +-- cheap abstract overapproximation
    +-- exact sparse/canonical representation
    +-- finite automaton view
    +-- numerical rigorous enclosure
    +-- compiled/native realization
```

Each view exposes:

```text
semantic relation to region
exactness / soundness direction
supported operations/query language
precision
cost model
representation size/growth
conversion routes
certificate routes
```

The search economy chooses which views to maintain and which work obligations to send to each.

---

## 13. New research obligations

1. Define a mathematically clean common contract for exact equivalence, overapproximation, underapproximation, abstraction, and refinement relations.
2. Study reduced-product/reduced-cardinal-power constructions as ways for heterogeneous mathematical backends to cooperate.
3. Investigate proof-producing/checked conversions between common numerical domains: intervals, polyhedra, congruences, Presburger/automata, and rigorous ball arithmetic.
4. Determine when multiple active representations should exchange facts eagerly versus lazily.
5. Study automatic representation-selection policies based on expected query cost and representation growth.
6. Investigate representation-specific garbage collection/restart without losing permanent semantic/provenance identity.
7. Determine how approximate/sound views participate in candidate-space elimination without accidentally proving false exact claims.
8. Investigate whether representation optimizations such as BDD reordering, basis change, sparse layout change, and algebraic normal-form change can share one higher-level search-economy abstraction.
9. Study representation portfolios where several views race/cooperate and a certificate layer reconciles results.
10. Determine whether useful representation policies can be promoted/distilled as deterministic campaign primitives without becoming mathematical truth themselves.
