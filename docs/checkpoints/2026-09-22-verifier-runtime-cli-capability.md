# Verifier Runtime CLI Capability Checkpoint

Status: SOURCE PROVED - DOCUMENTATION-HEAD PROOF PENDING

This capability is additive over the finally frozen Verifier Equivalence, RSA Public Representation, Modular Number Theory, and P12 authority boundaries.

## Frozen predecessor authority

Verifier Equivalence frozen proof boundary:

944d7d5c6a0094b7dd623519b6c1ff3c899188b1

## Canonical branch

capability/verifier-runtime-cli

## Source-under-test proof

source head:
318546f536e810c62056c3ed35f79e0e969c07e1

workflow:
Verifier runtime CLI capability proof

workflow path:
.github/workflows/verifier-runtime-cli-proof.yml

workflow blob:
d2059b937352492921b4c07aa53e121cb47de0cb

run:
35707043210

job:
106678372162

conclusion:
success

## Architecture correction provenance

The first candidate head 31927ea216e43e63e9813c9cc07a3e0f7b9da4b6 attempted to link formula-check into canonical formula-cli.

Run 35706547568 / job 106676744106 rejected that design at the existing runtime_network_policy authority test because formula-cli runtime closure contained unapproved package formula-check.

The correction did not weaken that firewall.

Instead, source head 318546f536e810c62056c3ed35f79e0e969c07e1 restores crates/formula-cli byte-for-byte to frozen predecessor 41961a542fd8c8d5a6c4bdfa90a99e1223dc627e and introduces a separate independent process crate:

crates/formula-verifier-cli

Its normal dependency closure contains formula-check and formula-core/number primitives only. It does not contain formula-engine, formula-realize, formula-first-light, or formula-store.

This matches the existing architecture rule that the engine may submit artifacts to an independent checker process but may not link checker implementation.

## Runtime protocol

Protocol schema:

formula-verifier-runtime-v1

Input:
newline-delimited key=value fields over stdin

Output:
newline-delimited key=value fields over stdout

Exit status:

0 = accepted / identity
2 = verification rejected
64 = malformed or unsupported protocol request

Supported read-only verification commands:

- identity
- powmod
- rsa-public
- montgomery
- type1
- rsa-type1

Verifier material is supplied over stdin rather than process arguments.

Maximum stdin request size is 1 MiB.

## Authority boundary

formula-verifier-cli is transport only.

It delegates to frozen formula-check entry points:

- check_modular_integer_result
- check_rsa_public_result
- check_montgomery_product_result
- check_type1_payload
- check_rsa_type1_payload

It does not implement modpow, modular inverse, RSA verification arithmetic, Montgomery arithmetic, or Type-1 parsing itself.

Identity output explicitly reports:

signing_supported=false
authorization_generation_supported=false
private_key_operations_supported=false

No signing, private-key generation, OEM-server emulation, flashing, unlock command, or protected state transition exists in this capability.

## Source proof gates

Run 35707043210 / job 106678372162 passed:

- exact source identity;
- frozen Verifier Equivalence predecessor authority;
- pinned Rust 1.98.0;
- locked/offline metadata;
- 9 focused runtime protocol tests;
- frozen verifier-equivalence tests;
- frozen RSA public-representation tests;
- frozen modular-number-theory tests;
- identity-output contract;
- full workspace tests;
- full workspace build;
- rustfmt;
- clippy with warnings denied;
- verifier-process dependency isolation;
- checker-backed transport source firewall;
- clean worktree.

## Explicit exclusions

This capability does not:

- generate signatures;
- generate authorization responses;
- hold private keys;
- infer vendor authorization material;
- emulate vendor servers;
- choose or submit unlock commands;
- alter device state.

## Documentation-head freeze requirement

Only these source-to-docs changes are permitted:

- CURRENT.md
- docs/checkpoints/2026-09-22-verifier-runtime-cli-capability.md

The permanent workflow blob must remain:

d2059b937352492921b4c07aa53e121cb47de0cb

The unchanged workflow must prove the exact documentation-bearing head before this capability is finally frozen.

## Freeze state

Verifier Equivalence predecessor: FINALLY FROZEN
Verifier Runtime CLI source proof: PROVED
Verifier Runtime CLI source head: 318546f536e810c62056c3ed35f79e0e969c07e1
Verifier Runtime CLI source run/job: 35707043210 / 106678372162
Verifier Runtime CLI source conclusion: success
Verifier Runtime CLI documentation proof: PENDING
Verifier Runtime CLI final freeze: PENDING
