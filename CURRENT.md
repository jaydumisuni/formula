# CURRENT — Cross-chat recovery authority

**Repository:** `jaydumisuni/formula`  
**Repository name:** `formula` is temporary only; it is not the final product name or mathematical identity.  
**Canonical frozen architecture checkpoint:** `50c0beb021d8bf02d59a177049b9c2cf1783b26a`  
**Recovery status date:** 2026-09-11

Recover repository evidence before reasoning. Do not reconstruct project state from chat memory when this file and the frozen design/roadmap are available.

## Exact current state

The architecture/research campaign is complete enough for implementation.

The canonical frozen architecture is:

1. **D1 — Mathematical Constitution**  
   `docs/design/2026-08-28-d1-mathematical-constitution.md`
2. **D1A — Self-Hosting and Bootstrap Trust Amendment**  
   `docs/design/2026-08-28-d1a-self-hosting-bootstrap-amendment.md`
3. **D2 — Core System Architecture**  
   `docs/design/2026-08-28-d2-core-system-architecture.md`
4. **D3 — Mathematical Compiler Architecture**  
   `docs/design/2026-08-28-d3-mathematical-compiler-architecture.md`
5. **D4 — Native Execution Architecture**  
   `docs/design/2026-08-28-d4-native-execution-architecture.md`
6. **D5 — Self-Expansion Architecture**  
   `docs/design/2026-08-28-d5-self-expansion-architecture.md`
7. **Canonical First-Light specification**  
   `docs/design/2026-08-28-first-light-specification.md`
8. **Implementation roadmap P0–P13**  
   `docs/roadmap/2026-08-28-implementation-roadmap.md`

The architecture/roadmap freeze is recorded in:

`docs/checkpoints/2026-08-28-d1-d5-roadmap-freeze.md`

The older D2 Operational Mathematical Machine and D3 First-Light Build Architecture documents remain preserved as precursor evidence, but they no longer define the canonical milestone numbering.

## Important recovery correction

An older version of this file stopped at the D1–D3 / F0-B01 planning boundary and said to begin B01.

That handoff is superseded by the later D1–D5 architecture normalization and frozen P0–P13 roadmap at commit:

`50c0beb021d8bf02d59a177049b9c2cf1783b26a`

Do **not** restart Formula from B01 merely because an older handoff or chat says so.

## Current executable milestone

**P0 — Repository and reproducible build skeleton.**

At the canonical frozen checkpoint, no implementation code was claimed complete. Current `main` contains the design/research/roadmap authority but does not contain the planned P0 Rust workspace (`Cargo.toml`, `rust-toolchain.toml`, or the planned `crates/` implementation tree).

Therefore P0 is the first unproven implementation milestone unless newer implementation evidence is present in a construction branch/worktree when this file is recovered.

### P0 required workspace

```text
crates/
  formula-core/
  formula-store/
  formula-check/
  formula-engine/
  formula-packages/
  formula-realize/
  formula-first-light/
  formula-cli/

rust-toolchain.toml
Cargo.toml
Cargo.lock
tests/authority-boundary/
```

### P0 proof obligations

```text
P0-01 pinned toolchain/source dependency manifest
P0-02 formula-check cannot depend on formula-engine/search crates
P0-03 sealed First-Light fixtures cannot be imported by discovery packages
P0-04 no network dependency in canonical First-Light runtime path
P0-05 deterministic test fixture identities
```

P0 is **not PROVEN** until a clean local build plus the declared architecture/dependency checks satisfy the roadmap gate.

## Implementation order

```text
P0  Repository/build skeleton
P1  Structural identity + authority store
P2  Independent checker/certificate core
P3  Theory packages + capability closure/federation contracts
P4  Query/compiler/campaign core
P5  CandidateSpace + bounded discovery
P6  First-Light target harness + blindness gates
P7  Promotion + generation transition
P8  Native realization + independent validation
P9  Canonical First-Light proof/freeze
P10 Self-expansion hardening
P11 Federation breadth
P12 Self-host/bootstrap trust reduction
P13 Ptah integration — explicitly deferred
```

**P9 is the first major implementation freeze.**

Do not make distributed execution, GPUs, models, Ptah, advanced proof federation, UI/API work, or large search infrastructure prerequisites for P9.

## Canonical First-Light boundary

First Light remains:

```text
local
ordinary CPU
model-free
network-free during canonical execution
GPU-free
Ptah-free
independently checked
content-addressed/replay-bound
```

The required end-to-end growth proof is:

```text
U_0
 -> blind discovery
 -> false near-miss rejection
 -> independent semantic certification
 -> atomic promotion
 -> U_1
 -> capability closure expansion
 -> native CPU realization
 -> independent realization validation
 -> second related query
 -> promoted capability reused without rediscovery
```

The frozen targets are:

```text
FL-A exact polynomial identity discovery
FL-B Boolean-XOR -> GF(2) representation/reduction discovery
FL-C U8 power-of-two primitive synthesis/self-expansion
```

## Cookpit coordination boundary

Formula is being integrated with the shared **THETECHGUY Project Cookpit** used for long-running engineering projects.

Cookpit is coordination and evidence collection only. It is **not Formula authority**.

Formula authority remains the frozen design, exact Git/source identities, independent checkers, certified Evidence, Promotion, Universe generations, and First-Light proof law.

Cookpit must never:

```text
self-declare Formula PROVEN
merge or force-push protected main
create mathematical authority from worker success
bypass independent checker/certificate requirements
promote candidate mathematics
rewrite frozen architecture/history
weaken an Authority Contract because a worker timed out or failed
```

### Formula Cookpit target layout on KRATOS

The Cookpit setup deliberately defines a new contained construction plane; it does not claim this path existed previously:

```text
/home/kratos/.oracle-work/formula-cookpit/
  authority/   # frozen architecture checkout bound to 50c0beb...
  worktree/    # active Formula construction worktree
```

Project id:

```text
formula
```

Repository:

```text
jaydumisuni/formula
```

Cadence class:

```text
active_fast
micro proof lane: 5 minutes
heavy proof lane: 20 minutes
reasoning wake target: 1 hour
daily review: exception-only
```

The Cookpit profile is merged on `jaydumisuni/cookpit` `main` as:

```text
profiles/formula.json
```

The deterministic KRATOS activation package is also merged on Cookpit `main`:

```text
scripts/activate_formula_kratos.sh
docs/FORMULA.md
Cookpit merge commit: 3052f2dc598a82f68dd5505ee251b939c57939a7
```

On KRATOS the host activation entrypoint is:

```bash
bash scripts/activate_formula_kratos.sh
```

The package creates/verifies the frozen authority checkout and `impl/p0-first-light` construction worktree, validates the Formula profile, provisions/verifies Rust `1.98.0` through an existing `rustup`, generates/enables the 5-minute and 20-minute user timers, and runs the initial micro proof. It fails closed and writes Cookpit attention state if required host capabilities are missing.

A merged repo-side activation package is **not proof that KRATOS activation has executed**. Local activation is complete only after verified worktrees, exact toolchain availability, active timers, and observed Cookpit evidence exist on KRATOS.

## Worker/proof reuse law

Cookpit evidence may be reused only when its exact bound Git HEAD/authority inputs remain unchanged.

A worker PASS means only that the declared worker gates passed for that exact state. It does not mean the corresponding Formula mathematical milestone is promoted or authoritative unless Formula's own milestone proof law says so.

Stale proof fails closed.

## Research policy

Broad research is closed as the lead activity.

Reopen research only for a concrete implementation/design uncertainty allowed by the frozen roadmap, including the roadmap-triggered targeted spikes. Do not restart open-ended technology collection while P0–P9 are implementable from frozen authority.

## Recovery procedure for a new reasoning session

1. Read this file.
2. Inspect current repository/branch/worktree HEADs before making any completion claim.
3. Recover `docs/design/README.md`, the D1–D5 canonical files, First-Light specification, and P0–P13 roadmap.
4. Treat commit `50c0beb021d8bf02d59a177049b9c2cf1783b26a` as the frozen architecture checkpoint, not necessarily the latest implementation HEAD.
5. Recover Cookpit evidence for the exact current construction HEAD when available.
6. Identify the latest **proven** P-stage from repository proof evidence.
7. Continue from the first unproven obligation in that stage; do not redo unchanged proof without cause.
8. If worker evidence is stale, failed, unavailable, or bound to another HEAD, fail closed and regenerate the required evidence.
9. Do not allow Cookpit, chat memory, models, compiler output, or search output to manufacture Formula authority.
10. Ptah remains deferred until the local canonical First-Light proof passes and the roadmap reaches its explicit Ptah integration phase.

## Evidence precedence

When sources disagree, use this order:

```text
current implementation + independently replayable proof evidence
    > later frozen design amendment explicitly superseding an older milestone
    > D5 self-expansion authority
    > D4 native execution authority
    > D3 compiler/campaign authority
    > D2 core system authority
    > D1/D1A constitutional authority
    > canonical First-Light specification
    > frozen implementation roadmap
    > preserved precursor/research evidence
    > Cookpit observational evidence
    > chat recollection
```

Cookpit can tell us what was executed and what needs attention. Formula itself decides what is mathematically authoritative.
