# Research Pass — Dimensional Analysis, Scaling Invariance, and Dimensionless Coordinate Discovery

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates dimensional analysis as both a compile-time admissibility system and an automatic representation-change mechanism for scientific and engineering mathematics.

The central finding is:

> **Units/dimensions can prune impossible mathematics at effectively zero runtime cost, while Buckingham-Pi-style null-space analysis can automatically quotient out scaling degrees of freedom and move a problem into lower-dimensional invariant coordinates before expensive search begins.**

---

## 1. Units can be part of static mathematical typing with zero runtime cost

F# integrates units of measure into its type system. Type inference propagates unit expressions, rejects mismatched operations, and the measure annotations are erased at compilation time, so the runtime representation remains the underlying numeric type.

Sources:

https://learn.microsoft.com/en-us/dotnet/fsharp/language-reference/units-of-measure

https://learn.microsoft.com/en-us/archive/msdn-magazine/2010/june/msdn-magazine-clr-inside-out-fsharp-fundamentals

### Architectural implication

The project can attach a certified dimension/measure expression to mathematical numeric objects during semantic analysis and erase it from specialized native realizations once admissibility has been established.

This supports the ordinary-hardware objective:

```text
rich mathematical typing at compile/search time
    -> zero-cost erased metadata in hot numeric kernels
```

where safe.

---

## 2. Dimensional polymorphism provides representation independence

Andrew Kennedy's work on units-of-measure type systems proves a dimensional-invariance/parametricity principle: well-typed programs can be invariant under changes in the chosen units, and type information can imply scaling relationships.

Sources:

https://www.microsoft.com/en-us/research/publication/relational-parametricity-and-units-of-measure/

https://www.microsoft.com/en-us/research/?p=145175

### Architectural implication

A mathematical construction can potentially carry a theorem-like property:

```text
unit_system_invariant: yes
```

meaning its behavior is independent of the arbitrary human choice of metres versus feet, seconds versus hours, etc., modulo the certified conversions.

This is stronger than merely checking that additions have matching units.

---

## 3. Buckingham Pi is a null-space/quotient construction

For quantities whose dimensions are represented by exponent vectors, collect those vectors as columns of a dimension matrix `D`.

Dimensionless products correspond to exponent vectors `a` satisfying:

```text
D a = 0
```

Thus the null space of the dimension matrix generates dimensionless coordinates.

Sources:

https://www.nature.com/articles/s41467-025-64425-8

https://www.nature.com/articles/s43588-022-00355-5

https://www.cambridge.org/core/books/abs/micro-and-nanoscale-fluid-mechanics/nondimensionalization-and-characteristic-parameters/37647B82465FC6EDB3D7E351F095FDF0

### Architectural implication

Dimensional reduction can be a deterministic metaprimitive:

```text
input quantities q1 ... qn
    -> construct dimension matrix D
    -> compute certified null-space basis
    -> produce dimensionless coordinates Π1 ... Πk
```

This is a concrete example of the project's broader idea:

> change the mathematical representation so irrelevant degrees of freedom disappear before search.

---

## 4. Dimensionless coordinates can reduce problem dimension substantially

AI Feynman provides a concrete example. A gravitational-force discovery problem began with nine independent variables. Dimensional analysis transformed it to six dimensionless independent variables; detected symmetries then removed more variables and separability split the residual problem into smaller subproblems.

Source:

https://pmc.ncbi.nlm.nih.gov/articles/PMC7159912/

### Architectural implication

The search compiler should stage simplifying structure discovery:

```text
dimensions/scaling quotient
    -> symmetry quotient
    -> separability/decomposition
    -> only then expensive formula/construction search
```

The important point is multiplicative reduction: each discovered structural property changes the search problem handed to the next stage.

---

## 5. Dimensional analysis is a search-space grammar constraint

Physics-informed symbolic-regression systems use dimensional consistency to reduce candidate equation space. Recent work explicitly notes that dimensional analysis acts as feature selection and reduces the search space, while other physical symbolic-regression work restricts candidate expressions to physically sensible dimensions before search.

Sources:

https://www.nature.com/articles/s41598-023-28328-2

https://www.nature.com/articles/s41598-022-15416-y

### Architectural implication

For a target with declared dimension `T`, the construction synthesizer should generate only candidates whose inferred output dimension is `T`.

Instead of:

```text
generate candidate
    -> evaluate
    -> discover nonsense dimensions
```

use:

```text
dimension-aware grammar/type inference
    -> impossible candidate never exists
```

This can remove huge regions of symbolic/program search before CPU work is spent.

---

## 6. The Pi basis is not unique

Buckingham-Pi gives a space of valid dimensionless combinations; the particular basis is not unique. Modern research therefore searches for dimensionless groups that best collapse data or optimize an information criterion.

Sources:

https://www.nature.com/articles/s43588-022-00355-5

https://www.nature.com/articles/s41467-025-64425-8

### Architectural implication

The project should distinguish:

```text
DIMENSIONLESS SUBSPACE / INVARIANT CONTENT
```

from:

```text
CHOSEN BASIS / COORDINATE REALIZATION
```

Basis selection is a representation-search problem.

Possible costs include:

```text
sparsity
integer exponent size
conditioning
factorization quality
data collapse / information value
execution cost
transfer value
```

Multiple bases may be mathematically equivalent but computationally very different.

---

## 7. Scaling invariance can unlock transfer across systems

Work on dimensionless policies shows that expressing a policy in dimensionless variables can allow exact rescaling/transfer among dimensionally similar systems under the required similarity assumptions.

Source:

https://arxiv.org/abs/2307.15852

### Architectural implication

A discovered construction in dimensionless coordinates may be more general than one tied to a particular system of units or parameter scale.

Primitive promotion should therefore test whether a construction can be lifted from:

```text
specific numerical scale
```

to:

```text
scale-invariant/dimensionless semantic primitive
```

before storing it as an over-specialized capability.

---

## 8. Dimension is not the same as mathematical or physical kind

The International Vocabulary of Metrology explicitly states:

- quantities of the same kind have the same dimension;
- quantities with the same dimension are not necessarily of the same kind;
- dimension does not encode whether a quantity is scalar, vector, or tensor.

Sources:

https://jcgm.bipm.org/vim/en/1.2.html

https://jcgm.bipm.org/vim/en/1.7.html

### Architectural implication

The structure system must not collapse to dimension vectors.

A physical quantity should be layered conceptually as:

```text
carrier / numeric structure
quantity kind / semantic role
dimension vector
unit / scale realization
shape: scalar/vector/tensor/...
additional domain structure
```

Dimensional equality permits further consideration; it does not prove semantic substitutability.

---

## 9. Dimensionless does not mean semantically interchangeable

A dimensionless quantity has zero dimensional exponents, but different dimensionless quantities can represent radically different semantics: angles, probabilities, counts/ratios, normalized invariants, logarithmic quantities, etc.

### Architectural implication

`dimension = 1` cannot be treated as a universal numeric type.

The quantity/structure witness remains necessary even after dimensional erasure.

---

## 10. Function domains impose dimensional obligations

Standard dimensional analysis explains why arguments to functions such as `exp`, `log`, and trigonometric functions require suitable dimensionless/scaled arguments in ordinary physical formulations.

Source:

https://epubs.siam.org/doi/pdf/10.1137/16M1107127

### Architectural implication

Primitive signatures should carry dimensional constraints, not merely input/output annotations.

Example:

```text
exp : Dimensionless -> Dimensionless
```

or a richer semantics where a domain-specific dimensionless-kind witness is required.

This lets type inference prune malformed compositions before symbolic evaluation.

---

## 11. Dimensional structure should integrate with theory morphisms

A change of units is a structure-preserving map. Earlier project research requires all implicit conversions to be certified morphisms rather than numeric coincidences.

### Architectural implication

Unit conversion can become a specialized morphism family:

```text
same quantity kind
same dimension
certified scale/offset transformation
```

with special care for affine units such as temperatures, where naïve multiplicative unit algebra is insufficient.

The mathematical semantic identity should normally remain invariant under representation-only unit changes.

---

## 12. Current dimensional-analysis hypothesis

```text
DOMAIN INPUT
    -> quantity-kind + dimension + unit witnesses

STATIC STRUCTURE INFERENCE
    -> reject impossible operations/compositions

DIMENSION MATRIX
    -> null-space / Pi-group invariant coordinates

REPRESENTATION SEARCH
    -> choose useful basis of dimensionless groups

FURTHER STRUCTURE DISCOVERY
    -> symmetry / separability / decomposition

SOLVER / SYNTHESIS
    -> search only reduced admissible space

SPECIALIZATION
    -> erase unit metadata where realization proof permits
```

This is an excellent example of strong mathematical semantics producing less runtime work rather than more.

---

## 13. New research obligations

1. Study quantity calculus beyond simple multiplicative dimensions, including affine/logarithmic units.
2. Define the boundary between quantity kind, dimension, unit, shape, and mathematical parent structures.
3. Investigate automatic inference of missing units/dimensions from relations as a constraint-solving problem.
4. Study certified integer/rational null-space bases and canonicalization of dimensionless groups.
5. Investigate basis-selection objectives for Pi groups: sparsity, conditioning, data collapse, execution cost, interpretability, transfer.
6. Connect dimensional scaling groups formally to the broader symmetry/quotient representation machinery.
7. Study dimension-aware program synthesis over executable constructions, not only expression trees.
8. Investigate dimensional analysis for stochastic quantities, derivatives, integrals, tensors, and distributions.
9. Determine how dimensional/quantity metadata transports through theory morphisms and semantic interfaces.
10. Study whether repeated dimensional reductions can be distilled into reusable cross-domain representation primitives.
