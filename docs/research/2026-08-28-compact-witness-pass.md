# Research Pass — Compact Witnesses, Conflict Cores, and Learned Boundaries

**Date:** 2026-08-28  
**Status:** RESEARCH supplement  

This pass investigates how the unnamed mathematical project should turn a large failed search, contradiction, proof, or counterexample into a small mathematically meaningful artifact that can prune future candidate spaces and guide representation/search changes.

The central finding is that there are several different compression targets and they must not be conflated:

```text
valid proof/refutation
    !=
proof dependency core
    !=
minimal conflict
    !=
minimal repair
    !=
interpolant / generalized boundary
```

Each establishes a different claim and therefore needs its own certification semantics.

---

## 1. Proof trimming can remove irrelevant derivation material

DRAT-trim validates propositional UNSAT certificates and can additionally emit:

- trimmed input formulas;
- optimized proofs containing only needed lemmas;
- dependency graphs.

Sources:

https://www.cs.utexas.edu/~marijn/drat-trim/

https://github.com/curtisbright/drat-trim-t

### Architectural implication

A massive solver trace need not become the durable mathematical artifact.

Possible pipeline:

```text
raw solver proof
    -> independently verify
    -> backward dependency analysis
    -> trimmed proof/core
    -> reverify trimmed artifact
```

The raw trace may remain archived for provenance, while the trimmed witness becomes the active dependency object.

The trimming/minimization transformation must not inherit truth authority from the original proof automatically; the reduced result should be rechecked.

---

## 2. Verified proof checking is necessary even for supposedly simpler checkers

Formal work on DRAT checking found a real missing check and parser/overflow issues in the optimized DRAT-trim implementation while creating machine-verified certificate checkers.

Source:

https://link.springer.com/article/10.1007/s10817-019-09525-z

### Architectural implication

The project's earlier law remains important:

```text
solver complexity > checker complexity
```

does not imply:

```text
checker = automatically trustworthy
```

Certificate envelopes should preserve exact checker identity/version/trust lineage, and high-value certificate families may justify formally verified checkers.

---

## 3. Minimal Unsatisfiable Subsets identify irreducible reasons for failure

For a set of constraints `F`, a Minimal Unsatisfiable Subset (MUS) is an unsatisfiable subset such that removing any one member makes it satisfiable.

This is stronger than an arbitrary UNSAT core.

Representative sources:

https://pmc.ncbi.nlm.nih.gov/articles/PMC9622537/

https://link.springer.com/chapter/10.1007/978-3-030-81688-9_15

SMT solvers are mature enough that SMT-COMP maintains a dedicated unsat-core track.

Source:

https://smt-comp.github.io/2024/results/results-unsat-core/

### Mathematical analogue

A failed mathematical world may involve hundreds of assumptions, constraints, transformations, or derived facts.

Instead of retaining only:

```text
World W is impossible
```

we should seek, when useful:

```text
Irreducible Conflict Core
    assumptions: A3, A17
    constraints: C4, C9
    relation: R12
```

such that this smaller set alone is sufficient for contradiction and no listed member is dispensable.

This is a strong pruning artifact.

---

## 4. QuickXplain generalizes minimal conflict extraction beyond SAT syntax

QuickXplain finds an irreducible subset with a declared monotone property using a divide-and-conquer strategy and a black-box predicate/constraint checker.

A formal correctness proof was published in 2022.

Source:

https://pmc.ncbi.nlm.nih.gov/articles/PMC9622537/

### Architectural implication

Minimal-conflict extraction can potentially be a **generic metaprimitive** whenever the target property is monotone over a finite set of candidate assumptions/artifacts:

```text
minimal_subset(subject_to = monotone_property)
```

This can apply beyond logical clauses to bounded families of:

- assumptions;
- rewrite rules;
- dependency hypotheses;
- transformation requirements;
- domain restrictions;
- candidate invariants.

It does not apply universally; the monotonicity condition must be established or declared.

---

## 5. Minimal Correction Sets give the dual repair information

A Minimal Correction Set (MCS) is a subset whose removal restores satisfiability, with no proper subset having that property.

MUS and MCS families are tightly related through hitting-set duality.

Sources:

https://www.ijcai.org/Proceedings/2018/188

https://link.springer.com/chapter/10.1007/978-3-030-81688-9_15

### Architectural implication

For a failed mathematical world, the useful question is not only:

> What is the irreducible reason this is impossible?

but also:

> What is the smallest declared structure that must change for this route to become possible?

Possible artifact:

```text
Minimal Repair Set
    remove/weaken/change: A7, C12
    restores: candidate-space nonemptiness
    witness: satisfiable/reachable model
```

This can guide assumption-world branching and representation refinement.

---

## 6. Prime implicates correspond to minimal reusable nogoods in propositional settings

An implicate is a logical consequence; a prime implicate is inclusion-minimal among implicates. In propositional SAT literature, implicates correspond to nogoods and prime implicates to minimal nogoods.

Source:

https://www.sciencedirect.com/science/article/pii/0004370295000534

### Architectural implication

The project should distinguish:

```text
one observed failure
```

from:

```text
minimal generalized exclusion implied by the current mathematical theory
```

For domains with a suitable implication language, compiling failures into prime/minimal nogoods can be much stronger than retaining individual counterexamples.

The exact analogue outside propositional logic will depend on the domain/theory and may require interpolation, algebraic ideals, inductive invariants, or another native generalization mechanism.

---

## 7. Craig interpolation derives a boundary condition using only shared vocabulary

If `A ∧ B` is unsatisfiable, a Craig interpolant `I` satisfies roughly:

```text
A => I
I ∧ B is unsatisfiable
```

while `I` mentions only symbols shared between `A` and `B`.

Craig interpolation is widely used in verification for automatic invariant inference and abstraction generation.

Recent overview:

https://arxiv.org/abs/2602.08532

SMTInterpol is an SMT solver specifically supporting interpolation and proof production/checking.

Sources:

https://ultimate.informatik.uni-freiburg.de/smtinterpol/proofs.html

https://ultimate.informatik.uni-freiburg.de/smtinterpol/online/proof.html

### Architectural implication

An interpolant is more than a smaller conflict.

It can summarize a **mathematical boundary** between two incompatible regions using only their common language.

Possible use:

```text
failed route / incompatible worlds
    -> derive interpolant I
    -> add I as invariant / abstraction / search constraint
```

This may convert one large proof into a compact reusable condition that future work cells can apply without knowing the entire original derivation.

---

## 8. Proof systems and elaborators can preserve checking while changing witness granularity

Carcara independently checks Alethe SMT proofs and can elaborate coarse proof steps into finer-grained steps that are easier for downstream checkers/proof assistants to verify.

Sources:

https://github.com/ufmg-smite/carcara

https://link.springer.com/chapter/10.1007/978-3-031-30823-9_19

### Architectural implication

Witness transformation may operate in both directions:

```text
coarse compact witness
    -> elaborate for foundational checking

large fine-grained proof
    -> trim/compress for active search use
```

The universal certificate envelope should therefore be able to bind a **witness transformation chain**, with each transformed artifact checked for the claim it is intended to preserve.

---

## 9. Correctness and minimality are separate proof obligations

Suppose a conflict set `K` is claimed to be minimal.

There are at least two properties:

```text
1. K is actually contradictory / impossible.
2. For every k in K, K - {k} is not contradictory.
```

The first can be shown by an UNSAT/refutation certificate.

The second can be shown, in finite constraint settings, by satisfiable witnesses/models for each single-member deletion or another certified minimality procedure.

### Architectural implication

A certificate envelope for a compact witness should explicitly distinguish:

```text
soundness/correctness certificate
minimality/irredundancy certificate
```

A mathematically valid but non-minimal core can still be useful.

The system must never relabel it `minimal` merely because a heuristic minimizer returned it.

---

## 10. Compact-witness artifacts should be first-class mathematical objects

Current evidence supports several distinct durable artifact families:

```text
Proof Dependency Core
    minimality not necessarily claimed

Irreducible Conflict Core
    subset-minimal contradiction

Minimal Repair / Correction Set
    subset-minimal change restoring feasibility

Distinguishing Witness
    compact evidence separating candidate families

Prime / Generalized Nogood
    reusable minimal exclusion when domain supports it

Interpolant / Learned Boundary
    consequence stated in shared vocabulary

Characterizing Witness Set
    compact identification/regression signature, not proof
```

These artifacts may reference one another but should not be collapsed into one generic `explanation` object.

---

## 11. Current compression pipeline hypothesis

A failed or successful large computation may undergo several post-processing stages:

```text
RAW COMPUTATION / PROOF
    |
    +-> independent validity check
    |
    +-> dependency slicing / trimming
    |
    +-> irreducible conflict extraction
    |
    +-> repair-set extraction
    |
    +-> interpolant / generalized nogood / invariant synthesis
    |
    +-> independent recheck of each promoted compact artifact
    |
    +-> candidate-space restriction / Theory Profile update
```

Not every stage applies to every mathematical domain.

The Theory Profile / certificate catalogue should declare which compact-witness operations are sound and available for a given region.

---

## 12. Search-economy implication

The value of a work cell should include not only whether it solves the immediate branch but also the **compression/pruning leverage** of the artifact it produces.

For example, a computation producing a certified interpolant that eliminates millions of candidates may be more valuable than one that merely rejects a single candidate.

Possible value dimensions:

```text
candidate-family elimination
assumption-world elimination
proof dependency compression
repair guidance
future search reuse
certificate checking cost
artifact generality
```

This strengthens the earlier search-economy hypothesis that mathematical work must be scored by reusable mathematical information, not only task completion.

---

## 13. New research obligations

1. Investigate proof-producing or independently checkable MUS/MCS extraction so minimality claims can be certified efficiently.
2. Study interpolation certificate formats and whether Alethe/SMT proof evidence can support independent interpolant checking rather than trusting an interpolating solver.
3. Investigate unsat-core extraction for nonlinear arithmetic, algebraic geometry, optimization, and other non-Boolean domains.
4. Study minimal proof/dependency slicing in proof assistants and theorem provers.
5. Investigate prime implicate/implicant compilation and compact representations such as BDD/ZBDD for large families of minimal nogoods.
6. Determine how interpolants and generalized nogoods should bind to assumption/world identity so they are not applied outside their valid scope.
7. Investigate minimal correction/repair sets for mathematical representation choices, not only logical constraints.
8. Determine when witness minimization cost is justified by expected future pruning value.
9. Investigate proof-core/interpolant extraction directly from e-graph/equality-saturation explanations.
10. Define how compact artifacts are invalidated when their supporting theory, assumptions, or checker changes.
