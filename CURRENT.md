# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

Recover repository evidence before reasoning. Do not reconstruct implementation state from chat memory when these files are available.

## Primary authorities

1. [`docs/design/README.md`](docs/design/README.md) — frozen D1-D5 design precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen roadmap P0 onward.
3. [`docs/checkpoints/2026-09-06-p11-federation-breadth.md`](docs/checkpoints/2026-09-06-p11-federation-breadth.md) — exact P11 source-proof checkpoint; docs-head proof pending.
4. [`docs/superpowers/specs/2026-09-06-p11-federation-breadth-design.md`](docs/superpowers/specs/2026-09-06-p11-federation-breadth-design.md) — frozen P11 design.
5. [`docs/superpowers/plans/2026-09-06-p11-federation-breadth.md`](docs/superpowers/plans/2026-09-06-p11-federation-breadth.md) — executed P11 implementation/freeze plan.
6. [`docs/checkpoints/2026-09-05-p10-self-expansion-hardening.md`](docs/checkpoints/2026-09-05-p10-self-expansion-hardening.md) — exact finally frozen P10 predecessor checkpoint.
7. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for a concrete contradiction or missing obligation.

## Exact current implementation state

**P11 — Federation Breadth: SOURCE PROVED; documentation-head proof pending.**

Canonical branch:

```text
implementation/p11-federation-breadth
```

Exact finally frozen P10 predecessor:

```text
3aeb61daf4d575db0f018245ee271597ad475e7b
workflow: P10 canonical proof
run:      34024846890
job:      101463880804
result:   success
```

Canonical P11 source-under-test proof boundary:

```text
b5377eb78c7540d927fca1aea9e04ca5b3a56371
workflow: P11 canonical proof
run:      34030804901
job:      101479821966
result:   success
```

P11 source implementation is complete under the permanent read-only canonical workflow. The final documentation-bearing proof has not yet been earned at the time of this recovery record; therefore P11 is not yet finally frozen.

The temporary P11 development workflow was removed before the source-under-test proof and is not part of canonical authority.

## Permanent canonical P11 workflow

```text
path:        .github/workflows/p11-canonical-proof.yml
blob SHA:    97192299b4ea2aac469da38b4885e2608b2d7bd3
permissions: contents: read
runner:      ubuntu-24.04
Rust:        1.98.0
rustc:       88d9e12ae178fab0fb5cc050a94da85685d449ea
cargo:       1.98.0 (797e8a9bc 2026-08-05)
host:        x86_64-unknown-linux-gnu
LLVM:        22.1.8
```

## What P11 proves at the source boundary

P11 extends the frozen federation architecture with two heterogeneous independently checked specialist routes while keeping producer identity untrusted:

```text
untrusted SAT/LRAT producer
 -> DIMACS semantic identity
 -> independent LRAT/RUP checker
 -> checker-bound evidence
 -> exact adapter-route admission
 -> provenance-bound CertifiedFederationFact
 -> explicit directional BridgeContract

untrusted exact-arithmetic producer
 -> arbitrary-precision IntegerOperation identity
 -> independent BigInt recomputation
 -> canonical decimal equality
 -> checker-bound evidence
 -> exact adapter-route admission
 -> provenance-bound CertifiedFederationFact

checked SAT fact + safe bridge/composition + checked arithmetic fact
 -> heterogeneous final target
 -> NC11-01...NC11-14
 -> FederationBreadthProofManifest
 -> independent final replay verifier
 -> PASS FEDERATION_BREADTH_PROVED
```

Unsupported LRAT behavior fails closed. Candidate-only routes cannot certify facts. Bridge direction, source package/subject/polarity, translation, composition class and non-strengthening polarity are explicit checked contracts. Package or producer identity alone never authorizes mathematics.

## Source-run proof identities

Exact source-run evidence from run `34030804901`, job `101479821966`:

```text
P11_SOURCE_SHA=b5377eb78c7540d927fca1aea9e04ca5b3a56371
P11_P10_PREDECESSOR=sha256:37a0309762564ecdec48792b951c1d0eaf6fb5342c354cdd7d58f57934079486
P11_MANIFEST=sha256:94cc3ddbf41b4993d50ca4e6e2fc37f0483804b46f3aadd7399f99d357eaf88b
P11_SAT_PACKAGE=sha256:c3351c1d8e030cf01d29c70d1d7f3119e578b97dcd20de013b7f780d37daede2
P11_SAT_ADAPTER=sha256:3d41a245fd2b28d0387b104e3efc0ed7bbd31a2582d3debead36cc2817aa4862
P11_ARITHMETIC_PACKAGE=sha256:adbabc997de811b254ad9e7052324d6a89d57014024c3c07b0fc9e1391f35e8d
P11_ARITHMETIC_ADAPTER=sha256:7fca1ad7b7b79a9af6de140769008e20d86df1036c8c584e745a1ea9ccb793b5
P11_BRIDGE=sha256:7ffc888a9ee6d9703ac6e52b61dc1f1fb086d974cd5af7981fa9d84e27266406
P11_NEGATIVE_CONTROLS=sha256:3faadf4580899f87e2a6c8a9a26e108b8d40121981f5a32b6de40f5700a40375
P11_FINAL_TARGET=sha256:dd35426c857ad922d8439e7d539fabe34d2daabb9f255dd35953b57a752a8844
```

## Executed NC11-01...NC11-14

```text
NC11-01 CANDIDATE_ONLY_AUTHORITY_ATTEMPT
NC11-02 FORGED_LRAT_HINT
NC11-03 LRAT_MISSING_EMPTY_CLAUSE
NC11-04 UNSUPPORTED_RAT_PROOF_FAILS_CLOSED
NC11-05 WRONG_SAT_CHECKER_ROUTE
NC11-06 INCORRECT_EXACT_ARITHMETIC_RESULT
NC11-07 MALFORMED_EXACT_ARITHMETIC_DECIMAL
NC11-08 WRONG_ARITHMETIC_TRANSLATION
NC11-09 STALE_SEMANTIC_INPUT_DIGEST
NC11-10 SHARED_FACT_POLARITY_UPGRADE
NC11-11 MISSING_BRIDGE_CONTRACT
NC11-12 WRONG_BRIDGE_DIRECTION
NC11-13 UNSAFE_COMPOSITION_CLASS
NC11-14 PRODUCER_IDENTITY_CANNOT_AUTHORIZE
```

The canonical integration constructs the negative-control manifest only after each concrete failure path is observed.

## P11 marker contract

The source proof emitted exactly:

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

This marker set is source-proved but does not become final cross-chat recovery authority until the unchanged canonical workflow proves the documentation-bearing head.

## Canonical source gates

Run `34030804901` passed:

```text
exact identity + pinned toolchain
locked/offline metadata
canonical heterogeneous federation proof
independent final verifier
frozen P10 predecessor proof
predecessor architecture firewalls
formula-core tests
formula-check tests
formula-store tests
formula-realize tests
formula-packages tests
formula-engine tests
formula-first-light tests
workspace tests
workspace build
rustfmt
Clippy with -D warnings
dependency trees
authority dependency firewall
source authority firewall
clean worktree
```

## Authority boundary

```text
search / external producers -> candidate/evidence only
independent checker -> evidence authority
adapter manifest -> semantic route contract, not authority
CertifiedFederationFact -> checked provenance binding
BridgeContract -> explicit directional translation contract
SharedFact -> world/subject/polarity/evidence-bound exchange
composition -> safe class required
independent P11 verifier -> final manifest replay authority
```

The P11 workflow preserves the frozen checker/engine/store/realization/First-Light dependency and source firewalls.

## P0-P10 remain authority

P11 consumes and extends, but does not rewrite, the frozen predecessor system:

```text
P0 reproducible repository/build + architecture firewall
P1 deterministic identity + immutable atomic authority store
P2 independent checker/certificate authority
P3 theory packages + capability closure
P4 deterministic query/compiler/campaign planning
P5 bounded CandidateSpace + discovery
P6 sealed First-Light harness + blindness gates
P7 checker-authorized atomic promotion into immutable U1
P8 independently authorized native realization + exact dispatch
P9 canonical First-Light closure + durable activation + zero-rediscovery reuse
P10 self-expansion hardening + explicit grammar/proof/realization evolution authority
```

Exact frozen P10 proof boundary remains `3aeb61daf4d575db0f018245ee271597ad475e7b`.

## Not proved by P11

Do not claim from P11:

```text
producer identity as mathematical authority
full LRAT including arbitrary unsupported RAT/deletion semantics
arbitrary external solver trust
polarity-strengthening bridges
unsafe heuristic composition as certified cooperation
network-backed proof authority
model-backed proof authority
unbounded specialist ecosystem coverage
P12 completion
```

## Documentation-head freeze requirement

The next exact operation is not more P11 implementation. It is recursive recovery proof:

1. this file and the P11 checkpoint must be the only source-to-docs changes;
2. `.github/workflows/p11-canonical-proof.yml` must remain blob `97192299b4ea2aac469da38b4885e2608b2d7bd3`;
3. the unchanged workflow must pass on the exact documentation-bearing head;
4. predecessor, package/adapter, bridge, negative-control, final-target and marker identities must reproduce; only `P11_SOURCE_SHA` and source-bound `P11_MANIFEST` may change;
5. exact docs SHA/run/job are then recorded as post-proof metadata without recursively moving the proof boundary;
6. only then is `FEDERATION_BREADTH_PROVED` final recovery authority and the next roadmap phase may begin.

## Constitutional laws to preserve

1. Search may propose mathematics. Only Certification + Promotion can create mathematical authority.
2. Execution may consume authority. Execution cannot manufacture authority.
3. Mathematical correctness and realization correctness remain separate proof obligations.
4. Resource exhaustion never becomes mathematical refutation or weakens an Authority Contract.
5. Promotion is atomic and history preserving.
6. Capability closure is derived state, never an authority source.
7. Heuristics cannot delete exact candidates or create authority.
8. Compilation success alone never creates realization authority.
9. Semantic reuse requires exact active-generation authority evidence.
10. Promotion-class effects and activation modes are checked policy, not producer discretion.
11. CandidateSpace semantics are bound to an explicit grammar-generation identity.
12. Non-conservative change cannot silently transport prior proof authority.
13. Realization-only evolution cannot create mathematical admission.
14. Federation producers and adapters are untrusted until independent checking establishes evidence authority.
15. Cross-package Shared Fact propagation requires an explicit safe directional bridge and may not strengthen polarity.
16. `FEDERATION_BREADTH_PROVED` becomes final recovery authority only after the unchanged canonical P11 workflow proves the exact documentation-bearing head.

## Recovery procedure

1. Read this file.
2. Read the P11 checkpoint, P11 design and P11 implementation/freeze plan.
3. Treat `3aeb61daf4d575db0f018245ee271597ad475e7b` as exact frozen P10 predecessor proof authority.
4. Treat `b5377eb78c7540d927fca1aea9e04ca5b3a56371`, run `34030804901`, job `101479821966`, as the exact P11 source-under-test proof boundary.
5. Verify `.github/workflows/p11-canonical-proof.yml` blob is `97192299b4ea2aac469da38b4885e2608b2d7bd3` and has `contents: read`.
6. Resolve the documentation-bearing candidate and its unchanged canonical run.
7. Do not mark P11 finally frozen until that exact run succeeds.

## Freeze state

```text
P11 source proof:          PROVED
source head:               b5377eb78c7540d927fca1aea9e04ca5b3a56371
source run:                34030804901
source job:                101479821966
source conclusion:         success
canonical workflow blob:   97192299b4ea2aac469da38b4885e2608b2d7bd3
documentation proof:       PENDING
P11 final freeze:          PENDING
FEDERATION_BREADTH_PROVED: SOURCE-PROVED ONLY
next roadmap phase:        blocked only by P11 docs-head proof
```
