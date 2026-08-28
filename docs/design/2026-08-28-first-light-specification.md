# First Light — Canonical End-to-End Proof Specification

**Date:** 2026-08-28  
**Status:** FROZEN FIRST-LIGHT SPECIFICATION  
**Repository name:** temporary only; not product identity  
**Depends on:** D1, D1A, D2, D3, D4, D5.

This document supersedes the milestone role of `2026-08-28-d3-first-light-build-architecture.md` while preserving that file as a precursor. The concrete target ideas from that precursor are retained where they provide the strongest bounded proof.

First Light is not a product demo. It is the smallest local experiment that proves the architecture's defining claim:

> **The machine can blindly discover useful executable mathematics, reject convincing false near-misses, independently certify the surviving mathematics, promote it into a new immutable Universe generation, compile and independently validate a native realization, then solve a second related problem by reusing the promoted capability without rediscovering it.**

---

## 1. Canonical environment

First Light is deliberately constrained:

```text
local machine
ordinary CPU
no GPU required
no Ptah
no model/LLM in canonical run
no network during canonical execution
exact/replay-bound inputs
independent checker process/library boundary
```

A conventional pinned compiler/toolchain may bootstrap the implementation. Toolchain trust is recorded but is not allowed to substitute for mathematical/realization certification.

---

## 2. Required implementation boundary

Canonical First Light may use a compact stable-Rust workspace because the prior precursor already selected Rust for the bounded implementation proof. This choice proves the architecture; it does not constitutionally bind D4 to Rust long-term.

Suggested workspace roles:

```text
formula-core
    D1 artifact schemas, identity, generation manifests

formula-store
    local content-addressed object store + indexes

formula-check
    independent semantic/certificate/realization checkers

formula-engine
    D2/D3 Universe, capability, compiler, Campaign IR

formula-packages
    minimal exact mathematical packages for target suite

formula-realize
    bounded D4 realization generation/compilation

formula-first-light
    sealed targets, blindness harness, proof manifest

formula-cli
    local orchestration/inspection only
```

The search/discovery implementation must not import sealed target definitions or independent-checker implementation logic.

---

## 3. Authority storage boundary

Canonical local layout:

```text
.formula/
  authority.sqlite            # optional local index/transaction layer
  objects/sha256/...          # immutable blobs
  generations/<digest>.json
  campaigns/<digest>/...
  evidence/<digest>/...
  realizations/<digest>/...
  tmp/                        # never authority
```

The exact physical database is First-Light implementation detail. All authority artifacts are content-addressed and canonical-encoded under a versioned schema.

---

## 4. Blindness contract

A target is blind when the active Discovery Fabric cannot recover the answer from target fixtures, source literals, hidden expected expressions, or imported solved artifacts.

Canonical blindness rules:

```text
1. sealed target/spec implementation is physically/module separated
2. search packages receive only declared oracle/query interfaces
3. expected closed forms/target programs are not linked into search binary
4. campaign manifest binds hashes of sealed fixture and active grammar/package set
5. independent checker opens sealed target only after candidate freeze
6. target answer is not present in searchable Universe generation U_0
7. CI/tests verify no direct dependency from discovery crates/modules to sealed fixtures
```

A discovered candidate is structurally frozen before the checker is allowed to compare it with the hidden specification.

---

## 5. Suite overview

First Light contains three complementary targets.

```text
FL-A  blind exact relation discovery
FL-B  representation/reduction selection
FL-C  full self-expansion + native reuse
```

FL-C is the canonical constitutional PASS target. FL-A and FL-B prove important D3 behaviors so a lucky FL-C implementation cannot bypass representation/candidate-space contracts.

---

# FL-A — Blind exact identity discovery

## 6. Hidden target

For integer `n`, sealed target semantics are:

```text
F(n) = (n + 1)^7 - n^7
```

The expanded polynomial is unavailable to the active Discovery Fabric.

The search side may request exact integer samples through a narrow oracle relation.

---

## 7. CandidateSpace

Use an exact polynomial coefficient space:

```text
P(n) = c0 + c1*n + ... + cd*n^d
```

The CandidateSpace stores linear constraints over exact rational/integer coefficients rather than enumerating coefficient tuples.

Required operations:

```text
add_exact_sample
restrict_degree
solve_affine_space
extract_min_degree
serialize/freeze candidate
```

---

## 8. Required result

The blind Discovery Fabric must recover the exact minimal-degree polynomial:

```text
7*n^6 + 21*n^5 + 35*n^4 + 35*n^3 + 21*n^2 + 7*n + 1
```

The engine does not receive this expression as search data.

---

## 9. FL-A false near-miss

A deliberate family must remain possible after an insufficient early sample set:

```text
P(n) + k * product(n - i, i=0..6)
```

For selected initial sample points this can agree exactly while being globally false for `k != 0`.

A discriminating unseen sample/certification step must eliminate the near-miss.

The proof must demonstrate that numerical/sample coincidence is not promoted as universal identity.

---

## 10. FL-A independent certification

After candidate freeze, `formula-check` independently expands/canonicalizes both the sealed semantic definition and candidate over exact integer polynomial arithmetic.

Certification establishes exact coefficient equality and therefore universal equality over integers.

Required proof output:

```text
candidate digest
sealed target digest
checker digest/version
normalized polynomial digests
exact equivalence verdict
Evidence envelope
```

---

# FL-B — Representation/reduction discovery

## 11. Public target

A Boolean XOR constraint system over 24 Boolean variables requests one exact satisfying witness.

The active engine receives the semantic XOR system but not a preselected algorithm.

---

## 12. Candidate routes

At minimum:

```text
Route A — direct Boolean assignment search
Route B — certified XOR -> GF(2) affine system -> exact Gaussian elimination
```

The reduction edge declares:

```text
Boolean <-> GF(2) value map
XOR preservation by addition mod 2
witness reconstruction
exactness
```

---

## 13. Required compiler behavior

D3 Theory Profile + representation/reduction frontier must discover/select Route B under the canonical local budget.

The fixture is sized so direct enumeration is operationally dominated while exact GF(2) elimination is trivial on ordinary CPU hardware.

The proof must show that the selected route arose from admitted representation/reduction contracts rather than a hard-coded `if XOR then Gaussian elimination` bypass.

---

## 14. FL-B certification

The independent checker verifies:

```text
every translated row corresponds exactly to original XOR semantics
returned values are Boolean/GF(2)-valid
returned witness satisfies every original constraint
reconstruction mapping is exact
```

A deliberately corrupted translation fixture must be rejected by the checker.

---

# FL-C — Full self-expansion proof

## 15. Hidden semantic target

For `x : U8`, synthesize a Boolean construction implementing:

```text
IsPowerOfTwoU8(x)
```

with semantic specification:

```text
true exactly for x in {1,2,4,8,16,32,64,128}
```

The classic compact formula is not part of `U_0` and is unavailable to the active Discovery Fabric.

---

## 16. Active synthesis grammar

The exact grammar is canonical/content-addressed. Minimal sufficient family:

```text
ByteExpr:
    x
    0
    1
    sub_wrap(ByteExpr, ByteExpr)
    bit_and(ByteExpr, ByteExpr)

BoolExpr:
    eq_zero(ByteExpr)
    neq_zero(ByteExpr)
    and(BoolExpr, BoolExpr)
```

Additional operators may not be added after target inspection without changing the campaign/grammar digest and invalidating the canonical proof run.

---

## 17. Discovery strategy

Use bounded observational/CEGIS-style synthesis over behavior-equivalence classes:

```text
initial discriminating sample set
    -> build observational CandidateSpace
    -> extract lowest-cost candidate
    -> query independent specification oracle for counterexample
    -> refine entire CandidateSpace
    -> repeat
```

The search should store one low-cost representative per current behavior class rather than every syntactic expression.

This is a bounded First-Light backend, not the final e-graph/hypergraph architecture.

---

## 18. Mandatory false near-miss

The campaign must encounter or explicitly inject the structurally plausible candidate:

```text
(x & (x - 1)) == 0
```

which incorrectly accepts `x = 0`.

The independent specification oracle returns `0` as a counterexample.

CandidateSpace refinement must preserve the resulting obligation, leading to the correct compact form semantically equivalent to:

```text
x != 0 && (x & (x - 1)) == 0
```

The hidden human-known expression is never authority; only the frozen discovered construction and exhaustive certification are.

---

## 19. Independent mathematical certification

Because `U8` is finite, `formula-check` exhaustively compares the frozen candidate with the independent sealed specification over all 256 values.

The certificate is an exact finite exhaustive witness bound to:

```text
candidate semantic digest
specification digest
U_0 digest
World digest
grammar digest
checker digest
all 256 comparison outcomes or compact authenticated equivalent
verdict
```

No sample-based or probabilistic evidence is accepted for semantic admission.

---

## 20. Promotion to U_1

After certification, perform the D5 transaction:

```text
U_0
 + candidate Entity/Relation
 + Judgement SpecificationEquivalent(candidate, sealed_spec)
 + Evidence envelope
 + CapabilityContract
    |
    v
Promotion
    |
    v
U_1
```

Required properties:

```text
U_0 unchanged and replayable
U_1 new immutable generation root
candidate now ADMITTED
capability activation explicit
candidate grammar/provenance retained
capability closure under U_1 includes new primitive
```

Promotion must be impossible through the Discovery Fabric directly.

---

## 21. Native realization

D4 takes the promoted semantic construction and creates a specialized forward U8->Bool realization.

For First Light, generation of standalone Rust source and compilation through a pinned `rustc -O` is sufficient.

The compiler is treated as untrusted with respect to semantic correctness.

Output artifacts:

```text
generated source digest
compiler/toolchain digest
binary digest
Realization manifest
```

---

## 22. Realization validation

A separate checker validates the compiled binary against the admitted semantic evaluator for all 256 inputs.

Only after exhaustive realization equivalence is established may the binary Realization be admitted.

This separately proves:

```text
P_M — discovered mathematics matches target semantics
P_R — native program matches admitted mathematics
```

---

## 23. Second-query reuse proof

Under `U_1`, submit a new query that requires the same semantics but is not the original synthesis request.

Canonical query:

```text
Given a canonical list/vector of U8 values,
return/filter/count the values satisfying IsPowerOfTwoU8.
```

Required engine behavior:

```text
Capability Resolver finds promoted primitive
D3 does not create the original synthesis CandidateSpace
no primitive-discovery Work Cell is launched
D4 selects admitted semantic/native Realization
result is exact
campaign manifest references U_1 primitive identity
```

The proof should include comparative campaign metrics showing the discovery work disappeared rather than merely becoming slightly faster.

This is the canonical evidence that:

```text
capability(U_1) > capability(U_0)
```

for the relevant problem family.

---

## 24. Negative-control suite

Canonical First Light must also prove fail-closed behavior.

```text
NC-01 modified sealed target digest -> manifest verification fails
NC-02 search code imports sealed fixture -> blindness gate fails
NC-03 FL-A sample-fitting near-miss -> rejected
NC-04 FL-B corrupted Boolean/GF2 translation -> rejected
NC-05 FL-C zero-accepting power-of-two near-miss -> rejected
NC-06 forged/mismatched Evidence digest -> promotion fails
NC-07 candidate changed after certificate -> promotion fails
NC-08 search process attempts authority write -> denied/fails
NC-09 compiled binary changed after realization proof -> dispatch rejects digest
NC-10 U_1 activation removed -> second query cannot claim promoted automatic reuse
NC-11 Authority Contract changed to stricter class without new evidence -> result rejected
NC-12 parent generation changed during promotion transaction -> transaction aborts
```

---

## 25. Canonical proof manifest

The final First-Light proof manifest binds:

```text
repository/source commit
Universe U_0 digest
Universe U_1 digest
World digests
activated package set
QueryIR digests
Theory Profile semantic inputs
CandidateSpace/grammar digests
campaign manifests
sealed target digests
candidate digests
Evidence envelopes/checker digests
promotion transaction digest
capability closure delta
native source/toolchain/binary digests
realization evidence
second-query campaign/result digests
negative-control results
```

The manifest must be independently replayable locally from required artifacts.

---

## 26. Canonical PASS markers

First Light is complete only when an independent verifier emits all markers:

```text
PASS D1_AUTHORITY_SEPARATION
PASS D2_IDENTITY_GENERATION_REPLAY
PASS D2_CERTIFICATE_ROUTING
PASS D2_SEARCH_STATE_SEPARATION
PASS D3_BLIND_SEMANTIC_ELABORATION
PASS D3_REPRESENTATION_REDUCTION
PASS D3_SYMBOLIC_CANDIDATE_SPACE
PASS D3_FALSE_NEARMISS_REJECTION
PASS D4_NATIVE_REALIZATION_EQUIVALENCE
PASS D4_CPU_LOCAL_OFFLINE
PASS D5_ATOMIC_PROMOTION
PASS D5_CAPABILITY_CLOSURE_EXPANDED
PASS D5_SECOND_QUERY_REUSE
PASS NEGATIVE_CONTROLS
PASS FIRST_LIGHT_COMPLETE
```

No single search executable may issue these markers without independent artifact replay.

---

## 27. What First Light proves

A complete proof establishes that the architecture can:

1. represent mathematical authority independently from search state;
2. bind exact identities/worlds/evidence;
3. search symbolic candidate spaces blindly;
4. reject attractive false mathematics;
5. discover and select representation/reduction routes;
6. independently certify candidate mathematics;
7. atomically promote it into a stronger generation;
8. derive new capability closure;
9. compile the new semantics into native CPU code;
10. independently validate the machine realization;
11. reuse the new capability without rediscovery.

---

## 28. What First Light does not prove

It does not prove:

- universal mathematics;
- unsolved-research discovery;
- distributed scaling;
- GPU benefit;
- final semantic hypergraph implementation;
- final proof-assistant integration;
- final storage/IR/backend choices;
- model-assisted discovery;
- self-host/bootstrap trust minimization.

Those remain downstream capability campaigns.

---

## 29. Ptah is explicitly deferred

Canonical First Light runs without Ptah.

Ptah integration is allowed only after the local proof establishes that:

```text
WorkCell/Campaign contracts serialize deterministically
returned artifacts are content-addressed
Evidence requirements are execution-location independent
promotion remains local authority logic
```

Ptah may later scale Work Cells; it must not be required to prove First Light or consume the resulting promoted primitive locally.

---

## 30. First-Light frozen rule

> **First Light passes only if a primitive absent from U_0 is blindly discovered, a convincing false near-miss is rejected, the surviving semantics are independently certified and promoted into U_1, a native realization is independently validated, and a second query demonstrably reuses that promoted capability without rediscovering it.**
