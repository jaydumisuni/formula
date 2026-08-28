# Research Pass — Mathematical Knowledge Ingestion, Formal Libraries, Databases, and Autoformalization Authority

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how the unnamed project can consume enormous existing mathematical knowledge—formal repositories, theorem libraries, databases, papers, sequence catalogs, and model-assisted formalizations—without allowing ingestion convenience to contaminate mathematical truth authority.

The central finding is:

> **The project needs an evidence-tiered ingestion pipeline. Kernel-checked formal repositories can contribute certified theorem/proof structure directly under exact version provenance; curated mathematical databases contribute typed data with their declared reliability/completeness; ordinary prose and model-generated formalizations contribute candidates only, because current autoformalization remains far from reliable enough to become an authority path.**

---

## 1. Formal repositories can be mined at semantic/dependency level

LeanDojo-v2 traces Lean repositories and extracts ASTs, proof states, tactics, premises, file dependencies, and theorem information rather than treating source files as opaque text.

Source:

https://leandojo.org/leandojo

The published LeanDojo Benchmark 4 contains over 122,000 theorem/proof entries and over 167,000 premises extracted from mathlib4.

### Architectural implication

Formal-library ingestion can operate on:

```text
exact repository/commit/toolchain
    -> build/kernel check
    -> semantic theorem extraction
    -> premise/dependency graph
    -> canonical project representation
```

The theorem's authority comes from the formal system/checker, not from the ingestion parser.

---

## 2. Formal repository provenance must include exact dependency closure

LeanDojo documentation records exact Lean/mathlib revisions and traces imported dependency DAGs.

Source:

https://leandojo.readthedocs.io/en/latest/user-guide.html

### Architectural implication

An imported formal theorem should bind to:

```text
source repository + commit
proof assistant/kernel version
dependency commits
source theorem identity
checked statement
proof/check result
```

Updating mathlib/Lean does not silently mutate the accepted theorem; it creates a new imported generation requiring compatibility/recheck as appropriate.

---

## 3. Dependency graphs are first-class imported mathematical structure

A 2026 Mathlib4 theorem-dependency graph derived from LeanDojo contains over 137,000 nodes and 304,000 edges.

Source:

https://zenodo.org/records/19837332

### Architectural implication

The project should ingest not only theorem statements but:

```text
proof dependency edges
structure/type dependencies
namespace/theory membership
proof method/certificate metadata where recoverable
```

This feeds retrieval, stale-proof invalidation, theorem-transfer analysis, and primitive-distillation mining immediately.

---

## 4. Curated mathematical databases already distinguish source, reliability, and completeness

LMFDB explicitly documents for each section/subsection:

- source and computational method;
- whether heuristics or unproved conjectures were used;
- consistency checks;
- which finite subset of the usually infinite mathematical universe is represented.

Sources:

https://www.lmfdb.org/rcs

https://www.lmfdb.org/NumberField/Reliability

### Architectural implication

Imported data should have an evidence envelope such as:

```text
source
computation_method
assumptions/conjectures used
reliability status
consistency checks
coverage/completeness scope
version/release
```

The system must never convert “present in respected database” into `PROVEN` without respecting that database's own reliability statement.

---

## 5. OEIS demonstrates versioned machine-readable mathematical corpora

OEIS publishes a Git-formatted export containing sequence entries, formulas/programs/references/supporting files under a timestamped export and explicit license.

Source:

https://github.com/oeis/oeisdata

### Architectural implication

A corpus like OEIS can feed multiple candidate channels:

```text
sequence terms -> exact data
formulas -> candidate/declared formulas
programs -> source-code semantic lifting
references -> literature provenance
cross-references -> relation graph
```

Each field retains its own authority status rather than inheriting one blanket trust level from the entry.

---

## 6. Autoformalization is still a candidate generator, not a truth bridge

A 2025 EMNLP study created improved autoformalization benchmarks and found current methods reaching up to 45.1% accuracy on undergraduate mathematics while struggling substantially on research-level mathematics without appropriate context.

Source:

https://aclanthology.org/2025.emnlp-main.907/

The study also introduced audited/corrected benchmarks because evaluating whether an informal and formal statement mean the same thing is itself difficult.

### Architectural implication

The project must enforce:

```text
natural-language theorem/paper
    -> model/parser-generated formal candidate
    -> CANDIDATE ONLY
```

Compilation/type checking is not sufficient: a perfectly valid Lean statement may formalize the wrong theorem.

---

## 7. Autoformalization quality needs semantic equivalence checking, not compile success

ProofNet# corrected substantial errors in earlier Lean4 versions of ProofNet; current benchmark work emphasizes semantic evaluation rather than only syntactic validity.

Sources:

https://github.com/marcusm117/ProofNet-Verified

https://aclanthology.org/2025.emnlp-main.907/

### Architectural implication

Informal ingestion requires at least two different obligations:

```text
FORMAL STATEMENT WELL-FORMED/CHECKED

SEMANTIC FAITHFULNESS TO SOURCE
```

The second may require independent translations, adversarial examples, formal consequences, domain checks, or human review until stronger automated methods exist.

No language model vote can substitute for that boundary.

---

## 8. Informal proofs can still be useful without being authority

Natural-language proof text can suggest:

```text
lemmas
representations
proof strategies
known reductions
candidate assumptions
references
```

### Architectural implication

Prose enters the Search Economy as proposal material:

```text
paper proof idea
    -> candidate decomposition/lemma/transform
    -> formal reconstruction
    -> proof/check
```

The original prose remains provenance and explanatory context, not the mathematical certificate.

---

## 9. Models can massively accelerate ingestion while remaining optional

LeanDojo and modern autoformalization research are heavily model-oriented, but the useful architectural donor is the interface:

```text
formal environment state
candidate theorem/tactic/formalization
checker response
```

### Architectural implication

Models can be attached as high-throughput candidate producers for:

```text
statement formalization
proof sketches
symbol/definition alignment
paper claim extraction
cross-library correspondence
```

while deterministic/kernel/certificate machinery supplies authority.

A model-free ingestion path remains available for already formal/machine-readable sources.

---

## 10. The same mathematical fact may enter from multiple sources with different assurance

Example:

```text
identity found in paper prose
same identity in OEIS entry
formal version in Lean
independent exact derivation by project
```

### Architectural implication

The ledger should merge semantic identity while retaining **multiple evidence/provenance paths**.

Authority can strengthen without duplicating the mathematical fact:

```text
CLAIM C
    provenance source 1: informal paper
    provenance source 2: curated database
    certificate source 3: Lean proof
    certificate source 4: project independent checker
```

Invalidating one source does not invalidate C if another independent valid path still establishes it.

---

## 11. Imported knowledge should immediately compile into retrieval indexes, not hot-load everything

Large formal corpora already exceed 100k theorems, and the project intends to ingest many corpora.

Earlier premise-selection research established that irrelevant mathematics actively hurts search.

### Architectural implication

Ingestion completion should trigger:

```text
semantic normalization
structure inference
proof/dependency graph update
term/subsumption/morphism indexes
source/reliability indexes
```

but normal problem-solving retrieves a bounded relevant context rather than loading the corpus wholesale.

---

## 12. Cross-formal-system ingestion should preserve native proof roots

Lean, Isabelle, Rocq, Metamath, Mizar and other libraries do not share one foundational logic or proof-object format.

Earlier Hets/Dedukti/FPC research established that interoperability and native certification are separate concerns.

### Architectural implication

The project should import a theorem through a universal **claim/certificate envelope** while preserving:

```text
native formal system
native statement identity
native checker/kernel
translation theorem if normalized into another system
```

No silent translation should make a theorem appear stronger or foundationally different than its source proof.

---

## 13. Ingestion can become a self-expansion campaign

After ingestion, the project can mine cross-source structure:

```text
duplicate/equivalent theorems
missing assumptions
stronger/weaker theorem variants
unformalized but repeatedly corroborated claims
common proof patterns
new reductions/theory morphisms
reusable constructions
```

### Architectural implication

Knowledge ingestion is not merely “download database.”

It feeds the same discovery/generalization pipeline and may create genuinely new certified mathematics through independent reconciliation and theory intersection.

---

## 14. Current ingestion-authority hypothesis

```text
SOURCE
    FORMAL REPOSITORY
    CURATED DATABASE
    SOURCE CODE
    STRUCTURED DATA
    PAPER/PROSE

    -> exact source/version provenance
    -> source-specific parser/tracer
    -> candidate claim/object/proof/data artifacts
    -> assign evidence tier
    -> semantic normalization/alignment

IF FORMAL/CERTIFIED
    -> native checker replay

IF INFORMAL/EMPIRICAL
    -> candidate only
    -> formalization/reconstruction/falsification campaign

    -> accepted claim only after valid authority path
    -> build dependency/retrieval indexes
```

This lets the mathematical universe consume far more knowledge than any model can memorize while keeping truth independent of ingestion technology.

---

## 15. New research obligations

1. Build a formal-source matrix for Lean/mathlib, Isabelle/AFP, Rocq/MathComp, Metamath, Mizar, TPTP, Fungrim, OEIS, LMFDB, DLMF, and other major sources.
2. Study canonical symbol/definition alignment across formal libraries without conflating similarly named concepts.
3. Investigate independent semantic-equivalence checking for autoformalized statements.
4. Define evidence-tier transitions from informal claim -> candidate -> empirically supported -> independently certified/formal.
5. Study paper claim extraction and assumption recovery as candidate-generation tasks.
6. Investigate legal/license constraints on retaining/reusing corpora, code, and generated derivatives.
7. Develop source-version/freshness rules for curated databases whose entries can be corrected or recomputed.
8. Study cross-source contradiction detection and automated conflict campaigns.
9. Investigate theorem/library mining for reusable proof patterns, reductions, and primitive candidates.
10. Define blind ingestion benchmarks where informal sources contain deliberately misleading statements and only independent mathematical checking can promote the correct subset.
