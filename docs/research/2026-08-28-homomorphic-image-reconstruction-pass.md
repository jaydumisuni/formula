# Research Pass — Homomorphic Images, Modular Computation, Evaluation/Interpolation, and Exact Reconstruction

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates a general high-performance exact-computation pattern: replace one expensive mathematical object/problem by many cheaper images under structure-preserving maps, compute there, and reconstruct/lift an exact result with independent verification.

The central finding is:

> **Modular algorithms, evaluation/interpolation, Hensel lifting, and rational reconstruction are instances of a broad execution schema: `map to cheap images -> compute independently -> stabilize/reconstruct -> verify exact lift`. This can provide massive CPU parallelism and avoid coefficient/expression swell while preserving exact mathematical authority.**

---

## 1. Modular computation changes representation before doing expensive algebra

Modern Computer Algebra describes modular algorithms as a central representation technique: solve a problem over easier residue rings/fields rather than directly over large integers/rationals, then reconstruct the result.

Source:

https://www.cambridge.org/core/books/abs/modern-computer-algebra/modular-algorithms-and-interpolation/6D3BFE7FECF1F8B37061D012AA75DA31

### Architectural implication

The compiler should search for certified structure-preserving maps:

```text
h_i : A -> B_i
```

where operations are much cheaper in each `B_i`.

Then a computation can become:

```text
P over A
    -> h_1(P), h_2(P), ...
    -> independent cheap solves
    -> reconstruct result in A
    -> verify
```

---

## 2. Modular jobs are naturally parallel

Modular Gröbner-basis algorithms over `Q` compute bases modulo many primes independently, lift using Chinese remaindering/rational reconstruction, and verify the reconstructed result.

Source:

https://arxiv.org/abs/1005.5663

Recent massively parallel frameworks apply this pattern across commutative algebra and algebraic geometry and report good scaling across processor cores.

Source:

https://arxiv.org/abs/2401.11606

### Architectural implication

This is an ideal Mathematical Work Cell formation:

```text
Cell p1 -> solve mod p1
Cell p2 -> solve mod p2
...
Cell pn -> solve mod pn

Reconstruction cell -> combine
Verification cell -> certify exact result
```

No GPU is required for substantial parallel speedup.

---

## 3. The map/lift boundary is a mathematical contract

A modular computation is valid only when the chosen image preserves enough relevant structure and the reconstruction conditions hold.

Some primes may be “bad” for a particular problem and produce misleading structural images.

Source:

https://arxiv.org/abs/1207.1651

### Architectural implication

An `ImageComputationPlan` must carry:

```text
homomorphism/image family
preserved operations/properties
exceptional/bad image conditions
reconstruction theorem
stabilization criterion
verification route
```

The execution engine cannot infer validity merely because several modular answers agree.

---

## 4. Error-tolerant reconstruction can survive bad images

Rational-reconstruction research develops methods that tolerate a bounded amount of incorrect modular information rather than requiring perfect prior identification of every good prime.

Source:

https://arxiv.org/abs/1207.1651

### Architectural implication

The project can treat individual image Work Cells as replaceable/untrusted producers.

A reconstruction algorithm may be certified to tolerate a declared failure/error model:

```text
many independent cheap images
    + bounded bad images
    -> exact reconstruction
```

This resembles fault-tolerant evidence combination and can increase robustness of distributed mathematical campaigns.

---

## 5. Evaluation/interpolation is another homomorphic-image pattern

A polynomial can be represented by coefficients or by values at selected points. Many algorithms evaluate a large symbolic object into simpler scalar/lower-dimensional instances, compute there, then interpolate the exact symbolic result.

Sources:

https://www.cambridge.org/core/books/abs/modern-computer-algebra/modular-algorithms-and-interpolation/6D3BFE7FECF1F8B37061D012AA75DA31

https://www.sciencedirect.com/science/article/abs/pii/S0885064X25000123

### Architectural implication

The representation compiler should recognize:

```text
coefficient-space computation
```

versus:

```text
evaluation-space computation
```

and choose the cheaper one under exact reconstruction guarantees.

This is analogous to Fourier/transform methods but driven by algebraic homomorphisms and interpolation.

---

## 6. Black-box executable mathematics can be reconstructed from evaluations

Sparse interpolation can recover a compact polynomial from evaluation access, including cases where the polynomial is represented by a straight-line program/black box rather than expanded coefficients.

Sources:

https://www.sciencedirect.com/science/article/pii/S0747717122000517

https://www.sciencedirect.com/science/article/pii/030439759190157W

### Architectural implication

A discovered executable construction may be analyzable through probes/images without opening or expanding its internal formula:

```text
black-box semantic primitive
    -> evaluate under certified chosen points/images
    -> infer sparse symbolic representation
    -> verify equivalence
```

This provides a possible **semantic compression/decompilation** route for program-as-mathematics artifacts.

---

## 7. Reconstruction is often cheaper than direct symbolic computation

Sparse factorization research performs work on evaluations and reconstructs output using sparse interpolation specifically to avoid direct symbolic overhead/expression swell.

Source:

https://www.sciencedirect.com/science/article/abs/pii/S0885064X25000123

### Architectural implication

The cost model should compare:

```text
direct exact solve cost
```

against:

```text
number_of_images * cheap_image_cost
+ reconstruction_cost
+ verification_cost
```

rather than assuming direct symbolic calculation is authoritative or preferable.

---

## 8. Stabilization can determine how many images are needed adaptively

Modern modular frameworks add primes/images until reconstruction stabilizes with high probability, then perform verification.

Source:

https://arxiv.org/abs/2401.11606

### Architectural implication

The campaign can be adaptive:

```text
launch batch of image cells
    -> reconstruct candidate
    -> stabilization/certification sufficient?
        yes -> stop remaining work
        no -> launch more images
```

This links directly to certified progress/search economy.

The number of images need not be fixed before computation begins.

---

## 9. Verification remains independent of reconstruction heuristics

Modular Gröbner algorithms explicitly contain a verification stage after lifting/reconstruction.

Sources:

https://arxiv.org/abs/1005.5663

https://link.springer.com/article/10.1007/s11786-022-00539-2

### Architectural implication

The familiar architecture remains:

```text
fast image producer(s)
    -> candidate exact lift
    -> independent exact/domain-native checker
```

Agreement/stabilization can choose when to attempt checking but does not itself become proof unless the reconstruction theorem/certificate makes it so.

---

## 10. Homomorphic images can be selected for native machine arithmetic

Reduction modulo word-sized primes converts huge rational/integer algebra into tight native integer operations, often avoiding expensive bignum coefficient growth.

This reinforces the earlier FLINT fast-exact result.

### Architectural implication

CPU-first realization search should actively ask:

```text
Can this exact problem be mapped into many u32/u64 arithmetic worlds?
```

before escalating to huge arbitrary-precision operations or GPU kernels.

---

## 11. Image computation is broader than modular primes

The abstract schema includes:

```text
quotient rings
finite fields
specialization/evaluation points
projections
homomorphic theory images
finite approximations with certified lift
```

provided a reconstruction/transport theorem exists.

### Architectural implication

The project should represent this generically as a **Homomorphic Image Plan**, not as hardcoded CRT infrastructure.

Domain plugins/theory packages can register new image/lift families with the Theory Profile.

---

## 12. Current homomorphic-image hypothesis

```text
EXACT MATHEMATICAL PROBLEM P over structure A
    -> Theory Profile finds admissible image family h_i:A->B_i
    -> choose image schedule/cost model
    -> run many cheap independent image computations
    -> detect/stabilize structural answer
    -> reconstruct/lift candidate result to A
    -> exact independent verification
    -> publish semantic result
```

This may become one of the core reasons very large exact mathematics remains fast on ordinary multicore CPUs.

---

## 13. New research obligations

1. Define generic HomomorphicImage/Lift contracts independent of rings/polynomials.
2. Study automatic discovery/selection of useful quotient/image maps.
3. Investigate certificate formats for reconstruction and image-goodness assumptions.
4. Study fault/error-tolerant reconstruction across distributed Work Cells.
5. Connect modular image scheduling to Sergeant-style large formations and Ptah later.
6. Investigate evaluation/interpolation over arithmetic circuits/e-hypergraphs without expansion.
7. Study image methods for matrices, ideals, rational maps, graph invariants, and other non-polynomial structures.
8. Determine how image computations interact with semiring-parametric factorized representations.
9. Develop cost/profile rules for when direct exact arithmetic is cheaper than image/lift strategies.
10. Investigate whether repeated successful image plans can be distilled into automatic domain-specific execution primitives.
