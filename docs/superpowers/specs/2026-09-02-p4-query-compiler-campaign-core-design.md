# P4 Design — Query, Theory Profile, Campaign IR, Obligation Compiler

**Date:** 2026-09-02  
**Status:** DESIGN FOR REVIEW  
**Branch:** `implementation/p4-query-compiler-campaign-core`  
**Exact predecessor:** `5c15368440ad9cc387708dae3c3d73135009f053`  
**Roadmap authority:** `docs/roadmap/2026-08-28-implementation-roadmap.md`, phase P4  
**Design authority:** canonical D3 Mathematical Compiler Architecture

---

## 1. Purpose

P4 implements the deterministic compiler/campaign front end needed between the proved P3 semantic package/capability substrate and the later P5 discovery engines.

P4 converts an exact mathematical query into a replayable, typed, authority-inert attack graph. It does **not** perform open-ended discovery, CandidateSpace refinement, solver search, promotion, native realization, or First-Light execution.

The constitutional boundary is:

```text
proved mathematical authority inputs
        ↓
QueryIR / semantic compilation
        ↓
TheoryProfile / RelevantRegion
        ↓
Representation / Reduction / Decomposition contracts
        ↓
CampaignIR / ObligationIR / WorkCellPlan
        ↓
replayable structural plan
```

No object produced by P4 can create or weaken mathematical authority.

---

## 2. Crate boundary

P4 lives in `crates/formula-engine`.

Existing role declaration already reserves this crate for:

```text
query campaign and search orchestration
```

P4 must preserve the existing dependency direction:

```text
formula-engine
├── formula-core
├── formula-store
└── formula-packages

formula-engine -/-> formula-check implementation
```

`formula-core` remains the durable authority/schema substrate. P4 does not move campaign/search state into core.

No new crate is introduced in P4.

No new external runtime dependency is introduced unless implementation evidence proves one is necessary. The default plan uses only the existing workspace dependency closure.

---

## 3. P4 module structure

`formula-engine` is split into focused modules:

```text
query.rs
region.rs
theory_profile.rs
representation.rs
reduction.rs
decomposition.rs
campaign.rs
obligation.rs
work_cell.rs
replay.rs
result_bundle.rs
compiler.rs
```

Each module owns one semantic responsibility and exposes immutable, structurally comparable data where replay identity matters.

`lib.rs` exports only the stable P4 API.

---

## 4. QueryIR

`QueryIR` binds the exact semantic inputs to compilation:

```text
QueryIR {
    universe_generation
    world
    known_bindings
    metavariables
    target_judgements
    observer
    authority_contract
    resource_contract
    side_effect_policy
    activated_package_set
}
```

P4 treats these fields as semantic inputs, not routing hints.

Required behavior:

- exact generation and World binding;
- observer identity preserved exactly;
- Authority Contract preserved exactly;
- resource limits represented separately from mathematical truth;
- activated package set bound explicitly;
- unknown artifact classes explicit rather than inferred from operation names.

No timeout/resource state may rewrite the requested Authority Contract.

---

## 5. Semantic elaboration

Semantic elaboration converts a `QueryIR` into explicit semantic prerequisites and open obligations.

P4 may:

- resolve exact structural identities;
- resolve admitted parents/domains;
- use admitted canonical lossless morphisms;
- detect ambiguity and emit an open/blocked obligation;
- recover admitted structure witnesses and capabilities from P3;
- normalize observer and authority semantics;
- create explicit missing-structure obligations.

P4 may **not**:

- insert a lossy morphism implicitly;
- silently choose among ambiguous common parents;
- synthesize a missing theorem/witness in P4;
- treat a heuristic estimate as a semantic fact.

This directly enforces D3-P01 and D3-P02.

---

## 6. RelevantRegion

`RelevantRegion` is a deterministic projection of the exact generation/world/package context into the semantic artifacts needed for the current query.

Conceptual fields:

```text
RelevantRegion {
    universe_generation
    world
    query_digest
    semantic_artifacts
    admitted_capabilities
    morphism_edges
    representation_edges
    reduction_edges
    retrieval_provenance
}
```

P4 v1 keeps retrieval deliberately bounded and deterministic. It uses only already-admitted/indexed semantic inputs available through P1/P3 interfaces.

P4 does not implement heuristic theorem retrieval or learned relevance ranking.

The region is expandable in later phases, but replay identity must bind the exact region/provenance used for a verdict.

---

## 7. TheoryProfile v1

`TheoryProfile` separates certified semantic properties from operational estimates.

```text
TheoryProfile {
    exact_properties
    available_capabilities
    supported_certificate_families
    admissible_representation_classes
    admissible_reduction_classes
    operational_estimates
}
```

Rules:

- only admitted/certified properties constrain correctness;
- operational estimates may rank routes but cannot discharge an obligation;
- absence of a property is not equivalent to its negation;
- capability availability is scoped to the exact P3 closure context.

P4 v1 supports only the profile facts needed by First-Light-era compiler tests. It does not attempt universal mathematical classification.

---

## 8. Representation contracts

Representation changes are explicit graph edges.

```text
RepresentationNode {
    semantic_target
    representation
    world
    exactness_class
    observer_binding
}

RepresentationEdge {
    source
    target
    transformation
    preservation_class
    information_loss
    reconstruction_route
    certificate_route
    assumptions
}
```

P4 rejects any representation edge lacking explicit preservation metadata.

A lossy representation cannot claim full-witness preservation without an admitted reconstruction contract.

D3-P03 is therefore structural, not conventional documentation.

---

## 9. Reduction contracts

`ReductionEdge` declares exactly which result classes survive a reduction.

P4 v1 supports a bounded preservation set:

```text
DECISION
WITNESS
COUNT
OPTIMUM
BOUND
```

Each edge carries:

```text
source_class
target_class
preserved_result_classes
encode_relation
decode_or_reconstruct_relation
assumptions
evidence/certification reference
```

Composition is permitted only when the requested result class is preserved by every edge in the composed path.

Examples:

- decision-only + decision-only may serve a decision query;
- decision-only cannot serve a counting query;
- witness preservation requires reconstruction semantics;
- a reduction with missing preservation evidence fails closed.

This enforces D3-P04.

---

## 10. Decomposition

`Decomposition` makes child obligations and reconstruction semantics explicit.

```text
Decomposition {
    parent_goal
    child_obligations
    interface_or_separator
    aggregation_semantics
    reconstruction_relation
    evidence
}
```

P4 v1 supports deterministic decomposition descriptions supplied by admitted semantic capabilities. It does not search for decompositions yet.

No decomposition is admissible without explicit reconstruction/aggregation semantics.

This enforces D3-P05.

---

## 11. CampaignIR

P4 compiles a typed AND/OR campaign graph.

Minimum node families:

```text
Goal
Route
Obligation
WorldRef
ArtifactRef
FactRef
WorkCellPlan
Result
```

Minimum edge families:

```text
REQUIRES
PRODUCES
SATISFIES
REFUTES
ALTERNATIVE_TO
DECOMPOSES_INTO
REDUCES_TO
TRANSPORTS_TO
UNLOCKS
```

Minimum aggregation semantics:

```text
AND
OR
```

P4 deliberately defers MIN/MAX/fixpoint/package-defined aggregation until implementation evidence requires them.

CampaignIR is planning/search state. It is never durable mathematical authority merely because it is well-formed.

---

## 12. ObligationIR

`ObligationIR` is the fundamental unit of mathematical work.

```text
ObligationIR {
    obligation_digest
    universe_generation
    world
    semantic_prerequisites
    target_family
    observer
    required_authority
    admissible_capabilities
    dependencies
    resource_contract
    terminal_state_policy
}
```

P4 freezes the following terminal states as distinct enums:

```text
SATISFIED
REFUTED
CERTIFIED_BOUND
SEMANTIC_UNKNOWN
RESOURCE_BOUNDED_UNKNOWN
UNDECIDABLE_GENERAL_CLASS
SUPERSEDED
BLOCKED_BY_AUTHORITY
```

Critical law:

```text
RESOURCE_BOUNDED_UNKNOWN != SEMANTIC_UNKNOWN != REFUTED
```

A timeout/resource ceiling can never become a mathematical negative result.

This enforces D3-P12.

---

## 13. WorkCellPlan

`WorkCellPlan` is an authority-inert execution request.

```text
WorkCellPlan {
    obligation_digest
    semantic_inputs
    allowed_packages
    allowed_capabilities
    evidence_requirement
    resource_budget
    deterministic_replay_key
    checkpoint_policy
    side_effect_limits
    stop_conditions
}
```

P4 Work Cells are **plans only**. They do not execute external solvers in P4.

The data type exposes no `AuthorityStore` mutation API and no method that can weaken the root Authority Contract.

`formula-engine` continues not to depend on `formula-check` implementation.

This enforces D3-P09 and keeps Ptah absence semantically irrelevant under D3-P13.

---

## 14. ReplayManifest

Every compiled campaign has a deterministic replay manifest binding all semantic and policy inputs that may affect its structure/verdict.

Minimum fields:

```text
universe_generation
world
query_digest
activated_package_set
relevant_region_digest
theory_profile_digest
compiler_policy_version
scheduler_policy_version
resource_contract
random_seed_or_key
campaign_digest
```

P4 v1 uses deterministic construction and does not require randomized search, but the seed/key field is frozen now so future adaptive phases cannot silently make verdict replay underspecified.

Changing any semantic/policy input changes replay identity.

Recompiling identical exact inputs produces identical campaign/replay identity.

This enforces D3-P11.

---

## 15. ResultBundle

P4 defines the structural result envelope without creating mathematical authority.

```text
ResultBundle {
    query_digest
    campaign_digest
    terminal_state
    observer_result_refs
    evidence_refs
    certified_bounds
    counterexample_refs
    unresolved_obligations
    promotion_candidate_refs
    provenance
}
```

A `ResultBundle` may reference evidence already certified elsewhere, but constructing the bundle itself does not certify that evidence.

---

## 16. Compiler v1

The P4 compiler performs deterministic structural compilation only:

```text
QueryIR
  -> validate exact context
  -> semantic elaboration
  -> RelevantRegion
  -> TheoryProfile
  -> validate available representation/reduction/decomposition routes
  -> build AND/OR CampaignIR
  -> create ObligationIR nodes
  -> create WorkCellPlan placeholders for runnable obligations
  -> produce ReplayManifest
```

P4 v1 does **not**:

- enumerate CandidateSpaces;
- perform CEGIS;
- synthesize new representations/reductions;
- run solvers;
- promote discoveries;
- compile native code;
- call models;
- require Ptah.

Those belong to later phases.

---

## 17. Structural rejection rules

P4 must fail closed for at least:

```text
World mismatch
Universe generation mismatch
Authority Contract mismatch
Observer mismatch
activated-package context mismatch
implicit lossy morphism
ambiguous common parent
representation edge with missing preservation metadata
reduction path that loses requested result class
reduction missing reconstruction for requested witness
invalid decomposition without reconstruction semantics
WorkCell plan attempting authority-write capability
replay manifest missing semantic/policy input
```

Failures remain compiler/structural errors unless an authoritative mathematical judgement already establishes a negative result.

---

## 18. Tests and proof obligations

P4 test groups map directly to the frozen roadmap obligations.

### D3-P01 Query semantic preservation

Prove generation, World, Observer, Authority, packages, known bindings, and target remain identical through compilation/replay.

### D3-P02 No lossy implicit morphism

Prove canonical lossless morphism may elaborate; lossy or ambiguous morphism fails closed unless explicit route semantics are supplied.

### D3-P03 Representation metadata mandatory

Reject representation edges missing preservation/information-loss semantics.

### D3-P04 Reduction result-class preservation

Reject decision-only route for witness/count/optimum requests; accept only classes preserved across the full path.

### D3-P05 Decomposition reconstruction explicit

Reject decomposition with missing aggregation/reconstruction semantics.

### D3-P09 Work Cells cannot modify authority

Dependency/API tests prove `formula-engine` has no checker/store authority mutation route through WorkCellPlan construction.

### D3-P11 Replay complete

Identical exact inputs replay to identical campaign/replay identities; changing generation, World, package context, policy, observer, authority, or resource contract changes the replay binding where semantically required.

### D3-P12 terminal-state distinction

Prove `REFUTED`, `SEMANTIC_UNKNOWN`, and `RESOURCE_BOUNDED_UNKNOWN` are distinct and cannot be coerced into one another.

### Gate P4

PASS only when:

1. identical exact inputs compile/replay to the same semantic campaign identity;
2. invalid representation/reduction/decomposition compositions are structurally rejected;
3. P0/P1/P2/P3 proof gates remain green;
4. the canonical P4 workflow passes locked/offline tests, build, formatting, Clippy `-D warnings`, dependency-tree checks, and clean-worktree verification.

---

## 19. Implementation sequence

The implementation plan should use RED -> GREEN tasks in this order:

```text
1. QueryIR + exact semantic identity
2. RelevantRegion + TheoryProfile
3. representation contracts
4. reduction contracts + composition validation
5. decomposition contracts
6. CampaignIR AND/OR graph validation
7. ObligationIR + terminal-state separation
8. WorkCellPlan authority-inert boundary
9. ReplayManifest deterministic binding
10. Compiler v1 end-to-end deterministic compile/replay
11. adversarial invalid-route integration tests
12. canonical P0-P4 proof workflow
13. exact P3->P4 diff review
14. checkpoint + CURRENT update
15. post-checkpoint exact-head proof
```

No later task may silently pull P5 CandidateSpace/discovery behavior into P4.

---

## 20. Freeze boundary

P4 is complete only when the exact documentation-bearing branch SHA independently passes the canonical P4 workflow.

The checkpoint must record:

```text
exact P3 predecessor
source-under-test SHA
canonical source proof run/job
P4 proof markers
exact P3->P4 scope review
checkpoint commit
CURRENT update
post-checkpoint proof run/job for exact documentation-bearing SHA
```

No merge to `main` is implied by P4 completion.

P5 must not begin before P4 is frozen and proved.
