# Research Pass — Parameterized Complexity, Kernelization, and Structural Tractability

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates the difference between a problem being globally hard and a concrete/problem-family instance becoming tractable once the correct structural parameter is exposed.

The central result is:

> **Complexity should be profiled as a function of mathematical structure, not only total input size.**

---

## 1. Fixed-parameter tractability separates global size from structural difficulty

A parameterized problem is fixed-parameter tractable (FPT) with respect to parameter `k` when it can be solved in time roughly:

```text
f(k) * n^c
```

where `c` is independent of `k`.

Sources:

https://link.springer.com/book/10.1007/3-540-29953-X

https://link.springer.com/book/10.1007/978-3-319-21275-3

### Architectural implication

A Theory/Profile cost model should not be limited to:

```text
input_size = n
complexity = O(g(n))
```

It should allow structural parameters:

```text
input size n
parameter vector k = [treewidth, dimension, rank, degree, symmetry quotient size, ...]
known complexity = f(k) * n^c
```

A huge problem with tiny structural parameter may be easy.

A small problem with a toxic parameter may be difficult.

---

## 2. Courcelle-style results show representation structure can collapse NP-hard graph problems

Courcelle's theorem implies that broad classes of MSO-definable graph properties can be decided in fixed-parameter tractable time on graphs of bounded treewidth. This includes properties that are NP-complete on unrestricted graphs, such as Hamiltonicity and 3-colorability.

Representative source:

https://link.springer.com/article/10.1007/s00453-022-00939-7

### Architectural implication

A problem-class label such as:

```text
NP-hard graph problem
```

is not sufficient for scheduling.

The system should first ask:

```text
Does the instance expose bounded treewidth / pathwidth / cliquewidth / another useful decomposition?
```

If yes, a structural route may replace generic exponential search.

This reinforces representation/decomposition search as a first-class work obligation.

---

## 3. Kernelization compresses a problem to an equivalent hard core

Kernelization transforms `(instance, parameter)` into an equivalent instance whose size is bounded by a function of the parameter.

Sources:

https://link.springer.com/chapter/10.1007/978-3-031-21534-6_6

https://link.springer.com/article/10.1186/s13673-020-00226-w

### Architectural implication

This is another concrete realization of the project's desired mathematical compression loop:

```text
large problem P
    -> certified reduction rules
    -> kernel K

solve(P) <=> solve(K)
```

where `K` ideally contains only the structurally difficult residue.

Possible metaprimitive family:

```text
kernelize(problem, parameterization)
```

with equivalence/preservation certificates for each reduction rule or the overall reduction.

---

## 4. Branch-and-reduce combines exact compression and selective search

Practical exact algorithms often repeatedly apply safe reduction rules until no useful reduction remains, then branch only on the remaining hard core, and repeat reductions in each branch.

Source:

https://link.springer.com/chapter/10.1007/978-3-031-21534-6_6

### Architectural implication

The mathematical campaign loop may frequently look like:

```text
canonicalize / reduce / kernelize
    -> branch only when necessary
    -> learn conflict/nogood
    -> reduce each branch again
```

rather than performing one preprocessing stage followed by one large search stage.

This fits naturally with the project's event-driven search-economy architecture.

---

## 5. The choice of parameter can completely change tractability

The same problem can be FPT under one structural parameter and W-hard/para-NP-hard under another. Parameterized-complexity studies routinely build fine-grained classification tables showing which parameters actually expose tractable structure.

Representative example:

https://link.springer.com/article/10.1007/s00224-021-10045-w

### Architectural implication

The project should not assume a human has already supplied the right parameterization.

A valuable research/work-cell family is:

```text
search_structural_parameters(problem)
```

Possible candidates include:

- treewidth/pathwidth/cliquewidth;
- solution size;
- rank;
- dimension;
- number of exceptional constraints;
- feedback vertex/set size;
- number of nonlinear variables;
- algebraic degree;
- sparsity width;
- symmetry quotient size;
- distance from a tractable class.

A discovered useful parameterization can itself become reusable theory knowledge.

---

## 6. Parameterized hardness is useful negative knowledge

W-hierarchy/parameterized lower-bound results show that some apparently natural parameters are unlikely to yield FPT algorithms under standard complexity assumptions.

Sources:

https://link.springer.com/book/10.1007/3-540-29953-X

https://link.springer.com/book/10.1007/978-3-319-21275-3

### Architectural implication

The Theory Graph should preserve not only successful tractability results but negative parameter knowledge:

```text
parameter k1 -> FPT known
parameter k2 -> W[1]-hard
parameter k3 -> no polynomial kernel under assumption X
```

This stops future campaigns from repeatedly investing heavily in parameterizations already known to be structurally unhelpful.

---

## 7. Kernel lower bounds constrain mathematical compression

Some FPT problems provably do not admit polynomial kernels unless widely believed complexity-theoretic collapses occur.

Representative discussion:

https://link.springer.com/book/10.1007/978-3-319-21275-3

### Architectural implication

The project's compression/generalization engine must admit:

```text
no compact kernel of requested class is expected under current complexity assumptions
```

rather than treating failure to find a small equivalent representation as merely a weak search effort.

Complexity-theoretic lower bounds are another type of durable negative mathematical knowledge.

---

## 8. Parameter discovery is closely related to representation discovery

A structural parameter often describes how far an object is from a representation where known efficient mathematics applies.

Examples:

```text
graph
    -> tree decomposition width

constraint system
    -> number of nonlinear dimensions

matrix
    -> rank / sparsity / bandwidth

logical theory
    -> quantifier alternation / number of exceptional symbols
```

### Architectural implication

The representation-search engine should be able to return not only:

```text
better representation R
```

but:

```text
parameterization k
+ decomposition/certificate D
+ algorithm family valid for k
```

This can route the problem into an FPT/specialized solver.

---

## 9. Decomposition certificates can become executable structure

Tree decompositions and related structural witnesses are concrete objects consumed by dynamic-programming/automata algorithms.

### Architectural implication

The project should treat structural decompositions as first-class mathematical artifacts:

```text
Decomposition
    source object
    parameter value/bound
    bags/parts/factors
    coverage/consistency obligations
    certificate/checker
    compatible solver families
```

Once checked, many work cells can share the same decomposition instead of recomputing it.

---

## 10. Parameterized tractability fits Theory Profile naturally

A stronger Theory Profile can contain:

```text
unparameterized complexity/hardness
known parameterizations
current instance parameter values/bounds
FPT/XP/hardness classification
kernel availability
known decomposition algorithms
certificate routes
```

The search economy can then estimate cost using structural bounds rather than naïve problem size.

---

## 11. Current structural-tractability routing hypothesis

```text
Problem P
    -> computability profile
    -> detect tractable exact fragment
    -> detect useful parameterization/decomposition
    -> kernelize/reduce if available
    -> run FPT/specialized structural solver
    -> generic campaign only on residual hard core
```

This should precede unconstrained brute-force or generic synthesis.

---

## 12. New research obligations

1. Build a catalogue of structural parameters relevant beyond graphs: algebra, polynomial systems, optimization, theorem proving, program synthesis, numerical systems.
2. Study automatic parameter detection and decomposition algorithms with independently checkable certificates.
3. Investigate kernelization certificates and translation validation of reduction rules.
4. Determine how the system should search for novel parameterizations from repeated successful decompositions.
5. Study parameterized complexity of candidate-space/equality-saturation structures themselves.
6. Investigate treewidth/hypertree-width of mathematical dependency/e-hypergraphs as a predictor of proof/search cost.
7. Study parameterized algorithms for theory-morphism discovery and candidate abstraction learning.
8. Integrate parameter values into search-economy value/cost prediction.
9. Preserve conditional complexity lower bounds and assumptions as certificate-bearing Theory Graph knowledge.
10. Investigate whether primitive promotion should include the parameterized algorithm together with its decomposition/kernelization route rather than only a monolithic solver.
