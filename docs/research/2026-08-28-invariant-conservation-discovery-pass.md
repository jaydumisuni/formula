# Research Pass — Invariant and Conservation-Law Discovery

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates automatic discovery of mathematical invariants: quantities, relations, regions, or structural properties that remain preserved under a construction/dynamical evolution.

The central finding is:

> **Invariant discovery can compress a large reachable/evolution space into a smaller mathematical description, and in important restricted classes an algorithm can compute a finite basis representing an entire infinite family of invariants. Candidate discovery and proof authority should remain separate.**

---

## 1. Invariants can drastically reduce state/search space

An invariant `I(x)` preserved by a transformation/evolution `T` satisfies a relation such as:

```text
I(T(x)) = I(x)
```

or, for a region/property `P`:

```text
P(x) -> P(T(x))
```

Conservation laws/first integrals are invariant quantities of dynamical systems and can significantly simplify integration; enough independent conservation laws can determine the motion strongly.

Source:

https://www.sciencedirect.com/science/article/pii/S0076539208618032

### Architectural implication

`discover_invariant` should be a first-class metaprimitive because a proved invariant can:

```text
prune unreachable states
rule out candidate solutions
create quotient coordinates
reduce dimensions
support decomposition
provide progress measures
establish safety
improve inversion
```

---

## 2. Dynamic invariant detection is useful candidate generation, not proof

Daikon observes executions and reports likely invariants that survive supplied samples and statistical filtering.

Sources:

https://plse.cs.washington.edu/daikon/

https://plse.cs.washington.edu/daikon/download/doc/daikon.html

Its manual explicitly describes these as likely invariants based on observed executions.

### Architectural implication

Empirical candidate generation belongs in the search layer:

```text
execution/data
    -> likely invariant candidate
    -> adversarial falsification
    -> symbolic/static proof attempt
    -> certified invariant or refuted candidate
```

Sampling confidence must never be promoted directly to mathematical invariance.

---

## 3. Comparability/typing massively reduces meaningless invariant search

Daikon's DynComp groups variables into richer abstract comparability classes. Without this, Daikon attempts many pair/triple invariants across unrelated values, increasing runtime/memory and producing semantically meaningless relationships.

Source:

https://plse.cs.washington.edu/daikon/download/doc/daikon.html

### Architectural implication

This independently supports the project's mathematical structure/type system:

> Search only relations among mathematically admissible/comparable objects.

Dimensions, quantity kinds, parents, structures, theory interfaces, and inferred relation types should constrain the invariant grammar before generation.

---

## 4. Restricted loop classes admit complete algebraic invariant generation

For extended P-solvable loops, algorithms compute polynomial invariant ideals using closed forms and Gröbner bases. Fixed-point combination across paths can yield the polynomial ideal of all polynomial invariants in the supported class.

Sources:

https://arxiv.org/abs/1801.03967

https://arxiv.org/abs/1705.02863

Recent 2024 work gives an algorithm computing the strongest algebraic invariant for simple linear loops, while also emphasizing that invariant generation is undecidable in general.

Source:

https://arxiv.org/abs/2407.09154

### Architectural implication

The project should distinguish:

```text
one discovered invariant
```

from:

```text
complete invariant basis for declared class
```

A finite algebraic basis can represent an infinite set of invariant equations and should be stored/promoted as a stronger mathematical artifact.

---

## 5. Invariant ideals are another “finite generators for infinite mathematics” pattern

A polynomial ideal may contain infinitely many polynomial relations but can be represented by a finite Gröbner basis.

Earlier research found the same broad pattern in relation-space discovery and canonicalization.

### Architectural implication

Invariant results may be stored as generator structures rather than enumerated theorems:

```text
InvariantSpace
    class: polynomial_ideal
    generators: G
    monomial_order: ...
    scope/domain: ...
    completeness_claim: ...
    certificate: ...
```

Future work can derive individual invariant equations on demand from the basis.

---

## 6. Continuous systems have automatic invariant-generation frameworks

Pegasus generates inductive invariant candidates for polynomial ODE systems and integrates with KeYmaera X.

Source:

https://keymaerax.org/Pegasus/

Pegasus can use several invariant-generation methods. Importantly, standalone Pegasus returns candidates, whereas KeYmaera X formally proves the invariant property.

### Architectural implication

This is a near-perfect donor for the project's solver/verifier law:

```text
complex invariant generator
    -> candidate invariant
    -> theorem prover / independent proof
    -> accepted invariant
```

The generator may evolve freely without becoming a truth root.

---

## 7. Differential invariants and barrier/Darboux certificates avoid solving ODEs explicitly

KeYmaera X supports differential invariants, barrier certificates, Darboux polynomial reasoning, and related ODE proof automation.

Source:

https://keymaerax.org/news.html

### Architectural implication

The project should not assume that understanding a dynamical system requires finding a closed-form trajectory.

Often it is cheaper and mathematically stronger for the requested goal to prove a preserved region/property directly:

```text
trajectory solution unknown/expensive
BUT
safety invariant certified
```

This is another case where solving the user's actual condition can be far cheaper than solving a stronger human-style problem.

---

## 8. First integrals can sometimes be computed algorithmically

For polynomial differential systems, algorithms compute Darboux polynomials and bounded-degree rational first integrals with explicit complexity guarantees in restricted settings.

Source:

https://www.sciencedirect.com/science/article/pii/S0885064X10000968

### Architectural implication

The Theory Profile should identify invariant/first-integral fragments where complete or bounded-complete algorithms exist.

Possible profile fields:

```text
invariant_language: polynomial | rational | semialgebraic | ...
complete_up_to_degree: N
first_integral_search: decidable_for_fragment / bounded / heuristic
certificate_route: ...
```

---

## 9. Invariants can turn dynamics into lower-dimensional/algebraic problems

A conserved quantity constrains reachable states to level sets:

```text
I(x) = c
```

Multiple independent invariants can restrict motion to intersections of such sets.

### Architectural implication

Representation/decomposition search should automatically try:

```text
find invariant I
    -> replace full state space with level-set coordinates
    -> eliminate variables where possible
    -> solve residual problem
```

This directly connects invariant discovery to the dimensional-analysis and automatic-decomposition passes.

---

## 10. Invariants should feed relational propagation

Once `I(x)=c` is proved, it becomes a permanent relation usable by propagators in every direction.

Example:

```text
I(x,y,z) = c
known x,y
    -> narrow z
```

### Architectural implication

A new invariant is not merely documentation/proof evidence. It changes the active constraint geometry and can immediately unlock stronger narrowing, conflict detection, and inverse solving.

Thus invariant promotion can directly increase execution capability.

---

## 11. Invariants can become quotient keys/equivalence structure

States that share invariant values can sometimes be grouped into classes or fibers relevant to the problem.

### Architectural implication

The semantic universe can index state/search space by certified invariant signatures:

```text
state -> (I1(state), I2(state), ...)
```

This may enable caching, symmetry reduction, decomposition, and cross-problem transfer.

It must remain explicit which invariants form a complete classifier versus merely useful necessary properties.

---

## 12. Candidate invariant languages should be extensible

Daikon illustrates a fixed/extensible library of invariant templates; algebraic methods use polynomial/rational families; differential logic uses barrier/Darboux/differential invariants.

### Architectural implication

The system should not have one global invariant grammar.

Theory Profile chooses admissible candidate languages based on structure:

```text
linear
polynomial ideal
rational
order/inequality
convex/barrier
logical/relational
spectral
symmetry-derived
coinductive
custom promoted invariant family
```

New certified invariant-generation methods can themselves become metaprimitives.

---

## 13. Current invariant-discovery hypothesis

```text
MATHEMATICAL DYNAMICS / TRANSFORMATION
    -> infer admissible invariant language from Theory Profile
    -> generate candidates using exact algebra / traces / symmetry / AD / solver methods
    -> cheap falsification
    -> proof of preservation
    -> compress surviving family into finite generators where possible
    -> promote invariant space
    -> update propagation, decomposition, quotient, and search structures
```

A proved invariant changes future search geometry immediately.

---

## 14. New research obligations

1. Study automatic discovery of functionally/algebraically independent invariants so redundant conservation laws are not overcounted.
2. Investigate invariant-coordinate construction and quotient/fiber representations.
3. Study polynomial/rational invariant bases for discrete, continuous, and probabilistic systems under one certificate envelope.
4. Investigate automatic barrier/Lyapunov function synthesis with independent proof.
5. Connect AD-derived gradients/Jacobians to invariant/first-integral discovery.
6. Study Noether-style symmetry-to-conservation-law derivation and its algorithmic limits.
7. Investigate invariant discovery over e-hypergraph/relational constructions rather than only explicit programs/ODEs.
8. Study how invariant spaces transport through theory morphisms and representation changes.
9. Determine how newly promoted invariants trigger incremental re-decomposition/recompilation of active mathematical campaigns.
10. Investigate when a finite invariant basis is complete enough to replace reachability/search for a declared problem class.
