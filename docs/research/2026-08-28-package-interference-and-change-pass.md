# Research Pass — Package Interference, Assume/Guarantee Contracts, and Semantic Change Management

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass follows the heterogeneous-theory-composition research and asks a more specific question:

> If two mathematical packages are each safe/certified relative to the same base, when can they be combined without re-proving everything, and how should changes to a package propagate through the accepted mathematical universe?

The strongest conclusion is:

> **Safe extension is contextual. Composition requires an explicit interference boundary, and package versioning must be semantic rather than merely textual.**

---

## 1. Conservative extensions compose under important separation conditions

For ordinary first-order theories, if `T1` and `T2` are conservative extensions of base theory `T`, and their extended signatures intersect only in the base signature, Robinson-style joint consistency results imply that `T1 ∪ T2` remains conservative over `T`.

Representative discussion/reference:

https://mathoverflow.net/questions/444781/is-the-union-of-two-conservative-extensions-of-a-theory-conservative

Related pushout/modularity result:

https://www.sciencedirect.com/science/article/pii/S0020019096001469

### Architectural implication

A package can sometimes receive a strong low-cost composition rule:

```text
P extends Base
Q extends Base

new_symbols(P) intersect new_symbols(Q) = empty
P conservative over Base
Q conservative over Base

=> use certified modular-composition theorem
```

This is valuable because it allows the universe to scale modularly without globally re-proving every old theorem after every independent addition.

---

## 2. Shared new vocabulary creates an interference surface

The separation condition is essential. If two individually safe extensions constrain the **same new symbol** incompatibly, their union can fail dramatically.

A simple pattern is:

```text
Base T

P introduces new R with property A(R)
Q introduces the same new R with incompatible property not-A(R)
```

Each package may be separately satisfiable/conservative relative to `T`, while their union is inconsistent.

Representative discussion:

https://mathoverflow.net/questions/444781/is-the-union-of-two-conservative-extensions-of-a-theory-conservative

Related counterexample discussion:

https://math.stackexchange.com/questions/2128593/consistent-and-conservative-extensions

### Architectural implication

The project needs an explicit **interference surface** for packages:

```text
owned symbols
shared symbols
shared relations
shared assumptions
shared rewrite heads
shared state/resources
shared model sorts
```

Composition analysis should focus first on this surface rather than reanalyzing every private/internal declaration.

---

## 3. Assume/guarantee contracts provide a mature compositional pattern

Formal contract theories separate:

- **assumptions** a component makes about its environment;
- **guarantees** it provides when those assumptions hold.

Composition is permitted only when the components' guarantees satisfy each other's relevant assumptions and the composite contract can be derived.

Current/recent sources:

https://link.springer.com/article/10.1007/s10703-025-00473-6

https://link.springer.com/article/10.1007/s10703-024-00447-0

https://link.springer.com/article/10.1007/s10703-017-0294-7

### Architectural implication

A mathematical package may need a composition-facing contract separate from its internal proof objects.

Research-level sketch:

```text
Package P

requires:
    theory properties
    available symbols/relations
    representation invariants
    assumptions about other packages

guarantees:
    semantic claims
    preservation properties
    effects on shared theory
    resources/transformations exported

interference:
    symbols/relations/rules it may change or constrain
```

This can make compatibility checking local and compositional.

---

## 4. Rely/guarantee shows how interference can itself be summarized mathematically

Rely-guarantee reasoning for concurrent systems summarizes:

- environment behavior that a component relies on;
- effects the component guarantees to its environment.

Compositional rules require the guarantees of cooperating components to fit within the relies of their peers.

Sources:

https://link.springer.com/article/10.1007/s10703-021-00370-8

https://link.springer.com/chapter/10.1007/978-3-032-22720-1_9

### Architectural implication

For mathematical packages with active transformations rather than only static axioms, interference contracts may need to cover **state/theory effects** such as:

```text
may_add_equalities
may_add_rewrite_rules
may_restrict_domain
may_strengthen_assumptions
may_create_new_symbols
may_change_canonical_form
```

A pure import-only theorem library has a very different interference class from a package that changes active rewrite/propagation semantics.

---

## 5. Definitional extensions are a particularly strong safe-change class

A definitional extension introduces new constants/types/symbols by definitions satisfying suitable non-circularity/existence conditions. Such extensions are expected to be conservative over the original theory.

Formal results for HOL/Isabelle-style definitional theories show model-theoretic conservativity under appropriate conditions.

Sources:

https://www.sciencedirect.com/science/article/pii/S1571066118300756

https://link.springer.com/article/10.1007/s10817-016-9366-4

### Architectural implication

The project should distinguish a new primitive that is **merely a new name/abbreviation/derived construction** from one that genuinely adds mathematical assumptions.

A definitional promotion may be much cheaper to admit and much safer to transport than an axiomatic extension.

Possible semantic change class:

```text
DEFINITIONAL_EXTENSION
    -> no new old-language consequences
    -> unique or controlled model expansion
    -> aggressive proof reuse allowed
```

---

## 6. Conservativity has multiple strengths

Research distinguishes several extension notions, with different strength:

```text
definitional extension
    -> model-unique extension
    -> model extension
    -> conservative extension
```

Source:

https://academic.oup.com/jigpal/article/30/1/101/5954216

Model-theoretic and proof-theoretic conservativity can also differ when the underlying logic is not complete for the intended semantics.

Source:

https://link.springer.com/article/10.1007/s10817-016-9366-4

### Architectural implication

One generic label `conservative=true` would lose important information.

The package graph should preserve **which conservativity notion was established, in which logic/semantics, under which assumptions**.

---

## 7. Semantic versioning for mathematics needs more than major/minor/patch

The research supports a semantic change taxonomy rather than conventional software version labels alone.

Possible research-level classes:

```text
REALIZATION_ONLY
    same semantic primitive; new CPU/GPU/native implementation

PROOF_ONLY
    same claim; new certificate/checker route

DEFINITIONAL_EXTENSION
    adds eliminable/defined symbols without new old-language consequences

CONSERVATIVE_EXTENSION
    adds genuine expressive material but no new consequences in old language

STRENGTHENING
    adds assumptions/axioms and may invalidate old models

WEAKENING
    removes assumptions and may invalidate previously proven conclusions

SIGNATURE_CHANGE
    changes symbol/domain interfaces or mappings

MORPHISM_CHANGE
    changes interpretation/translation to another theory

RULE_SEMANTICS_CHANGE
    changes active rewrite/propagation behavior
```

These names are not frozen.

### Architectural implication

Downstream invalidation should depend on the semantic change class, not simply `version changed`.

---

## 8. Change impact can be formally verified and localized

Change-impact analysis over dependency graphs can itself be formally proved correct. Existing work has machine-checked a general dependency-graph impact analysis in Coq and extracted it into executable code.

Source:

https://pmc.ncbi.nlm.nih.gov/articles/PMC7480691/

Heterogeneous development-graph work likewise tracks proof dependencies and preserves as much prior verification as possible after local changes.

Source:

https://www.researchgate.net/publication/221350774_Change_Management_for_Heterogeneous_Development_Graphs

### Architectural implication

The project's invalidation mechanism does not have to be trusted heuristically.

Possible path:

```text
semantic diff
    -> classify changed nodes/links
    -> verified dependency impact analysis
    -> mark exact affected claims/realizations/morphisms stale
    -> preserve unaffected certified regions
    -> generate open re-certification obligations
```

This is stronger than invalidating every descendant mechanically when a more precise proof-support path remains valid.

---

## 9. Alternative proof paths should prevent unnecessary invalidation

Earlier provenance research already suggested a result may have multiple sufficient derivations.

The composition/change layer strengthens that requirement.

If theorem `R` is established through either:

```text
A + B
```

or independently through:

```text
C + D
```

and package `A` changes, `R` should not become globally stale if the `C + D` route remains certified.

### Architectural implication

The durable theory graph should retain **alternative proof/support paths**, not only one parent list.

Change impact becomes:

```text
invalidate support edge/path
    -> recompute whether any certified support path remains
    -> stale claim only if required assurance can no longer be established
```

This connects development graphs with the earlier provenance-semiring/proof-graph research.

---

## 10. Package compatibility may itself be a solved mathematical problem

Contract/interface theory provides algebraic operations such as:

- composition;
- refinement;
- quotient;
- merge;
- compatibility.

Recent hypercontract work develops algebraic manipulation of structured assume/guarantee specifications.

Source:

https://link.springer.com/article/10.1007/s10703-025-00473-6

### Architectural implication

Rather than hard-coding package compatibility as procedural checks, the project may eventually represent:

```text
compatible(P, Q)
refines(P, Contract)
quotient(SystemContract, P)
```

as mathematical relations with their own decision/certificate procedures where available.

The composition layer can therefore itself become part of the mathematical substrate.

---

## 11. Pushout/amalgamation results suggest safe structured merging under explicit conditions

Formal-specification research generalizes consistency/modularity/interpolation from ordinary union to pushout constructions and studies preservation of properties under amalgamation.

Sources:

https://www.sciencedirect.com/science/article/pii/S0020019096001469

https://www.sciencedirect.com/science/article/pii/S0304397520305235

The 2020 work gives conditions under which termination and sufficient-completeness are stable under specific pushout constructions in constructor-based order-sorted algebra.

### Architectural implication

A theory merger should search for a **certified structured merge theorem** appropriate to the package class rather than defaulting to naive union.

```text
A <- Interface -> B
      |
      v
candidate amalgam / pushout C
      |
      +-> property preservation obligations
      +-> conflict/interference obligations
      +-> conservativity obligations
```

---

## 12. Current package-interference hypothesis

A future package generation may therefore have three layers:

```text
PRIVATE MATHEMATICS
    internal symbols/rules/proofs

EXPORTED SEMANTIC INTERFACE
    public relations/theorems/primitives/morphisms

COMPOSITION CONTRACT
    assumptions
    guarantees
    interference surface
    preserved-property conditions
    known incompatible combinations
    composition certificate routes
```

The package-admission layer verifies the package itself.

The **composition layer separately verifies each actual attachment** to the current universe.

This avoids both extremes:

- re-prove the entire mathematical universe after every extension;
- assume local certification implies global compatibility.

---

## 13. Current semantic-change hypothesis

A mathematical universe update should likely produce an explicit **semantic diff artifact**:

```text
old generation G
new candidate generation G'

added semantic nodes
removed semantic nodes
changed assumptions
changed signatures
changed morphisms
changed rewrite/propagation rules
realization-only changes
proof/certificate-only changes
conservativity results
composition-contract changes
exact affected support paths
```

This diff can drive verified impact analysis and precise proof reuse.

---

## 14. New research obligations

1. Formalize the exact conditions under which independently conservative packages can be composed without new global proof obligations.
2. Study Robinson joint consistency, amalgamation, and interpolation as donors for automated package-interference checking.
3. Investigate contract/interface theories for mathematical rather than software components.
4. Define a package interference surface that works across theorem, rewrite, constraint, and numerical packages.
5. Study automatically synthesizing assume/guarantee contracts from certified package behavior.
6. Investigate proof-producing compatibility/refinement/quotient operations for package contracts.
7. Define semantic change classes and prove their invalidation rules.
8. Investigate verified semantic-diff and dependency-impact analysis over heterogeneous theory graphs.
9. Study how alternative proof paths should be represented compactly so one package change does not over-invalidate the universe.
10. Investigate conflicts between two individually conservative extensions sharing new symbols or shared active rules.
11. Study whether definitional/model-unique extensions should receive a faster primitive-promotion path.
12. Determine how package contracts interact with transported mathematics through theory morphisms.
13. Investigate automatic extraction of the minimal shared interface required for two packages to interact.
14. Study quotient-style reasoning: given a target combined mathematical capability and an existing package P, synthesize the weakest package/contract Q needed to complete it.
15. Determine how package compatibility evidence fits inside the universal certificate envelope.
