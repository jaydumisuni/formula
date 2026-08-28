# Research Pass — Fast Exact Arithmetic, Modular Images, Reconstruction, and Randomized Producers

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how the unnamed mathematical project can execute exact mathematics at very high CPU efficiency without carrying large arbitrary-precision objects through every intermediate operation.

The strongest pattern is:

> **Compute in cheaper exact images, reconstruct the global exact object, then verify the reconstruction.**

---

## 1. Chinese-remainder computation turns large integer arithmetic into many small modular computations

FLINT provides extensive CRT/multimodular machinery. Large integer values or matrices can be reduced modulo many small coprime primes, computed in machine-word arithmetic, and reconstructed exactly by the Chinese Remainder Theorem.

Sources:

https://flintlib.org/doc/fmpz.html

https://flintlib.org/doc/fmpz_mat.html

https://flintlib.org/doc/examples.html

### Architectural implication

A semantic exact integer computation does not require every hot operation to use the final huge integer representation.

Possible execution plan:

```text
exact integer obligation
    -> select prime basis
    -> execute independent modular work cells
    -> CRT reconstruction
    -> exact bound/reconstruction check
    -> result
```

This is naturally parallel across CPU cores and, where appropriate, accelerators.

---

## 2. Rational reconstruction recovers exact fractions from modular residues

FLINT supports rational reconstruction: under explicit numerator/denominator bounds, a rational value can be uniquely recovered from its residue modulo a sufficiently large modulus.

Source:

https://flintlib.org/doc/fmpq.html

### Architectural implication

Exact rational computation can often avoid fraction swell:

```text
rational problem
    -> modular images
    -> solve cheaply modulo primes / p-adically
    -> rational reconstruction
    -> exact substitution/check
```

The reconstruction conditions/bounds are mathematical proof obligations and should be part of the execution/certificate artifact.

---

## 3. Large exact rational linear systems already use multimodular/p-adic solving in production libraries

FLINT's rational matrix solver provides fraction-free, Dixon p-adic, and multimodular algorithms. The latter approaches solve over cheaper arithmetic and perform rational reconstruction with adaptive stopping; the documentation identifies them as generally preferable for large systems.

Source:

https://flintlib.org/doc/fmpq_mat.html

### Architectural implication

The project's algorithm selector should consider **coefficient growth/bit complexity**, not just operation count.

A mathematically equivalent route with more scalar operations may be dramatically faster if each scalar operation stays in machine-word modular arithmetic.

This should feed the search-economy cost model.

---

## 4. Multimodular algorithms are a general representation change, not a special number-theory trick

FLINT uses multimodular approaches across:

- integer matrix multiplication;
- rational solving;
- polynomial interpolation;
- polynomial arithmetic/factorization;
- reconstruction tasks.

Sources:

https://flintlib.org/flint-2.1.pdf

https://flintlib.org/doc/fmpq_poly.html

### Architectural implication

The project should consider a generic metaprimitive family:

```text
map_to_modular_images
compute_componentwise
reconstruct_exact
verify_reconstruction
```

whenever the underlying algebra admits suitable homomorphisms.

This connects directly to Theory Graph/morphism research: the modular map is a structure-preserving map into cheaper quotient structures.

---

## 5. Modular images can be scheduled independently and stopped adaptively

CRT/rational reconstruction often does not require knowing the exact number of primes in advance. Additional modular images can be accumulated until a rigorous bound, successful reconstruction, or verified stabilization condition is reached.

FLINT exposes incremental and balanced-tree CRT methods and adaptive reconstruction.

### Architectural implication

This fits the Mathematical Work Cell model extremely well:

```text
Prime cell p1
Prime cell p2
Prime cell p3
...
    -> asynchronous exact residues
    -> reconstruction monitor
    -> stop when certification condition is satisfied
```

Extra cells can be cancelled once enough information exists.

The search economy can therefore treat modular samples as additive exact information rather than independent speculative attempts.

---

## 6. Randomization can improve exact algorithms without making results probabilistic

Las Vegas algorithms use random choices to find favorable reductions/search paths but return a correct answer when they terminate; randomness affects expected runtime rather than result correctness.

Examples exist throughout computer algebra, including modular polynomial factorization/composition.

Sources:

https://arxiv.org/abs/0911.5024

https://arxiv.org/abs/2110.08354

### Architectural implication

The project should distinguish:

```text
DETERMINISTIC EXACT

LAS_VEGAS_EXACT
    random search/runtime, exact admitted result

MONTE_CARLO_PROBABILISTIC
    bounded probability of incorrect verdict
```

These are different execution/certification classes.

A Las Vegas producer can be entirely acceptable for high-assurance mathematics if its final result is independently exact-checkable.

---

## 7. Probabilistic verification can be much cheaper than recomputation

Freivalds' algorithm verifies an `n x n` matrix product in `O(n^2)` randomized time rather than recomputing the product, with exponentially decreasing false-positive probability under repetition.

Sources:

https://link.springer.com/article/10.1007/s00453-016-0202-3

https://arxiv.org/abs/1806.09189

### Architectural implication

Probabilistic checkers can be useful as:

- cheap early filters;
- distributed-compute integrity checks;
- search-economy validation stages;
- low-assurance client results with explicit error bounds.

But a probabilistic checker should not silently promote a theorem into the same authority class as a deterministic/exact certificate.

Possible result metadata:

```text
verification: probabilistic
soundness_error_bound: 2^-k
```

A stronger exact checker may run before permanent primitive/theorem promotion.

---

## 8. Verified modular algorithms show speed techniques can themselves be formally justified

Formal verification work has mechanized algorithms such as Berlekamp-Zassenhaus polynomial factorization, whose real algorithmic structure moves from integers to finite fields/modular rings and lifts/reconstructs factors.

Source:

https://link.springer.com/article/10.1007/s10817-019-09526-y

### Architectural implication

Modular/reconstruction execution is not inherently a heuristic implementation trick.

For mature primitives, the project can establish a theorem that the entire modular computation/reconstruction scheme realizes the semantic mathematical relation.

Then individual executions need only satisfy the scheme's checkable conditions.

---

## 9. Fast exact computation should be representation-polymorphic

A single exact semantic object may flow through several execution representations:

```text
large integer/rational
    -> finite-field residues
    -> machine-word vectors
    -> p-adic approximation
    -> CRT aggregate
    -> reconstructed exact rational/integer
```

Each transition has algebraic semantics.

### Architectural implication

The mathematical compiler should be capable of choosing these representations automatically based on:

- coefficient bit size;
- matrix/polynomial dimensions;
- sparsity;
- available bounds;
- parallel hardware;
- reconstruction cost;
- certificate/checking route.

---

## 10. Modular computation is naturally distributable but does not require distributed infrastructure

Each modular image is often independent.

Therefore ordinary local multicore CPU execution can exploit the structure first:

```text
core 1 -> mod p1
core 2 -> mod p2
...
```

Only very large campaigns need Ptah/distributed nodes.

This aligns with the project constraint that ordinary useful mathematics should remain cheap on ordinary hardware.

---

## 11. Current fast-exact execution hypothesis

For exact algebraic domains, the compiler/search economy may choose among:

```text
direct small exact arithmetic
fraction-free algorithms
modular / multimodular algorithms
p-adic lifting
rational reconstruction
rigorous interval filtering
probabilistic precheck + exact final check
```

based on a Theory Profile and operand/instance cost model.

The client still sees one semantic exact result.

---

## 12. New research obligations

1. Study modular algorithms across Gröbner bases, determinants, characteristic polynomials, resultants, linear solving, interpolation, factorization, and symbolic integration.
2. Investigate certified adaptive termination/reconstruction conditions so modular campaigns stop as early as mathematically safe.
3. Study automatic selection of good primes and handling of unlucky/bad primes with deterministic provenance.
4. Determine how modular work cells should be parallelized/cache-shared on CPU before GPU/distributed execution.
5. Investigate black-box exact linear algebra (Wiedemann/Lanczos families) for huge sparse systems.
6. Study deterministic and probabilistic certifying algorithms for rank, determinant, inverse, matrix products, and polynomial identities.
7. Define explicit Las Vegas vs Monte Carlo execution/certification classes in Theory Profile/result metadata.
8. Investigate whether modular-image computations can be reused across several simultaneous mathematical obligations.
9. Study proof/certificate compression for reconstructed exact results so clients need not retain all intermediate residues indefinitely.
10. Integrate bit-complexity/coefficient-growth prediction into the mathematical search economy and native compiler.
