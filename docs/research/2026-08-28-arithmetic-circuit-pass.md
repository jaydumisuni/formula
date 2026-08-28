# Research Pass — Arithmetic Circuits, Succinct Formula Programs, and Identity Testing

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates mathematical structures whose natural representation is already executable/program-like: arithmetic circuits and straight-line programs. These are especially relevant because the project must allow a discovered formula to exist as a compact program rather than force expansion into conventional human notation.

---

## 1. Circuit/DAG representations can be exponentially more compact than expanded mathematics

Arithmetic circuits represent polynomials through shared addition/multiplication subcomputations rather than explicit monomial expansion. Related straight-line/grammar representations can encode objects exponentially larger than the representation itself.

Representative compressed-SLP discussion:

https://link.springer.com/article/10.1007/s00224-013-9443-6

Arithmetic-circuit PIT literature demonstrates circuits with extremely high degree and enormous numbers of monomials relative to circuit size.

Representative source:

https://arxiv.org/abs/1611.07235

### Architectural implication

The project must never assume:

```text
canonical semantic object
    -> expand into explicit formula
```

is a safe normalization strategy.

A discovered mathematical construction may be canonically represented by a compact DAG/circuit/program even when its full symbolic expansion is astronomically larger.

---

## 2. Syntax/DAG size and semantic expansion size are different complexity measures

A circuit of modest size can compute a polynomial whose degree, term count, or expanded representation is enormous.

### Architectural implication

Theory Profile / representation metrics should track multiple sizes:

```text
circuit/DAG node count
syntactic degree
semantic degree
estimated expanded term count
shared-substructure ratio
coefficient bit growth
```

Search/extraction should optimize the compact executable representation rather than defaulting to expression-tree size.

---

## 3. Polynomial Identity Testing is the natural equivalence problem for arithmetic circuits

Polynomial Identity Testing (PIT) asks whether an arithmetic circuit computes the zero polynomial, and equivalence of circuits reduces to testing the difference circuit for identity.

General randomized tests based on Schwartz-Zippel give probabilistic polynomial-time algorithms over suitable fields, while broad deterministic polynomial-time PIT remains a major open problem in algebraic complexity.

Sources:

https://simons.berkeley.edu/talks/albert-atserias-upc-barcelona-2024-04-17

https://link.springer.com/article/10.1007/s00037-007-0226-9

### Architectural implication

Compact representation does **not** imply cheap canonical equality.

A circuit backend may expose:

```text
identity_test:
    randomized complete-with-error-bound
```

for the broad class, while narrower circuit profiles expose deterministic exact tests.

This belongs in Theory Profile and result certification metadata.

---

## 4. Restricted circuit structure can recover deterministic identity testing

PIT has deterministic polynomial-time algorithms for various restricted circuit classes; current research continues expanding these tractable structural fragments.

Example 2026 result:

https://arxiv.org/abs/2602.20832

### Architectural implication

Again, the right response to a hard broad class is structural profiling:

```text
circuit depth
fan-in
sparsity
rank
powering structure
commutativity
algebra dimension
```

may move an obligation into a deterministic exact identity-testing fragment.

This directly connects arithmetic-circuit semantics to parameterized/structural complexity research.

---

## 5. Arithmetic circuits are a concrete instance of 'formula that happens to be a program'

An arithmetic circuit is simultaneously:

```text
mathematical representation
executable computation DAG
compressed formula
```

### Architectural implication

The project's deepest semantic model should be able to embed circuit-like constructions naturally without treating them as foreign source code.

A circuit can participate in:

- composition;
- substitution;
- differentiation;
- modular evaluation;
- identity testing;
- factorization/reconstruction;
- compilation to native CPU/GPU kernels;
- theory morphisms.

---

## 6. Evaluation images can test circuit identities without expansion

Schwartz-Zippel-style PIT evaluates a compact polynomial/circuit at selected field points rather than expanding every monomial.

### Architectural implication

This is another example of the project-wide principle:

```text
do not materialize semantic expansion
use structure-preserving observations/images
```

A candidate equivalence can first pass cheap probabilistic image tests before stronger certification is attempted.

---

## 7. Exact deterministic checking may exploit domain-native algebra rather than generic circuit expansion

Hardware/arithmetic-circuit verification systems use Gröbner bases, polynomial reduction, and order/phase optimizations to prove circuit correctness algebraically.

Representative current result:

https://link.springer.com/article/10.1007/s10703-026-00494-9

Older formal-methods work similarly verifies arithmetic circuits via computer algebra.

Source:

https://link.springer.com/article/10.1007/s10703-018-00329-2

### Architectural implication

The same semantic circuit may support several identity/equivalence routes:

```text
random evaluation / PIT
symbolic polynomial reduction
Gröbner/ideal reasoning
bit-level SMT/SAT reduction
formal proof
```

The search economy can choose based on structure and required assurance.

---

## 8. Succinct representations change hardness

Problems that are easy on explicitly expanded objects can become much harder when the same object is given by a succinct circuit/SLP representation, because one small input encodes an exponentially larger semantic object.

Representative compressed-word discussion:

https://link.springer.com/article/10.1007/s00224-024-10173-z

### Architectural implication

Theory Profile must include **representation-sensitive complexity**.

The semantic problem class alone does not determine cost:

```text
explicit polynomial
vs
arithmetic circuit polynomial
```

can have radically different algorithmic profiles.

---

## 9. Circuit representations should preserve sharing as semantic/optimization evidence

If a subconstruction appears repeatedly, a DAG/circuit shares it physically:

```text
T
 -> used by A
 -> used by B
 -> used by C
```

instead of storing/evaluating three independent copies.

### Architectural implication

The mathematical compiler should treat shared substructure as first-class and preserve common-subexpression structure through semantic transformations whenever beneficial.

This supports the CPU-first goal through:

- memoization;
- DAG scheduling;
- vectorization;
- cache-aware execution;
- common-subproof reuse;
- modular-image reuse.

---

## 10. Arithmetic circuit identity highlights a trust-class distinction

For general PIT:

```text
randomized evaluation passes
```

is powerful evidence with a known soundness-error bound but is not the same as:

```text
deterministic exact identity certificate
```

### Architectural implication

Candidate formula/program equivalence should retain certification type explicitly. A probabilistic PIT result can prune/search aggressively, but permanent semantic merging may require stronger evidence depending on project policy.

---

## 11. Current succinct-formula hypothesis

The semantic substrate should permit mathematical objects whose primary representation is:

```text
acyclic computation DAG / arithmetic circuit
cyclic/recursive relation with separate productivity semantics
hypergraph relation
symbolic automaton
```

without requiring conversion to an explicit expression tree.

Representation-specific equality/canonicalization algorithms are chosen through Theory Profile.

---

## 12. New research obligations

1. Study arithmetic circuit normal forms/canonicalization for restricted classes and how they relate to e-graph equivalence.
2. Investigate deterministic and certificate-producing PIT for important practical circuit families.
3. Study black-box versus white-box circuit algorithms and what semantic metadata each requires.
4. Investigate circuit factorization/reconstruction without full expansion.
5. Study automatic differentiation, substitution, elimination, and composition directly on compact circuits.
6. Determine how arithmetic circuits map into the project's proposed semantic e-hypergraph representation while preserving sharing.
7. Investigate circuit lower bounds/hardness metadata as Theory Profile knowledge.
8. Study modular-image/PIT reuse across many candidate circuits in one campaign.
9. Determine when probabilistic equivalence evidence is sufficient for temporary e-graph merging versus permanent mathematical identity.
10. Investigate translation validation from semantic arithmetic circuits into native CPU/SIMD/GPU realization graphs.
