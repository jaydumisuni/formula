# P11 — Federation Breadth Design

**Date:** 2026-09-06  
**Status:** IMPLEMENTATION DESIGN  
**Predecessor:** P10 is finally frozen; this phase must not rewrite P10 authority.

## 1. Goal

P11 proves that Formula can consume results from two heterogeneous specialist ecosystems while preserving the D1/D2 authority boundary. The first canonical pair is:

1. SAT via DIMACS CNF plus independently checked LRAT proof evidence;
2. exact integer arithmetic via a FLINT/GMP-compatible decimal-result envelope independently recomputed with Formula's exact checker path.

The producer executable, library, brand, or package identity never creates authority. Authority exists only after exact semantic binding, declared translation, declared checker route, independent verification, and bridge-safe Shared Fact use.

## 2. Predecessor and branch law

Implementation branch:

```text
implementation/p11-federation-breadth
```

The branch starts from the P10 recovery head `a18210ae67557f54e07dcab2893000168fbdeba0`. The finally frozen P10 documentation proof boundary remains `3aeb61daf4d575db0f018245ee271597ad475e7b`; later P10 recovery metadata does not move that boundary.

P11 may extend federation/checker/package/test code. It may not weaken P0-P10 architecture firewalls, change P10 proof semantics, or make external producer identity an authority source.

## 3. Reuse existing architecture

P11 extends the already implemented D2 components:

```text
FederationAdapterManifest
FederationMode
FederationRequest
validate_federation_adapter
SharedFact
FactPolarity
fact_satisfies
CompositionClaim
Certificate Router
Evidence/structural digests
```

No second federation framework is introduced.

## 4. SAT/LRAT adapter

### 4.1 Input contract

The SAT semantic input is a deterministic CNF object:

```text
SatCnf {
    variable_count: u32,
    clauses: Vec<Vec<i32>>
}
```

Literals are non-zero signed variable identifiers. Clauses are canonicalized for structural checking where safe, but LRAT clause identifiers remain proof-local and are never semantic identity.

### 4.2 Evidence contract

The first P11 checker implements the sound LRAT **RUP subset**. A proof line is:

```text
id literals... 0 hints... 0
```

Deletion lines and general RAT pivot reasoning are not required by the first gate. Unsupported proof forms fail closed with an explicit error; they cannot be treated as candidate truth.

A RUP addition is accepted only when unit propagation over the referenced clauses plus the negation of the proposed clause reaches contradiction. The empty clause establishes UNSAT.

The checker is independent of the SAT producer. A forged hint, missing clause, invalid literal, duplicate proof id, or proof that never derives the empty clause is rejected.

### 4.3 Shared Fact output

After successful independent checking, the SAT route may emit an `EXACT` Shared Fact stating the certified proposition bound to:

```text
world
subject
CNF digest
LRAT proof digest
adapter/package digest
translation digest
checker-route digest
```

A raw SAT producer result without checked evidence remains `CANDIDATE_ONLY`.

## 5. Exact arithmetic adapter

### 5.1 Input contract

The first exact arithmetic operation set is deliberately bounded and useful:

```text
IntegerOperation::Add(BigInt, BigInt)
IntegerOperation::Sub(BigInt, BigInt)
IntegerOperation::Mul(BigInt, BigInt)
```

External FLINT/GMP-family producers may supply a canonical signed decimal result string. Formula parses that string to `BigInt` and independently recomputes the operation.

### 5.2 Evidence contract

The result is accepted only if:

```text
parsed producer result == independently recomputed exact result
```

Malformed decimal text, operation mismatch, stale semantic input digest, wrong translation/checker route, or numerically incorrect output is rejected.

The canonical test uses values larger than `u128` so the gate proves arbitrary-precision behavior rather than accidentally relying on machine integers.

### 5.3 Shared Fact output

A checked result becomes an `EXACT` Shared Fact bound to the same provenance classes as the SAT fact: world, subject, package/adapter, translation, checker route, input digest, and evidence digest.

## 6. Provenance-bound federation facts

P11 introduces a structural wrapper around `SharedFact` rather than changing its polarity semantics:

```text
CertifiedFederationFact {
    fact
    package
    adapter
    translation
    checker_route
    semantic_input
    evidence
}
```

The wrapper's structural digest binds all fields. Cross-package cooperation uses this wrapper; a bare producer fact is insufficient.

Validation requires:

1. the adapter manifest declares the exact result class;
2. `FederationMode` is `CheckedResult` or `CertifiedTranslation` as appropriate;
3. the exact checker route and translation are declared by the adapter;
4. package identity matches the adapter manifest;
5. semantic input and evidence digests match the independently checked artifact;
6. the Shared Fact polarity satisfies the consumer requirement.

## 7. Bridge contract

Cross-domain propagation is explicit:

```text
BridgeContract {
    source_package
    target_package
    source_subject
    target_subject
    source_polarity
    target_polarity
    translation
    evidence
}
```

A bridge is directional. Reverse use is rejected unless a separate reverse bridge exists. A bridge cannot upgrade information polarity: for example an over-approximation cannot become an exact fact or existence witness.

The bridge itself is structural evidence and must be covered by a `CompositionClaim` whose class is one of:

```text
CERTIFIED_COMBINATION
CONSERVATIVE_EXTENSION
SOUND_COOPERATION
```

`HEURISTIC_ONLY`, `UNSUPPORTED`, and `QUARANTINED` cannot authorize canonical P11 cooperation.

## 8. Canonical cooperation proof

The canonical integration uses one logical/arithmetic target with two independent specialist contributions:

1. SAT CNF encodes a branch-choice relation.
2. LRAT-RUP evidence proves one alternate branch impossible, yielding an exact branch-selection fact.
3. The bridge translates that exact branch-selection fact into an exact arithmetic branch condition.
4. The exact arithmetic adapter independently verifies a large-integer branch computation.
5. The final checker combines the bridge-authorized branch fact and checked arithmetic fact to establish the target result.

Neither package can establish the final target alone. The proof records both adapter/package/checker/translation/evidence identities.

## 9. Negative controls

The canonical P11 proof must execute, not merely list, these failures:

```text
NC11-01 CandidateOnlyAuthorityAttempt
NC11-02 ForgedLratHint
NC11-03 LratMissingEmptyClause
NC11-04 UnsupportedRatProofFailsClosed
NC11-05 WrongSatCheckerRoute
NC11-06 IncorrectExactArithmeticResult
NC11-07 MalformedExactArithmeticDecimal
NC11-08 WrongArithmeticTranslation
NC11-09 StaleSemanticInputDigest
NC11-10 SharedFactPolarityUpgrade
NC11-11 MissingBridgeContract
NC11-12 WrongBridgeDirection
NC11-13 UnsafeCompositionClass
NC11-14 ProducerIdentityCannotAuthorize
```

## 10. P11 proof markers

The independent verifier must emit exactly these ordered markers:

```text
PASS P11_SAT_LRAT_CHECKED
PASS P11_EXACT_ARITHMETIC_CHECKED
PASS P11_FEDERATION_PROVENANCE_BOUND
PASS P11_SHARED_FACT_POLARITY_PRESERVED
PASS P11_BRIDGE_CONTRACT_ENFORCED
PASS P11_HETEROGENEOUS_COOPERATION
PASS P11_PRODUCER_IDENTITY_UNTRUSTED
PASS P11_NEGATIVE_CONTROLS
PASS FEDERATION_BREADTH_PROVED
```

## 11. Canonical workflow

P11 receives its own permanent read-only workflow on `ubuntu-24.04` with Rust `1.98.0`, `contents: read`, locked dependencies, offline execution after `cargo fetch --locked`, workspace tests/build, rustfmt, Clippy with warnings denied, dependency/source firewalls, P10 predecessor proof, and clean-tree enforcement.

The canonical P11 proof itself must not require network access or a live external specialist executable. It consumes standard specialist formats and verifies them independently. This is intentional: external producer availability is operational, while P11 authority is defined by semantic format + certificate + checker route. Live producer invocation can be added later without changing what constitutes authority.

## 12. Gate

P11 passes only if:

1. the SAT/LRAT and exact-arithmetic adapters are both real, separately versioned federation routes;
2. each independently verifies a standard specialist output form;
3. neither producer identity can create authority;
4. both contribute to one final target through Shared Facts and an explicit certified bridge;
5. every NC11-01...NC11-14 path is executed and rejected for the intended reason;
6. the independent verifier reproduces the proof manifest and all ordered markers;
7. P0-P10 tests/firewalls remain green;
8. the exact source SHA is proven under the permanent read-only P11 workflow.

## 13. Explicit exclusions

P11 does not claim:

```text
complete LRAT RAT checking
all SAT/SMT certificate families
all FLINT/GMP operations
floating-point/numerical rigor
proof-assistant export
P12 bootstrap trust reduction
P13 Ptah integration
network-backed authority
producer trust by package name/version
```

Those remain later breadth or roadmap work. The P11 gate is heterogeneous certified cooperation, not exhaustive ecosystem coverage.
