# Research Pass — Relational Propagation, Contractors, and Branch-and-Prune

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates a likely core execution mechanism for relational mathematical semantics: instead of evaluating every construction in a single forward direction, use relation-specific propagators/contractors that repeatedly narrow the admissible values/structures of all unknowns while preserving every valid solution.

---

## 1. Constraint programming treats a constraint as a relation plus a propagator

Constraint programming represents a constraint as a relation over several variables. A propagator/filtering algorithm removes domain values that cannot participate in any satisfying assignment while preserving all actual solutions.

Source:

https://link.springer.com/article/10.1007/s10601-016-9251-0

### Architectural implication

This is directly compatible with the project's relational semantic hypothesis.

A mathematical relation may expose:

```text
relation R(x, y, z)

propagators:
    narrow_x(given domains y,z)
    narrow_y(given domains x,z)
    narrow_z(given domains x,y)
    stronger_global_propagator(...)
```

without requiring an explicit inverse function for every argument.

---

## 2. Propagation computes a fixed point of mutually reinforcing deductions

Practical propagation engines repeatedly execute affected propagators until no domain changes further.

Efficient engines track:

- which propagators are already at fixpoint;
- domain-change events;
- propagator priorities;
- staged/alternative propagators for the same relation.

Source:

https://arxiv.org/abs/cs/0611009

### Architectural implication

The project may need a low-level **Mathematical Propagation Fabric**:

```text
new fact / domain contraction
    -> wake affected relations
    -> run cheapest useful propagators
    -> derive more contractions
    -> fixed point
```

This is much cheaper than relaunching a full solver after each discovered fact.

---

## 3. Some problem classes are decidable by propagation alone

There are structural/language-based classes of constraint problems where generalized arc consistency (GAC) propagation is sufficient to decide feasibility without search.

Source:

https://link.springer.com/article/10.1007/s10601-016-9251-0

### Architectural implication

Theory Profile should record when a relation family/structure has a **complete propagation regime**.

Possible profile:

```text
propagation:
    sound: yes
    complete_for_fragment: F
```

Then a problem inside `F` can be solved by fixed-point propagation instead of branch/search.

---

## 4. Propagation strength is relation-specific and representation-specific

A global constraint often has a specialized propagator stronger than decomposing it into many small primitive constraints.

Recent 2026 work on difference constraints, for example, constructs a global propagator using graph shortest-path structure and reports benefits over treating each difference inequality independently.

Source:

https://arxiv.org/abs/2607.20022

### Architectural implication

Primitive promotion may include **global propagators** discovered for families of relations:

```text
many local constraints
    -> discover common global structure
    -> compile global propagator
    -> certify preservation/completeness properties
```

This can be a major source of new mathematical problem-solving capability.

---

## 5. Interval contractors extend propagation to nonlinear real mathematics with guarantees

Interval constraint propagation associates rigorous intervals with variables and repeatedly applies direct/inverse filtering to nonlinear arithmetic relations. Sound contraction removes regions that cannot contain a solution while retaining all valid solutions.

Sources:

https://link.springer.com/article/10.1007/s10703-013-0203-7

https://link.springer.com/book/10.1007/978-3-030-13795-3

Recent complexity analysis:

https://arxiv.org/abs/2603.19965

### Architectural implication

For continuous nonlinear systems, relational propagation can operate on rigorous enclosures rather than point values:

```text
x in X
y in Y
z in Z
R(x,y,z)
    -> contractor
    -> X' subseteq X
       Y' subseteq Y
       Z' subseteq Z
```

with a proof/soundness guarantee that no actual solution was removed.

---

## 6. Branch-and-prune solves residual uncertainty only where contraction stops

Interval/global-constraint solvers repeatedly contract a domain and split/branch only when the remaining region is too broad to decide.

Current interval optimization systems combine propagation, linear relaxations, and branch-and-bound.

Source:

https://link.springer.com/article/10.1007/s10898-024-01449-2

### Architectural implication

The generic relational search loop may often be:

```text
PROPAGATE
    -> if contradiction: close world
    -> if solved: certify solution
    -> otherwise choose high-value split
        -> branch worlds
        -> PROPAGATE again
```

This is much closer to a mathematical reasoning machine than simple function composition.

---

## 7. Direct and inverse propagation can arise from the same arithmetic relation

Floating-point/interval constraint work explicitly uses both direct and inverse filtering algorithms for arithmetic constraints and reaches a joint fixed point.

Source:

https://link.springer.com/article/10.1007/s10601-021-09322-9

### Architectural implication

This supports the project's earlier conclusion:

```text
semantic relation
    !=
forward implementation
```

A relation may compile into many direction-specific propagators that cooperate without any one being the canonical mathematics.

---

## 8. Propagation is sound but frequently incomplete

Interval propagation can prove contradictions extremely quickly that bit-level SMT takes much longer to establish, but may fail to capture relationships that a more expressive solver handles easily.

Source:

https://link.springer.com/article/10.1007/s10703-013-0203-7

### Architectural implication

Propagation should be an early inexpensive narrowing layer, not universal authority unless Theory Profile proves completeness for the relevant fragment.

Possible routing:

```text
cheap sound propagation
    -> if closed: done
    -> otherwise pass reduced residual problem to stronger solver
```

---

## 9. Multiple propagation techniques can cooperate

Modern nonlinear real arithmetic solvers combine:

- interval propagation;
- linearization;
- CAD/cylindrical methods;
- subtropical reasoning;
- virtual substitution;
- conflict-driven frameworks.

Source:

https://link.springer.com/chapter/10.1007/978-3-031-10769-6_7

### Architectural implication

A mathematical relation should be able to advertise several propagators ordered by:

- cost;
- strength;
- exactness;
- representation requirements;
- explanation/certificate ability.

The search economy selects or stages them dynamically.

---

## 10. Explainable propagation connects directly to learned nogoods

Lazy-clause-generation/SMT-style systems require propagators to explain why a narrowing followed from current constraints. Recent global difference-propagator work explicitly constructs such explanations.

Source:

https://arxiv.org/abs/2607.20022

### Architectural implication

A propagation step should ideally return:

```text
new restriction
support/dependencies
explanation/certificate
```

If contraction leads to contradiction, those explanations can be compressed into:

- conflict cores;
- nogoods;
- interpolants;
- assumption-world restrictions.

Thus propagation participates directly in durable mathematical learning.

---

## 11. Propagator scheduling is itself a mathematical search-economy problem

Propagation-engine research shows dynamic priorities and event-specific wakeups substantially influence performance.

Source:

https://arxiv.org/abs/cs/0611009

### Architectural implication

The lowest-level search economy may operate even before Work Cell allocation:

```text
which propagator should run next?
```

based on:

- affected variables/relations;
- predicted contraction strength;
- historical usefulness;
- execution cost;
- completeness/strength tier.

This should remain ephemeral search-control state, distinct from permanent mathematical truth.

---

## 12. Propagators can become compiled mathematical primitives

Suppose the system repeatedly solves a relation family using expensive generic inversion/search and discovers a deterministic narrowing algorithm `P`.

If it establishes:

```text
P never removes a valid solution
```

and optionally stronger completeness/consistency properties, `P` can be promoted as a reusable propagator primitive.

This gives another self-expansion route:

```text
repeated solver behavior
    -> distill relation-specific propagator
    -> certify
    -> compile native
    -> future fixed-point search becomes cheaper
```

---

## 13. Current relational execution hypothesis

A semantic relation may eventually contain or reference:

```text
forward evaluators
inverse/partial evaluators
propagators / contractors
complete decision procedure if available
candidate generators
certificate checkers
native specialized realizations
```

The relational substrate selects/composes these according to what variables/structures are known and what result class is required.

---

## 14. New research obligations

1. Study propagator algebras/compositional semantics so independently developed mathematical propagators cooperate soundly.
2. Investigate automatic synthesis of propagators/contractors from declarative relations.
3. Determine certificate/explanation formats for domain narrowing in algebraic, graph, integer, and interval domains.
4. Study local consistency hierarchies and Theory Profile conditions under which propagation is complete.
5. Investigate propagator strength/cost prediction as a search-economy metric.
6. Study branch-variable/region selection using information gain and proof/disproof estimates.
7. Investigate explanation-based nogood learning across non-Boolean propagator domains.
8. Determine how candidate-space automata/e-graphs can receive restrictions from propagation without materialization.
9. Study incremental propagation across assumption/version worlds with shared semantic state.
10. Investigate promotion of discovered global constraints/propagators as new mathematical primitives.
