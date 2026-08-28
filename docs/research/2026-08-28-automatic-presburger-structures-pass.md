# Research Pass — Automatic Structures, Presburger Relations, and Decidable Query Backends

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates a stronger form of symbolic representation: mathematical representations that do not merely compress huge/infinite sets, but also come with a **decidable query language and exact closure operations**.

---

## 1. Automatic presentations turn first-order mathematics into finite automata

A structure has an automatic presentation when its domain and basic relations/operations are represented by finite strings and synchronous finite automata. A standard theorem is that every such automatic structure has a decidable first-order theory.

Survey:

https://www.cambridge.org/core/journals/bulletin-of-symbolic-logic/article/abs/automata-presenting-structures-a-survey-of-the-finite-string-case/BC3328DD16774DC1860B34B34EDC3AFE

### Architectural implication

A representation backend may expose more than `membership` or `intersection`.

It can expose a declared **logic/query fragment** for which arbitrary formulas can be compiled into a decision procedure.

Possible Theory Profile capability:

```text
query_fragment:
    first_order_logic

query_semantics:
    exact

decision_procedure:
    automata_compilation
```

This is materially stronger than a generic symbolic set container.

---

## 2. Logic-to-automata compilation can produce witnesses/counterexamples

MONA translates WS1S/WS2S formulas into finite/tree automata and decides validity or unsatisfiability by analyzing the resulting automata. It can produce satisfying examples/counterexamples.

Sources:

https://cs.au.dk/~amoeller/mona/

https://cs.au.dk/~amoeller/papers/mona14/

### Architectural implication

A query work cell can potentially return:

```text
claim verdict
+ compiled automaton
+ witness/counterexample path
+ exact logic/representation profile
```

rather than a model-generated explanation or opaque solver answer.

The resulting automaton itself may be reusable mathematical structure.

---

## 3. Decidability does not imply operational cheapness

WS1S/WS2S decision procedures have non-elementary worst-case complexity even though MONA is often practical because of implementation techniques such as:

- formula reduction;
- DAG sharing;
- automaton minimization;
- BDD-based representation;
- guided tree automata;
- cache-conscious structures.

Source:

https://cs.au.dk/~amoeller/papers/secrets/

### Architectural implication

Theory Profile must separate:

```text
decidable
```

from:

```text
operationally practical under current representation
```

Possible fields:

```text
decidability: proven
worst_case_complexity: non_elementary
current_representation_size: ...
known_reduction_methods: ...
blowup_risk: high
```

A theoretically complete backend should not monopolize compute when a cheaper incomplete/specialized route is available.

---

## 4. Presburger arithmetic gives exact symbolic integer relations

Presburger arithmetic over integers/naturals with addition/order has decidable first-order theory and close connections to semilinear/automata representations.

Overview:

https://www.cs.ox.ac.uk/people/christoph.haase/home/publication/haa-18/haa-18.pdf

Recent automata/algebra reasoning discussion:

https://link.springer.com/chapter/10.1007/978-3-031-65627-9_3

### Architectural implication

The project should treat linear integer arithmetic not merely as an SMT theory, but as a rich symbolic relation domain with exact projection, quantifier elimination, automata, and semilinear representations.

---

## 5. isl demonstrates a mature executable relation algebra for integer sets/maps

The Integer Set Library (`isl`) manipulates parameterized sets and relations of integer points described by affine constraints. It uses exact integer arithmetic and supports operations including:

- union/intersection/difference;
- projection;
- emptiness;
- affine/convex hulls;
- lexicographic optimization;
- parametric vertices;
- map composition;
- transitive closure;
- code generation / scanning.

Sources:

https://libisl.sourceforge.io/user.html

https://libisl.sourceforge.io/

Repository:

https://github.com/Meinersbur/isl

### Architectural implication

This is a concrete example of the desired relational semantics:

```text
Set / Relation
    -> compose
    -> project
    -> restrict
    -> optimize
    -> generate executable iteration/code
```

with exact mathematical semantics separated from the eventual generated program.

`isl` should be studied as a donor/backend candidate for affine integer regions, not generalized into the identity of the project.

---

## 6. The older Omega library demonstrates formula-level Presburger relation composition

The Omega library represents integer tuple sets and relations described by Presburger formulas and supports composition, intersection, union, difference, examples, existential variables, and code generation.

Sources:

https://www.cs.umd.edu/projects/omega/omega-lib.html

https://www.cs.umd.edu/projects/omega/interface_doc/node5.html

### Architectural implication

This is another strong precedent for mathematical objects being **relations first** and executable loops/code being derived realizations.

It supports the current hypothesis:

```text
semantic relation
    !=
execution direction
```

---

## 7. Automatic theorem provers such as Walnut/Pecan prove whole infinite families

Walnut is an automated theorem prover for automatic words/sequences; Pecan uses Büchi automata to prove properties over automatic/Sturmian-style infinite sequences.

Sources:

https://github.com/Walnut-Theorem-Prover/Walnut

https://arxiv.org/abs/2102.01727

### Architectural implication

The project should not assume a theorem over an infinite sequence requires induction in human textbook form.

If the mathematical object admits an automatic presentation, a finite automaton can decide/prove the property over the entire infinite family.

This provides another route in the certificate/query catalogue:

```text
automata-theoretic decision proof
```

---

## 8. Representation backend should advertise a query algebra, not only a storage format

Current evidence suggests a representation backend may need a contract closer to:

```text
RepresentationBackend
    semantic domain
    exactness class
    supported logical/query fragment
    closed operations
    projection/elimination capabilities
    witness/counterexample capabilities
    canonicalization/minimization
    complexity profile
    known blow-up triggers
    conversion routes
    certificate routes
```

rather than merely:

```text
serialize / deserialize / iterate
```

This significantly strengthens the earlier symbolic-family hypothesis.

---

## 9. Query compilation can itself become a reusable mathematical primitive

A first-order formula over an automatic structure can be compiled to a finite automaton recognizing exactly the satisfying assignments.

This suggests a general project pattern:

```text
logical/property specification
    -> compile to symbolic representation
    -> manipulate representation
    -> extract witness / code / proof
```

The compiled query object can then be composed with other symbolic objects rather than immediately reduced to a Boolean answer.

---

## 10. Current routing hypothesis

When a problem region is detected as belonging to a strong decidable symbolic class, the system should prefer that structural route:

```text
automatic presentation
    -> logic-to-automata decision/query compilation

Presburger/affine integer relation
    -> exact relation algebra / projection / optimization

regular infinite behavior
    -> automata/transducer / coinductive route
```

Only leave the domain for general search when the requested operation or extension escapes the decidable/closed fragment.

---

## 11. New research obligations

1. Study automatic structures beyond strings, including tree-/omega-automatic structures and practical complexity.
2. Investigate semilinear canonical forms and conversions between Presburger formulas, automata, and polyhedral/isl-style relations.
3. Determine how query compilation can emit independently checkable certificates rather than trusting a large automata compiler.
4. Study minimization/canonicalization certificates for automata and Presburger relations.
5. Investigate dynamic routing between SMT, automata-based, polyhedral, and quantifier-elimination backends for the same integer theory.
6. Determine which fragment features cause representation explosion and how Theory Profile can predict them.
7. Investigate whether compiled query automata can be promoted as reusable primitives when they recur across campaigns.
8. Study code generation from exact symbolic relations and translation validation of the generated native loops/kernels.
9. Determine how automatic/Presburger representations participate in theory morphisms and cross-domain transfer.
10. Study whether representation-specific decidability claims themselves can be certificate-bearing/profiled rather than hard-coded by backend name.
