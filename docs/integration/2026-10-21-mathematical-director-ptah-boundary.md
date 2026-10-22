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

## Original P13 workplan (retained for historical context)

1. Validate Ptah acceptance of exact immutable packet, side-effect limits, authority and resource constraints.
2. Map the workload to qualified Ptah A04 Activity/Operation/Attempt lifecycle, E02 placement/fencing, transfer/recovery and appropriate A05/provider mechanics. Do not treat an A05 process launch as permission to violate Formula SideEffectPolicy::deny_all.
3. Run identical mathematical obligations locally and through a real Ptah execution path. Collect candidate artifacts with exact provenance.
4. Independently check both artifacts under identical Formula authority, observation, generation, and evidence requirements.
5. Verify semantic equivalence and stable U_g across location, retry, corrupt receipt, stale lease and interrupted operation tests.
6. Qualify platform acceptance where requested. Only then mark P13 PROVED.

The original workplan is retained for context; current physical acceptance and limitations are recorded below.

## P13 canonical two-machine qualification — 2026-10-21

### Tested boundary and evidence

- Formula's Director uses its existing CompilerV1 over a fully grounded COUNT QueryIR for the canonical P9 nineteen-element vector; this is not a new mathematical compiler.
- The versioned Formula-to-Ptah envelope contains canonical query, manifest, and work-cell JSON with exact SHA-256 identities. The Ptah adapter validates the COUNT request, bound input, finite resource limits, no side effects, and independent caller-owned custody.
- Real loopback E01 TLS / E02 lease-fenced dispatch / A05 native process test passes with candidate output exactly 9 followed by newline, successful observed process exit, output digest, and durable Node journal.
- Physical KRATOS (Control) to ATHENA (Node) E01/E02/A05 one-shot passes on its separate Node identity/configuration. Ptah Control performed Formula preflight BEFORE accepting the connection; Control confirmed formula_handoff_bound=true, and the ATHENA node journal recorded exit 0 and the candidate digest.
- The already-running ATHENA Ptah worker retained its original configuration and process. Frozen Formula P9/P12 and active P1 were not changed.

### Canonical identifiers

- Formula manifest: sha256:778e11206defae987198ad7174606ab063a64855a4d0775a10bce66087e2ec07
- Work cell: sha256:d156d75e30f1da5b7fc32e1a5f55fec0f126c0b11ae0cd361ea1b925071a8d09
- P9 vector digest: sha256:f59815a3040e4d9220324960729c8508f9aa24ce596a09fd6a7509abd5b5894e
- Actual physical A05 stdout SHA-256: sha256:2e6d31a5983a91251bfae5aefa1c0a19d8ba3cf601d0e8a706b4cfa9661a6b8a (for 9 followed by newline)
- Physical Attempt: 01a15df2-7378-7331-a13d-f4a57120b132; Dispatch: 01a15df2-7378-7331-a13d-f4d384a1267b; isolated Node: 01a15de7-fba9-7cf0-a3f1-ebd6941e7e4a; generation/epoch 5/5.

### Independent checker and negative controls

- Formula-check independently verifies canonical output encoding, SHA-256 byte binding, P9 vector identity, and exact mathematical membership in {1,2,4,8,16,32,64,128}; it does not trust the executor's bitwise predicate or turn a Ptah success into mathematical promotion.
- The checker rejects wrong counts, noncanonical candidate bytes, altered input identities, and incorrect digest. Ptah rejects swapped campaign/work-cell identities, unauthorized side effects, mismatched COUNT input/target, and invalid resource limits.
- A physical-facing negative preflight with a forged authority returned exit code 2 before Ptah Control opened the listener or created a dispatch journal.
- Ptah Control uses exact three-part optional Formula handoff arguments. The original non-Formula Ptah E07 path remains unchanged.

### Acceptance scope and explicit limitations

P13 CANONICAL P9 LOCAL-VS-PTAH SEMANTIC EQUIVALENCE: PASS. The representative same COUNT obligation executes under Ptah remotely and checks to the same answer and bound identities locally; Formula retains exclusive mathematical authority. This is a scoped integration gate, NOT universal completion of arbitrary distributed mathematical campaigns.

Control's physical receipt currently says sent_unconfirmed and acknowledges completion; ATHENA's durable journal independently records exit/output digest. The Ptah TLS Ack is not an independently signed effect witness and does not carry arbitrary candidate bytes. The output digest was corroborated via physical Node journal readback. General E03 candidate artifact transfer, arbitrary multi-work-cell distributed scheduling, every interrupted recovery path, and natural-language-to-QueryIR are not qualified by this case.

Keep the original P13 workplan above as history. Integrate on the post-P12 integration branch only after exact-head CI passes. Do not merge to frozen P12 or modify active P1.

### Post-review P1 corrections

- The Formula Director enforces SideEffectPolicy::deny_all on BOTH normal planning and exact-capability reuse. An unsafe reuse query fails before CompilerV1::compile_reuse and is covered by a negative regression test.


## 2026-10-22: Optional SUP and multi-Director Ptah candidate collection

- Formula's new sup_handoff library emits an optional request to SUP's
  existing G4 Production Director only when neural help is genuinely
  justified. Caller supplies an exact SUP generation, instruction,
  measured baseline/evidence digest, acceptance criteria and forbidden
  regressions. Formula's own mathematical discovery, truth, promotion and
  second-use remain its own responsibility; there is no mandatory SUP
  dependency or training authorization.
- Live SUP G4 intake was exercised on KRATOS. With no externally qualified
  Formula neural learning package, SUP correctly returned AWAIT_SOURCE,
  training_executed=false, authority_granted=false. This is not an
  assertion that a neural model was trained.
- Formula's new ptah_campaign library collects actual Director-produced
  WorkCell plans across multiple grounded campaigns. It binds exact
  per-plan manifests and candidate byte digests, allows identical
  evidence replay, and rejects conflicting retries, missing candidate
  bytes or changed work identities. The stable observation is
  caller-checkpoint metadata, not mathematical proof or authority.
- Ptah continues as optional workstation/workspace and execution
  substrate. Ordinary Formula CPU work remains local and available
  if Ptah, SUP or another Node is offline.

Verified with Formula Rust workspace tests, strict Clippy, format
checks, negative tests and live SUP schema acceptance. The existing
physical P13 COUNT equivalence evidence remains unchanged.

Still unqualified by these additions: general physical E03 candidate
artifact transfer, signed effect readback, arbitrary interrupted
operation/lease recovery, autonomous remote scheduling, and grounded
English-to-QueryIR. Such broader Ptah facilities need their own real
native-transport verification when the actual workload needs them.
Do not treat this scoped additional interface as universal P13 proof.
