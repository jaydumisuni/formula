# Research Pass — Certificate Interoperability and Verified Native Realizations

**Date:** 2026-08-28  
**Status:** RESEARCH supplement  

This pass investigates two connected questions:

1. whether unrelated mathematical domains can expose a common trust boundary without forcing all proofs into one proof language;
2. whether discovered/certified mathematics can be aggressively optimized into native code without trusting the optimizer/compiler that performed the optimization.

---

## 1. Strongest current conclusion: universal envelope, domain-native certificate bodies

The research does **not** support forcing every mathematical result into one certificate language.

Different domains already have highly effective native certificate formats/checkers:

- rewriting: CPF + CeTA;
- SAT: LRAT/DRAT-family proof traces and independent checkers;
- SMT: Alethe and independent checkers such as Carcara;
- MILP: VIPR certificates and exact checkers;
- graph canonicalization/isomorphism: canonical-labeling proof systems and independent checkers;
- Gröbner/polynomial reasoning: ideal-membership/remainder/change-of-basis certificates checked in Lean/Isabelle and related proof systems;
- rigorous numerics: interval/root/integration certificates checked by Lean/Coq kernels;
- equality saturation: merge/extraction certificates with an Isabelle-verified checker.

The likely cross-domain abstraction is therefore:

```text
UNIVERSAL CERTIFICATE ENVELOPE
    |
    +-- exact claim identity
    +-- assumptions/world identity
    +-- scope
    +-- mathematical property established
    +-- certificate family + version
    +-- certificate artifact digest
    +-- producer identity/version
    +-- checker identity/version
    +-- checker trust root / formal lineage
    +-- checker verdict
    +-- translation/normalization chain
    +-- dependency/input digests
    +-- replay information
    +-- freshness state
```

with the **certificate body remaining domain-native**.

This preserves cheap specialized checking while giving the larger mathematical system one common trust/provenance interface.

This is a research hypothesis, not a frozen schema.

---

## 2. Foundational Proof Certificates show broad-spectrum proof evidence is possible

The Foundational Proof Certificate (FPC) project was created because theorem provers produce radically different proof evidence: resolution, tableaux, natural deduction, proof scripts, Herbrand expansions, and other forms.

FPCs use focused proof systems and programmable clerks/experts so a relatively small checker can elaborate many styles of proof evidence into foundational proofs.

Sources:

https://researchportal.ip-paris.fr/en/publications/foundational-proof-certificates-in-first-order-logic/

https://www.lix.polytechnique.fr/~dale/ProofCert/

The important donor lesson is:

> A common trust layer can be defined above heterogeneous proof evidence without requiring every producer to use the same internal proof search.

However, FPCs are primarily proof-theoretic/logical infrastructure. They do not remove the value of specialized domain certificates whose checks may be much smaller and faster.

---

## 3. Dedukti demonstrates proof-system interoperability

Dedukti is a proof checker based on the lambda-Pi calculus modulo rewriting and is used as a backend for proofs translated from different theorem-proving systems.

The Deducteam project explicitly studies interoperability and translators involving systems such as Agda, HOL Light, Isabelle, Lean, Rocq, automated theorem provers, and SMT solvers.

Sources:

https://github.com/Deducteam/Dedukti

https://deducteam.gitlabpages.inria.fr/

### Architectural implication

The unnamed project may eventually support a **foundational export/recheck layer** for selected accepted results, but should not require all domain-native mathematical certification to be translated through Dedukti/Lean/Rocq synchronously.

Possible trust tiers may eventually include:

```text
native domain checker
    -> accepted domain certificate

optional foundational translation
    -> theorem-prover/logical-kernel replay
```

The exact tiering is not decided.

---

## 4. Gröbner computation now has concrete external-solver / internal-checker patterns

2026 work on polynomial reasoning in Lean delegates heavy Gröbner-basis computation to external CAS systems such as SageMath or SymPy and checks returned computational certificates inside Lean.

Supported certified operations include:

- remainder verification;
- Gröbner-basis checking;
- ideal equality;
- ideal membership;
- radical membership.

Sources:

https://arxiv.org/abs/2604.13514

https://arxiv.org/abs/2602.12772

https://github.com/WuProver/GroebnerTactic

A separate Lean-Macaulay2 interface similarly turns Macaulay2 answers such as ideal-membership/change-of-basis outputs into Lean proof terms.

Source:

https://github.com/riyazahuja/lean-m2

Older Isabelle tooling also constructs Nullstellensatz/ideal-membership certificates from Gröbner computations.

### Architectural implication

The project's `relation_space` / `canonical_basis` operations can potentially use large external algebra engines while keeping mathematical authority in smaller certificate checkers.

---

## 5. Graph canonicalization can be certificate-producing

Canonical labeling algorithms are complex and difficult to trust directly.

A proof system for the McKay/Piperno graph-canonicalization scheme allows a canonical-labeling engine to emit a proof certificate, which is checked by a separate simpler checker. Central soundness results and the checker specification have been formalized in Isabelle/HOL.

Sources:

https://lmcs.episciences.org/10892

https://github.com/milanbankovic/isocert

### Architectural implication

A future `canonicalize_graph` capability need not make nauty/bliss/morphi or another high-performance canonizer part of the trusted mathematical kernel.

A canonicalization output can instead carry:

```text
canonical representative
+ canonicalization certificate
+ checker result
```

This reinforces the project-wide pattern:

> expensive canonicalizer != mathematical authority.

---

## 6. Rigorous numerics can also be certificate-based

LeanCert (2026) separates numerical certificate search from certificate checking.

Its untrusted search machinery can construct candidates for:

- interval bounds;
- global optimization bounds;
- root existence/uniqueness;
- integration;
- transcendental inequalities;
- analytic-number-theory bounds.

Executable checkers then connect the finite certificate to Lean theorems through proved soundness results.

Source:

https://github.com/alerad/leancert

Earlier Coq work likewise established certificate-based formally verified approximation methods using interval arithmetic, Banach fixed-point validation, and Chebyshev approximations.

Source:

https://doi.org/10.4230/LIPIcs.ITP.2019.8

### Architectural implication

Numerical outputs should be able to participate in the same envelope as symbolic proofs while retaining their native semantics:

```text
claim: f(x) in [a,b] under domain D
certificate_family: rigorous_interval
checker: ...
truth_scope: rigorous enclosure
```

They must not be flattened into the same status as exact symbolic equality without preserving verification class.

---

## 7. Optimization certificates continue to mature

VIPR is a general certificate format for mixed-integer linear programming and supports claims including:

- optimality;
- infeasibility;
- relaxed optimality bounds.

Recent work formalizes VIPR verification through SMT encodings to remove ambiguities in the checker semantics.

Source:

https://doi.org/10.1016/j.jsc.2025.102543

Current Lean work on exact linear programming uses SoPlex as an untrusted oracle and validates exact certificates in Lean before returning proof-carrying results.

Source:

https://github.com/leanprover/lp

### Architectural implication

Optimization should be routed through certificate-bearing solvers when the domain permits it, rather than treating a numerical optimizer's reported objective as authoritative.

---

## 8. Translation validation is highly relevant to compiled mathematical primitives

Alive2 validates specific LLVM transformations by proving that optimized target IR refines source IR.

It does **not** require the LLVM optimizer itself to be proven correct.

Source:

https://github.com/AliveToolkit/alive2

### Architectural implication

The unnamed project may eventually use this pattern when turning a certified mathematical relation/construction into fast machine implementations:

```text
certified mathematical semantics
    -> reference executable realization
    -> aggressive optimizer / superoptimizer
    -> optimized native realization
    -> translation/equivalence validation
    -> admitted implementation
```

The optimizer can remain replaceable and untrusted.

This is especially important if primitive generation uses heuristic, randomized, search-based, model-assisted, or hardware-specific optimization.

---

## 9. CryptOpt is a narrow-domain prototype of the desired optimization loop

CryptOpt takes high-level finite-field arithmetic from Fiat Cryptography, performs randomized search over x86-64 assembly candidates, benchmarks those candidates on actual hardware, and then verifies the selected assembly using a formally verified program-equivalence checker connected back to Fiat Cryptography.

Source:

https://github.com/0xADE1A1DE/CryptOpt

Reported results include assembly outperforming GCC/Clang significantly and sometimes surpassing hand-optimized assembly.

The important architectural pattern is:

```text
trusted mathematical specification
    -> huge untrusted performance search
    -> measure on real hardware
    -> select winner
    -> verify semantic equivalence
    -> keep fast implementation
```

This strongly supports the project's earlier CPU-first hypothesis.

A discovery campaign can be expensive, while the promoted primitive can be extremely fast.

---

## 10. Fiat Cryptography proves math specification -> specialized verified code is practical

Fiat Cryptography synthesizes specialized big-integer modular/finite-field arithmetic from simple mathematical specifications and produces correctness proofs.

It targets languages including C, Rust, Zig, Go, and Bedrock2 in current tooling.

Source:

https://github.com/mit-plv/fiat-crypto

Its verified rewriting/partial-evaluation infrastructure has also demonstrated that proved rewrite rules can be assembled into fast compiler engines.

Relevant work:

https://arxiv.org/abs/2205.00862

### Architectural implication

This is one of the strongest existing proofs of the concept:

> high-level mathematics can be specialized into architecture-efficient executable code without sacrificing formal correctness.

The unnamed project seeks to generalize this beyond finite-field arithmetic and beyond a fixed hand-designed transformation family.

---

## 11. Jasmin provides another verified native-code route

Jasmin is a high-assurance language/compiler for efficient cryptographic implementations. Its compiler is formally verified and generates predictable assembly with no intended abstraction overhead.

Current 2026 releases remain active, and recent work extends compiler-security preservation proofs beyond basic functional correctness.

Sources:

https://jasmin-lang.readthedocs.io/en/stable/

https://formosa-crypto.org/news/2026-07-16/jasmin-2026.03.2

https://arxiv.org/abs/2511.11292

### Architectural implication

The project should keep the native-realization layer replaceable. Some primitive families may be best lowered through LLVM/MLIR; others may justify a higher-assurance backend such as Jasmin or a domain-specific verified code generator.

No one backend should become mathematical identity.

---

## 12. New hypothesis: mathematical primitive admission has two independent proofs

A promoted primitive may eventually require separate evidence for:

### A. Mathematical semantics

```text
This relation/construction is mathematically valid under assumptions A.
```

### B. Executable realization

```text
This CPU/SIMD/GPU/native program implements that certified relation under execution contract E.
```

These are different obligations.

A mathematically valid construction can have a broken compiler implementation.

A perfectly implemented program can faithfully execute a mathematically false construction.

Therefore primitive admission should never collapse these into one status.

Conceptually:

```text
MATHEMATICAL CERTIFICATE
        |
        v
certified semantic object
        |
        v
optimization / compilation
        |
        v
REALIZATION EQUIVALENCE CERTIFICATE
        |
        v
admitted fast primitive
```

This is a major research conclusion.

---

## 13. New research obligations

1. Define the minimum cross-domain fields for a universal certificate envelope without designing a universal proof body.
2. Investigate whether Foundational Proof Certificates, Dedukti, or another logical framework should be an optional final recheck/export tier for selected claims.
3. Map domain-native certificate formats for the initial mathematical capability families.
4. Investigate translation-validation/proof-carrying compilation routes for generated CPU-native primitives.
5. Study CryptOpt/Fiat's optimizer/verifier split in detail as a donor for hardware-specific primitive promotion.
6. Determine how certificate bodies larger than RAM should be streamed, hashed, checkpointed, and independently replayed.
7. Determine whether implementation equivalence should be exact equality, refinement, observational equivalence, or domain-specific relation depending on primitive semantics.

---

## 14. Current synthesis

The strongest trust architecture now looks like:

```text
UNTRUSTED / REPLACEABLE SEARCH
solver / CAS / optimizer / model / GPU campaign / superoptimizer
        |
        v
candidate mathematical artifact
        |
        v
DOMAIN-NATIVE CERTIFICATE
        |
        v
SMALL / INDEPENDENT CHECKER
        |
        v
UNIVERSAL CERTIFICATE ENVELOPE
        |
        v
certified semantic object
        |
        v
aggressive implementation optimization
        |
        v
IMPLEMENTATION EQUIVALENCE / TRANSLATION VALIDATION
        |
        v
fast admitted mathematical primitive
```

The project should try to **own the envelope, semantic identity, trust routing, and primitive admission law**, while reusing strong domain-native solvers and checkers wherever possible.