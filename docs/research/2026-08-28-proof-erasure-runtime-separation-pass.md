# Proof Erasure and Runtime Separation Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Question

If mathematical proof/certification is mandatory before a discovered primitive becomes trusted, does every execution of that primitive need to carry or re-run the proof machinery?

The answer is no in important formal systems.

Proof authority and runtime representation can be separated.

## Evidence

### Lean

Lean classifies propositions in `Prop` as runtime-irrelevant. Proofs of propositions are definitionally proof-irrelevant and erased from compiled code. Types and theorem statements are also erased from runtime representations where they do not carry computational content.

Sources:
- https://lean-lang.org/doc/reference/latest/The-Type-System/Propositions/
- https://lean-lang.org/doc/reference/latest/The-Type-System/Inductive-Types/
- https://lean-lang.org/theorem_proving_in_lean4/Axioms-and-Computation/

Example principle:

```
Fin.mk {n} (val) : proof(val < n) -> Fin n
```

The proof argument can establish safety at the type/theorem level yet need not occupy the runtime representation.

### Rocq / MetaCoq

Rocq's extraction mechanism removes computationally irrelevant propositions, proof terms, and type information when extracting programs. MetaCoq provides a formalized erasure pipeline; verified extraction plugins can preserve evaluation semantics for supported axiom-free closed terms while generating executable code.

Sources:
- https://rocq-prover.org/why
- https://rocq-prover.org/p/coq-metacoq-erasure/1.3.4%2B9.0
- https://discourse.rocq-prover.org/t/verified-extraction-for-coq-8-19/2386

### CakeML

CakeML demonstrates another route: implementation generation/translation produces correctness theorems, then a verified compiler carries those properties down to machine code. The proof checker/runtime need not rediscover the source proof on every invocation.

Sources:
- https://cakeml.org/checkers.html
- https://cakeml.org/pldi19.pdf

## Architecture-changing conclusion

The project should distinguish at least three artifact classes:

### Semantic authority

```
Mathematical primitive C
proof/certificate Pm
assumption/foundation identity
```

### Realization authority

```
Implementation R
proof/certificate Pr:
    R realizes C under declared machine/numeric semantics
```

### Runtime artifact

```
stripped executable / native kernel
```

The hot runtime need only execute the computationally relevant realization.

The proof artifacts remain stored, content-addressed, and independently recheckable.

## Promotion pipeline

Conceptually:

```
candidate construction
      ↓
mathematical proof/certificate
      ↓
CERTIFIED SEMANTIC PRIMITIVE
      ↓
compile / specialize / optimize
      ↓
realization validation
      ↓
CERTIFIED REALIZATION
      ↓
proof/type erasure where sound
      ↓
LEAN NATIVE HOT PATH
```

The runtime may therefore approach ordinary optimized-code cost even when the primitive was created through extremely heavy formal reasoning.

## Important restriction

Not all proof-like information is erasable.

If a value affects computation, branching, witness reconstruction, or data representation, it is computational content rather than merely proof evidence.

Examples:

```
existence proof with extracted witness
    -> witness is runtime-relevant

proof that array index is in bounds
    -> proof may be erased once code/typing ensures safe access

certificate required only for admission
    -> certificate can be stored out-of-band after verification

runtime branch on undecidable proposition
    -> cannot be manufactured by erasing the proposition
```

The project must classify proof/evidence relevance before erasure.

## Capability-distribution implication

A client does not need the complete theorem prover to use a promoted primitive.

Potential distribution package:

```
primitive C184
  |- semantic digest
  |- native implementation
  |- realization digest
  |- authority/certificate references
  |- optional local checker
```

A minimal consumer can run the native implementation.

A higher-assurance consumer can independently fetch/recheck the proof chain.

This supports GitHub/local distribution and ordinary-hardware use.

## Cache and freshness implications

Once a certificate has been checked against exact semantic and realization digests, repeated execution should not require repeated proof search.

Instead:

```
certificate check
    ↓
cache authority binding
    ↓
execute realization many times
```

If any of these change:

```
semantic digest
assumptions
foundation/checker requirement
compiler/realization digest
numeric contract
```

the relevant authority binding becomes stale and must be re-established.

This matches the existing proof-freshness research derived from Tenfold.

## Two-level trust deployment

The project can support both:

### Small deployment

```
native primitive + signed/content-addressed authority metadata
```

for speed and low footprint.

### Verification deployment

```
native primitive
+ certificates
+ independent checker(s)
```

for reproducibility/audit.

Neither requires carrying full proof-search engines through every invocation.

## Core law

> **Proof is a condition for trust, not necessarily a tax on every execution.**

This is essential to the project's performance target.

## Open research

1. Which certificate families can be safely checked once and cached versus needing per-input witnesses?
2. How to distinguish proof-irrelevant metadata from computational witnesses automatically.
3. How certificate erasure interacts with untrusted input-dependent execution.
4. Whether local clients should ship compact checkers for realization integrity even when full proofs are remote.
5. How to content-address proof-erased binaries so authority chains remain reproducible across platforms.
6. How to combine proof erasure with runtime specialization/JIT without allowing unverified generated code into the trusted path.
