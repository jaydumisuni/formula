# Research Pass — Proof-to-Program Extraction and Constructive Solver Synthesis

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates a direct route from new mathematical proof to new executable capability: constructive proofs and proof-producing synthesis can contain enough computational content to mechanically extract a program satisfying the proved specification.

The central finding is:

> **For appropriately constructive/executable specifications, a proof can be more than a certificate about an algorithm: the proof itself can determine an executable solver/witness constructor. This creates a direct `prove -> extract -> specialize -> validate -> promote` path for new mathematical primitives.**

---

## 1. Rocq/Coq extracts executable programs from proofs/specifications

Rocq's extraction framework builds certified functional programs from Rocq functions or proofs of specifications and emits executable source such as OCaml or Haskell.

Sources:

https://rocq-prover.org/doc/V8.19.1/refman/addendum/extraction.html

https://rocq-prover.org/docs/tour-of-rocq

### Architectural implication

A constructive theorem of shape conceptually like:

```text
forall x, exists y, P(x,y)
```

may carry a computational witness constructor:

```text
solve(x) -> y
```

with correctness inherited from the proof.

Thus a newly proved mathematical existence theorem can sometimes become a new executable primitive automatically.

---

## 2. Logical proof content and executable content can be separated

Rocq extraction distinguishes informative/computational content from purely logical proof content and erases logical material during program extraction.

Sources:

https://rocq-prover.org/papers/extracting-fs-programs-from-proofs-in-the-calculus-of-constructions

https://rocq-prover.org/doc/V8.19.1/refman/addendum/extraction.html

### Architectural implication

A rich proof can produce a much smaller runtime program:

```text
large theorem/proof object
    -> erase noncomputational evidence
    -> residual witness algorithm
```

This strongly supports the project's principle that **mathematical assurance need not impose its full representation overhead on ordinary execution**.

---

## 3. Certified-program development can also start from an algorithm skeleton

Rocq's `Program` mechanism lets a developer provide an algorithmic skeleton under a rich dependent/refinement-style specification; the system generates proof obligations that must be discharged before the program is accepted.

Sources:

https://rocq-prover.org/doc/master/refman/addendum/program.html

https://rocq-prover.org/doc/v8.12/refman/addendum/program.html

### Architectural implication

The project can support two complementary paths:

```text
PROOF-FIRST
specification -> proof -> extracted program

PROGRAM-FIRST
candidate program -> generated proof obligations -> certified program
```

Both produce the same broader artifact family:

```text
semantic specification
executable realization
correctness lineage
```

---

## 4. Automated theorem proving can synthesize programs while proving correctness

Recent saturation-based synthesis work extends first-order theorem proving so a proof of a functional specification simultaneously constructs a recursion-free program satisfying the specification.

Source:

https://link.springer.com/chapter/10.1007/978-3-031-38499-8_18

The implementation extends the Vampire theorem prover and tracks answer substitutions/program branches during proof search.

### Architectural implication

Program synthesis and theorem proving need not be separate sequential campaigns.

A Work Cell may search in a calculus whose successful proof object **contains the executable construction**.

This gives another powerful solver architecture:

```text
functional mathematical specification
    -> proof search
    -> proof + executable witness program
```

---

## 5. Verified decision procedures can be extracted once and reused cheaply

Current 2026 work formalizes modal/fixpoint-logic tableau procedures in Coq, proves their soundness/completeness/termination, then extracts executable OCaml implementations.

Source:

https://link.springer.com/article/10.1007/s10817-026-09754-z

### Architectural implication

This reinforces the earlier reflective-computation result:

```text
discover/design decision procedure D
    -> prove soundness/completeness/termination for fragment F
    -> extract executable D
    -> optimize realization
    -> future F queries use cheap D
```

The system has gained capability for an entire theory fragment, not merely cached individual theorem answers.

---

## 6. Extracted code still has a realization boundary

Extraction can involve assumptions/axioms and target-language/runtime behavior. Rocq documentation warns that informative axioms must be realized and that arbitrary external realizations become user responsibility.

Source:

https://docs.rocq-prover.org/V8.15.0/refman/addendum/extraction.html

### Architectural implication

The project must not collapse:

```text
proof-certified computational term
```

and

```text
arbitrary external/native code implementing it
```

into one identity.

The previous two-proof law remains:

```text
semantic/proof correctness
    +
realization/translation correctness
```

before a highly optimized native primitive is trusted.

---

## 7. Proof-extracted programs may require aggressive optimization

Constructively extracted programs are not automatically optimal. Research explicitly studies simplification/optimization of programs extracted from proofs while preserving extensional behavior.

Source:

https://www.cambridge.org/core/journals/mathematical-structures-in-computer-science/article/abs/an-application-of-per-models-to-program-extraction/EEEAFDE016625AE6EAF8E6A9E9B7F8BB

### Architectural implication

Program extraction should feed directly into the project's established specialization pipeline:

```text
proof-extracted reference program
    -> partial evaluation / supercompilation
    -> equality saturation / algorithm search
    -> native optimization
    -> translation/equivalence validation
```

The extracted program is a trusted semantic reference, not necessarily the final hot-path realization.

---

## 8. Constructive proof search is itself a restricted synthesis language

Not every true classical theorem yields a useful computable witness directly. Extraction depends on the constructive/informative form of the proof/specification.

### Architectural implication

The Theory Profile / goal compiler should classify when a requested theorem can reasonably target:

```text
constructive witness extraction
```

versus:

```text
nonconstructive existence proof only
certificate of impossibility
classical theorem with separate algorithm synthesis required
```

The system should not assume “proved existence” always means an executable solver is recoverable.

---

## 9. Proof-directed synthesis can generate branch structure

Saturation-based program synthesis derives conditional program fragments during proof search.

Source:

https://link.springer.com/chapter/10.1007/978-3-031-38499-8_18

### Architectural implication

A discovered mathematical formula/program may naturally emerge as a large decision structure:

```text
if condition C1 -> expression E1
else if C2 -> E2
...
```

The project must permit such non-human constructions if they are compact/executable/certified.

Subsequent abstraction learning may then discover a simpler general structure.

---

## 10. Proof extraction creates a direct primitive-promotion route

A stronger self-expansion loop now exists:

```text
NEW MATHEMATICAL SPECIFICATION/THEOREM
    -> constructive proof / proof-directed synthesis
    -> executable witness constructor W
    -> reference semantics automatically linked to proof
    -> specialize/optimize W
    -> validate realization
    -> promote W as new primitive
```

This is different from:

```text
search program first -> prove afterward
```

Both routes should coexist.

---

## 11. A proof can expose more than one computational artifact

Constructive derivations may contain:

```text
witness
algorithm
branch conditions
recursion structure
auxiliary lemmas
bounds
```

### Architectural implication

The extractor should not necessarily publish only the final function.

Useful internal artifacts can enter other project layers:

```text
branch conditions -> representation regimes
lemmas -> rewrite/proof primitives
bounds -> search economy/progress contracts
witness constructors -> execution primitive
```

Thus proof mining can increase capability at several levels simultaneously.

---

## 12. Current proof-to-program hypothesis

```text
FORMAL MATHEMATICAL GOAL
    -> classify constructive/executable content
    -> choose proof calculus supporting witness extraction
    -> automated/hybrid proof search
    -> accepted proof
    -> extract computational content
    -> establish extracted semantic identity
    -> specialize / optimize / lower
    -> realization validation
    -> primitive promotion
```

This is perhaps the cleanest possible case of a **new mathematical formula that literally becomes a program because of how it was proved**.

---

## 13. New research obligations

1. Study proof extraction from Lean, Rocq, Isabelle/HOL code generation, Nuprl, and other systems comparatively.
2. Determine which theorem/specification forms yield useful computational content and which erase to nothing/nonconstructive existence.
3. Investigate automated reformulation of classical goals into constructive witness-producing equivalents where valid.
4. Study proof-directed synthesis beyond recursion-free first-order programs.
5. Investigate extraction from relational proofs into multi-directional programs/propagators rather than one forward function.
6. Study automatic optimization of extracted programs with certified equivalence.
7. Determine how proof-extracted programs retain assumption/world/certificate lineage through specialization.
8. Investigate proof mining for auxiliary rewrite rules, bounds, invariants, and branch regimes.
9. Study when extracted decision procedures should replace generic proof search after sufficient reuse.
10. Investigate whether learned/generalized constructive proofs can automatically synthesize a whole family of new primitives.
