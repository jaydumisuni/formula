# Research Pass — Mathematical Structure Typing, Parents, Morphisms, and Capability Witnesses

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how the unnamed mathematical project should know which operations, transformations, theorems, representations, and solvers are mathematically admissible for an object **before** spending search compute.

The strongest conclusion is:

> **Raw data type is not mathematical type. Mathematical capability depends on a runtime mathematical parent/context plus certified structure witnesses and morphisms.**

This is foundational for the project's search-space algebra: impossible compositions should be rejected structurally rather than discovered later by failed computation.

---

## 1. The same carrier can represent different mathematical structures

Sage distinguishes an **element** from its **parent**. A parent models a mathematical set together with relevant structure, and the same apparent carrier can occur in different mathematical contexts.

Sources:

https://doc.sagemath.org/html/en/reference/categories/sage/categories/primer.html

https://doc.sagemath.org/html/en/reference/structure/sage/structure/parent.html

### Architectural implication

The project should not identify a mathematical object solely from bytes/value representation such as:

```text
42
array[...]
polynomial coefficients
```

The authoritative object identity needs mathematical context:

```text
value / semantic object
parent/domain
assumption world
structure witnesses
representation
```

For example, an integer-like value can inhabit different structures with different valid operations and theorems.

---

## 2. Runtime-parameterized parents are essential in computer algebra

Nemo/AbstractAlgebra explicitly notes that ordinary programming-language types are insufficient for many algebraic objects because the mathematical structure depends on runtime values.

Examples include:

- a residue ring modulo a runtime modulus;
- a quotient ring by a runtime ideal;
- a polynomial ring over a runtime coefficient parent;
- a matrix space over a runtime base ring.

Sources:

https://nemocas.github.io/Nemo.jl/stable/developer/parent_object/

https://nemocas.github.io/AbstractAlgebra.jl/stable/types/

### Architectural implication

The project's mathematical type system cannot be only compile-time nominal/generic types.

It needs first-class **Parent/Domain objects** whose identity can depend on mathematical values:

```text
Zmod(17)
PolynomialRing(QQ, [x,y])
QuotientRing(R, I)
MatrixSpace(R, m, n)
```

A primitive may therefore require a parent predicate rather than a language-level machine type.

---

## 3. Structure should be expressed as capabilities/laws, not only names

MathComp and Lean/mathlib build algebraic hierarchies such as:

```text
additive structure
ring
commutative ring
integral domain
field
algebraically closed field
...
```

with operations plus proofs of laws.

Sources:

https://math-comp.github.io/htmldoc_2_1_0/mathcomp.algebra.ssralg.html

https://leanprover-community.github.io/mathematics_in_lean/C08_Hierarchies.html

### Architectural implication

A generic primitive should declare mathematical requirements such as:

```text
requires:
    AdditiveCommutativeGroup(D)
    Field(K)
    Module(K, V)
    FiniteDimensional(K, V)
```

rather than relying on domain names.

If the current universe contains certified witnesses for those requirements, the primitive becomes admissible automatically.

This gives a possible first-class artifact:

```text
Structure Witness
    parent/domain: D
    structure/property: Field
    operations: ...
    laws: ...
    certificate/proof lineage: ...
    assumptions/world: ...
```

The name is not frozen.

---

## 4. Generic mathematics can be unlocked automatically by structure witnesses

MathComp's hierarchy is designed so theorems proved for a structure automatically apply to every instance of that structure. Lean typeclass inference similarly recovers the required algebraic capabilities from available instances.

Sources:

https://math-comp.github.io/htmldoc_2_1_0/mathcomp.algebra.ssralg.html

https://leanprover-community.github.io/mathematics_in_lean/C08_Hierarchies.html

### Architectural implication

A powerful self-expansion route is:

```text
new domain D
    -> certify D is a Field
    -> every generic Field primitive/theorem becomes available on D
```

without manually re-registering each algorithm.

Likewise, discovering that a problem representation has additional structure can instantly enlarge the applicable mathematical vocabulary.

Thus **structure discovery is itself search-space expansion**.

---

## 5. Hierarchies can explode or become ambiguous

Lean/mathlib documents a real engineering difficulty: mathematical structure hierarchies become large directed graphs, especially when independent dimensions such as algebra, order, topology, finiteness, and norm structure interact.

Mathlib also has to manage **diamonds**, where multiple inheritance/typeclass paths can construct non-definitionally-equal structure instances.

Sources:

https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Group/Defs.html

https://leanprover-community.github.io/lean3/glossary.html

https://leanprover-community.github.io/papers/mathlib-paper.pdf

### Architectural implication

The project should not model the entire mathematical universe as one giant nominal inheritance tree.

Prefer compositional capability facts/witnesses:

```text
D has Add
D has Mul
D satisfies Associative(Mul)
D satisfies Distributive(Add,Mul)
D satisfies NoZeroDivisors
...
```

with higher structures derivable/cached from these facts when useful.

Named structures such as `Field` remain valuable **bundles/interfaces**, but the durable semantics should preserve the underlying operations/laws and witness identity.

This may reduce hierarchy explosion and make cross-domain structure discovery easier.

---

## 6. Bundled versus unbundled structure is a real design tradeoff

Lean formalization distinguishes bundled and unbundled/semi-bundled structures. Bundling operations and laws provides canonical objects and better morphism handling, while unbundled propositions can be more flexible for properties discovered about existing objects.

Sources:

https://leanprover-community.github.io/lean3/glossary.html

https://leanprover-community.github.io/papers/mathlib-paper.pdf

### Architectural implication

The unnamed project likely needs both:

```text
BUNDLED PARENT / STRUCTURE
    canonical operations + stable mathematical context

PROPERTY / STRUCTURE WITNESS
    discovered certified fact about an existing semantic object/parent
```

For example:

```text
Parent D = polynomial ring ...
Witness: D is Noetherian
Witness: ideal I is prime
Witness: quotient D/I is an integral domain
```

The property need not require minting a completely new programming-language type.

---

## 7. Coercion must mean canonical structure-preserving transport

Sage explicitly separates **coercions** from arbitrary conversions.

Coercions are canonical morphisms intended to form a coherent commuting diagram. When two operands have different parents, Sage searches for a common parent reachable through canonical coercions.

Source:

https://doc.sagemath.org/html/en/reference/coercion/sage/structure/coerce.html

### Architectural implication

The project should reserve implicit conversion for transformations satisfying a strong contract such as:

```text
total over declared source domain
canonical / unambiguous
structure-preserving for required operations
certified
```

A lossy, partial, heuristic, approximate, or assumption-changing conversion must remain explicit.

For example:

```text
Integer -> Rational
```

may be canonical.

But:

```text
Real interval -> floating point
symbolic expression -> sampled numeric approximation
field element -> integer representative
```

must not silently masquerade as semantic equality.

This protects mathematical truth during composition.

---

## 8. Common-parent search is itself a mathematical construction problem

Sage uses construction functors and pushout-like machinery to find a common parent for operands constructed from related bases.

Source:

https://doc.sagemath.org/html/en/reference/coercion/sage/categories/pushout.html

Example pattern:

```text
QQ
ZZ[x]
    -> common construction parent QQ[x]
```

### Architectural implication

The project may need a generic relation:

```text
common_parent(A, B) -> C
```

with proof that source-to-`C` maps are admissible morphisms.

More generally:

```text
find_common_semantic_space(objects, required_operation)
```

may become an important representation-search metaprimitive.

The answer depends on which structures must be preserved, not merely whether values can be converted.

---

## 9. Construction functors can explain parent provenance

Sage construction functors represent operations such as:

- fraction-field formation;
- polynomial-ring construction;
- matrix-ring construction;
- completion/base extension.

They participate in coercion/common-parent reasoning.

Source:

https://doc.sagemath.org/html/en/reference/coercion/sage/categories/pushout.html

### Architectural implication

Parent objects should preserve **construction provenance**:

```text
D = PolynomialRing(FractionField(ZZ), x)
```

rather than only a generated identifier.

That provenance can support:

- common-parent inference;
- theory morphism discovery;
- inversion/deconstruction;
- semantic diff;
- package dependency analysis;
- representation search.

This fits the project's broader provenance-first architecture.

---

## 10. Dependent types demonstrate structural dimensions can be part of mathematical typing

Lean/mathlib defines dependent matrices whose row/column types and even entry types may depend on indices.

Source:

https://leanprover-community.github.io/mathlib4_docs/Mathlib/Data/Matrix/DMatrix.html

Ordinary matrices are represented as functions `m -> n -> α`, so their index sets/dimensions participate directly in their type-level mathematics.

Source:

https://leanprover-community.github.io/mathlib_docs/data/matrix/basic.html

### Architectural implication

The project should be able to express constraints such as:

```text
Matrix(K, m, n)
Matrix(K, n, p)
    -> multiplication admissible
    -> result Matrix(K, m, p)
```

before execution.

Similar dependent/indexed structure applies to:

- tensors and axis spaces;
- vector spaces over scalar fields;
- polynomials over coefficient rings;
- maps with explicit domain/codomain;
- graph morphisms;
- probability measures on measurable spaces.

This provides strong early pruning of impossible construction compositions.

---

## 11. Mathematical dimensions/units are another zero-cost structural layer

F# units of measure demonstrate that dimensional structure can be tracked statically, normalized algebraically, and erased before runtime with no representation cost.

Source:

https://learn.microsoft.com/en-us/dotnet/fsharp/language-reference/units-of-measure

2025 work formalized dimensional analysis in Lean, treating physical dimensions algebraically and proving dimensional-homogeneity properties.

Source:

https://arxiv.org/abs/2509.13142

### Architectural implication

A generalized mathematical structure system can treat units/dimensions as algebraic structure metadata:

```text
Length
Time
Length / Time
Mass * Length / Time^2
```

A construction can be rejected before numeric evaluation if dimensions cannot compose.

More broadly, the same mechanism can carry **semantic grades** other than physical units when they have sound algebraic composition rules.

---

## 12. Type/structure information should often disappear from the hot path

F# units are compile-time information erased from runtime values. Lean/MathComp structure inference similarly allows high-level proofs/algorithms to be elaborated into concrete operations.

Source:

https://learn.microsoft.com/en-us/dotnet/fsharp/language-reference/units-of-measure

### Architectural implication

The project can retain very rich mathematical typing during:

```text
search
composition checking
proof obligation generation
specialization
```

then compile a mature primitive into:

```text
raw native arithmetic / memory operations
```

once all structure requirements are fixed and validated.

This directly supports the CPU-first performance target:

> **rich mathematical semantics at construction time; minimal semantic overhead in the residual native realization.**

---

## 13. Search-space algebra should be structure-directed

This pass strengthens the earlier search-space-algebra hypothesis.

Each mathematical primitive/transformation may declare something like:

```text
inputs:
    X with structures Sx
    Y with structures Sy

requires:
    predicates/laws R

produces:
    Z with derived structure Sz

preserves:
    structures/invariants P

destroys/forgets:
    structure D

morphisms:
    canonical maps available
```

Then candidate composition is generated only when requirements can be satisfied by certified witnesses or by reachable witness-generating obligations.

### Example

Instead of trying every transformation after every other transformation:

```text
if T2 requires Field(output(T1))
    and output(T1) has certified Field witness
        -> T2 admissible
    else if field-structure can be established as an open obligation
        -> candidate branch
    else
        -> reject composition structurally
```

This can collapse the construction search space by orders of magnitude.

---

## 14. Missing structure can itself become a mathematical subproblem

A capability requirement does not always have to fail immediately.

Suppose an algorithm requires:

```text
IntegralDomain(D)
```

and the system currently knows only:

```text
CommutativeRing(D)
```

It may spawn the obligation:

```text
prove/refute IntegralDomain(D)
```

If proved, an entire new family of algorithms becomes available.

### Architectural implication

Type/structure inference can therefore be integrated with the search economy:

```text
missing witness W
    -> estimate unlock value
    -> perhaps launch proof/search campaign for W
```

A single structure theorem can unlock thousands of downstream primitives and transported results.

This gives structure discovery very high potential information value.

---

## 15. Structure witnesses need world/scope identity

A domain may possess a property only under assumptions.

For example:

```text
Matrix A invertible
```

may hold in one assumption world and fail in another.

Likewise a quotient may be a field only if an ideal is maximal.

### Architectural implication

A structure witness must bind to:

```text
semantic object/parent identity
assumptions/world
scope
proof/certificate
freshness/dependencies
```

No global cache entry like:

```text
D.is_field = true
```

is sufficient without provenance and world identity.

---

## 16. Morphisms should be typed by preserved structure

MathComp bundles several morphism families (additive, ring, linear, algebra morphisms). Sage coercions likewise depend on structure-preserving maps.

Sources:

https://math-comp.github.io/htmldoc_2_1_0/mathcomp.algebra.ssralg.html

https://doc.sagemath.org/html/en/reference/coercion/sage/structure/coerce.html

### Architectural implication

A map is not simply:

```text
A -> B
```

It may carry certified properties such as:

```text
homomorphism
monomorphism / injective
isomorphism
linear map
order embedding
measure-preserving map
approximation / abstraction
```

These properties determine which mathematics can be transported through the map.

This integrates directly with theory morphisms and cross-domain theorem transfer.

---

## 17. Current mathematical structure-system hypothesis

The strongest synthesis from this pass is a layered model:

```text
SEMANTIC OBJECT
    immutable mathematical identity
        |
        v
PARENT / DOMAIN
    runtime mathematical context
        |
        +-- operations
        +-- construction provenance
        +-- parameters
        |
        v
STRUCTURE / PROPERTY WITNESSES
    certified laws/capabilities
        |
        v
MORPHISM / COERCION GRAPH
    certified structure-preserving transports
        |
        v
REPRESENTATIONS / REALIZATIONS
    symbolic / exact / interval / native / GPU / etc.
```

The layers must not be collapsed.

Especially:

```text
same representation != same semantic object
same carrier != same mathematical structure
convertible != canonically coercible
has operation != satisfies laws of named structure
```

---

## 18. New research obligations

1. Study algebraic hierarchy design in Lean/mathlib, MathComp/Hierarchy Builder, Sage categories, Nemo/AbstractAlgebra, and GAP to extract the best structure-witness model without inheriting their implementation-specific constraints.
2. Investigate proof-relevant structure witnesses versus proof-irrelevant property caches.
3. Determine how operations should be identified when one parent supports multiple valid operation families of the same arity/name.
4. Study automatic inference of high-level structures from primitive laws without creating typeclass-resolution explosion.
5. Investigate dependent/refinement typing for dimensions, index sets, domains, sparsity patterns, symmetry classes, and other mathematical constraints.
6. Define canonical-coercion requirements and ambiguity resolution; study when multiple structure-preserving maps prohibit implicit coercion.
7. Investigate common-parent/pushout discovery over the heterogeneous theory/package graph.
8. Study category-theoretic functors/natural transformations as possible representation of reusable structure-preserving constructions without making category theory mandatory for every primitive.
9. Determine how structure witnesses participate in certificate envelopes and dependency invalidation.
10. Study structure discovery as a search-economy operation: estimate downstream capability unlock value.
11. Investigate semantic subtyping/refinement: when one witnessed structure safely satisfies requirements of another without nominal inheritance.
12. Study erased mathematical typing so specialization can remove parent/witness dispatch from hot native kernels.
13. Investigate units/dimensional analysis as a prototype of algebraic semantic grades that can be statically composed and erased.
14. Determine how structure inference interacts with assumption/version worlds and alternate representations.
15. Study whether a compact `Structure Capability Graph` can be derived from the durable heterogeneous theory graph for fast runtime/search queries.
