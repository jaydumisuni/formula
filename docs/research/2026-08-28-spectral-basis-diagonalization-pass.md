# Research Pass — Spectral Bases, Diagonalization, Block Decomposition, and Transform-Space Solving

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates a broad representation-changing problem-solving pattern: find a basis/transform in which coupled mathematical operations become diagonal, block-diagonal, pointwise, sparse, or otherwise separable.

The central finding is:

> **Fourier analysis is only one instance of a large family. Eigen/spectral bases, symmetry-adapted representation-theoretic bases, graph Fourier transforms, zeta/Möbius transforms, and related transforms can convert one large coupled computation into many independent scalar/small-block computations. Basis discovery should therefore be a first-class metaprimitive, not merely a numerical linear-algebra backend detail.**

---

## 1. Diagonalization is exact decomposition of an operator

For a diagonalizable operator/matrix:

```text
A = U Λ U^-1
```

changing coordinates with `U` replaces the coupled action of `A` by independent multiplication in the eigenbasis.

Graph Fourier transforms use eigenvectors of a graph Laplacian as a spectral basis.

Source:

https://pmc.ncbi.nlm.nih.gov/articles/PMC7872285/

### Architectural implication

The representation-search layer can explicitly ask:

```text
find_basis_that_simplifies(operator family)
```

where the target simplification can be:

```text
diagonal
block diagonal
sparse
low bandwidth
separable
low rank
```

---

## 2. Symmetry can generate the correct basis automatically

Group representation theory decomposes a space into invariant subspaces. Operators commuting with the group action become block diagonal in a symmetry-adapted basis.

Sources:

https://www.sciencedirect.com/science/article/pii/S0024379508003753

https://www.sciencedirect.com/science/article/pii/S0141029623011239

### Architectural implication

The symmetry-discovery pass can automatically trigger:

```text
proved group action G
    -> compute irreducible/isotypic decomposition
    -> synthesize symmetry-adapted basis
    -> block-decompose compatible operators
```

A symmetry therefore creates both invariants and an execution decomposition.

---

## 3. Full operators need not always be materialized before decomposition

Structural-mechanics literature notes that symmetry-adapted subspace matrices can be computed directly, avoiding assembly of the full matrix.

Source:

https://www.sciencedirect.com/science/article/pii/004578259190128S

### Architectural implication

The compiler should prefer **structure-first lowering**:

```text
symmetry/decomposition known
    -> directly generate independent block kernels
```

rather than:

```text
construct huge matrix
    -> diagonalize/block it afterward
```

This directly supports RAM and CPU efficiency.

---

## 4. The target may be block diagonal rather than diagonal

A complete scalar diagonalization may be unavailable, unnecessary, unstable, or expensive. Representation theory naturally yields irreducible/invariant blocks.

### Architectural implication

The metaprimitive objective should be graded:

```text
find maximal useful independent decomposition
```

not:

```text
force full diagonalization
```

Block structure already exposes parallelism and smaller subproblems.

---

## 5. Generalized Fourier transforms depend on underlying algebraic structure

Classical Fourier analysis can be understood through representation theory of abelian groups; generalized Fourier transforms extend the idea to graphs/groups/other structures.

Graph Fourier analysis builds a transform from a graph operator's eigenbasis rather than Euclidean translation symmetry.

Sources:

https://pmc.ncbi.nlm.nih.gov/articles/PMC7872285/

https://www.sciencedirect.com/science/article/abs/pii/S1051200424003762

### Architectural implication

The project should not register one primitive named `FourierTransform` and stop.

It needs a transform-family abstraction:

```text
source structure
analysis operator / characters / eigenobjects
forward transform
inverse/reconstruction
operation made simpler
certificate/exactness
cost
```

The Theory Profile determines which transform families are admissible.

---

## 6. Möbius/zeta transforms show transform-space solving outside classical spectral analysis

Finite-poset zeta and Möbius transforms are mutually inverse linear transforms used in combinatorics. Modern algorithms exploit poset width/chain decomposition to reduce cost substantially.

Source:

https://arxiv.org/abs/2211.13706

### Architectural implication

Transform search should include algebraic incidence/poset transforms, not only eigenvector-based transforms.

The general pattern is:

```text
operation hard in native representation
    -> invertible transform T
    -> operation simple/triangular/pointwise in transform representation
    -> compute
    -> T^-1
```

---

## 7. Simultaneous simplification of several operators is especially valuable

A problem rarely contains only one operator. If several operators commute/share symmetry, one basis can simplify many of them simultaneously.

### Architectural implication

Basis selection should score an entire operator family:

```text
{A1, A2, ..., Ak}
```

rather than optimizing one matrix in isolation.

A slightly worse basis for `A1` may be globally superior if it decomposes the full mathematical workload.

---

## 8. Basis choice can reveal hidden independent Work Cells

After block diagonalization:

```text
A -> diag(B1, B2, ..., Bk)
```

the corresponding subspaces can often be processed independently.

### Architectural implication

A certified basis transform can rewrite the active campaign graph automatically:

```text
one coupled problem
    -> k independent mathematical cells
    -> combine transformed results
```

This is decomposition discovered through algebra rather than graph separators.

---

## 9. Transform cost and reconstruction cost matter

Computing a basis/eigendecomposition can be expensive. A transform may be worthwhile only if:

```text
solve cost saved + reuse value
    > basis/transform + inverse cost
```

### Architectural implication

The Search Economy should distinguish:

```text
one-time basis discovery/compilation cost
per-query transform cost
per-query block/pointwise solve cost
inverse/reconstruction cost
reuse count
```

A costly spectral compilation may become an excellent permanent primitive for repeated queries.

---

## 10. Exact, numerical, and approximate spectral claims are different

An exact algebraic eigendecomposition, a floating-point numerical eigensystem, and an approximate low-rank spectral model carry different mathematical guarantees.

### Architectural implication

Transform artifacts require explicit status:

```text
EXACT
RIGOROUS_ENCLOSURE
CERTIFIED_APPROXIMATE
HEURISTIC_NUMERICAL
```

with appropriate reconstruction/error contracts.

The numerical-realization layer can optimize an exact semantic transform separately.

---

## 11. Spectral transforms connect to conjugacy

Diagonalization is a special case of conjugating an operator into a simpler normal form:

```text
Λ = U^-1 A U
```

### Architectural implication

The broader transformation search hierarchy may be:

```text
exact diagonalization
block diagonalization
canonical/normal form
linearizing conjugacy
higher-dimensional embedding
approximate certified simplification
```

The compiler searches the cheapest strong transformation applicable to the Theory Profile.

---

## 12. Current spectral-basis hypothesis

```text
PROBLEM / OPERATOR FAMILY
    -> detect symmetry/commutation/spectral structure
    -> synthesize candidate basis/transform
    -> certify invertibility/domain and simplification property
    -> quantify decomposition/conditioning/cost
    -> transform problem
    -> run independent block/pointwise primitives
    -> inverse/reconstruct
    -> compose certificates
    -> promote reusable transform/basis if generalizable
```

This is another major route to making difficult mathematics cheap before adding compute power.

---

## 13. New research obligations

1. Study exact algorithms for simultaneous diagonalization/block diagonalization and their certificate forms.
2. Investigate automatic symmetry-adapted basis construction from discovered group actions.
3. Study generalized Fourier transforms over finite/nonabelian groups and whether a common transform interface is practical.
4. Investigate transform discovery over posets/incidence algebras and subset transforms.
5. Study spectral decomposition of semantic hypergraphs/factor graphs without materializing dense matrices.
6. Investigate conditioning/stability-aware basis selection for numerical realizations.
7. Connect basis search to AD-derived Jacobians, invariants, and Koopman/conjugacy discovery.
8. Study cost/reuse models for one-time transform compilation versus repeated direct solves.
9. Investigate proof transport through exact transform/inverse pairs.
10. Build First-Light problems where the key capability is automatically finding a basis that splits one coupled problem into independent subproblems.
