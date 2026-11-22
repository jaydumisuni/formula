# Data Fabric G0 — source-bound semantic pre-assurance, 2026-11-22

Status: CANDIDATE-ONLY. This is a bounded operator-directed review in a Formula-owned isolated branch, not an authenticated Formula certification, accepted G0 generation or a replacement for Data Fabric project authority.

## Exact reviewed source

- Data Fabric repository: jaydumisuni/TTG-Data-Fabric
- Unified candidate branch: supervisor/shift-e-g0-seven-obligation-regression-20261122
- Reviewed commit: 2170fa435b9e7f67a5ffba2695351e7cd197469f
- Reviewer record: docs/reviews/data-fabric-g0-2170fa4-semantic-preassurance-20261122.json
- Input source files: G0 constitution, hostile-world corpus, Business Core compatibility scope, donor contract and complete 33-row matrix; SHA-256 values are bound in the JSON.

## Findings

- 33/33 required review rows have a candidate discriminator, a distinguishable counterexample and a proposed measurement/evidence requirement.
- 25 are provisionally classified MEASURABLE at semantic-contract level only.
- 8 contain unresolved law choices and are explicitly NEEDS_CLARIFICATION; these cannot be made MEASURABLE by a test count or replaying a synthetic fixture.

### Clarification requirements

1. G0_CONSTITUTION: MVCC / visibility — Constitution requires a defined snapshot but does not itself freeze the isolation level or snapshot cut-point.
   Required evidence: Define required isolation/snapshot cut-point from pinned Business Core usage; concurrent read/write trace.
2. G0_CONSTITUTION: integrity / constraints — Integrity evidence is qualified by an unspecified declared threat model.
   Required evidence: Specify threat model, checksum coverage, constraint catalog and corruption/constraint negative fixture.
3. G0_CONSTITUTION: catalog / schema — Resume-versus-rollback decision criteria are not yet stated.
   Required evidence: Freeze migration checkpoint choice and restart/replay observations.
4. G0_CONSTITUTION: WAL / recovery ordering — Generation-relative WAL ordering is stated but not a complete durability-barrier contract.
   Required evidence: Freeze exact WAL durability/order and bounded tail validity law before acceptance.
5. G0_HOSTILE_WORLD_CORPUS: HW-03 — No explicit maximal acceptable damage window or WAL record validation rule is frozen.
   Required evidence: Specify bounded tail truncation/fence policy; inject bitflip/truncation across record boundary.
6. G0_HOSTILE_WORLD_CORPUS: HW-06 — Defined snapshot cut point and isolation level are not yet pinned to Business Core behavior.
   Required evidence: Freeze snapshot cut point/isolation clause; concurrent writer-reader schedule.
7. G0_HOSTILE_WORLD_CORPUS: HW-07 — Deterministic conflict winner/timeout/abort precedence is not specified at G0.
   Required evidence: Lock arbitration/abort precedence and pinned workload concurrency trace required.
8. G0_HOSTILE_WORLD_CORPUS: HW-10 — Resume/rollback selection and schema generation cut-over are not specified.
   Required evidence: Define migration branch selection and version/rollback receipt.

## Verification and authority boundary

Run these commands from the Formula review worktree:

1) python3 -m pytest -q tests/test_data_fabric_g0_review.py
2) python3 tools/verify_data_fabric_g0_review.py docs/reviews/data-fabric-g0-2170fa4-semantic-preassurance-20261122.json --source-repo /home/kratos/projects/TTG-Data-Fabric/.workspace/incoming/g0-unified-final-head-20261122

The validator checks exact source HEAD, clean source checkout, five input SHA-256 digests, all 33 matrix rows, explicit positive/negative discrimination, and fail-closed reviewer claims. It cannot independently prove the correctness of the 25 proposed semantic discriminators or promote G0.

## Gate disposition

- Formula source-bound pre-assurance: PREPARED, NOT CERTIFIED.
- Eight source-law ambiguities: OPEN; return to Data Fabric owner for bounded correction and fresh-head review.
- TTG-Simulation: HW-02 reference traces only; 17 other worlds and full independent G0 acceptance remain open.
- Sergeant: full G0 generation assurance NOT PROVEN.
- G0 accepted freeze: NOT AUTHORIZED; G1 remains blocked.
