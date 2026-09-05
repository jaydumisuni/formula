# P10 Self-Expansion Hardening Design

**Date:** 2026-09-05  
**Status:** implementation design derived from frozen D5 + roadmap P10  
**Branch:** `implementation/p10-self-expansion-hardening`  
**Exact frozen P9 proof predecessor:** `b353365fa8b20a13b658c07b3027334b69eff108`  
**P9 canonical proof:** run `33950470295`, job `101264153162`, conclusion `success`  
**Authority:** `docs/design/2026-08-28-d5-self-expansion-architecture.md`, `docs/roadmap/2026-08-28-implementation-roadmap.md`, frozen P9 proof/checkpoint authority.

## 1. Goal

P10 generalizes the D5 promotion and evolution machinery beyond the one bounded semantic primitive used by First Light.

The canonical P10 proof is intentionally a **non-primitive** promotion:

```text
U_g has Rational package active
U_g lacks goal:rational:field witness
therefore cap:rational:field is unavailable
    |
    v
freeze StructureWitness candidate
independently check evidence
classify as STRUCTURE_WITNESS
Promotion -> U_(g+1)
    |
    v
replay admitted witness under exact world/generation
recompute capability closure
    |
    v
cap:rational:field becomes available
without adding or changing solver/package code
```

That directly satisfies roadmap P10's proof gate: a non-primitive promotion produces measurable future capability change using an already-existing generic capability contract.

P10 also supplies the bounded infrastructure required by the roadmap for promotion-class registration, nogood/reduction/morphism activation metadata, semantic change classification, generation-bound discovery grammar identity, staged metaprimitive activation, proof transport/repair planning, realization-only upgrades, and rollback/supersession.

## 2. Constitutional boundaries

P10 preserves these laws exactly:

1. Search/discovery can propose a promotable artifact but cannot authorize it.
2. Existing P7 `PromotionCandidate` and `PromotionAuthorization` identities are not rewritten; P10 wraps them with additional class/effect identity so P7-P9 frozen digests remain reproducible.
3. Mathematical admission remains an immutable `UniverseGeneration` transition through the existing checker-authorized promotion path.
4. Capability closure remains derived from admitted authority; P10 does not add mutable caller-side capability injection.
5. `Lambda_g` is derived, generation-bound discovery grammar identity. It cannot create authority.
6. CandidateSpace identity remains bound to the exact grammar/routes digest used to construct it.
7. Metaprimitives may be admitted while remaining `SHADOW_ONLY`; they cannot become default automatic because a producer asks for it.
8. Proof transport/repair produces new evidence identities and requires independent rechecking before authority can consume the result.
9. `REALIZATION_ONLY` changes cannot mutate mathematical truth or force a new semantic generation merely because a faster binary exists.
10. Rollback changes active selection/activation and preserves all historical generations/artifacts.
11. P10 does not add P11 external federation breadth, P12 bootstrap self-hosting, P13 Ptah integration, GPU/SIMD/JIT work, or a universal primitive-worth metric.

## 3. Existing implementation surfaces to preserve

P10 extends existing contracts instead of replacing them:

- `formula-core::promotion::PromotionCandidate` freezes semantic candidate identity and dependency/supersession cone.
- `formula-check::promotion::authorize_promotion_v1` independently authorizes the exact frozen promotion and returns opaque `PromotionAuthorization`.
- `AuthorityStore::promote` performs atomic history-preserving `U_g -> U_(g+1)` publication.
- `StructureWitness`, `CapabilityContract`, and `TheoryPackageManifest` already model the non-primitive capability relation.
- `AdmittedStructureWitness` rejects witnesses not admitted or lacking authority-bound evidence.
- `derive_capabilities` already unlocks package capabilities only when every required structure goal is admitted for the exact world/generation.
- `CandidateSpaceContext` already binds `grammar_or_routes_digest`; P10 supplies a first-class generation-derived `Lambda_g` identity for this field.
- P8 realization admission/dispatch and P9 semantic activation stores remain authoritative predecessors and are not weakened.

## 4. P10 structural identities

P10 adds `formula-core::self_expansion` with schema `formula-self-expansion-v1`.

### 4.1 `PromotionClass`

The registry covers the frozen D5 classes:

```text
THEOREM_JUDGEMENT
STRUCTURE_WITNESS
COUNTEREXAMPLE_NOGOOD
INVARIANT_CERTIFIED_BOUND
REPRESENTATION
REDUCTION
MORPHISM_THEORY_INTERPRETATION
DECOMPOSITION_SUFFICIENT_SUMMARY
SEMANTIC_PRIMITIVE
CAPABILITY
METAPRIMITIVE_SEARCH_METHOD
REALIZATION
PACKAGE_THEORY_EXTENSION
TOOLCHAIN_CHECKER_REALIZATION
```

Class identity is explicit because downstream activation/revalidation semantics differ by class.

### 4.2 `ActivationMode`

```text
MANUAL_ONLY
SHADOW_ONLY
BOUNDED_AUTOMATIC
DEFAULT_AUTOMATIC
SUPERSEDED
QUARANTINED
```

Activation is separate from admission. `SUPERSEDED` and `QUARANTINED` are fail-closed selection modes, not deletion operations.

### 4.3 `SemanticChangeClass`

```text
REALIZATION_ONLY
DEFINITIONAL_EQUIVALENT
CONSERVATIVE_EXTENSION
THEOREM_STRENGTHENING
ASSUMPTION_WEAKENING
SIGNATURE_CHANGE
NON_CONSERVATIVE_CHANGE
AUTHORITY_POLICY_CHANGE
```

Textual diff size is excluded from identity. Revalidation is driven by semantic class and dependency cone.

### 4.4 `EvidenceFreshness`

```text
UNCHANGED_FRESH
TRANSPORTABLE
REPAIRABLE
RECHECK_REQUIRED
REPROVE_REQUIRED
QUARANTINED
```

This is a classification result, not a proof itself.

### 4.5 `SupersessionKind`

```text
SUPERSEDED_BY
REFUTED_BY
REPLACED_REALIZATION_BY
WITHDRAWN_FROM_DEFAULT_ACTIVATION
```

### 4.6 `ClassifiedPromotionCandidate`

P10 does **not** alter the P7 `PromotionCandidate` schema. It adds a wrapper:

```text
ClassifiedPromotionCandidate {
    base_promotion_candidate
    class
    requested_activation_mode
    semantic_change_class
    activation_effects[]
    grammar_effects[]
    scope[]
}
```

Every list is canonical sorted/deduplicated. The wrapper is the P10 class/effect identity; the referenced P7 base candidate remains the exact object checked for authority publication.

### 4.7 `PromotionClassPolicy`

A deterministic class registry defines, per class:

```text
may_change_universe
may_change_capability_closure
may_change_grammar
may_change_realization_selection
allowed_activation_modes
requires_shadow_gate
```

The registry is code/static policy with its own digest, not mutable database configuration in P10.

Important rules:

- `STRUCTURE_WITNESS` may change U and capability closure but not grammar directly.
- `COUNTEREXAMPLE_NOGOOD` may change U and bounded pruning activation; applicability scope is mandatory.
- `REDUCTION` / `MORPHISM_THEORY_INTERPRETATION` may change U, capability reachability, and route grammar only after preservation/evidence checks.
- `METAPRIMITIVE_SEARCH_METHOD` may change grammar but may initially activate only `MANUAL_ONLY` or `SHADOW_ONLY` unless a strict metaprimitive gate authorization is present.
- `REALIZATION` uses semantic class `REALIZATION_ONLY` for P10's realization-only upgrade route and cannot add mathematical admissions through that route.

## 5. Class-specific authorization

`formula-check::self_expansion` introduces a second authorization layer **after** ordinary promotion authorization.

```text
base PromotionAuthorization
 + ClassifiedPromotionCandidate
 + exact parent generation
 + class-specific checked evidence/scope
 + registry policy
      |
      v
ExpansionAuthorization
```

`ExpansionAuthorization` is opaque outside `formula-check`. It binds:

```text
base promotion candidate digest
classified candidate digest
class
parent generation
requested activation mode
semantic change class
activation effects
grammar effects
scope
registry policy digest
```

The checker rejects:

- classified wrapper referencing a different base promotion candidate;
- activation mode forbidden for the class;
- grammar effects from a class that may not change grammar;
- capability effects from a class that may not change capability closure;
- `REALIZATION_ONLY` coupled to new semantic admissions through the realization-only route;
- unscoped automatic nogood activation;
- default/automatic metaprimitive activation without strict gate evidence;
- dependency/scope bindings that do not belong to the exact parent generation/evidence set.

Ordinary P7 authorization remains the sole authorization consumed by `AuthorityStore::promote` for mathematical publication. P10 authorization proves that the additional class/effect semantics are admissible.

## 6. Promotion-class registry

`PromotionClassRegistryV1` is deterministic and complete for all frozen D5 classes. Missing/unknown class policy is impossible because `PromotionClass` is a closed enum.

The registry exposes:

```text
policy(class) -> PromotionClassPolicy
registry_digest() -> ArtifactDigest
```

Tests prove every enum variant has exactly one policy and the registry digest is deterministic independent of iteration order.

The registry is intentionally not a plugin registry in P10. Dynamic external package/class policy belongs to later breadth phases.

## 7. Structure-witness promotion and canonical P10 capability proof

P10's canonical non-primitive proof uses the existing Rational package.

The package already has:

```text
cap:rational:field requires goal:rational:field
```

Canonical sequence:

1. Create/replay `U_g` admitting the Integer and Rational package manifests and activation evidence required by the existing package activation contract.
2. Activate the package set under a fixed P10 world.
3. Derive `closure_before` with no `goal:rational:field` witness and prove `cap:rational:field` is absent.
4. Freeze a `StructureWitness(world, goal:rational:field, evidence)` as the semantic candidate.
5. Build ordinary P7 frozen/promotion manifests and obtain ordinary checker `PromotionAuthorization`.
6. Wrap that promotion as `PromotionClass::StructureWitness` and obtain P10 `ExpansionAuthorization`.
7. Publish `U_(g+1)` only through `AuthorityStore::promote`.
8. Reconstruct `AdmittedStructureWitness` from `U_(g+1)`; this must fail if the witness is absent or its evidence is not authority-bound.
9. Re-activate the unchanged package manifests for `U_(g+1)` and derive `closure_after`.
10. Prove `cap:rational:field` is now present and `CapabilityClosureDelta` adds it.
11. Prove the Rational package manifest digest and capability contract digest are unchanged across the transition.

This proves capability growth came from promoted mathematical structure, not from new solver/package implementation.

## 8. Nogood/counterexample activation

P10 introduces `ScopedNogoodActivation` as derived activation metadata:

```text
nogood_artifact
admitted_generation
world
applicability_scope[]
mode
checked_evidence[]
```

Rules:

- the nogood artifact must be admitted in the bound generation;
- evidence must be authority-bound;
- `BOUNDED_AUTOMATIC` requires non-empty exact applicability scope;
- no P10 API promotes a local counterexample to universal scope by heuristic generalization;
- activated nogoods expose a deterministic set of admissible pruning digests to CandidateSpace consumers, but they cannot mark a mathematical Judgement true/false themselves.

P10 tests prove empty-scope automatic activation is rejected and a scope mismatch does not prune an unrelated CandidateSpace.

## 9. Reduction/morphism promotion

P10 reuses existing `CanonicalMorphism` and D3 reduction contracts rather than inventing a second representation system.

A `PromotedRouteActivation` binds:

```text
route_artifact
class = REDUCTION or MORPHISM_THEORY_INTERPRETATION
source/target semantic digests
preserved result classes[]
reconstruction/translation evidence[]
generation
world/scope
mode
```

Automatic route activation requires the route artifact and evidence to be admitted/authority-bound under the exact generation. The derived active-route digest becomes an input to `Lambda_g`/route identity and therefore changes future CandidateSpace identity explicitly.

P10 proves a wrong generation, missing preservation evidence, or unsupported class fails closed.

## 10. Generation-bound discovery grammar `Lambda_g`

P10 introduces canonical `GrammarGeneration`:

```text
GrammarGeneration {
    universe_generation
    parent_grammar: Option<digest>
    activated_constructors[]
    activated_metaprimitives[]
    activated_route_rules[]
    activated_theory_rules[]
}
```

The digest is `Lambda_g`.

Derivation rules:

- only admitted + validly activated subjects contribute;
- `SHADOW_ONLY` metaprimitives are recorded in `shadow_metaprimitives[]` but do not enter authoritative active constructors/routes;
- `SUPERSEDED` / `QUARANTINED` subjects are excluded from active grammar;
- parent grammar is provenance; active contents are fully explicit and replayable;
- grammar derivation never changes `UniverseGeneration`.

CandidateSpace construction continues to use its existing `grammar_or_routes_digest`; canonical P10 tests bind it to exact `GrammarGeneration::structural_digest()`.

A CandidateSpace built under `Lambda_g` remains byte/digest distinct from one explicitly rebuilt under `Lambda_(g+1)`. No silent reinterpretation occurs.

## 11. Shadow-mode metaprimitive activation

P10 adds `MetaprimitiveGateEvidence` referencing independently checked evidence for:

```text
soundness
applicability domain
termination/finiteness where required
preservation/information-loss contract
negative/adversarial cases
transfer beyond discovery examples
comparison/shadow evidence
fallback/rollback contract
```

P10 does not claim a universal metaprimitive benchmark corpus.

Policy behavior:

- admission + `SHADOW_ONLY` is allowed when ordinary promotion/class authorization succeeds;
- `DEFAULT_AUTOMATIC` and `BOUNDED_AUTOMATIC` require a checked `MetaprimitiveGateAuthorization` whose evidence digests are authority-bound to the exact generation and whose scope covers the requested activation;
- shadow output cannot influence the authoritative campaign route or discharge Judgements;
- shadow evidence can later become input to a new promotion/activation decision.

The canonical P10 negative test requests default automatic activation without the strict gate and must fail.

## 12. Semantic change classification and dependency-cone revalidation

P10 adds `SemanticChange`:

```text
old_artifact
new_artifact
class
changed_dependencies[]
affected_authority_cone[]
evidence[]
```

`classify_freshness(change, evidence_dependency_cone)` returns the bounded D5 freshness state.

Rules for P10:

- `REALIZATION_ONLY` -> `UNCHANGED_FRESH` for semantic evidence when semantic contract digest is unchanged;
- `DEFINITIONAL_EQUIVALENT` / `CONSERVATIVE_EXTENSION` may be `TRANSPORTABLE` only when the exact certified transport relation is supplied;
- changes intersecting an evidence dependency cone without transport evidence become `RECHECK_REQUIRED` or `REPROVE_REQUIRED` according to class;
- `NON_CONSERVATIVE_CHANGE` and `AUTHORITY_POLICY_CHANGE` never silently transport;
- only the intersecting dependency cone is affected; unrelated evidence remains fresh.

This is deterministic framework classification. It does not fabricate transported proofs.

## 13. Proof transport and repair framework

P10 adds content-addressed plans, not a domain-specific theorem prover:

```text
ProofTransportPlan {
    source_evidence
    source_target
    destination_target
    certified_relation
    destination_dependencies[]
    required_checker
}

ProofRepairPlan {
    source_evidence
    semantic_change
    affected_slice[]
    repair_obligations[]
    required_checker
}
```

A plan is not authoritative evidence.

`TransportedEvidenceRecord` / `RepairedEvidenceRecord` may be created only from an opaque checker authorization and receive a new structural identity bound to the destination target/dependencies/checker result.

Tests prove simply editing/relabeling an old evidence digest cannot satisfy the authorization path.

## 14. Realization-only upgrade path

P10 keeps mathematical truth and realization evolution separate.

`RealizationUpgrade` binds:

```text
semantic_artifact
universe_generation
old_realization
new_realization
semantic_change_class = REALIZATION_ONLY
validation_evidence
selection_policy
```

The new realization must first pass the existing D4 realization authorization/admission path. P10 then records replacement/supersession selection metadata.

The route must prove:

```text
active UniverseGeneration digest unchanged
admitted semantic artifact unchanged
new realization admitted independently
selection may prefer new realization
old realization remains replay-addressable
rollback may reselect old realization
```

No P10 method is allowed to add a semantic artifact through `RealizationUpgrade`.

## 15. Rollback and supersession tooling

P10 adds append-only `SupersessionRecord` and explicit active-selection rollback APIs.

A supersession record binds:

```text
subject
successor/replacement
kind
source_generation
selection_generation_or_scope
evidence[]
```

Rollback rules:

- historical `UniverseGeneration` rows and content-addressed blobs are never deleted;
- selecting an older generation changes only the active pointer after exact replay validation;
- rolling back an activation changes selection state/mode, not the admitted theorem/artifact;
- rolling back a realization changes dispatch preference, not mathematical authority;
- quarantine prevents dependent automatic activation/closure use but preserves history.

`AuthorityStore` therefore gains a narrowly scoped `select_active_generation` operation requiring the target generation already exist and replay exactly. It may move backward or forward only among persisted history; it cannot synthesize a generation.

## 16. Persistence model

P10 separates immutable authority publication from derived evolution ledgers.

Existing `AuthorityStore::promote` remains the atomic mathematical generation transaction.

New derived tables/records persist:

```text
expansion_activations
supersession_records
realization_selection
```

These records are content-addressed and generation-scoped. A failure to persist derived activation/selection **cannot** make an unadmitted artifact usable; it fails closed by leaving the capability/grammar inactive.

P10 does not rewrite generation manifest schema solely to bundle derived rollout metadata. That avoids invalidating P1-P9 generation identities.

## 17. Canonical P10 proof manifest

P10 introduces `SelfExpansionProofManifest` binding:

```text
source_commit
frozen_P9_predecessor
registry_digest
U_g
U_(g+1)
world
package_manifest_digests
structure_goal
structure_witness
ordinary_promotion_candidate/authorization evidence
classified_promotion_candidate/expansion authorization
closure_before
closure_after
closure_delta
newly_unlocked_capability
Lambda_g
Lambda_(g+1)
shadow_metaprimitive negative/positive evidence
nogood scope negative evidence
semantic-change/freshness evidence
transport/repair framework evidence
realization-only upgrade/rollback evidence
supersession/rollback evidence
negative-control manifest
checker/verifier identity
```

The final verifier must prove the Rational package/capability implementation digest did not change between the before/after closure proof.

## 18. Canonical P10 markers

The independent P10 verifier emits these markers only after replaying the complete proof manifest:

```text
PASS P10_PROMOTION_CLASS_REGISTRY
PASS P10_STRUCTURE_WITNESS_PROMOTION
PASS P10_NON_PRIMITIVE_CAPABILITY_UNLOCK
PASS P10_NOGOOD_SCOPE_ENFORCED
PASS P10_ROUTE_PROMOTION_GATED
PASS P10_GRAMMAR_GENERATION_BOUND
PASS P10_METAPRIMITIVE_SHADOW_GATE
PASS P10_SEMANTIC_CHANGE_REVALIDATION
PASS P10_PROOF_TRANSPORT_REPAIR_GATED
PASS P10_REALIZATION_ONLY_UPGRADE
PASS P10_ROLLBACK_HISTORY_PRESERVED
PASS P10_NEGATIVE_CONTROLS
PASS SELF_EXPANSION_HARDENED
```

No partial marker set constitutes P10 completion.

## 19. Negative controls

At minimum P10 executes:

```text
NC10-01 class wrapper references wrong base promotion -> reject
NC10-02 class requests forbidden effect -> reject
NC10-03 unadmitted structure witness -> closure reject
NC10-04 structure evidence not authority-bound -> closure reject
NC10-05 unscoped automatic nogood -> reject
NC10-06 route missing preservation evidence -> reject
NC10-07 CandidateSpace silently reused across Lambda generation -> identity mismatch/reject
NC10-08 metaprimitive DEFAULT_AUTOMATIC without strict gate -> reject
NC10-09 non-conservative semantic change silently transported -> reject
NC10-10 repaired/transported evidence without checker authorization -> reject
NC10-11 realization-only upgrade attempts semantic admission -> reject
NC10-12 rollback deletes/rewrites newer generation history -> impossible/replay preserved
```

## 20. File boundaries

Planned implementation surfaces:

```text
crates/formula-core/src/self_expansion.rs
crates/formula-core/tests/p10_self_expansion_identity.rs

crates/formula-check/src/self_expansion.rs
crates/formula-check/tests/p10_self_expansion_policy.rs

crates/formula-store/src/authority_store/expansion_store.rs
crates/formula-store/tests/p10_expansion_store.rs

crates/formula-packages/src/expansion.rs
crates/formula-packages/src/grammar.rs
crates/formula-packages/tests/p10_non_primitive_unlock.rs
crates/formula-packages/tests/p10_activation_hardening.rs

crates/formula-engine/src/self_expansion.rs
crates/formula-engine/tests/p10_grammar_binding.rs

crates/formula-realize/tests/p10_realization_upgrade.rs

crates/formula-first-light/src/p10.rs
crates/formula-first-light/tests/p10_self_expansion_hardening.rs

.github/workflows/p10-development.yml
.github/workflows/p10-canonical-proof.yml

docs/checkpoints/2026-09-05-p10-self-expansion-hardening.md
CURRENT.md
```

Existing files are modified only to export modules or add narrowly required store methods/errors. P7-P9 schemas are not rewritten.

## 21. Development and freeze sequence

P10 follows the existing phase discipline:

1. create P10 design and implementation plan from frozen authority;
2. add a temporary branch-only development workflow;
3. implement class identities/registry with TDD;
4. implement class authorization and activation hardening;
5. implement structure-witness non-primitive promotion proof;
6. implement `Lambda_g`, nogood/route activation metadata, semantic-change and transport/repair frameworks;
7. implement realization-only upgrade + supersession/rollback tooling;
8. assemble independent P10 proof manifest/verifier and negative controls;
9. run the development workflow until the complete suite is green;
10. remove the temporary development workflow;
11. add the permanent read-only canonical P10 workflow;
12. prove the exact source-under-test head;
13. review P9 -> P10 source delta;
14. write checkpoint and `CURRENT.md` as `source proved; docs-head proof pending`;
15. run the unchanged canonical workflow on the documentation-bearing head;
16. record the already-earned final run as non-recursive post-freeze recovery metadata.

## 22. Completion boundary

P10 is complete only when:

- all roadmap build-scope contracts above exist in bounded, independently testable form;
- the canonical structure-witness promotion unlocks `cap:rational:field` without changing Rational package/solver code;
- all P10 negative controls execute concrete fail-closed paths;
- all thirteen P10 markers are emitted by the independent verifier;
- the permanent canonical workflow is read-only and succeeds on the exact documentation-bearing candidate head;
- the P9 frozen source authority remains reproducible and unchanged.

The next roadmap boundary after a frozen P10 is P11 federation breadth (with P12/P13 remaining separate downstream phases).