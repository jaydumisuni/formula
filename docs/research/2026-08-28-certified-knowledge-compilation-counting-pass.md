# Certified Knowledge Compilation and Exact Counting Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

The certificate atlas identified exact counting (#SAT/model counting) as a major case where the final scalar count does not naturally provide a tiny deterministic witness.

Verifiable computation gives one generic fallback, but knowledge compilation offers a stronger route when the Boolean structure can be compiled into a tractable representation.

## 1. d-DNNF makes counting structural

Deterministic decomposable negation-normal form (d-DNNF) supports efficient exact model counting because:

- decomposable AND nodes combine subproblems over disjoint variable sets;
- deterministic OR nodes represent mutually exclusive alternatives.

As a result, the count can be evaluated bottom-up over the circuit rather than enumerating all models.

Sources:
- https://www.logicng.org/documentation/knowledge-compilation/dnnf/
- https://www.ijcai.org/proceedings/2024/367

For d-DNNF, model counting is linear/polynomial in the size of the compiled representation even though the original formula may have exponentially many satisfying assignments.

This is another instance of:

```
cardinality of solution set
      !=
size of useful representation
```

## 2. Certified knowledge compilation

Bryant, Nawrocki, Avigad, and Heule developed a certified knowledge-compilation pipeline using Partitioned-Operation Graphs (POGs) and CPOG equivalence proofs.

Pipeline:

```
CNF formula φ
      ↓
untrusted D4 knowledge compiler
      ↓
decision-DNNF / POG representation G
      ↓
untrusted proof generator
      ↓
CPOG equivalence proof
      ↓
small proof checker verifies G ≡ φ
      ↓
ring evaluator computes exact/weighted model count
```

The POG proof system, checker, and model counter have formally verified Lean 4 implementations.

Sources:
- https://arxiv.org/abs/2501.12906
- https://www.cs.cmu.edu/~bryant/pubdir/sat23.pdf
- https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.SAT.2023.6

The key trust transfer is explicit:

> the complex compiler/proof generator need not be trusted; equivalence is established by the checker.

This is almost exactly the project's producer/checker constitution.

## 3. Earlier certifying decision-DNNF work

Top-down decision-DNNF compilers have also been modified to emit annotations/certificates allowing a polynomial-time checker to verify that a compiled decision-DNNF is equivalent to the source CNF.

Source:
- https://ojs.aaai.org/index.php/AAAI/article/view/16776

This confirms that certification is not tied to one specific POG implementation.

## 4. #SAT proof systems exist

Model-counting proof systems include:

- knowledge-compilation-based proof systems (`kcps(#SAT)` / certifiable decision-DNNF);
- MICE proof logging for model counters;
- POG/CPOG-style certified compilation.

Source:
- https://journals.sagepub.com/doi/full/10.3233/SAT-231507

So exact model counting is not inherently outside certificate-producing computation.

The certificate may simply be a **compiled representation plus equivalence proof**, rather than a tiny scalar witness.

## 5. Architecture-changing conclusion

The project should recognize a distinct certificate family:

```
CERTIFIED_COMPILED_REPRESENTATION
```

where authority comes from:

```
source semantics
      ≡
compiled tractable representation
```

and many later queries are answered by evaluating that representation.

This differs from:

- per-query proof traces;
- interactive statistical proofs;
- exact recomputation;
- foundational theorem proofs.

## 6. Representation as reusable mathematical primitive

A certified compiled representation can answer more than one query.

Depending on the language, it may support:

```
model count
weighted model count
probability
conditioning
model enumeration
sampling
marginals
existence
```

without repeating the expensive compilation.

Therefore the project should value the discovery of a good representation according to **future query leverage**, not only the first query cost.

This connects directly to the search economy:

```
one expensive compilation
      ↓
many cheap future mathematical queries
```

may dominate:

```
one specialized answer per query
```

## 7. Semiring connection

POGs/d-DNNFs can be evaluated using different rings/semirings for ordinary or weighted counting.

That connects certified knowledge compilation to the prior semiring-parametric evaluation research:

```
certified structural graph
       ↓ choose evaluation algebra
Boolean       -> satisfiability/existence
Natural       -> model count
Weighted sum-product -> weighted count/probability
other algebra -> other aggregate
```

One certified structural compilation can therefore become multiple mathematical capabilities.

## 8. Structural parameterization

Knowledge compilation can still explode exponentially.

2024/2025 work shows fixed-parameter tractable d-DNNF compilation for classes parameterized by incidence treewidth, while general d-DNNF representations can be exponentially large.

Sources:
- https://www.ijcai.org/proceedings/2024/367
- https://arxiv.org/abs/2502.00434

Therefore the Theory Profile should record:

```
compiled_language
expected/known width parameters
size bounds
canonicality properties
tractable queries
operations closed in representation
```

The project should search for decomposition/low-width structure before committing to knowledge compilation.

## 9. Choosing between compiled certificate and interactive proof

For exact counting, there are now at least three routes:

### Route A: certified knowledge compilation

Best when:
- compiled representation remains manageable;
- many repeated queries are expected;
- structure supports decomposability/low width;
- deterministic authority is desired.

### Route B: line/proof-log model-counting certificate

Best when:
- solver naturally emits a checkable derivation;
- reusable compiled representation is unnecessary or too large.

### Route C: interactive proof / sum-check

Best when:
- computation can be arithmetized efficiently;
- native deterministic certificates/compiled representations are too expensive;
- statistical assurance is acceptable under declared error.

This is a direct example of Certificate Router economics.

## 10. Partial compilation

Partial knowledge compilation can stop before full expansion and leave unresolved/unknown leaves, using sampling/other methods for approximation.

Source:
- https://arxiv.org/abs/1805.07180

For the project this suggests a richer hybrid possibility:

```
certified exact compiled region
      +
unresolved region
      ↓
additional Work Cells / interactive proof / exact fallback
```

A partially compiled structure should not be presented as an exact total count unless unresolved regions are subsequently closed by an admissible authority path.

## 11. Core law

> **When a hard query can be transformed into a certified representation where the query is structurally cheap, preserve the representation—not merely the answer.**

This is one of the clearest examples of mathematics becoming reusable computation.

## 12. Open research

1. Which compiled languages (d-DNNF, SDD, OBDD, POG, arithmetic circuits, automata) are best for which query families?
2. Certificate-preserving transformations between compiled languages.
3. Canonical forms and structural identity for compiled representations.
4. Certified weighted model counting over richer semirings/rings.
5. Extending knowledge compilation to #SMT and richer theories.
6. Automatic prediction of compiled representation size before paying full compilation cost.
7. Hybrid compiled/oracle/interactive representations for instances that cannot be fully compiled.
8. Black-boxing compiled representations into smaller interfaces for downstream packages.
9. Promotion of frequently queried certified compiled objects into permanent primitives/indexes.
