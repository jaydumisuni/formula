# D1A — Self-Hosting and Bootstrap Trust Amendment

**Date:** 2026-08-28  
**Status:** NORMATIVE AMENDMENT TO FROZEN D1  
**Applies to:** `docs/design/2026-08-28-d1-mathematical-constitution.md`

D1 remains immutable. This amendment adds a missing constitutional requirement: the mathematical machine must ultimately be able to self-host substantial portions of its own compiler, checker, runtime, and mathematical capability machinery **without circularly trusting the current generation simply because it produced the next one**.

This is not merely "deploy the service on our own hardware." It is a bootstrap/trust property.

---

## 1. Self-hosting law

The project shall support a path where later generations use admitted mathematics and admitted realizations to rebuild, check, specialize, and improve substantial parts of the machinery that implements the project itself.

However:

> **Self-hosting does not grant self-signing authority.**

A system may generate its successor. It may not authorize that successor merely because it generated it.

All self-produced compiler passes, checkers, search methods, metaprimitives, runtime components, or optimized realizations cross the same Certification + Promotion boundary as external artifacts.

---

## 2. Staged bootstrap model

The target trust chain is staged rather than circular.

```text
Stage 0
    tiny, auditable bootstrap root
        |
        v
Stage 1
    minimal semantic loader
    minimal certificate checker(s)
    minimal deterministic build/runtime substrate
        |
        | Stage 0 establishes/builds Stage 1
        v
Stage 2
    richer checker/compiler/runtime machinery
        |
        | Stage 1 establishes/builds Stage 2
        v
Stage N
    ordinary self-hosted mathematical machine
        |
        | certified rebuild + independent validation
        v
Stage N+1
```

The exact Stage-0 language, instruction subset, or seed binary is **not frozen by D1A**.

D2 must design the practical bootstrap chain.

---

## 3. Bootstrap root must be smaller than the normal system

The normal machine may contain millions of lines of code, federated libraries, theorem packages, generated kernels, and promoted mathematics.

The bootstrap authority must deliberately minimize the software that needs prior trust.

The direction is:

```text
large opaque bootstrap dependency
    -> smaller source bootstrap
    -> smaller auditable seed
    -> independently reproducible stages
```

The architecture does not claim zero physical trust. CPU, memory, firmware, and hardware remain assumptions unless separately addressed.

The software trust root must nevertheless be explicit, measurable, and reducible.

---

## 4. Verified self-bootstrap is technically feasible

The design is supported by existing donor patterns rather than speculative circular reasoning.

CakeML demonstrates a verified compiler that can bootstrap itself inside HOL: a proved compiler implementation can compile itself and produce a binary tied back to its formal semantics.

Bootstrappable-build efforts such as GNU Mes/stage0 demonstrate how modern toolchains can be reached through deliberately small source/bootstrap seeds instead of treating a large opaque compiler binary as an unavoidable starting assumption.

Diverse double compilation provides an additional defense against compiler-substitution / "trusting trust" attacks by comparing independently derived compilation paths where the required reproducibility assumptions hold.

D1A takes the architectural principle from these systems, not their exact languages or implementation stacks.

---

## 5. Self-hosted generation transition

A self-hosted upgrade is a special Promotion transaction.

Conceptually:

```text
U_g
 + admitted compiler/checker/runtime semantics
 + exact source/semantic inputs
 + candidate successor machinery
 + build evidence
 + semantic/realization evidence
 + bootstrap/diverse validation policy
    |
    v
Promotion
    |
    +-- reject/quarantine
    |
    `-- admit successor realizations in U_(g+1)
```

The currently running system may orchestrate this transaction, but it cannot be the only authority establishing the successor's correctness.

---

## 6. Two independent correctness obligations remain mandatory

Self-hosting does not collapse the D1 two-proof model.

### Mathematical / semantic correctness

```text
P_M:
The compiler/checker/metaprimitive/runtime component
has the claimed mathematical semantics.
```

### Realization correctness

```text
P_R:
This concrete executable binary/realization implements
those admitted semantics under execution contract E.
```

A self-reproducing compiler that faithfully reproduces the wrong semantics is still wrong.

A mathematically valid compiler description with a corrupted bootstrap binary is still unsafe.

Both layers remain distinct.

---

## 7. Reproducible build identity

Self-hosting generations must bind build provenance to exact identities where reproducibility is claimed.

A build record should eventually be able to bind at least:

```text
universe generation
semantic/source input digests
compiler semantic identity
compiler realization digest
checker identities
bootstrap-stage identity
backend/toolchain identities
build-policy version
target architecture / execution contract
output realization digest
```

Undeclared environment dependencies must not silently become authority inputs.

Reproducibility is not itself proof of semantic correctness, but it is powerful evidence that the claimed source/toolchain path actually produced the admitted binary.

---

## 8. Diverse validation

Critical self-hosted components should support an independent derivation path where feasible.

Possible patterns include:

```text
verified compiler path
        vs
independent bootstrap compiler path
```

or:

```text
current compiler -> candidate -> candidate
        compared with
independent trusted compiler -> candidate -> candidate
```

or a future equivalent established through translation validation / proof-producing compilation.

No one specific diverse-compilation method is mandatory constitutionally.

The law is:

> **A critical authority-producing toolchain must be capable of independent validation outside the exact implementation path it is validating.**

---

## 9. Checker self-hosting boundary

Certificate checkers are especially sensitive because they sit near mathematical authority.

A new checker realization must not become trusted simply because the old checker accepted a proof claiming the new checker is valid while both rely on the same unexamined implementation path.

For authority-critical checkers, D2 should design one or more of:

- formally verified checker semantics with proof extraction / verified compilation;
- independent checker realizations;
- foundational replay for selected certificate families;
- bootstrap-stage checking from a smaller authority root;
- diverse compilation/realization comparison.

The trust chain must be explicit in the Evidence envelope.

---

## 10. Self-hosted metaprimitives

The machine may discover:

- a faster invariant finder;
- a new rewrite strategy;
- a better compiler optimization;
- a stronger CandidateSpace backend;
- a new proof-search method;
- a new capability inference rule.

It may use its current system to synthesize and test these candidates.

Activation still requires the D1 metaprimitive gate:

```text
soundness/applicability contract
termination/finiteness/completeness claims scoped precisely
negative/adversarial controls
transfer beyond discovery examples
composition/interference analysis
independent evidence
rollback/fallback path
```

Self-hosting therefore allows the machine to improve its own mathematical machinery while preserving the rule:

> **Candidate self-improvement is allowed. Uncertified self-authority is not.**

---

## 11. Bootstrap generations vs mathematical Universe generations

Bootstrap/toolchain generations and mathematical Universe generations are related but not identical.

A new mathematical theorem may create `U_(g+1)` without changing the compiler binary.

A new verified compiler realization may be admitted in `U_(g+1)` without changing mathematical semantics.

The system must therefore preserve separate identities for:

```text
semantic mathematical generation
capability activation generation
compiler/runtime realization generation
bootstrap/trust-chain generation
```

They may be bundled in a release manifest but must not be collapsed semantically.

---

## 12. Local/offline/self-hosted operation

The core authority path must not require a remote proprietary service to function.

A local installation with the required admitted artifacts/checkers must be capable of:

- loading a Universe generation;
- evaluating admitted primitives;
- checking supported certificates;
- compiling/specializing local realizations;
- running bounded discovery/search;
- producing promotion candidates;
- replaying authority evidence available locally.

Cloud or Ptah-scale resources may accelerate campaigns, but they are not constitutional dependencies for ordinary mathematical capability.

This preserves the original ordinary-hardware goal.

---

## 13. Ptah relationship under self-hosting

Ptah can later execute bootstrap/build/proof activities across machines, but Ptah itself is not mathematical authority.

Self-hosting therefore remains:

```text
mathematical machine
    defines semantics + proof obligations

Ptah
    executes requested build/proof/work activities

Certification
    checks returned artifacts

Promotion
    decides authority transition
```

The mathematical machine must retain a local path that does not require Ptah merely to execute already promoted mathematics.

---

## 14. New D1 invariant

D1's frozen laws are extended by one additional invariant:

> **21. Self-host without circular trust:** the machine may rebuild and improve substantial parts of itself, but successor authority must be grounded in an explicit smaller/bootstrap trust chain and/or independent verification path; no generation may authorize itself solely by executing itself.

This amendment is normative for D2 and all later architecture.

---

## 15. D2 obligations introduced by D1A

D2 must now design:

1. the practical Stage-0 trust root;
2. Stage-1 minimal semantic/certificate runtime;
3. source/semantic bootstrap artifact format;
4. reproducible build manifest;
5. checker bootstrap strategy;
6. independent/diverse validation path for authority-critical toolchain components;
7. local/offline execution packaging;
8. self-hosted upgrade transaction;
9. rollback to previous toolchain + Universe authority generation;
10. proof that self-hosted tooling still cannot write mathematical authority directly.

No implementation roadmap is frozen until these contracts are demonstrated on the First-Light path.

---

## 16. Evidence basis

This amendment is supported by targeted bootstrap research plus the existing D1 research authority.

Key external donors investigated for this amendment:

- CakeML verified compiler bootstrapping;
- GNU Mes / Bootstrappable Builds / stage0-style source bootstrapping;
- Diverse Double Compilation for detecting compiler substitution / trusting-trust attacks.

These establish feasibility of the design principle but do not dictate the final implementation technology.
