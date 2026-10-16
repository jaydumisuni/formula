# Formula P1 candidate CLI — structural identity only

This branch replaces the empty Formula CLI entry point with a small, bounded Rust interface that **uses the existing `formula-core::Entity` canonical structural digest implementation**.

Commands:
- `formula-cli capabilities` — explicit P1 candidate operations; denies certification, solving and effect authority.
- `formula-cli entity-digest <kind> <lowercase-hex>` — canonical Formula Entity identity for an explicit kind and at most 32 KiB of structure bytes.
- `formula-cli verify-entity-digest <kind> <lowercase-hex> <64-hex-digest>` — deny mismatches.

Candidate Rust tests cover deterministic identity, changing kind/content, malformed/big input, unsupported operations and mismatch denial. The workspace lockfile records the internal local `formula-core` dependency for offline reproducibility.

## Mandatory local proof before promotion

```sh
cargo test --locked --offline -p formula-cli
cargo test --locked --offline -p formula-core
cargo run --locked --offline -p formula-cli -- capabilities
cargo run --locked --offline -p formula-cli -- entity-digest ttg.re:fixture 00ff
```

**This is not Formula First-Light proof, mathematical certification, a qualified solver, a promotion authority, or a production Ptah API.** Work remains on an exact-head draft branch pending locally tested compiler evidence; no GitHub Actions payment dependency is required. The Oracle KRATOS/ATHENA connection was unavailable when this branch was authored, so no physical compilation is claimed. Do not merge until project-owner admission/review passes.
