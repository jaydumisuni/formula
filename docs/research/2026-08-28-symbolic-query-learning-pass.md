# Research Pass — Symbolic Hypothesis Spaces, Queries, and Refinement

**Date:** 2026-08-28  
**Status:** RESEARCH supplement  

This pass investigates how the unnamed mathematical project can measure progress and choose informative work when the candidate mathematical space is symbolic, structured, or infinite, so ordinary entropy over an explicitly enumerated finite hypothesis set is unavailable or inappropriate.

---

## 1. Exact learning provides a non-probabilistic notion of informative queries

Classical exact-learning theory studies concept classes through queries such as:

- membership queries;
- equivalence queries;
- subset/superset queries;
- disjointness/exhaustiveness queries.

A key combinatorial result is that efficient query learning can be characterized by the existence of queries that reject a guaranteed fraction of the remaining candidate concepts regardless of the answer.

Source:

https://doi.org/10.1016/S0304-3975(02)00177-9

### Architectural implication

The project does not always need a probability distribution or Shannon entropy over candidate mathematics.

For a symbolic hypothesis class `H`, a useful work obligation may instead maximize a worst-case elimination measure:

```text
query_value(Q, H)
    = minimum candidate-space reduction
      over all admissible answers to Q
```

This provides a deterministic analogue of information gain.

Possible mathematical use cases:

- select a finite test that distinguishes candidate recurrences;
- select a counterexample search region that splits candidate transformations;
- select a lemma/property query that eliminates many proof routes;
- select an exact arithmetic probe that separates competing symbolic structures.

---

## 2. Membership + equivalence query loops are a reusable discovery pattern

Active automata learning provides a particularly clean skeleton:

```text
construct hypothesis H
    -> equivalence check
        -> PASS: done
        -> counterexample c
             -> refine H using c
             -> repeat
```

A 2026 register-automata learner continues this model for richer symbolic systems, using membership tests and equivalence counterexamples to discover locations, transitions, and registers.

Source:

https://doi.org/10.1007/s10817-026-09758-9

General active-learning background:

https://proceedings.mlr.press/v34/isberner14a.html

### Architectural implication

A mathematical discovery campaign can potentially use the same structure:

```text
candidate theory / construction / representation
    -> strongest available equivalence or property checker
    -> counterexample / distinguishing witness
    -> structural refinement
    -> repeat
```

The hypothesis family can remain implicit. The system only needs compact boundaries/constraints defining the currently admissible candidates.

---

## 3. Counterexamples should be compressed into discriminating witnesses

Active automata learning research shows that a long counterexample does not necessarily need to be retained as an undigested object: algorithms can locate a small suffix/distinguishing fragment sufficient to force progress, sometimes using logarithmically many queries in the counterexample length.

Source:

https://proceedings.mlr.press/v34/isberner14a.html

### Architectural implication

When a mathematical candidate fails, the project should seek the **minimal or high-leverage distinguishing witness** rather than merely archive the full failing computation.

Possible artifact:

```text
Distinguishing Witness
    claim/candidate family
    witness
    exact disagreement
    assumptions/world
    candidates eliminated
    certificate
```

This can become a reusable discriminator in future campaigns.

---

## 4. Teaching dimension suggests a dual notion: smallest evidence that uniquely identifies structure

Teaching-dimension research asks for the smallest witness/example set that uniquely identifies a target concept inside a concept class.

Source:

https://doi.org/10.1006/jcss.1995.1003

### Architectural implication

For an accepted mathematical construction or relation, the project may eventually search for a compact **characterizing witness set**:

```text
large discovered structure
    -> minimal discriminating examples/properties
    -> compact identification signature
```

This does not replace proof.

It could instead help:

- retrieval/routing;
- rapid candidate rejection;
- regression testing;
- rediscovery benchmarks;
- representation identification.

Computing optimal teaching sets is itself hard in general, so this is a heuristic/certified-subdomain opportunity rather than a universal primitive.

---

## 5. IC3/PDR shows how local failures can grow into global invariants

Property Directed Reachability (IC3/PDR) incrementally learns clauses that block bad states and generalizes them so a single learned fact can exclude many states at once. It maintains proof obligations and strengthens approximations until either:

- an inductive invariant proving the property is found; or
- a real counterexample is reached.

Sources:

https://www.mdpi.com/1999-4893/17/6/253

https://link.springer.com/article/10.1007/s10703-023-00434-x

https://pmc.ncbi.nlm.nih.gov/articles/PMC7363441/

### Architectural implication

This suggests a powerful mathematical loop:

```text
specific failing candidate / state
    -> prove why it fails
    -> generalize failure condition
    -> certify generalized exclusion
    -> add exclusion to search theory
```

and dually:

```text
specific successful examples
    -> infer candidate invariant
    -> prove inductiveness/general scope
    -> promote invariant
```

The project should therefore treat **inductive generalization of evidence** as a first-class metapriming opportunity, not only direct theorem/construction synthesis.

---

## 6. CEGAR shows how a representation can refine itself only where needed

Counterexample-Guided Abstraction Refinement starts with a coarse representation, checks the target property, and uses spurious counterexamples to add precisely the distinctions required to make the abstraction more accurate.

Sources:

https://doi.org/10.1145/876638.876643

https://www2.eecs.berkeley.edu/Pubs/TechRpts/2002/6190.html

### Architectural implication

The unnamed project should not necessarily begin every mathematical problem in the richest available representation.

Possible loop:

```text
coarse mathematical representation
    -> solve/check
    -> apparent counterexample
    -> determine concrete vs representation artifact
    -> refine only the missing distinction
    -> repeat
```

This may be substantially cheaper than representing every mathematical detail from the start.

It also gives a precise mechanism for the previously proposed metaprimitive:

```text
change_representation_to_make_problem_easier
```

where the representation can evolve incrementally under proof obligations rather than being replaced blindly.

---

## 7. Auxiliary structure can be invented to make proofs/search simpler

Counterexample-guided prophecy work for array model checking automatically introduces auxiliary variables so proofs that would require quantified reasoning can sometimes be reduced to quantifier-free reasoning.

Source:

https://pmc.ncbi.nlm.nih.gov/articles/PMC7979195/

### Architectural implication

The project should permit **invented auxiliary mathematical objects** whose purpose is to make a relation, invariant, proof, or representation simpler.

Examples may include:

- helper variables;
- latent coordinates;
- introduced basis elements;
- auxiliary invariants;
- state/history variables;
- quotient identifiers.

These auxiliaries must carry exact elimination/reconstruction semantics so they do not silently alter the original problem.

---

## 8. Current symbolic-space progress model

For structured/infinite candidate families, the project may combine:

```text
IMPLICIT HYPOTHESIS CLASS
    -> grammar / theory / relation family / constraints

QUERY OR DISCRIMINATOR SELECTION
    -> maximize guaranteed elimination or expected discrimination

CHECK / ORACLE / CERTIFICATE ENGINE
    -> membership / equivalence / satisfiability / proof / counterexample

COUNTEREXAMPLE ANALYSIS
    -> extract compact distinguishing witness

GENERALIZATION
    -> derive broader exclusion/invariant when certifiable

REPRESENTATION REFINEMENT
    -> add only distinctions exposed as necessary

REPEAT
```

This avoids requiring explicit enumeration or a probabilistic distribution over the full mathematical hypothesis space.

---

## 9. Relation to the search-economy layer

The search-economy scheduler can therefore have more than one notion of `information value`:

```text
probabilistic expected information gain
    -> when a meaningful probabilistic candidate model exists

worst-case elimination fraction
    -> exact-learning/query-learning style

proof/disproof work reduction
    -> AND/OR proof graph style

abstraction refinement value
    -> amount of spurious state removed

nogood/generalization value
    -> future candidate space eliminated
```

No single scalar has yet been chosen as the universal scheduler utility.

A multi-objective/typed value model may be more appropriate.

---

## 10. New research obligations

1. Investigate compact representations of version spaces for algebraic/program/formula families without explicit enumeration.
2. Map exact-learning query types to mathematical work-cell operations and determine which domains admit true equivalence queries versus approximations.
3. Investigate interpolation, unsat cores, and minimal conflicting subsets as mechanisms for extracting compact mathematical distinguishing witnesses.
4. Investigate inductive generalization algorithms beyond IC3/PDR for algebraic, combinatorial, and program-synthesis domains.
5. Determine how automatically introduced auxiliary mathematical objects can be eliminated or translated back while preserving proof/certificate identity.
6. Investigate query-complexity lower bounds as a way to estimate when a proposed discovery family is inherently expensive.
7. Determine whether characterizing witness sets can become compact regression/identity signatures for promoted mathematical primitives.
8. Integrate symbolic worst-case elimination measures with the existing value-of-computation/search-economy hypothesis.
