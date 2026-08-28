# Research Pass — Factorized Semiring Elimination, FAQ/InsideOut, and Cross-Domain Structural Reuse

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates whether one discovered factorization/decomposition can support many different mathematical queries by combining structural decomposition with semiring-parametric evaluation.

The central finding is unusually strong:

> **The Functional Aggregate Query (FAQ) / InsideOut framework already unifies substantial parts of databases, constraint satisfaction, probabilistic graphical models, matrix/tensor computation, logic, and signal-processing-style computation through factorized functions, aggregate operators, and variable elimination.**

This is one of the closest existing donors to the project's desired cross-domain mathematical execution substrate.

---

## 1. FAQ represents many domains as aggregates of products of factors

The Functional Aggregate Query framework defines a broad problem class in which input functions/factors are combined and variables are aggregated/eliminated according to semiring-like algebraic operations.

Sources:

https://arxiv.org/abs/1703.03147

https://researchconnect.buffalo.edu/en/publications/faq-questions-asked-frequently/

https://simons.berkeley.edu/talks/answering-faqs-csps-pgms-databases-logic-matrix-operations

The authors explicitly connect the same framework to:

- relational databases;
- logic;
- matrix and tensor computation;
- probabilistic graphical models;
- constraint satisfaction;
- signal processing.

### Architectural implication

The unnamed project's semantic substrate should investigate a **factorized query form** in which many client problems reduce to:

```text
variables
factors / relations
aggregation operators
product/composition operator
output variables
```

without embedding domain names such as “database” or “probability” into the core.

---

## 2. InsideOut is variable elimination generalized across domains

InsideOut evaluates FAQ expressions by choosing an elimination order and repeatedly replacing portions of the problem by derived smaller factors.

Source:

https://arxiv.org/abs/1703.03147

### Architectural implication

A general mathematical Work Cell can potentially execute:

```text
select variable/substructure
combine relevant factors
aggregate/eliminate variable
emit residual factor
repeat
```

where the algebra determines what “combine” and “aggregate” mean.

This gives a concrete native execution pattern for factorized relations.

---

## 3. One structure can implement existence, counting, probability, optimization, and more

Earlier semiring research established that changing the evaluation algebra can change the mathematical question while preserving the structural skeleton.

The FAQ framework demonstrates this principle at a much larger cross-domain level.

Conceptually:

```text
same factor graph / hypergraph
    + Boolean algebra
        -> feasibility / logical existence

    + counting semiring
        -> number of solutions

    + sum-product
        -> probability / partition function

    + min-plus / tropical
        -> minimum cost

    + other certified algebra
        -> other aggregate query
```

### Architectural implication

A decomposition should not be stored merely as an execution plan for one operation.

It may become a **reusable mathematical skeleton** with a declared set of admissible evaluation algebras.

---

## 4. Variable ordering is itself a mathematical optimization problem

FAQ/InsideOut performance depends heavily on variable ordering. The framework formally characterizes which variable orders are semantically equivalent and analyzes width measures such as fractional FAQ-width.

Sources:

https://researchconnect.buffalo.edu/en/publications/faq-questions-asked-frequently/

https://fdbresearch.github.io/visitors.html

### Architectural implication

The project must distinguish:

```text
semantic query
```

from:

```text
legal elimination orders
```

from:

```text
best execution order under cost/width model
```

Legality/equivalence can be mathematical and certified.

Order quality is an optimization/search-economy problem.

---

## 5. Width measures connect structure directly to runtime

FAQ complexity is governed by structural width measures related to fractional edge covers / hypertree-style decompositions, not simply by raw variable or factor count.

Sources:

https://researchconnect.buffalo.edu/en/publications/faq-questions-asked-frequently/

https://arxiv.org/abs/1812.09526

### Architectural implication

The Theory Profile should support hypergraph-width properties, not only ordinary treewidth.

Potential fields:

```text
primal_treewidth
incidence_treewidth
fractional_hypertree_width
submodular_width
FAQ_width
representation-specific widths
```

This provides stronger routing information for multi-relational mathematical problems.

---

## 6. Worst-case optimal joins show execution can respect mathematical output bounds

FAQ combines variable elimination with worst-case optimal join algorithms rather than naïve pairwise joining.

Source:

https://arxiv.org/abs/1703.03147

### Architectural implication

The mathematical compiler should search not only for asymptotically good *algorithms* but for execution operators whose intermediate representation cannot explode far beyond structurally necessary bounds when such algorithms exist.

This matters for RAM discipline: avoiding a bad intermediate can be more important than making primitive arithmetic faster.

---

## 7. Semantic equivalence of execution orders is a reusable proof obligation

FAQ research explicitly characterizes when variable orders are semantically equivalent to the original aggregate expression.

### Architectural implication

This suggests a generic separation:

```text
semantic expression E

legal transformation/order proof:
    plan P ≡ E

cost optimization:
    choose best P among legal plans
```

This is exactly aligned with the project's two-proof principle:

- mathematical semantic correctness;
- optimized realization correctness.

---

## 8. Cross-domain algorithms can emerge automatically from specialization

The FAQ literature reports that specializations recover known algorithms across several domains; the Simons Institute description notes specializations including exact probabilistic inference and FFT-related computation.

Source:

https://simons.berkeley.edu/talks/answering-faqs-csps-pgms-databases-logic-matrix-operations

### Architectural implication

A sufficiently general factorized relation plus algebra may specialize into algorithms humans would normally classify under completely different fields.

This is strong evidence for the project's principle:

> **Internal mathematics should be organized by structural semantics, not by human academic subject labels.**

---

## 9. Factorized structure can be compiled once and queried many times

Knowledge compilation and FAQ-style execution point toward a two-stage architecture:

```text
expensive structural compilation/decomposition
    -> reusable factorized representation

many later queries
    -> cheap algebra-specific evaluation
```

Recent semiring dynamic-programming work reinforces the idea that semiring extensions can often be added without changing the asymptotic complexity of the underlying structural algorithm.

Source:

https://www.sciencedirect.com/science/article/pii/S0166218X25007462

### Architectural implication

Primitive promotion may include a pair:

```text
structural primitive
    reusable decomposition/factor skeleton

algebra evaluator
    certified semiring/aggregate semantics
```

The Cartesian combination of these can create many capabilities without storing each as a separately hand-designed solver.

---

## 10. Factorization can outperform materializing intermediate reality

FAQ/InsideOut and database join theory highlight a powerful execution law:

> Do not materialize a huge intermediate relation/tensor/search set if the final aggregate can be computed directly over its factorized representation.

### Architectural implication

The project's compiler should aggressively ask:

```text
Can this aggregate be pushed through the factorization?
Can variables be eliminated before materialization?
Can a compact symbolic factor replace explicit enumeration?
```

This directly supports the project's ordinary-hardware target.

---

## 11. Current factorized-semiring hypothesis

A major executable region of the project may eventually look like:

```text
SEMANTIC FACTOR HYPERGRAPH
    variables
    factors/relations
    constraints

QUERY ALGEBRA
    certified combine/aggregate laws

THEORY PROFILE
    decomposition/width/closure properties

PLAN COMPILER
    choose semantically valid elimination/decomposition order

EXECUTION
    InsideOut / worst-case-optimal join / specialized kernels

RESULT
    existence / count / probability / optimum / provenance / ...
```

The mathematical identity remains independent of the chosen execution order.

---

## 12. New research obligations

1. Determine how much of the project's relational semantic core can be normalized into FAQ-like factorized queries.
2. Study cases that FAQ cannot naturally represent and identify the required extension boundary.
3. Investigate semiring/aggregate laws needed for safe reordering and elimination.
4. Map FAQ hypergraphs against the emerging semantic e-hypergraph substrate.
5. Study semiring-parametric compiled decompositions that retain domain-native certificates.
6. Investigate incremental updates: when one factor changes, how much of the compiled elimination structure can be reused?
7. Study automatic algebra selection from requested result type.
8. Investigate exact execution over sparse/compact factors without materialization.
9. Determine how factorized query plans interact with assumption worlds/versioned equality.
10. Study whether newly discovered mathematical relations can be automatically factorized into a form that unlocks FAQ/InsideOut-style execution.
