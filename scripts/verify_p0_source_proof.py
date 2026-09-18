#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

EXPECTED_SCHEMA = "formula.p0-01.source-proof.v1"
EXPECTED_STATUS = "CANDIDATE_HOST_EVIDENCE_NOT_AUTHORITY"
EXPECTED_BRANCH = "impl/p0-first-light"
EXPECTED_ARCH = "50c0beb021d8bf02d59a177049b9c2cf1783b26a"
EXPECTED_TOOLCHAIN = "1.98.0"
REQUIRED_FILES = {
    "record.json",
    "git-head.txt",
    "git-branch.txt",
    "frozen-architecture.txt",
    "worktree-state.txt",
    "rustc-version.txt",
    "cargo-version.txt",
    "manifest-sha256.txt",
    "cargo-metadata.json",
    "cargo-metadata.sha256",
    "evidence-files.sha256",
}
HEX40 = re.compile(r"^[0-9a-f]{40}$")
HEX64 = re.compile(r"^[0-9a-f]{64}$")


class ProofError(ValueError):
    pass


def _sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def _text(path: Path) -> str:
    return path.read_text(encoding="utf-8").strip()


def _parse_sha256_file(path: Path) -> dict[str, str]:
    result: dict[str, str] = {}
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line:
            continue
        parts = line.split(maxsplit=1)
        if len(parts) != 2 or not HEX64.fullmatch(parts[0]):
            raise ProofError(f"malformed sha256 line in {path.name}: {raw!r}")
        name = parts[1].lstrip("*").strip()
        if not name:
            raise ProofError(f"missing filename in {path.name}")
        result[name] = parts[0]
    return result


def _git(*args: str, cwd: Path) -> str:
    cp = subprocess.run(["git", *args], cwd=cwd, text=True, capture_output=True, check=False)
    if cp.returncode != 0:
        raise ProofError(f"git {' '.join(args)} failed: {cp.stderr.strip()}")
    return cp.stdout.strip()


def verify(root: Path, evidence: Path, *, require_live_checkout: bool = True) -> dict[str, str]:
    root = root.resolve()
    evidence = evidence.resolve()
    if not evidence.is_dir():
        raise ProofError(f"evidence directory does not exist: {evidence}")

    missing = sorted(name for name in REQUIRED_FILES if not (evidence / name).is_file())
    if missing:
        raise ProofError("missing evidence files: " + ", ".join(missing))

    try:
        record = json.loads((evidence / "record.json").read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise ProofError(f"invalid record.json: {exc}") from exc

    expected_record = {
        "schema": EXPECTED_SCHEMA,
        "status": EXPECTED_STATUS,
        "git_branch": EXPECTED_BRANCH,
        "frozen_architecture": EXPECTED_ARCH,
        "worktree_state": "clean",
        "rust_toolchain": EXPECTED_TOOLCHAIN,
    }
    for key, value in expected_record.items():
        if record.get(key) != value:
            raise ProofError(f"record {key} mismatch: expected {value!r}, found {record.get(key)!r}")

    head = str(record.get("git_head") or "")
    if not HEX40.fullmatch(head):
        raise ProofError("record git_head must be a 40-character lowercase Git SHA")

    text_bindings = {
        "git-head.txt": head,
        "git-branch.txt": EXPECTED_BRANCH,
        "frozen-architecture.txt": EXPECTED_ARCH,
        "worktree-state.txt": "clean",
    }
    for name, expected in text_bindings.items():
        found = _text(evidence / name)
        if found != expected:
            raise ProofError(f"{name} mismatch: expected {expected!r}, found {found!r}")

    rustc_text = (evidence / "rustc-version.txt").read_text(encoding="utf-8")
    if not rustc_text.startswith("rustc 1.98.0"):
        raise ProofError("rustc-version.txt is not Rust 1.98.0")
    cargo_text = (evidence / "cargo-version.txt").read_text(encoding="utf-8")
    if not cargo_text.startswith("cargo 1.98.0"):
        raise ProofError("cargo-version.txt is not Cargo 1.98.0")

    metadata_digest_lines = _parse_sha256_file(evidence / "cargo-metadata.sha256")
    metadata_expected = None
    for name, digest in metadata_digest_lines.items():
        if Path(name).name == "cargo-metadata.json":
            metadata_expected = digest
            break
    if metadata_expected is None:
        raise ProofError("cargo-metadata.sha256 does not bind cargo-metadata.json")
    if _sha256(evidence / "cargo-metadata.json") != metadata_expected:
        raise ProofError("cargo-metadata.json digest mismatch")

    evidence_hashes = _parse_sha256_file(evidence / "evidence-files.sha256")
    for name in REQUIRED_FILES - {"evidence-files.sha256"}:
        matching = [digest for path, digest in evidence_hashes.items() if Path(path).name == name]
        if len(matching) != 1:
            raise ProofError(f"evidence-files.sha256 must bind exactly one {name}")
        if _sha256(evidence / name) != matching[0]:
            raise ProofError(f"evidence file digest mismatch: {name}")

    manifest_hashes = _parse_sha256_file(evidence / "manifest-sha256.txt")
    for name in (
        "rust-toolchain.toml",
        "Cargo.toml",
        "Cargo.lock",
        "docs/implementation/P0_SOURCE_MANIFEST.md",
    ):
        expected = manifest_hashes.get(name)
        if expected is None:
            raise ProofError(f"manifest-sha256.txt missing {name}")
        source = root / name
        if not source.is_file():
            raise ProofError(f"source manifest missing from checkout: {name}")
        if _sha256(source) != expected:
            raise ProofError(f"source manifest digest mismatch: {name}")

    if require_live_checkout:
        live_head = _git("rev-parse", "HEAD", cwd=root)
        live_branch = _git("branch", "--show-current", cwd=root)
        live_dirty = _git("status", "--porcelain", cwd=root)
        if live_head != head:
            raise ProofError(f"live HEAD mismatch: evidence={head}, checkout={live_head}")
        if live_branch != EXPECTED_BRANCH:
            raise ProofError(f"live branch mismatch: expected {EXPECTED_BRANCH}, found {live_branch or 'DETACHED'}")
        # The evidence directory itself may be ignored/untracked; any tracked/source change still fails closed.
        if live_dirty:
            raise ProofError("live worktree is not clean")

    return {
        "status": EXPECTED_STATUS,
        "git_head": head,
        "git_branch": EXPECTED_BRANCH,
        "frozen_architecture": EXPECTED_ARCH,
        "rust_toolchain": EXPECTED_TOOLCHAIN,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Verify Formula P0-01 host evidence without promoting authority")
    parser.add_argument("evidence", nargs="?", default=".formula/evidence/p0-01")
    parser.add_argument("--root", default=".")
    parser.add_argument("--no-live-checkout", action="store_true", help="verify preserved bundle integrity without comparing current Git state")
    ns = parser.parse_args(argv)
    try:
        result = verify(Path(ns.root), Path(ns.evidence), require_live_checkout=not ns.no_live_checkout)
    except ProofError as exc:
        print(f"P0-01 verification refused: {exc}", file=sys.stderr)
        return 1
    print(json.dumps(result, indent=2, sort_keys=True))
    print("Verified candidate host evidence only. No Formula authority or P0 promotion is created.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
