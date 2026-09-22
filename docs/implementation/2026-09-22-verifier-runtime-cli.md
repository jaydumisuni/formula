# Formula Verifier Runtime CLI

Status: additive runtime-transport capability candidate over the finally frozen Verifier Equivalence capability.

Frozen predecessor:
944d7d5c6a0094b7dd623519b6c1ff3c899188b1

## Purpose

Expose the already-proved public verification semantics through a dedicated formula-verifier-cli executable so external planners such as TokenIt can call the independent checker at runtime.

The CLI is a separate checker process and transport only. Mathematical and verifier authority remains in formula-check. The canonical formula-cli dependency closure remains unchanged.

## Protocol

Requests are newline-delimited key=value fields on stdin.

The protocol schema is:

formula-verifier-runtime-v1

Supported commands:

- identity
- powmod
- rsa-public
- montgomery
- type1
- rsa-type1

Responses are newline-delimited key=value fields on stdout.

Exit status:

- 0: verification accepted / identity returned
- 2: verification rejected by Formula
- 64: malformed or unsupported protocol request

## Privacy boundary

Verifier material is supplied over stdin rather than command-line arguments so raw public representatives, expected payloads, and producer evidence do not need to appear in process argument listings.

The CLI caps stdin at 1 MiB.

## Authorization boundary

The runtime CLI explicitly reports:

signing_supported=false
authorization_generation_supported=false
private_key_operations_supported=false

It contains no signing, private-key, vendor-server, unlock-command, flashing, or state-transition operation.

## Authority

The CLI must remain a thin wrapper around frozen formula-check operations.

It does not reimplement PowMod, RSA public operations, Montgomery arithmetic, or Type-1 parsing.
