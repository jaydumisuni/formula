# Incremental Proof and Certificate Repair Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

The project already has:

- immutable/generation-scoped mathematical truth;
- dependency/freshness tracking;
- incremental mathematical computation;
- cached certificates;
- theory morphisms/equivalences;
- proof/certificate families.

The missing question was what happens after a small semantic change:

> Must every affected certificate be thrown away and reproved from zero, or can previous proof structure be reused and repaired?

Evidence from SAT, SMT/program verification, and proof assistants shows that **incremental certification and genuine proof repair are feasible**.

## 1. Incremental SAT certification

Modern applications often solve a sequence of related SAT problems under changing clause sets and temporary assumptions.

Recent work introduced explicit proof formats/checkers for certifying these incremental queries rather than flattening every query into an unrelated one-shot proof.

### Incremental SAT with inprocessing

Kiesl-Reiter and Whalen showed how incremental SAT solvers using inprocessing can still emit independently verifiable UNSAT proofs. Clause-restoration steps are recorded during the incremental run and postprocessed into a standard DRAT proof.

Sources:
- https://assets.amazon.science/0f/27/487a4cb24557b9a5d8d4613af264/proofs-for-incremental-sat-with-inprocessing.pdf
- https://www.amazon.science/blog/proving-that-solutions-to-incremental-satisfiability-problems-are-correct

The reported implementation produced valid proofs for all UNSAT examples in a 300-instance evaluation with modest proof-generation overhead.

### IDRUP / LIDRUP

2024 work on certifying incremental SAT introduced IDRUP/LIDRUP-style formats that explicitly capture:

- incremental clause additions/deletions;
- per-query assumptions;
- SAT assignments;
- UNSAT derivations;
- unsatisfiable cores;
- failed assumptions;
- antecedent/resolution hints.

Sources:
- https://easychair.org/publications/paper/TbPs
- https://kfazekas.github.io/papers/FazekasPollittFleuryBiere-LPAR24.pdf

This establishes a key principle:

> **The certificate stream itself can be incremental and generation-aware.**

The authority layer does not need to flatten every related query into a new unrelated proof object.

## 2. Parallel/incremental proof checking

A formally verified LRAT UNSAT checker has been implemented so it can process proof output in parallel with the SAT solver rather than waiting for the whole proof to finish.

Source:
- https://link.springer.com/chapter/10.1007/978-3-031-63498-7_26

This suggests future Mathematical Work Cells can stream certification evidence while computation is still running:

```
producer
   ↓ proof/certificate segments
incremental checker
   ↓
validated prefix/state
```

A long campaign may therefore discover an invalid branch early instead of completing an enormous uncheckable artifact first.

## 3. Certificate caching in theorem-prover integrations

Isabelle's SMT integration supports persisted SMT certificate files so repeated calls using the same configuration can reuse stored certificates instead of invoking the external SMT solver again.

Source:
- https://isabelle.in.tum.de/website-Isabelle2025/dist/library/HOL/HOL-Proofs/HOL.SMT.html

This confirms the practical distinction between:

```
proof search cost
```

and

```
proof replay/check cost
```

that the project already uses architecturally.

## 4. Dependency-based incremental proof checking

Large proof systems can track theorem/definition dependencies and reverify only the transitive dependency cone affected by a change. ImandraX documents an incremental proof architecture where unchanged definitions/theorems are cached and independent affected proofs can run in parallel.

Source:
- https://imandrax.dev/docs/verification/incremental-proofs/

This is the basic lowest-cost tier:

```
changed semantic object
      ↓
compute dependency cone
      ↓
all unrelated certificates remain current
```

The project should never invalidate globally when exact dependency information permits local invalidation.

## 5. Summary/certificate repair after program changes

Research on incremental verification through **summary repair** starts with:

```
old program P1
old correctness summary/certificate σ1
changed program P2
```

and attempts to adapt `σ1` into a valid summary `σ2` for the new program, falling back to counterexample/fresh verification only when repair cannot establish correctness.

Source:
- https://doi.org/10.1007/s10703-023-00423-0

This is highly relevant to the project because many mathematical certificates are also summaries/invariants:

- loop invariants;
- abstractions;
- interval bounds;
- decomposition summaries;
- compiled knowledge representations;
- relation interfaces.

A local semantic change may admit a repaired certificate instead of a total rebuild.

## 6. Proof repair in Coq/Rocq

Talia Ringer and collaborators developed proof-repair approaches that treat proof maintenance as change-aware proof transformation rather than fresh proof search.

### Repair using historical patches

Earlier PUMPKIN PATCH work analyzes changes to specifications/proofs and searches for analogous patches that can be applied to broken proofs.

Source:
- https://doi.org/10.1145/3167094

### Transport across type equivalences

PUMPKIN Pi automatically transforms proof terms when types change by a certified equivalence, then decompiles the repaired proof term back toward user-facing tactics where useful.

Sources:
- https://rocq-prover.org/papers/proof-repair-across-type-equivalences
- https://doi.org/10.1145/3453483.3454033

The transformation removes references to the old type and transports the proof across the equivalence without adding axioms beyond those accepted by Coq.

### Quotient-type equivalence repair

Later work extends this idea to quotient/setoid-style equivalences, enabling some behavior/representation changes previously outside the repair system.

Source:
- https://arxiv.org/abs/2310.06959

This is extremely important to the project because it provides a direct mathematical form of proof repair:

```
old structure A
      ↓ certified equivalence/morphism E
new structure B

old proof over A
      ↓ transport(E)
new proof over B
```

The proof is not merely textually patched. It is **transported through mathematics**.

## 7. Architecture-changing conclusion: three repair tiers

The project should distinguish at least three increasingly expensive responses to a semantic change.

### Tier 0 — reuse unchanged authority

```
certificate node dependencies unchanged
      ↓
retain certificate as current
```

No checking/search beyond freshness validation is necessary.

### Tier 1 — transport/repair

When the change has a certified relationship to the previous semantics:

```
equivalence
isomorphism
theory morphism
refinement
conservative extension
change action
summary update
```

attempt to transform the old certificate into a certificate for the new generation.

### Tier 2 — fresh proof/search

Only when repair is unavailable, fails, or would cost more than regeneration:

```
launch new proof/certificate campaign
```

This mirrors incremental computation's `delta vs rebuild` policy.

## 8. Proof delta as a first-class artifact

Potential object:

```
ProofDelta {
    old_claim_digest,
    new_claim_digest,
    old_certificate_digest,
    semantic_change,
    transport_or_patch,
    newly_required_obligations,
    discharged_obligations,
    checker,
    result_certificate_digest
}
```

A repaired proof should not overwrite history.

Instead:

```
old generation proof
     +
certified proof delta
     ->
new generation proof
```

The full authority chain remains replayable.

## 9. Proof repair through morphisms

The earlier theory-transfer research already established that a certified theory morphism can transport derivable mathematics.

This pass adds the operational consequence:

> **the same morphism can be used as a proof-maintenance primitive when the representation/theory evolves.**

So `transport theorem library` and `repair stale proof` may share machinery.

Potential metaprimitive:

```
TRANSPORT_CERTIFICATE(
    certificate,
    semantic_morphism,
    target_claim
)
```

with generated proof obligations for anything the morphism does not preserve automatically.

## 10. Repair of compiled/certificate representations

The principle extends beyond theorem proof terms.

Examples:

### Certified d-DNNF/POG

Small CNF change:

```
old compiled graph
      ↓
repair affected subgraph / proof nodes
      ↓
recheck equivalence locally
```

rather than recompile from zero, when supported.

### Optimization

Change one coefficient/constraint:

```
old incumbent/bound/cuts/certificate
      ↓
retain unaffected valid bounds/cuts
      ↓
repair invalidated proof region
```

### Interval/numerical proof

Changed parameter/domain:

```
reuse old subdivision/enclosure tree
      ↓
revalidate affected boxes only
```

### E-graph/equality proof

New/removed assumption:

```
reuse generation/version structure
      ↓
recompute only context-dependent merges/extractions
```

These are research obligations, but the common architecture is now clear.

## 11. Repair versus weakening/strengthening

The change class strongly affects reuse.

Examples:

```
stronger assumptions
    may preserve prior theorem proof directly

weaker assumptions
    may invalidate proof and require new obligations

stronger theorem conclusion
    requires additional proof

weaker theorem conclusion
    old proof may imply new claim

conservative definition refactor
    may admit equivalence transport

non-conservative semantic change
    may require substantial fresh proof
```

This should connect directly to the package semantic-version classes already researched.

## 12. Search-economy implication

For a stale certificate, the scheduler should compare:

```
Cost(replay unchanged nodes)
Cost(repair/transport)
Cost(fresh proof)
```

and choose the cheapest admissible authority route.

Historical repair success can influence route ordering, but never whether an invalid repair is accepted.

## 13. Durable proof graph versus disposable proof-search state

This pass reinforces the earlier separation:

### Durable

```
claims
proof DAG
certificates
morphisms
assumptions
semantic generations
proof deltas
```

### Disposable/rebuildable

```
solver heuristics
clause activities
search queues
proof-search tactics
temporary lemmas not promoted
```

Repair works primarily over durable semantic/proof artifacts, not volatile solver internals.

## 14. Core law

> **A changed mathematical world should invalidate only what the change actually breaks; if a certified semantic relation connects the old and new worlds, transport or repair proof before searching again from zero.**

## 15. Open research

1. Incremental Alethe/SMT proof formats and theory-lemma reuse under changing assumptions.
2. Repair of heterogeneous certificate envelopes spanning multiple solver families.
3. Incremental checking/repair of CPF/CeTA, VIPR, SOS, Gröbner, and interval certificates.
4. Generic proof-delta semantics independent of one proof assistant.
5. Automatic semantic differencing between two mathematical generations.
6. Cost models for transport versus repair versus complete reproving.
7. Proof repair through theory morphisms, equivalences, quotient maps, and refinements.
8. Incremental reconstruction of proof-minimized cores/interpolants after small changes.
9. Parallel proof repair over independent dependency cones.
10. Certified garbage collection of obsolete proof-search artifacts while preserving authority lineage.
11. Whether proof repair can itself become a self-learned metaprimitive family from recurring change patterns.
