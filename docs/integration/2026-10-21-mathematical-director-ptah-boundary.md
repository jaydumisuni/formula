# Mathematical Director and Ptah WorkCell boundary — 2026-10-21

Scope: additive integration on an isolated branch derived from the frozen P12 implementation. This document does not revise P9 or P12 mathematical proof authority.

## Product-owned Director

- Input is a **semantically grounded Formula QueryIR**, an exact CompilerAuthoritySnapshot, and CompilerInputs. Text-to-QueryIR interpretation is not yet qualified: no English request is silently converted into mathematical authority or fabricated digests.
- FormulaMathematicalDirector calls existing CompilerV1; it does not replace its query compiler, searches, checker, or native realization.
- The Director either plans discovery work cells or uses exact admitted capability reuse with zero discovery work cells.
- Output carries a deterministic manifest binding QueryIR, CampaignIR, ReplayManifest and exact WorkCellPlan digest list. Work cells are checked against obligation and authority identities, and must have no side effects or authority writes.
- The Director never issues certificates, promotes candidates, writes U_g, or converts worker success into mathematical proof.

## Ptah contract

- Schema: formula-ptah-handoff-v1, canonical JSON envelope.
- The packet contains the original Formula manifest and WorkCellPlan as canonical JSON *strings* plus their SHA-256 structural digest labels; it binds the obligation and required mathematical authority.
- Result class: CANDIDATE_ONLY. Ptah is the workforce/workspace/transport lifecycle, not the math authority.
- The Formula return boundary verifies exact manifest and work-cell identities and candidate byte digest. This is *transport integrity only*, not an independent mathematical proof.
- Independent formula-check, Formula promotion policy, durable authority store and native realization checks retain their frozen obligations.

## Ptah P13 qualification still required

1. Validate Ptah acceptance of exact immutable packet, side-effect limits, authority and resource constraints.
2. Map the workload to qualified Ptah A04 Activity/Operation/Attempt lifecycle, E02 placement/fencing, transfer/recovery and appropriate A05/provider mechanics. Do not treat an A05 process launch as permission to violate Formula SideEffectPolicy::deny_all.
3. Run identical mathematical obligations locally and through a real Ptah execution path. Collect candidate artifacts with exact provenance.
4. Independently check both artifacts under identical Formula authority, observation, generation, and evidence requirements.
5. Verify semantic equivalence and stable U_g across location, retry, corrupt receipt, stale lease and interrupted operation tests.
6. Qualify platform acceptance where requested. Only then mark P13 PROVED.

Current scope is a Director + transport-boundary candidate, not a completed Ptah execution integration.
