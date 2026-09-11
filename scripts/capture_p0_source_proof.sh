#!/usr/bin/env bash
set -euo pipefail

EXPECTED_ARCH="50c0beb021d8bf02d59a177049b9c2cf1783b26a"
EXPECTED_BRANCH="impl/p0-first-light"
OUT_DIR="${1:-.formula/evidence/p0-01}"

fail() {
  printf 'P0-01 capture refused: %s\n' "$1" >&2
  exit 1
}

command -v git >/dev/null 2>&1 || fail "git is required"
command -v rustc >/dev/null 2>&1 || fail "rustc is required"
command -v cargo >/dev/null 2>&1 || fail "cargo is required"
command -v sha256sum >/dev/null 2>&1 || fail "sha256sum is required"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || fail "not inside a Git worktree"
cd "$ROOT"

BRANCH="$(git branch --show-current)"
[ "$BRANCH" = "$EXPECTED_BRANCH" ] || fail "expected branch $EXPECTED_BRANCH, found ${BRANCH:-DETACHED}"

[ -z "$(git status --porcelain)" ] || fail "worktree is not clean"

for path in rust-toolchain.toml Cargo.toml Cargo.lock docs/implementation/P0_SOURCE_MANIFEST.md; do
  [ -f "$path" ] || fail "missing required file: $path"
done

grep -Fq 'channel = "1.98.0"' rust-toolchain.toml || fail "rust-toolchain.toml is not pinned to 1.98.0"

RUSTC_VERSION="$(rustc --version)"
case "$RUSTC_VERSION" in
  'rustc 1.98.0 '*) ;;
  'rustc 1.98.0') ;;
  *) fail "expected rustc 1.98.0, found $RUSTC_VERSION" ;;
esac

HEAD="$(git rev-parse HEAD)"
mkdir -p "$OUT_DIR"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

rustc --version --verbose > "$TMP/rustc-version.txt"
cargo --version --verbose > "$TMP/cargo-version.txt"
sha256sum rust-toolchain.toml Cargo.toml Cargo.lock > "$TMP/manifest-sha256.txt"
cargo metadata --locked --format-version 1 > "$TMP/cargo-metadata.json"
sha256sum "$TMP/cargo-metadata.json" > "$TMP/cargo-metadata.sha256"
printf '%s\n' "$HEAD" > "$TMP/git-head.txt"
printf '%s\n' "$BRANCH" > "$TMP/git-branch.txt"
printf '%s\n' "$EXPECTED_ARCH" > "$TMP/frozen-architecture.txt"
printf 'clean\n' > "$TMP/worktree-state.txt"

cat > "$TMP/record.json" <<JSON
{
  "schema": "formula.p0-01.source-proof.v1",
  "status": "CANDIDATE_HOST_EVIDENCE_NOT_AUTHORITY",
  "git_head": "$HEAD",
  "git_branch": "$BRANCH",
  "frozen_architecture": "$EXPECTED_ARCH",
  "worktree_state": "clean",
  "rust_toolchain": "1.98.0",
  "authority_note": "This record captures P0-01 host evidence inputs only. It does not promote P0-01 or P0."
}
JSON

for file in "$TMP"/*; do
  cp "$file" "$OUT_DIR/"
done

sha256sum "$OUT_DIR"/* > "$OUT_DIR/evidence-files.sha256"
printf 'Captured candidate P0-01 host evidence for %s at %s\n' "$HEAD" "$OUT_DIR"
