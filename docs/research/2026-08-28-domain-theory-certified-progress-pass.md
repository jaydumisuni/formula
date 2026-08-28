# Research Pass — Domain Theory, Partial Computation, and Certified Mathematical Progress

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates whether a long-running mathematical computation can expose *mathematically meaningful partial progress* rather than only returning either a final answer or `UNKNOWN`.

The central finding is:

> **In many domains, progress can itself be a mathematical object: an increasing information state, a shrinking certified enclosure, a tightening lower/upper bound pair, or a lattice-theoretic progress measure. But there is no single universal progress scalar.**

---

## 1. Domain theory models computation as increasing information

Domain theory equips computational objects with a partial order whose direction represents increasing information. An element can represent an incomplete result; higher elements contain more information.

A classic example is the interval domain for exact real arithmetic. A real number is approximated by nested rational intervals, ordered by reverse inclusion:

```text
[a,b] ⊑ [c,d]
```

when `[c,d]` is a tighter enclosure and therefore carries more information.

Sources:

https://www.sciencedirect.com/topics/mathematics/domain-theory

https://www2.mathematik.tu-darmstadt.de/~logik/research/Domains/Whats.html

https://www.cambridge.org/core/journals/bulletin-of-symbolic-logic/article/abs/domains-for-computation-in-mathematics-physics-and-exact-real-arithmetic/160ABCD37E837E7A9437745930411D30

### Architectural implication

A mathematical relation/construction may optionally expose an **information order**:

```text
progress_domain: D
information_order: ⊑
```

Then a computation can produce certified states

```text
I0 ⊑ I1 ⊑ I2 ⊑ ...
```

rather than only a terminal value.

This gives the system a rigorous meaning for “more progress” whenever the domain supplies such an order.

---

## 2. Partial recursive programs can still have exact mathematical semantics

Lean's current `partial_fixpoint` feature is explicitly built on least fixed points of monotone functions over chain-complete partial orders. Recursive equations can therefore remain usable in logic even when they are not justified by ordinary structural/well-founded termination.

Sources:

https://lean-lang.org/doc/reference/latest/Definitions/Recursive-Definitions/

https://lean-lang.org/doc/api/Init/Internal/Order/Basic.html

Lean's construction uses an approximation relation and a least-fixed-point theorem for monotone functions.

### Architectural implication

The executable-semantics taxonomy should distinguish:

```text
TOTAL
PARTIAL_WITH_SEMANTIC_DOMAIN
OPAQUE_PARTIAL_RUNTIME
```

A partial construction is admissible as mathematics only when its partiality has a declared mathematical semantics/domain or when it is explicitly marked operational/opaque rather than truth-authoritative.

---

## 3. Exact real arithmetic demonstrates certified refining computation

Exact-real systems can represent a real value by an infinite stream or by successively tighter rational intervals. Finite computation produces finite but certified information; additional computation can refine the result to arbitrary precision.

Sources:

https://www.sciencedirect.com/science/article/abs/pii/S0304397522005849

https://www.cl.cam.ac.uk/research/ls/Talks/1997_98/97_11_14.Abstract.html

https://www.researchgate.net/publication/226645324_From_Coinductive_Proofs_to_Exact_Real_Arithmetic

The 2023 work on exact-real lookahead is especially relevant because the amount of input required for a specified amount of output precision is carried explicitly in the specification.

### Architectural implication

A mature primitive may expose a **refinement contract**:

```text
request_precision(p)
    -> certified enclosure E_p
```

with guarantees such as:

```text
exact_value ∈ E_p
E_(p+1) refines E_p
```

The project can stop when the client's requested precision/evidence threshold is reached rather than insisting on an impossible finite exact numeral representation.

---

## 4. Optimization has a different but equally mathematical progress order

Exact branch-and-bound maintains valid lower and upper bounds on the unknown optimum. A shrinking gap gives a rigorous quality guarantee even before optimality is established.

Sources:

https://gamma-opt.github.io/linopt-notes/pdfs/linopt-notes.pdf

https://www.researchgate.net/publication/220532141_A_Framework_for_Certified_Boolean_Branch-and-Bound_Optimization

An optimality certificate can combine a feasible incumbent with a certified lower bound. Equality of the bounds proves optimality.

### Architectural implication

Optimization work can expose a state like:

```text
lower_bound: L
upper_bound: U
incumbent: x
certificates: ...
```

with progress defined by:

```text
L increases
U decreases
```

and completion when the required gap closes.

This is a mathematically certified anytime result rather than heuristic confidence.

---

## 5. Fixed-point systems have explicit progress measures

Lattice-theoretic and parity-game research defines **progress measures** as mathematical witnesses guiding/characterizing least/greatest fixed-point solutions.

Sources:

https://doi.org/10.1145/3290339

https://arxiv.org/abs/1511.00346

https://wrap.warwick.ac.uk/id/eprint/87818/

Progress measures generalize concepts related to ranking functions and invariants and can themselves be characterized through fixed-point constructions.

### Architectural implication

For suitable fixed-point theories, campaign progress can be expressed through a certified progress-measure object rather than generic counters such as “nodes searched.”

This is particularly relevant to recursive verification, reachability, coinductive reasoning, and nested least/greatest fixed-point problems.

---

## 6. Operational improvement is not always semantic information improvement

Abstract interpretation uses widening to force convergence of otherwise long/infinite ascending chains. Widening may intentionally sacrifice precision, followed by narrowing to regain some precision.

Sources:

https://www.sciencedirect.com/science/article/pii/S1477842410000254

https://link.springer.com/article/10.1007/s10817-026-09756-x

### Architectural implication

The scheduler must distinguish:

```text
SEMANTIC INFORMATION PROGRESS
    result becomes mathematically more informative

OPERATIONAL PROGRESS
    algorithm moves toward termination/coverage but may temporarily lose precision
```

A widening step can be operationally useful without being an upward move in the result's information order.

Therefore the project must not derive truth-status changes from runtime progress metrics alone.

---

## 7. There should be no universal scalar `progress = 73%`

Different mathematical domains admit incomparable notions of progress:

```text
exact real:
    enclosure width / information refinement

optimization:
    certified lower-upper gap

fixed-point proof:
    progress measure

constraint propagation:
    domain contraction

candidate-space synthesis:
    eliminated version-space region

proof search:
    certified lemmas / cores / obligations closed
```

Some domains may expose several progress dimensions simultaneously.

### Architectural implication

A Theory Profile may optionally declare:

```text
progress_semantics:
    type: partial_order | bounds | measure | none
    direction: ...
    certificate_route: ...
    completion_condition: ...
```

The search economy can reason about these contracts, but should not pretend unrelated domains share one numeric progress metric.

---

## 8. Certified partial results can be first-class outputs

The result taxonomy should distinguish:

```text
FINAL_CERTIFIED
PARTIAL_CERTIFIED
BOUND_CERTIFIED
REFINING_CERTIFIED
OPEN_NO_CERTIFIED_PROGRESS
```

Example:

```text
result_state: REFINING_CERTIFIED
value_enclosure: [a,b]
refinement_order: reverse_inclusion
certificate: ...
next_refinement_available: yes
```

This is stronger than returning a heuristic intermediate result and weaker than claiming the final mathematical object is fully resolved.

---

## 9. Search economy can consume mathematical progress evidence

Earlier scheduling research proposed value-of-computation, information gain, proof/disproof estimates, fairness, and portfolio selection.

This pass adds a stronger input:

```text
mathematically certified progress state
```

A work cell can report not merely:

```text
100000 nodes explored
```

but, where available:

```text
certified enclosure shrank by factor 8
optimality gap fell from 0.12 to 0.01
version space lost 40% of admissible classes
fixed-point progress measure advanced on 300 obligations
```

These metrics are domain-native and can influence scheduling without becoming mathematical truth themselves.

---

## 10. Progress dominance may be a partial order, not a total ranking

Two partial results may be incomparable.

For example one work cell may produce:

```text
better lower bound
```

while another produces:

```text
better feasible incumbent
```

Neither necessarily dominates the other until combined.

### Architectural implication

The scheduler should support **Pareto/partial-order progress states** rather than requiring every mathematical state to collapse to one score.

A separate operational utility model may rank incomparable states for resource allocation, but the underlying semantic ordering remains explicit.

---

## 11. Current semantic-progress hypothesis

The strongest current model is:

```text
MATHEMATICAL DOMAIN
    -> optional information/progress structure

WORK CELL
    -> computation
    -> certified partial state P_n

P_n
    -> semantic comparison with P_(n-1)
    -> progress / incomparable / regression-invalid

SEARCH ECONOMY
    -> combine semantic progress evidence
       with cost, information gain, portfolio/fairness policy
    -> allocate next compute
```

This is the first concrete mechanism connecting the mathematical semantics themselves to campaign scheduling.

---

## 12. New research obligations

1. Study generalized information orders beyond intervals and optimization bounds.
2. Investigate certified progress states for symbolic theorem proving and program synthesis.
3. Determine how branch-and-bound, interval refinement, version spaces, and fixed-point progress measures fit a common progress-envelope schema without erasing domain semantics.
4. Study Pareto/frontier scheduling when progress states are partially ordered and incomparable.
5. Investigate compositional progress: how progress in independent subproblems combines into progress of an AND/OR mathematical goal.
6. Study progress certificates whose verification is significantly cheaper than recomputing the underlying work.
7. Determine how widening/abstraction steps report operational progress without being mistaken for semantic refinement.
8. Investigate whether proof obligations themselves can form an information lattice suitable for certified progress.
9. Study resource-aware exact-real algorithms whose lookahead/precision contracts can be used by the search economy.
10. Define stop policies based on client-required certified information rather than generic wall-clock or iteration limits.
