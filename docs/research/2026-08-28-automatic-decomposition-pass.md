# Research Pass — Automatic Decomposition, Separators, Treewidth, and Reusable Factorization

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how the unnamed mathematical project can automatically discover structure that splits one difficult problem into independent or weakly coupled subproblems, creating AND/OR campaign structure rather than requiring decomposition to be supplied manually.

The central finding is:

> **Discovering the right decomposition can change the complexity class of practical search, reduce exponential search dramatically, expose parallel work cells, and produce a reusable compiled representation for future queries.**

---

## 1. AND/OR graphical-model search makes independence explicit

AND/OR search spaces for graphical models expose conditional independencies that ordinary OR-only search hides. In suitable structures the resulting search graph can be exponentially smaller.

Source:

https://www.sciencedirect.com/science/article/pii/S000437020600138X

The framework applies across probabilistic networks and constraint networks, with treewidth/pathwidth-related structure governing complexity.

### Architectural implication

The problem compiler should actively seek:

```text
independent components
conditional independencies
separator sets
cut variables
factorization
```

and turn them into AND/OR mathematical campaign structure.

Decomposition is therefore itself a metaprimitive/discovery obligation.

---

## 2. Treewidth identifies where combinatorial difficulty lives

Tree decompositions represent a graph as overlapping small bags arranged in a tree. Many otherwise hard dynamic-programming computations become tractable when treewidth is small.

Tensor-network research gives an unusually concrete correspondence: contraction complexity is tied to the treewidth of a related line graph.

Sources:

https://pmc.ncbi.nlm.nih.gov/articles/PMC6298732/

https://academic.oup.com/comjnl/article/69/4/613/8555574

### Architectural implication

Theory/Profile analysis should search not only for a problem class but also for **structural width parameters**:

```text
treewidth
pathwidth
branchwidth
separator size
hypertree width
other domain-specific width
```

A small width certificate may justify an entirely different exact algorithm.

---

## 3. Separators create natural parallel work cells

A separator divides a problem graph/network into components that interact only through the separator boundary.

Tensor-network contraction algorithms explicitly use separators and contraction trees to split large networks into smaller subnetworks and later combine them.

Sources:

https://www.nature.com/articles/s43588-021-00119-7

https://academic.oup.com/comjnl/article/69/4/613/8555574

### Architectural implication

A discovered separator can compile directly into:

```text
boundary state B
    -> component C1
    -> component C2
    -> ...
    -> combine through B
```

Each component can become an independent Work Cell or nested campaign once the boundary conditions are fixed.

This is a mathematically justified source of parallelism rather than generic task splitting.

---

## 4. Decomposition order itself may dominate runtime

Tensor-network contraction demonstrates that the mathematical result is invariant while the chosen contraction/elimination order can change runtime and memory enormously.

Sources:

https://journals.plos.org/plosone/article?id=10.1371%2Fjournal.pone.0207827

https://epubs.siam.org/doi/10.1137/23M161286X

Optimal ordering is hard in general, but special structures such as trees admit polynomial algorithms.

### Architectural implication

The search economy should distinguish:

```text
mathematical decomposition identity
```

from:

```text
execution/contraction/elimination order
```

The latter is an operational optimization artifact and can be searched/benchmark-driven without becoming mathematical truth authority.

---

## 5. Knowledge compilation turns decomposition into a reusable object

d-DNNF and related knowledge-compilation languages exploit **decomposability** so many later queries become efficient.

Recent results show fixed-parameter compilation based on incidence treewidth and distributed compilation through independent partitions.

Sources:

https://www.ijcai.org/proceedings/2024/367

https://proceedings.kr.org/2026/74/

https://arxiv.org/abs/2607.13642

### Architectural implication

After an expensive decomposition/discovery campaign, the project may retain a compiled mathematical representation so later tasks such as:

```text
count
sample
condition
optimize
query existence
```

reuse the same structural work.

This is another form of primitive promotion: not only new formulas/algorithms, but **new compiled factorized representations**.

---

## 6. Decomposability and determinism create tractable query algebras

In d-DNNF, decomposable conjunctions do not share variables; deterministic disjunctions have disjoint model sets. Those structural properties enable efficient model counting and other queries.

Sources:

https://www.logicng.org/documentation/knowledge-compilation/dnnf/

https://proceedings.kr.org/2024/48/

### Architectural implication

The project should profile candidate representations for algebraic properties such as:

```text
independence/decomposability
determinism/disjointness
sharing/reuse
closure under conditioning
closure under projection
```

because the same mathematical content may become dramatically more useful after compilation into a structurally tractable form.

---

## 7. Decomposition can expose exact repeated-query capability

A difficult problem may be expensive to compile once but cheap to query repeatedly afterward.

Current knowledge-compilation work explicitly treats expensive offline compilation as amortized across many online queries, including current work extending d-DNNF ideas to SMT-level queries.

Source:

https://arxiv.org/abs/2603.09975

### Architectural implication

The search economy should estimate **future reuse value** when deciding whether expensive decomposition/compilation is worthwhile.

A transformation that costs far more than one direct solve may still be optimal if it produces a reusable structure used by thousands of later problems.

---

## 8. Dynamic decomposition can arise during search

Assignments, newly proved equalities, propagated constraints, or representation changes can disconnect a problem that was originally coupled.

This means decomposition should not be a one-time front-end pass.

### Architectural implication

Campaigns should periodically/event-driven check whether current evidence has created new:

```text
connected components
separators
conditional independencies
factorization opportunities
```

A new decomposition can restructure the active AND/OR graph and reassign work without changing the underlying mathematical truth.

---

## 9. Decomposition discovery should itself be certifiable

A claimed decomposition must establish that solving/combining the pieces is equivalent to solving the original problem under the declared boundary variables/assumptions.

The certificate need not prove that the decomposition is *optimal*.

It must prove that it is **sound**.

Possible claims:

```text
components independent given separator S

F = combine(F1, F2, ..., S)

solution_set(F) corresponds exactly to composition of child solution sets
```

Optimality of the chosen separator/order is a separate operational property.

---

## 10. Width/structure discovery is another representation search

Earlier research established that changing representation may make a hard problem easy.

This pass adds:

> A representation should also be judged by the decomposition structure it exposes.

One representation may have poor treewidth/coupling while an equivalent representation reveals sparse or separable structure.

### Architectural implication

Representation search objectives should include metrics such as:

```text
separator quality
induced width
factor sparsity
component count
shared-boundary size
expected decomposition reuse
```

not only expression size or native execution cost.

---

## 11. Current decomposition hypothesis

```text
PROBLEM / CURRENT MATHEMATICAL WORLD
    -> construct structural dependency/hypergraph views
    -> detect exact components / separators / conditional independence
    -> estimate width and cost
    -> certify chosen decomposition
    -> rewrite campaign as AND/OR/factor graph
    -> solve/compile components independently
    -> combine certificates/results
    -> retain reusable factorized representation when valuable
```

This is one of the most direct ways to turn additional mathematical understanding into multiplicative compute reduction.

---

## 12. New research obligations

1. Study hypergraph and hypertree decompositions because mathematical relations are naturally multi-input/multi-output.
2. Investigate automatic separator discovery with exact certificates.
3. Study dynamic decomposition under propagation/assumption-world updates.
4. Connect treewidth/width profiles to the parameterized-complexity Theory Profile.
5. Investigate decomposition of arithmetic-circuit/e-hypergraph representations rather than only variable graphs.
6. Study reusable compiled decompositions that support several semiring evaluations.
7. Determine how decomposition certificates compose with child proof/certificate envelopes.
8. Investigate optimal or near-optimal decomposition search as a Search Economy campaign in its own right.
9. Study conditions where decomposition overhead is provably not worthwhile.
10. Investigate cross-domain structural transfer: whether a decomposition primitive discovered in one theory can be transported through a certified theory morphism.
