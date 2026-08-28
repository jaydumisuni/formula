# Research Pass — Semiring-Parametric Evaluation and Reusable Mathematical Skeletons

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates a recurring pattern across probabilistic inference, graph algorithms, parsing, provenance, optimization, and symbolic computation:

> **The same structural computation can often answer different mathematical questions by evaluating it in a different algebraic structure.**

The strongest conclusion is:

> **The project should distinguish a reusable structural computation/skeleton from the algebra used to evaluate that skeleton. A certified algebra instance can turn one compiled structure into several different problem-solving primitives without duplicating the search topology.**

This is not a claim that all mathematics is semiring computation. It is a powerful common fragment that appears across many domains.

---

## 1. Algebraic Model Counting generalizes weighted counting by a semiring

Algebraic Model Counting (AMC) generalizes weighted model counting by interpreting models in an arbitrary semiring rather than only ordinary numeric probability weights.

AMC unifies tasks in areas including:

- probabilistic inference;
- soft constraints;
- database/network analysis;
- other weighted logical queries.

Source:

https://arxiv.org/abs/1211.4475

Knowledge compilation lets the logical structure be compiled once into circuits such as sd-DNNF and then evaluated according to the chosen algebra when the required semiring conditions hold.

### Architectural implication

A problem can separate into:

```text
STRUCTURAL SKELETON
    which worlds/solutions/dependencies exist

EVALUATION ALGEBRA
    how alternatives combine
    how conjunction/composition combines
```

The same structural representation can potentially be reused across several quantitative queries.

---

## 2. Semiring parsing proves one algorithmic skeleton can compute many outputs

Goodman's semiring parsing framework describes a parser once and changes the semiring to compute different quantities such as:

- recognition;
- derivation forests;
- inside probabilities;
- Viterbi/best derivations;
- n-best related values;
- other aggregate quantities.

Source:

https://aclanthology.org/J99-4004/

### Architectural implication

A generic mathematical dynamic program should not necessarily hard-code its result algebra.

Possible abstraction:

```text
Deductive/structural recurrence
    parameterized by
EvaluationAlgebra A
```

Then the same recurrence can answer distinct questions by supplying a certified `A`.

This supports the project's general search-space algebra: some constructions are **algebra-parametric families**, not single-purpose functions.

---

## 3. GraphBLAS uses semirings to unify graph algorithms with sparse linear algebra

GraphBLAS represents graphs as sparse matrices/vectors and generalizes the scalar operations in matrix multiplication to semiring operations.

Changing the semiring enables the same sparse-linear-algebra primitives to express many graph computations.

Sources:

https://graphblas.org/

https://graphblas.org/graphblas-api-cpp/

https://graphblas.org/docs/GraphBLAS_API_C_v2.0.0.pdf

### Architectural implication

A structural operation such as generalized sparse matrix multiplication can become a reusable native primitive whose mathematical behavior is parameterized by algebraic operations/laws.

This is especially attractive for CPU/GPU compilation:

```text
semantic generalized operation
    + concrete semiring witness
    + sparse/dense representation
        -> specialized native kernel
```

The same high-level primitive can target graph traversal, reachability, path-cost problems, and other computations.

---

## 4. The semiring determines the question, not only implementation details

Examples conceptually include:

```text
Boolean semiring
    alternatives = OR
    composition = AND
    -> existence/reachability

Natural-number semiring
    alternatives = +
    composition = ×
    -> counting

Probability/sum-product
    -> total probability

Tropical/min-plus
    alternatives = min
    composition = +
    -> shortest/minimum-cost structure

Viterbi/max-product
    -> most probable score

Provenance polynomial semiring
    -> derivation/support information
```

### Architectural implication

Changing the evaluation algebra is a **semantic transformation of the query/result**, not merely swapping a faster implementation.

The project must represent:

```text
Skeleton S
Evaluation algebra A
Query semantics Q(S,A)
```

rather than pretending all evaluations are the same mathematical construction.

---

## 5. Structure laws determine which rewrites/evaluations are sound

A semiring supplies laws including associativity/distributivity and identity elements; additional properties such as commutativity or idempotence may or may not hold.

AMC/knowledge-compilation results explicitly identify algebraic properties that determine which circuit transformations/evaluation schemes are valid.

Source:

https://arxiv.org/abs/1211.4475

### Architectural implication

An optimizer must query structure witnesses:

```text
CommutativeAddition(A)?
IdempotentAddition(A)?
CommutativeMultiplication(A)?
Distributive(A)?
ZeroAnnihilates(A)?
```

before applying algebraic rewrites.

This directly integrates semiring-parametric execution with the mathematical structure/type system.

---

## 6. A compiled structural object can support multiple algebras without recompiling topology

Recent probabilistic-programming work shows the same compiled BDD can support exact, interval/imprecise, and differentiable inference by changing semiring-like evaluation machinery.

Source:

https://arxiv.org/abs/2607.20801

### Architectural implication

The project may cache two levels separately:

```text
STRUCTURAL COMPILE
    expensive decomposition / knowledge compilation

ALGEBRA SPECIALIZATION
    cheap or specialized evaluation over that structure
```

This can dramatically reduce repeated problem-solving work when clients ask multiple quantitative questions about the same mathematical structure.

---

## 7. Tensor-network/model-counting research shows evaluation algebra can switch optimization mode

2024 work on tensor-network model counting notes that the same tensor contraction structure can perform probability-of-evidence under sum-product and maximization under tropical/max-sum or Viterbi/max-product semirings.

Source:

https://proceedings.mlr.press/v246/wenig24a.html

### Architectural implication

Representation discovery may search for a compact factorization/tensor network **once**, then expose several derived query modes:

```text
sum
max
count
probability
sensitivity
```

The costly discovery of structure becomes reusable mathematical capital.

---

## 8. Provenance and numeric evaluation may share the same skeleton

Database provenance research and algebraic counting both use semiring annotations to track how results depend on inputs.

Earlier project research identified provenance semirings as a possible compact support representation.

### Architectural implication

A structural calculation can potentially be run simultaneously or separately in:

```text
value algebra
provenance algebra
```

so one computation yields both:

- the result;
- an algebraic support/derivation object.

However, provenance under recursive/infinite derivations remains subtle and must preserve the earlier provenance caveats.

---

## 9. Weighted automata show state-machine semantics can also be algebra-parametric

Weighted automata generalize ordinary automata by attaching weights from algebraic structures such as semirings; tropical-semiring automata, for example, encode min-plus quantitative behavior.

Recent 2026 work continues to study determinization/minimization properties for tropical weighted automata.

Source:

https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.CONCUR.2026.10

### Architectural implication

The project's automatic/infinite-space representations may eventually generalize from:

```text
accept/reject automata
```

to:

```text
weighted/algebraic automata
```

where the same transition skeleton supports richer quantitative mathematics.

---

## 10. Algebraic path problems unify many graph computations

GraphBLAS references the algebraic path problem, in which different semirings give different meanings to aggregation over graph paths.

Sources:

https://graphblas.org/GraphBLAS-Pointers/

https://graphblas.org/

### Architectural implication

A relation graph inside the unnamed project may admit several exact path queries without designing separate algorithms for each one:

```text
Does a path exist?
How many paths?
What is the minimum-cost path?
What is the maximum-reliability path?
What provenance supports reachability?
```

when the required algebraic conditions and finiteness/convergence assumptions hold.

---

## 11. Semiring-parametric computation can become a primitive generator

Suppose the system discovers a structural recurrence `S` that is valid over any algebra satisfying interface `AReq`.

Instead of promoting one primitive:

```text
solve_boolean_S
```

it may promote a generic theorem/construction:

```text
S : forall A satisfying AReq, Evaluation(A)
```

### Architectural implication

One mathematical discovery can unlock an entire **family of executable primitives** as new algebra witnesses appear.

This is highly compatible with structure inference:

```text
new algebra A certified
    -> generic skeleton S becomes applicable
    -> specialize S[A]
    -> compile native realization
```

A new structure witness therefore creates algorithmic capability automatically.

---

## 12. Semiring polymorphism should not become generic-runtime overhead

GraphBLAS and other systems demonstrate that abstract algebraic operations can ultimately map to optimized sparse/native kernels.

### Architectural implication

The desired path is:

```text
Generic skeleton S<A>
    + fixed certified algebra A0
    + fixed representation/layout R
        -> specialize
        -> inline algebra operations
        -> eliminate witness dispatch
        -> vectorize/parallelize where lawful
        -> native kernel
```

The mathematical abstraction exists during construction/proof; the hot path can be monomorphic and near-native.

---

## 13. Multiple evaluation algebras can become a representation-search tool

A structural object may reveal useful properties under one algebra that are opaque under another.

Example pattern:

```text
Boolean evaluation
    -> discover feasibility/reachability

Tropical evaluation
    -> expose optimal cost

Provenance evaluation
    -> expose critical dependencies
```

### Architectural implication

The search compiler may intentionally **re-interpret the same structural skeleton under several algebras** to obtain different information about the problem.

This is analogous to changing coordinates/representations, but at the evaluation-law level.

---

## 14. Algebra choice belongs in the Theory Profile/search economy

Different algebras can have radically different cost, convergence, storage, and certificate behavior.

### Architectural implication

A Theory Profile for a structural computation may include:

```text
supported evaluation algebras
required algebra laws
finite/infinite aggregation behavior
closure/convergence requirements
available native kernels
certificate route
estimated cost
```

The scheduler can choose an evaluation algebra based on the information sought, not merely raw speed.

---

## 15. Current parametric-evaluation hypothesis

```text
STRUCTURAL MATHEMATICAL SKELETON
    graph / circuit / recurrence / deduction DAG / automaton
        |
        +-- topology and dependency structure
        |
        v
EVALUATION ALGEBRA WITNESS
    operations + identities + certified laws
        |
        v
QUERY SEMANTICS
    existence / count / probability / min / max / provenance / ...
        |
        v
SPECIALIZED REALIZATION
    native CPU / sparse BLAS / GPU / symbolic
        |
        v
CERTIFIED RESULT
```

The skeleton and algebra are independently reusable mathematical assets.

---

## 16. New research obligations

1. Build a catalogue of semiring/semimodule/quantale-style algebraic patterns already used across graph, probability, optimization, automata, parsing, provenance, and dynamic programming.
2. Determine which structure is general enough for each operation without overgeneralizing everything to `Semiring` where weaker/stronger laws are needed.
3. Study algebraic dynamic programming and generic fixed-point algorithms over ordered/idempotent semirings.
4. Investigate certificate generation for algebraic model counting and semiring evaluations.
5. Study convergence/infinite-sum semantics: complete semirings, Kleene algebras, star semirings, ω-continuous structures.
6. Investigate semiring-parametric e-graph/automata/provenance structures.
7. Determine how to represent generic skeleton theorems in the heterogeneous theory/package graph.
8. Study automatic synthesis/discovery of the weakest algebraic laws required by a structural algorithm.
9. Investigate multi-evaluation/fused execution that computes value + provenance or multiple algebras efficiently in one traversal.
10. Study structure-specialized native compilation of generic algebraic kernels.
11. Determine how algebra changes alter semantic identity and result types without forcing recompilation of structural topology.
12. Investigate whether representation search can choose an algebra specifically to reveal invariants or pruning information.
13. Study GraphBLAS/SuiteSparse performance as a donor for CPU/GPU algebra-parametric execution.
14. Investigate semiring homomorphisms as a way to transport/approximate results between evaluation algebras.
15. Determine which semiring-parametric constructions can become foundational metaprimitives for the unnamed project rather than domain-specific adapters.
