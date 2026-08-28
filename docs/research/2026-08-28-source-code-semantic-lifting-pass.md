# Research Pass — Source-Code Semantic Lifting, Program Mining, and GitHub as Candidate Mathematics

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how ordinary source-code repositories can contribute algorithms and mathematical structure to the unnamed project without treating code execution, tests, comments, or popularity as mathematical truth.

The central finding is:

> **Source code can be translated into formal semantic representations—transition systems, symbolic formulas, constrained Horn clauses, refinement relations, invariants, and executable reference semantics—then analyzed to recover candidate mathematical structure. Repository code should enter as candidate semantics and earn mathematical authority through independent proof/certification.**

---

## 1. LLVM-level semantics give a common analysis boundary for many source languages

Alive2 translates LLVM IR into its own semantic IR and symbolically checks program refinement, including LLVM undefined behavior, poison, and memory behavior for its supported intraprocedural fragment.

Sources:

https://github.com/AliveToolkit/alive2

https://users.cs.utah.edu/~regehr/alive2-pldi21.pdf

### Architectural implication

For compatible source languages, one ingestion route can be:

```text
source repository
    -> compiler front-end
    -> well-specified intermediate representation
    -> semantic lifting/analyzer
```

rather than implementing a custom semantics for every programming language immediately.

The compiler/frontend version becomes part of provenance.

---

## 2. Undefined behavior means code semantics cannot be inferred from happy-path execution

Alive2's work emphasizes that LLVM's undefined/poison semantics materially affect whether two pieces of code are equivalent or one refines another.

Source:

https://users.cs.utah.edu/~regehr/alive2-pldi21.pdf

### Architectural implication

The project must never infer a mathematical function merely from observed input/output behavior of low-level code.

Semantic lifting must model or explicitly exclude:

```text
undefined behavior
partiality
memory effects
integer overflow semantics
floating-point semantics
nondeterminism
external calls
```

before the code can be interpreted as a mathematical construction.

---

## 3. SeaHorn lowers ordinary programs to logical verification conditions

SeaHorn compiles C through LLVM and generates constrained Horn clauses (CHCs)/SMT verification conditions. It combines abstract interpretation, symbolic execution/bounded model checking, CHC solving, invariant inference, and executable counterexamples.

Source:

https://github.com/seahorn/seahorn

### Architectural implication

A code-ingestion Work Cell can transform an imperative program into a relational/logical object:

```text
program control/data flow
    -> transition relations / CHCs
    -> inferred inductive predicates/invariants
```

This fits the project's relational semantic substrate far better than treating source syntax as the canonical mathematical object.

---

## 4. Program verification via CHCs explicitly recovers invariants

CHC-based program verification encodes safety as the existence of inductive predicates satisfying logical constraints.

Source:

https://link.springer.com/chapter/10.1007/978-3-031-13185-1_2

### Architectural implication

Source-code mining can feed the same invariant-discovery machinery already researched:

```text
code
    -> CHC semantics
    -> candidate invariant relation
    -> independent proof/check
    -> promoted mathematical fact
```

The algorithm and its discovered invariants become separate artifacts.

---

## 5. Translation validation can establish equivalence between mined and simplified code

Alive2 checks whether a target program refines a source program without trusting the optimization that produced the target.

Source:

https://github.com/AliveToolkit/alive2

### Architectural implication

After lifting a complicated repository algorithm, the project can attempt:

```text
original reference code
    -> simplification / supercompilation / decompilation
    -> candidate compact executable mathematics
    -> refinement/equivalence validation
```

This gives a route from messy implementation code to a smaller semantic realization without trusting the simplifier.

---

## 6. Candidate semantic extraction should be staged by trust level

A repository may contain:

```text
formal proofs
verified implementation
specifications/contracts
assertions
property tests
unit tests
comments/papers
plain code only
```

These are not equivalent evidence.

### Architectural implication

Code ingestion should produce an evidence ladder:

```text
FORMALLY CERTIFIED
    imported proof/checkable certificate

SEMANTICALLY VERIFIED
    independently established program property/equivalence

EXACTLY RECOMPUTED / EXHAUSTIVE FOR SCOPE

EMPIRICALLY TESTED

UNVERIFIED CANDIDATE
```

Only the first appropriate authority levels can feed permanent mathematical truth.

---

## 7. Tests are valuable falsifiers, not proof authority

Repository test suites and property tests provide examples, edge cases, and likely domain assumptions.

### Architectural implication

Tests should be mined into:

```text
falsification fixtures
candidate preconditions
example/counterexample corpus
behavioral probes
```

not converted into `PROVEN` claims.

This aligns with Sergeant/Tenfold adversarial evidence discipline.

---

## 8. Comments/docs can propose mathematical structure but remain untrusted

A README may state “this algorithm is O(n log n)” or “works for prime moduli,” but those statements may be wrong, stale, or incomplete.

### Architectural implication

Natural-language documentation should enter as **candidate metadata/claims**:

```text
claim: complexity O(n log n)
source: README line ...
status: UNVERIFIED
```

Then resource analysis/theorem proving/falsification can establish or reject it independently.

Models may help parse such claims, but parsing does not establish truth.

---

## 9. Program synthesis can reconstruct implementations from formalized semantics

Synquid synthesizes recursive functional programs from refinement-type specifications; proof-directed synthesis research similarly constructs programs while satisfying logical specifications.

Sources:

https://github.com/nadia-polikarpova/synquid

https://www.microsoft.com/en-us/research/video/program-synthesis-from-refinement-types/

### Architectural implication

Once repository code has yielded a stable semantic specification, the project may no longer need to retain the original implementation on the hot path:

```text
mined/verified semantic specification
    -> synthesize clean reference implementation
    -> prove against specification
    -> optimize/validate native realization
```

This is mathematical distillation from code rather than code reuse by copy/paste.

---

## 10. Code can reveal new metaprimitives, not only value-level algorithms

Repeated code patterns may encode:

```text
rewrite strategies
search policies
decomposition methods
representation transforms
propagators
reconstruction algorithms
```

### Architectural implication

The mining pipeline should classify candidate semantics at multiple levels:

```text
mathematical primitive
rewrite rule
reduction
representation transform
propagator
search strategy
certificate checker
metaprimitive
```

Each category follows its own promotion contract.

---

## 11. Repository provenance must remain exact

A mined algorithm can change as upstream code changes.

### Architectural implication

Every semantic-lifting artifact should bind to:

```text
repository
commit/tree/blob digests
build/compiler/frontend version
compile options
language semantics version
exact analyzed entrypoints
external assumptions/dependencies
```

Later repository updates create new candidate artifacts rather than silently changing accepted mathematics.

This directly reuses Tenfold-style content identity/proof freshness.

---

## 12. GitHub becomes a massive candidate-capability supply, not a trust root

The practical consequence is important:

```text
millions of open-source algorithms
    -> searchable candidate mathematical implementations
```

but admission remains:

```text
recover semantics
    -> infer structure/claims
    -> falsify
    -> prove/certify
    -> distill
    -> promote
```

### Architectural implication

The system can borrow from the enormous existing software world while remaining mathematically self-contained and deterministic after promotion.

---

## 13. Current source-code-lifting hypothesis

```text
SOURCE REPOSITORY @ EXACT COMMIT
    -> classify language/build/entrypoints
    -> compile/lift to supported formal IR where possible
    -> derive transition/relational semantics
    -> mine invariants/preconditions/postconditions/cost candidates
    -> compare against docs/tests as candidate evidence
    -> simplify/decompile semantic construction
    -> adversarial validation
    -> formal/certificate proof of useful properties
    -> promote clean semantic primitive / transform / strategy
    -> retain exact source provenance
```

This is how “a useful GitHub project” can become mathematical capability without making GitHub code itself an axiom.

---

## 14. New research obligations

1. Study formal semantics/lifting boundaries for Rust MIR, LLVM IR, WebAssembly, JVM bytecode, Python subsets, and other likely source ecosystems.
2. Investigate automatic loop/recurrence summarization and closed-form extraction from imperative programs.
3. Study interprocedural semantic lifting beyond Alive2's current intraprocedural boundary.
4. Investigate aliasing/heap/data-structure abstractions suitable for mathematical mining.
5. Study code-to-CHC/e-graph/hypergraph normalization for cross-language algorithm comparison.
6. Investigate mining exact preconditions/domains automatically from source plus counterexamples.
7. Study semantic deduplication: detect two repositories implementing the same mathematical construction.
8. Investigate extraction of complexity/resource proofs from source implementations.
9. Define a repository-ingestion safety/trust policy for generated code, unsafe operations, external dependencies, and licenses.
10. Build benchmark tasks where a known algorithm exists only as ordinary code and the project must recover, certify, and distill its mathematical semantics without using its documentation as truth.
