# Research Pass — Structure Inference, Mathematical Dispatch, and Coherent Capability Resolution

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass follows the mathematical-structure/type-system research and asks how the system should **derive structure capabilities and select applicable mathematical primitives efficiently**.

The strongest conclusion is:

> **Mathematical structure inference should behave like proof search over certified capability facts, while algorithm dispatch should react dynamically to newly established properties. These are related but distinct mechanisms.**

A static inheritance hierarchy alone is insufficient, and a naive Prolog/typeclass engine risks ambiguity, nontermination, and hierarchy explosion.

---

## 1. Type-class inference can be formulated as logical constraint solving

Research on Haskell-style type classes formalizes class inference using Constraint Handling Rules (CHR). Under suitable conditions, properties such as soundness, completeness, decidability, termination, and confluence can be established.

Sources:

https://arxiv.org/abs/cs/0006034

https://www.cambridge.org/core/journals/journal-of-functional-programming/article/understanding-functional-dependencies-via-constraint-handling-rules/49E533CD7975431B5339456255DA9BE5

https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/article/abs/on-termination-confluence-and-consistent-chrbased-type-inference/C738147162B0D36C4CA6B0F856852B16

### Architectural implication

A mathematical capability query can be represented as a proof goal:

```text
Goal:
    Field(D, Ops)
```

with certified clauses such as:

```text
EuclideanDomain(D, Ops) -> IntegralDomain(D, Ops)
Field(D, Ops) -> DivisionRing(D, Ops)
Field(D, Ops) -> CommutativeRing(D, Ops)
...
```

The capability resolver attempts to establish the goal from current certified witnesses and inference rules.

This is much stronger than string/category matching.

---

## 2. The operation bundle must participate in structure identity

Ordinary software trait systems often try to enforce one implementation of a trait for a type. Mathematical carriers are different: the same underlying set/carrier can admit multiple distinct group/ring/order/topology structures.

Therefore a query like:

```text
Field(Carrier)
```

can be mathematically ambiguous.

### Architectural implication

Structure goals should generally identify the relevant operations/parent context:

```text
Field(Parent D)
```

where `D` canonically bundles its operations, or explicitly:

```text
Field(Carrier X, Add=a1, Mul=m1, Zero=z1, One=o1)
```

A second field structure on the same raw carrier is a different structure witness.

This prevents a software-style coherence rule from incorrectly collapsing genuine mathematical alternatives.

---

## 3. Rust Chalk demonstrates a scalable goal/clause architecture

Chalk lowers Rust trait questions into logical goals and clauses. Its goal language includes conjunction, disjunction, universal/existential quantification, implication, well-formedness, normalization, and explicit ambiguity.

Sources:

https://rust-lang.github.io/chalk/book/what_is_chalk.html

https://rust-lang.github.io/chalk/book/clauses/goals_and_clauses.html

The solver canonicalizes queries, caches common subproblems, and can distinguish unique, ambiguous, floundered, or unresolved solutions.

Sources:

https://rust-lang.github.io/chalk/chalk_solve/solve/trait.Solver.html

https://rust-lang.github.io/chalk/chalk_solve/solve/index.html

### Architectural implication

The project should consider a small **Structure Goal IR** rather than hard-coded recursive lookup.

Possible goal families:

```text
HasStructure(D, Field)
HasProperty(Object, Invertible)
MorphismPreserves(f, RingStructure)
CommonParent(A, B, C)
Normalize(AssociatedStructure(D, Scalars), K)
WellFormed(Construction)
```

The resolver can then reuse generic tabling/canonicalization techniques.

---

## 4. Ambiguity is a valid mathematical result

Chalk explicitly represents ambiguous solutions rather than choosing arbitrarily.

Source:

https://rust-lang.github.io/chalk/chalk_solve/solve/index.html

### Architectural implication

The mathematical resolver needs outcomes richer than Boolean:

```text
PROVEN UNIQUE
PROVEN MULTIPLE / AMBIGUOUS
REFUTED
UNKNOWN
RESOURCE_BOUNDED_UNKNOWN
```

If two valid non-equivalent common parents/coercions exist, the system must not silently pick one because a method table happens to rank it first.

A search campaign may branch into separate semantic worlds instead.

---

## 5. Coherence means something different in mathematics

Rust coherence prevents overlapping incompatible implementations of a trait for the same type because arbitrary method resolution would be unsound/unstable for programming-language semantics.

Chalk has a dedicated coherence layer.

Sources:

https://rust-lang.github.io/chalk/book/what_is_chalk/walkthrough.html

https://rust-lang.github.io/goals/2026/next-solver.html

### Architectural implication

The project should preserve **coherence of implicit inference**, not prohibit multiple mathematical structures.

Possible rule:

```text
Multiple structures may exist.
Implicit resolution requires one canonical/equivalent answer.
Otherwise resolution is explicit or branches.
```

This mirrors Sage's distinction between canonical coercions and arbitrary conversions.

---

## 6. Cache validity must bind to mathematical universe generation

Chalk's solver documentation warns that cached state is valid only while the program clauses supplied to the solver remain the same.

Source:

https://rust-lang.github.io/chalk/chalk_solve/solve/trait.Solver.html

### Architectural implication

Structure-resolution caches must be keyed by at least:

```text
accepted theory/package generation
assumption/world generation
query semantic digest
```

Promoting a new structure inference rule or changing a package may invalidate cached resolution results even when individual object values did not change.

This connects directly to semantic change management.

---

## 7. Lean demonstrates hierarchy and resolution failure modes

Lean/mathlib uses typeclass inference heavily for mathematical structures, but large hierarchies produce known challenges:

- diamonds in the instance graph;
- expensive instance search;
- definitional-equality sensitivity;
- interactions among independent structure dimensions.

Sources:

https://leanprover-community.github.io/lean3/glossary.html

https://leanprover-community.github.io/papers/mathlib-paper.pdf

### Architectural implication

The project should avoid exposing the durable mathematical universe directly as an unconstrained recursive instance-search graph.

Instead:

```text
Durable certified structure facts/theorems
    -> compiled/canonical Structure Capability Graph
    -> tabled goal resolver
```

The compiled graph can contain precomputed closures, canonical routes, ambiguity markers, and invalidation lineage.

---

## 8. GAP provides a particularly relevant dynamic mathematical dispatch model

GAP categories are filters that determine which operations an object admits. An object may satisfy several categories/properties simultaneously.

Source:

https://docs.gap-system.org/doc/ref/manual.pdf

GAP's method system is operation-centric rather than object-centric: multiple methods for an operation can be installed with applicability depending on the known properties/categories of **multiple arguments**.

The homalg documentation emphasizes this as a mathematical advantage: as more properties become known, GAP can draw chains of logical conclusions and select more specialized efficient algorithms, sometimes avoiding computation entirely.

Source:

https://docs.gap-system.org/pkg/homalg/doc/chapB.html

### Architectural implication

This is close to the desired runtime behavior:

```text
Known initially:
    CommutativeRing(D)

Later prove:
    Field(D)

Immediate consequence:
    new structure witnesses derived
    stronger algorithms become applicable
    weaker generic algorithms remain fallback
```

A mathematical proof can therefore change **algorithm availability** without changing the object itself.

---

## 9. Mathematical dispatch should be multi-argument and relation-aware

GAP installs methods on operations depending on properties of all relevant arguments, not only a receiver object.

Source:

https://docs.gap-system.org/pkg/homalg/doc/chapB.html

### Architectural implication

Primitive dispatch should solve a relation such as:

```text
Applicable(Algorithm A,
           Operation Op,
           Inputs [X,Y,...],
           RequiredResultClass R,
           TheoryProfile P)
```

rather than:

```text
X.method(...)
```

This fits mathematics where applicability often depends on several interacting structures:

```text
matrix algorithm depends on coefficient field + dimensions + sparsity
linear map theorem depends on scalar field + source module + target module
solver depends on all constraint families present
```

---

## 10. Dispatch preference must not become truth authority

GAP-style method ranking and Rust-style specialization choose implementations for efficiency or specificity.

### Architectural implication

The project needs a hard separation:

```text
STRUCTURE RESOLVER
    establishes which mathematical properties are certified

PRIMITIVE APPLICABILITY
    establishes which implementations are semantically admissible

SEARCH/EXECUTION DISPATCH
    chooses among admissible implementations by cost/strength/history
```

The dispatcher may choose poorly and lose performance.

It must not be able to make an inadmissible algorithm mathematically valid by ranking it highly.

---

## 11. Functional dependencies suggest deterministic inference relations

Type-class functional dependencies express relationships such as:

```text
class C a b | a -> b
```

meaning one parameter determines another. CHR analysis gives conditions under which such inference remains sound/complete/decidable.

Source:

https://www.cambridge.org/core/journals/journal-of-functional-programming/article/understanding-functional-dependencies-via-constraint-handling-rules/49E533CD7975431B5339456255DA9BE5

### Architectural implication

Mathematics contains many analogous dependencies:

```text
Parent -> ScalarRing
MatrixSpace -> RowIndex / ColIndex / BaseRing
QuotientRing(R,I) -> BaseRing R and ideal I
Morphism -> Domain and Codomain
```

Encoding these as declared functional/relational dependencies can make structure inference dramatically more precise and reduce ambiguity.

But the dependencies themselves need semantic justification; they are not merely compiler hints.

---

## 12. Coinductive structure goals may be useful in recursive semantic classes

Modern Rust trait-solver work explicitly studies coinductive trait semantics for recursively defined capabilities.

Source:

https://rust-lang.github.io/goals/2026/next-solver.html

### Architectural implication

Some mathematical semantic properties may be naturally coinductive or recursively self-supporting, particularly around infinite structures, coalgebras, streams, and bisimulation-style interfaces.

The structure resolver should not assume all cycles in the proof graph are errors.

However, coinductive acceptance requires a declared sound semantic rule, not generic cycle detection.

This connects to the existing coinduction research pass.

---

## 13. Structure inference can become a compilation phase

A concrete problem can begin with partial structural knowledge:

```text
objects
parents
known properties
relations
```

The system can run structure closure before expensive mathematical search:

```text
recover explicit witnesses
    -> apply certified implication rules
    -> normalize parent/associated structures
    -> detect canonical coercions/common parents
    -> detect ambiguity/conflicts
    -> materialize capability graph for this world
```

### Architectural implication

This yields a **mathematical elaboration phase** analogous to a compiler type checker, but its output is much richer:

- admissible primitive families;
- invalid compositions eliminated;
- new proof obligations exposed;
- candidate common representations;
- structure ambiguities causing world branches;
- opportunities for specialized algorithms.

---

## 14. Missing structure proofs should be ranked by unlock value

Suppose two unresolved obligations exist:

```text
A: prove IsPrimeIdeal(I)
B: prove SomeMinorProperty(X)
```

If proving `A` establishes:

```text
QuotientRing(R,I) is IntegralDomain
```

which enables hundreds of algebraic primitives, it may deserve far more compute.

### Architectural implication

The search economy should compute a **capability unlock graph**:

```text
open structure witness
    -> primitives unlocked if proven
    -> theories/morphisms unlocked
    -> downstream proof obligations simplified
```

This makes structure inference an active mathematical planning mechanism rather than passive type checking.

---

## 15. Current structure-resolution hypothesis

The strongest current synthesis is:

```text
DURABLE THEORY/PACKAGE GRAPH
    certified structure implications
    parent constructions
    morphisms
    operations/laws
        |
        v
COMPILED STRUCTURE CAPABILITY GRAPH
    generation-specific closure/index
        |
        v
TABLED STRUCTURE GOAL SOLVER
    prove/refute/ambiguous/unknown
        |
        +-> generate open witness obligations
        |
        v
PRIMITIVE APPLICABILITY SET
        |
        v
SEARCH/EXECUTION DISPATCH
    cost/strength/history-based selection
```

No single layer should own all four responsibilities.

---

## 16. New research obligations

1. Study SLG/tabling and modern logic-programming engines as a foundation for terminating/cached structure-goal resolution.
2. Investigate canonicalization of structure goals so equivalent queries share cache entries.
3. Define coherence rules for implicit mathematical structure resolution when multiple valid structures exist on one carrier.
4. Study GAP method ranking and property implication mechanisms in detail, including how stale properties are handled for mutable objects.
5. Investigate compiling certified structure implications into CHR/Datalog/egglog rules while preserving proof lineage.
6. Determine how a structure solver returns compact proof/witness objects, not only yes/no.
7. Study ambiguity explanation: derive the minimal competing structure/morphism alternatives preventing implicit resolution.
8. Investigate how structure inference interacts with versioned assumption worlds and candidate packages.
9. Define functional dependencies/associated structures for mathematical parents and certify them.
10. Study method/primitive specialization where several admissible algorithms overlap but have different completeness, cost, and certificate strengths.
11. Investigate automatic generation of property implication rules from theorem libraries and theory morphisms.
12. Determine whether capability closure should be eager, lazy, or hybrid by property class.
13. Study cycle/coinduction handling for recursive structure goals.
14. Investigate proof-producing common-parent/coercion inference.
15. Define unlock-value metrics for open structure proofs so the search economy can prioritize them rationally.
