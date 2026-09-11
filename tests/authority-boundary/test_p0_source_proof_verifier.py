from __future__ import annotations

import hashlib
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).resolve().parents[2] / "scripts" / "verify_p0_source_proof.py"
spec = importlib.util.spec_from_file_location("verify_p0_source_proof", MODULE_PATH)
mod = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(mod)


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


class P0SourceProofVerifierTests(unittest.TestCase):
    def make_bundle(self, root: Path) -> Path:
        manifests = {
            "rust-toolchain.toml": b'[toolchain]\nchannel = "1.98.0"\n',
            "Cargo.toml": b"[workspace]\nresolver = \"2\"\n",
            "Cargo.lock": b"version = 4\n",
        }
        for name, data in manifests.items():
            (root / name).write_bytes(data)

        evidence = root / "evidence"
        evidence.mkdir()
        head = "a" * 40
        record = {
            "schema": mod.EXPECTED_SCHEMA,
            "status": mod.EXPECTED_STATUS,
            "git_head": head,
            "git_branch": mod.EXPECTED_BRANCH,
            "frozen_architecture": mod.EXPECTED_ARCH,
            "worktree_state": "clean",
            "rust_toolchain": mod.EXPECTED_TOOLCHAIN,
            "authority_note": "candidate evidence only",
        }
        (evidence / "record.json").write_text(json.dumps(record, sort_keys=True) + "\n")
        (evidence / "git-head.txt").write_text(head + "\n")
        (evidence / "git-branch.txt").write_text(mod.EXPECTED_BRANCH + "\n")
        (evidence / "frozen-architecture.txt").write_text(mod.EXPECTED_ARCH + "\n")
        (evidence / "worktree-state.txt").write_text("clean\n")
        (evidence / "rustc-version.txt").write_text("rustc 1.98.0 (test)\nrelease: 1.98.0\n")
        (evidence / "cargo-version.txt").write_text("cargo 1.98.0 (test)\nrelease: 1.98.0\n")
        metadata = b'{"packages":[],"version":1}\n'
        (evidence / "cargo-metadata.json").write_bytes(metadata)
        (evidence / "cargo-metadata.sha256").write_text(f"{sha(metadata)}  {evidence / 'cargo-metadata.json'}\n")
        (evidence / "manifest-sha256.txt").write_text(
            "".join(f"{sha(data)}  {name}\n" for name, data in manifests.items())
        )
        names = sorted(mod.REQUIRED_FILES - {"evidence-files.sha256"})
        (evidence / "evidence-files.sha256").write_text(
            "".join(f"{sha((evidence / name).read_bytes())}  {evidence / name}\n" for name in names)
        )
        return evidence

    def test_valid_preserved_bundle_verifies_without_creating_authority(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            result = mod.verify(root, evidence, require_live_checkout=False)
            self.assertEqual(result["status"], mod.EXPECTED_STATUS)
            self.assertEqual(result["git_head"], "a" * 40)

    def test_tampered_evidence_file_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            (evidence / "rustc-version.txt").write_text("rustc 9.9.9\n")
            with self.assertRaises(mod.ProofError):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_source_manifest_drift_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            (root / "Cargo.toml").write_text("[workspace]\nresolver = \"3\"\n")
            with self.assertRaises(mod.ProofError):
                mod.verify(root, evidence, require_live_checkout=False)


if __name__ == "__main__":
    unittest.main()
