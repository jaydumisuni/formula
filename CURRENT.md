# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

Recover repository evidence before reasoning. Do not reconstruct implementation state from chat memory when these files are available.

## Primary authorities

1. [`docs/design/README.md`](docs/design/README.md) — frozen D1–D5 design precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen roadmap P0 onward.
3. [`docs/checkpoints/2026-09-04-p8-native-realization-validation.md`](docs/checkpoints/2026-09-04-p8-native-realization-validation.md) — finally frozen P8 checkpoint.
4. [`docs/checkpoints/2026-09-04-p7-promotion-generation-transition.md`](docs/checkpoints/2026-09-04-p7-promotion-generation-transition.md) — exact P7 predecessor checkpoint.
5. [`docs/superpowers/specs/2026-09-04-p8-native-realization-validation-design.md`](docs/superpowers/specs/2026-09-04-p8-native-realization-validation-design.md) — frozen P8 design.
6. [`docs/superpowers/plans/2026-09-04-p8-native-realization-validation.md`](docs/superpowers/plans/2026-09-04-p8-native-realization-validation.md) — executed P8 implementation plan.
7. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for a concrete contradiction or missing obligation.

## Exact current implementation state

**P8 — D4 Native Realization + Validation: FINALLY FROZEN.**

Canonical branch:

```text
implementation/p8-native-realization-validation
```

Exact finally frozen P7 predecessor:

```text
e82f7b0535694285baeeb4baae37edc27b6864b8
workflow: P7 canonical proof
workflow run: 33862079731
job: 100988579155
conclusion: success
```

Canonical P8 source-under-test proof boundary:

```text
3ca395e13de7dbbc611347f37b8cbaf3875d4236
workflow: P8 canonical proof
workflow run: 33898667278
job: 101107293202
conclusion: success
```

Exact finally frozen P8 documentation-bearing proof head:

```text
fa369b6241c0c069176e5939acf4d5ec74eb8085
workflow: P8 canonical proof
workflow run: 33899079722
job: 101108627933
conclusion: success
```

The successful final workflow checked out exact SHA `fa369b6241c0c069176e5939acf4d5ec74eb8085`, ran on Ubuntu 24.04 with Rust 1.98.0, re-earned every P8 marker, passed the real FL-C native CPU proof, all predecessor/workspace tests, build, rustfmt, Clippy with warnings denied, dependency firewalls, and clean-worktree checks.

Exact P7 -> P8 source compare:

```text
base:    e82f7b0535694285baeeb4baae37edc27b6864b8
head:    3ca395e13de7dbbc611347f37b8cbaf3875d4236
status:  ahead
ahead:   37 commits
behind:  0 commits
files:   20
```

Exact source-proof -> frozen-docs-head compare:

```text
base:    3ca395e13de7dbbc611347f37b8cbaf3875d4236
head:    fa369b6241c0c069176e5939acf4d5ec74eb8085
status:  ahead
ahead:   1 commit
behind:  0 commits
files:   2 documentation files only
```

Pinned canonical proof toolchain:

```text
Rust 1.98.0
host: x86_64-unknown-linux-gnu
runner: ubuntu-24.04
Cargo proof commands: --locked / --offline where applicable
canonical workflow permissions: contents: read
```

## What P8 proves

P8 converts the P7-admitted FL-C semantic primitive into one bounded native CPU realization while keeping mathematical authority and realization authority separate:

```text
P7-admitted semantic primitive in U1
 -> exact SpecializationIdentity
 -> deterministic standalone Rust source
 -> rustc 1.98.0 -O
 -> native binary
 -> execute all 256 U8 inputs
 -> independent checker validation
 -> opaque RealizationAuthorization
 -> AuthorityStore::admit_realization
 -> exact context dispatch
```

P8 proves:

```text
canonical specialization identity
EXACT_EQUIVALENCE lowering identity
canonical source/toolchain/binary realization manifest
realization generator cannot reach checker/store authority
compiler success cannot self-admit
checker-issued opaque realization authorization
source mutation rejection
binary mutation rejection
specialization/toolchain/context mismatch rejection
exact 256-input native CPU equivalence
active-generation admission check
content-addressed binary admission
exact dispatch-context matching
tampered admitted binary fails closed
U0 mathematical generation unchanged
U1 mathematical generation unchanged
P7 mathematical authority preserved
CPU-local/offline proof after dependency priming
```

## Authority boundary

The P8 authority split is executable:

```text
mathematical authority:
  P7 checker + promotion -> immutable U1 semantic primitive

realization production:
  formula-realize + rustc -> untrusted source/binary candidate

realization authority:
  formula-check -> opaque RealizationAuthorization
  formula-store -> consume authorization + persist immutable binary

execution:
  RealizationDispatchContext -> exact admitted artifact only
```

Production realization generation remains outside authority publication:

```text
formula-realize -> formula-core + formula-engine
formula-realize -/-> formula-check
formula-realize -/-> formula-store
formula-realize -/-> formula-first-light

formula-engine -/-> formula-check
formula-engine -/-> formula-store
formula-engine -/-> formula-first-light
```

`formula-first-light` links checker/store/realize only as **dev-dependencies** for integration proof. Its production dependencies remain `formula-core` + `formula-engine`.

## Realization identity law

`SpecializationIdentity` binds:

```text
semantic target
generation
world
authority contract
observer
EXACT_EQUIVALENCE lowering class
```

`NativeRealizationManifest` additionally binds exact source digest, toolchain identity, and binary digest. A changed source, compiler identity, binary, generation, world, authority contract, observer, or specialization cannot silently reuse an existing authorization.

## Independent checker law

The compiler/generator is not trusted to certify realization correctness.

The checker requires exact native/check-manifest bindings and exhaustive 256-input output equality. Successful validation is the only constructor path to opaque `RealizationAuthorization`; its authority-bearing fields are private and it has no public constructor.

Negative controls include:

```text
changed specialization -> reject
changed toolchain -> reject
changed source bytes -> reject
changed binary bytes -> reject
short output set -> reject
one wrong native output -> exact counterexample
compiler output without authorization -> not dispatchable
stale generation at admission -> reject
changed binary at admission -> reject
wrong dispatch context -> no artifact
tampered stored binary -> verified blob failure
```

## Store and dispatch law

P8 extends the existing `AuthorityStore`; it does not create a second authority root.

Admission requires checker authorization, exact active generation, and exact binary digest before bytes enter the existing content-addressed immutable BlobStore and realization registry.

Dispatch is exact over semantic target, generation, world, authority contract, and observer. Context mismatch fails closed.

## FL-C real native CPU proof

The canonical P8 proof re-runs the P7 FL-C discovery/promotion path, specializes the U1-admitted semantic expression, generates Rust, compiles it with pinned `rustc 1.98.0 -O`, executes the native binary for every input `0..=255`, independently checks those outputs, admits only the checker-authorized binary, and dispatches it through the exact context.

The targeted native proof passed on both the source proof head and the final documentation-bearing head. The final exact-head run repeated the path through First-Light and workspace sweeps.

Realization admission does not create a new mathematical generation:

```text
U0 digest/bytes unchanged
U1 digest/bytes unchanged
```

A failed realization therefore cannot invalidate or rewrite already-admitted mathematics.

## P8 proof markers

Both canonical source run `33898667278` and final exact-head run `33899079722` earned the required P8 markers after their corresponding gates passed:

```text
PASS P8_SEMANTIC_BINDING
PASS P8_SPECIALIZATION_IDENTITY
PASS P8_EXACT_LOWERING_CLASS
PASS P8_SOURCE_TOOLCHAIN_BINARY_BOUND
PASS P8_COMPILER_CANNOT_SELF_ADMIT
PASS P8_ALL_256_INPUTS_EQUIVALENT
PASS P8_MUTATED_SOURCE_REJECTED
PASS P8_MUTATED_BINARY_REJECTED
PASS P8_DISPATCH_IDENTITY_ENFORCED
PASS P8_CPU_LOCAL_OFFLINE
PASS P8_P7_MATH_AUTHORITY_PRESERVED
```

## P0–P7 remain authority

P8 extends rather than replaces:

```text
P0 reproducible repository/build + architecture firewall
P1 deterministic identity + immutable atomic authority store
P2 independent checker/certificate authority
P3 theory packages + capability closure
P4 deterministic query/compiler/campaign planning
P5 bounded CandidateSpace + discovery
P6 sealed First-Light harness + blindness gates
P7 checker-authorized atomic promotion into immutable U1
```

## Not proved by P8

Do not claim from P8:

```text
second-query reuse under U1
proof that search/synthesis is skipped on reuse
P9 reusable dispatch policy
complete P9 First-Light proof manifest
FIRST_LIGHT_COMPLETE
GPU/SIMD/JIT realization
Ptah/distributed execution
P9 completion
P10 completion
```

## Next implementation boundary

The next frozen-roadmap phase is **P9 — reuse / complete First-Light proof**.

P8 is now fully frozen, so P9 may begin from exact frozen P8 proof head:

```text
fa369b6241c0c069176e5939acf4d5ec74eb8085
```

P9 must be implemented as a new phase/branch. It must consume the P8-authorized realization and prove next-query reuse behavior without silently re-running synthesis when reuse is valid. `FIRST_LIGHT_COMPLETE` is not earned by P8.

**P9 has not started.**

## Constitutional laws to preserve

1. Search may propose mathematics. Only Certification + Promotion can create mathematical authority.
2. Execution may consume authority. Execution cannot manufacture authority.
3. Mathematical correctness and realization correctness are separate proof obligations.
4. Candidate/search/compiler state remains outside admitted generation authority until explicitly promoted.
5. Resource exhaustion never becomes mathematical refutation or weakens an Authority Contract.
6. Promotion is generation-producing, atomic, and history preserving.
7. A false realization cannot invalidate already admitted mathematics.
8. Capability closure is derived state, never an authority source.
9. Heuristics cannot delete exact candidates or create authority.
10. Sealed First-Light targets cannot leak backward into discovery.
11. Raw generation publication cannot bypass checker-issued promotion authorization.
12. Realization admission requires independent checker authorization and exact semantic/source/toolchain/binary/context binding.
13. Compilation success alone never creates realization authority.
14. Dispatch may select only an admitted realization matching the exact authority context.

## Recovery procedure

1. Read this file.
2. Read the P8 checkpoint, P8 design, and P8 implementation plan.
3. Treat `fa369b6241c0c069176e5939acf4d5ec74eb8085` as the exact finally frozen P8 proof head.
4. Verify final canonical run `33899079722`, job `101108627933`, if P8 freeze authority is questioned.
5. Preserve frozen P7 predecessor `e82f7b0535694285baeeb4baae37edc27b6864b8` as the predecessor review boundary.
6. Start P9 only as a separate phase from the frozen P8 proof head; do not rewrite P8 history.
7. Do not reopen broad research unless repository evidence exposes a concrete contradiction or missing requirement.

## Freeze state

P8 is **FINALLY FROZEN** on exact documentation-bearing proof head:

```text
fa369b6241c0c069176e5939acf4d5ec74eb8085
workflow: P8 canonical proof
run: 33899079722
job: 101108627933
conclusion: success
```

The source-under-test proof remains independently recorded at `3ca395e13de7dbbc611347f37b8cbaf3875d4236`, run `33898667278`, job `101107293202`, success.

Any later commit that changes only recovery documentation to record this already-earned result is **post-freeze metadata**. It does not redefine the P8 frozen proof head and does not create a recursive requirement to prove the run ID that records itself.

The P8 branch has **not** been merged to `main`. P9 has **not** started.
