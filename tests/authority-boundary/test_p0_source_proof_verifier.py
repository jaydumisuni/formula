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
    def make_bundle(self, root: Path, *, branch: str = mod.EXPECTED_BRANCH) -> Path:
        manifests = {
            "rust-toolchain.toml": b'[toolchain]\nchannel = "1.98.0"\n',
            "Cargo.toml": b"[workspace]\nresolver = \"2\"\n",
            "Cargo.lock": b"version = 4\n",
            "docs/implementation/P0_SOURCE_MANIFEST.md": b"# P0 Source and Toolchain Manifest\n",
        }
        for name, data in manifests.items():
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)

        evidence = root / "evidence"
        evidence.mkdir()
        head = "a" * 40
        record = {
            "schema": mod.EXPECTED_SCHEMA,
            "status": mod.EXPECTED_STATUS,
            "git_head": head,
            "git_branch": branch,
            "frozen_architecture": mod.EXPECTED_ARCH,
            "worktree_state": "clean",
            "rust_toolchain": mod.EXPECTED_TOOLCHAIN,
            "authority_note": "candidate evidence only",
        }
        (evidence / "record.json").write_text(json.dumps(record, sort_keys=True) + "\n")
        (evidence / "git-head.txt").write_text(head + "\n")
        (evidence / "git-branch.txt").write_text(branch + "\n")
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

    def test_current_construction_branch_bundle_verifies_without_authority(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            branch = "c3/p1-continuation-20261006"
            evidence = self.make_bundle(root, branch=branch)
            result = mod.verify(root, evidence, require_live_checkout=False)
            self.assertEqual(result["git_branch"], branch)
            self.assertEqual(result["status"], mod.EXPECTED_STATUS)

    def test_text_binding_with_extra_whitespace_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            (evidence / "git-branch.txt").write_text(mod.EXPECTED_BRANCH + "  \n")
            with self.assertRaisesRegex(mod.ProofError, "noncanonical text binding"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_text_binding_with_multiple_lines_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            (evidence / "git-head.txt").write_text("a" * 40 + "\n\n")
            with self.assertRaisesRegex(mod.ProofError, "noncanonical text binding"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_unadmitted_branch_bundle_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root, branch="untrusted/p0")
            with self.assertRaisesRegex(mod.ProofError, "git_branch not admitted"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_symlinked_evidence_directory_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            alias = root / "evidence-alias"
            alias.symlink_to(evidence, target_is_directory=True)
            with self.assertRaisesRegex(mod.ProofError, "symlink evidence directory refused"):
                mod.verify(root, alias, require_live_checkout=False)

    def test_symlinked_evidence_file_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            original = evidence / "rustc-version.txt"
            external = root / "external-rustc.txt"
            original.rename(external)
            original.symlink_to(external)
            with self.assertRaisesRegex(mod.ProofError, "symlink evidence files refused"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_hardlinked_evidence_file_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            external = root / "external-rustc.txt"
            external.hardlink_to(evidence / "rustc-version.txt")
            with self.assertRaisesRegex(mod.ProofError, "hardlinked evidence files refused"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_hardlinked_source_manifest_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            external = root / "external-cargo.toml"
            external.hardlink_to(root / "Cargo.toml")
            with self.assertRaisesRegex(mod.ProofError, "hardlinked source manifest refused"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_tampered_evidence_file_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            (evidence / "rustc-version.txt").write_text("rustc 9.9.9\n")
            with self.assertRaises(mod.ProofError):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_duplicate_record_keys_fail_closed_even_if_values_match(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            record_path = evidence / "record.json"
            raw = record_path.read_text()
            record_path.write_text(raw.replace(
                '"schema": ', '"schema": "formula.p0-01.source-proof.v1", "schema": ', 1
            ))
            with self.assertRaisesRegex(mod.ProofError, "duplicate record.json key"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_non_object_record_fails_closed(self):
        for invalid in ([], None, "not an object", 42):
            with self.subTest(invalid=invalid), tempfile.TemporaryDirectory() as td:
                root = Path(td)
                evidence = self.make_bundle(root)
                (evidence / "record.json").write_text(json.dumps(invalid))
                with self.assertRaisesRegex(mod.ProofError, "record.json must contain a JSON object"):
                    mod.verify(root, evidence, require_live_checkout=False)

    def test_toolchain_version_prefix_spoof_fails_closed(self):
        for filename, spoof in (("rustc-version.txt", "rustc 1.98.01 (spoof)\n"), ("cargo-version.txt", "cargo 1.98.0evil (spoof)\n")):
            with self.subTest(filename=filename), tempfile.TemporaryDirectory() as td:
                root = Path(td)
                evidence = self.make_bundle(root)
                (evidence / filename).write_text(spoof)
                with self.assertRaisesRegex(mod.ProofError, "is not (Rust|Cargo) 1.98.0"):
                    mod.verify(root, evidence, require_live_checkout=False)

    def test_duplicate_sha256_filename_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            digest_path = evidence / "cargo-metadata.sha256"
            digest_path.write_text(digest_path.read_text() * 2)
            with self.assertRaisesRegex(mod.ProofError, "duplicate sha256 filename"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_ambiguous_metadata_basename_bindings_fail_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            digest_path = evidence / "cargo-metadata.sha256"
            digest_path.write_text(
                digest_path.read_text()
                + f"{sha((evidence / 'cargo-metadata.json').read_bytes())}  alternate/cargo-metadata.json\n"
            )
            with self.assertRaisesRegex(mod.ProofError, "must bind exactly one cargo-metadata.json"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_single_wrong_directory_metadata_binding_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            digest_path = evidence / "cargo-metadata.sha256"
            digest_path.write_text(
                f"{sha((evidence / 'cargo-metadata.json').read_bytes())}  alternate/cargo-metadata.json\n"
            )
            with self.assertRaisesRegex(mod.ProofError, "must bind exactly one cargo-metadata.json"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_single_wrong_directory_evidence_binding_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            digest_path = evidence / "evidence-files.sha256"
            digest_path.write_text(
                digest_path.read_text().replace(
                    str(evidence / "git-head.txt"), "alternate/git-head.txt"
                )
            )
            with self.assertRaisesRegex(mod.ProofError, "must bind exactly one git-head.txt"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_repeated_binary_filename_marker_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            digest_path = evidence / "cargo-metadata.sha256"
            original = digest_path.read_text()
            digest_path.write_text(original.replace("  ", "  **", 1))
            with self.assertRaisesRegex(mod.ProofError, "malformed sha256 filename marker"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_nul_sha256_filename_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            digest_path = evidence / "cargo-metadata.sha256"
            digest_path.write_bytes(digest_path.read_bytes().replace(b"cargo-metadata.json", b"cargo-metadata.json\x00", 1))
            with self.assertRaisesRegex(mod.ProofError, "NUL byte in sha256 filename"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_ascii_control_sha256_filename_fails_closed(self):
        for control in ("\x01", "\x07", "\x1b", "\x7f"):
            with self.subTest(control=hex(ord(control))):
                with tempfile.TemporaryDirectory() as td:
                    root = Path(td)
                    evidence = self.make_bundle(root)
                    digest_path = evidence / "cargo-metadata.sha256"
                    digest_path.write_text(digest_path.read_text().replace(
                        "cargo-metadata.json", "cargo-" + control + "metadata.json", 1
                    ))
                    with self.assertRaisesRegex(mod.ProofError, "control byte in sha256 filename"):
                        mod.verify(root, evidence, require_live_checkout=False)

    def test_bidirectional_control_sha256_filename_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            digest_path = evidence / "cargo-metadata.sha256"
            digest_path.write_text(digest_path.read_text().replace("cargo-metadata.json", "cargo-\u202emetadata.json", 1))
            with self.assertRaisesRegex(mod.ProofError, "bidirectional control in sha256 filename"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_bidirectional_marks_sha256_filename_fail_closed(self):
        for mark in ("\u061c", "\u200e", "\u200f"):
            with self.subTest(mark=hex(ord(mark))):
                with tempfile.TemporaryDirectory() as td:
                    root = Path(td)
                    evidence = self.make_bundle(root)
                    digest_path = evidence / "cargo-metadata.sha256"
                    digest_path.write_text(digest_path.read_text().replace("cargo-metadata.json", "cargo-" + mark + "metadata.json", 1))
                    with self.assertRaisesRegex(mod.ProofError, "bidirectional control in sha256 filename"):
                        mod.verify(root, evidence, require_live_checkout=False)

    def test_invisible_format_controls_sha256_filename_fail_closed(self):
        for mark in ("\u200b", "\u200c", "\u200d", "\u2060", "\ufeff"):
            with self.subTest(mark=hex(ord(mark))):
                with tempfile.TemporaryDirectory() as td:
                    root = Path(td)
                    evidence = self.make_bundle(root)
                    digest_path = evidence / "cargo-metadata.sha256"
                    digest_path.write_text(digest_path.read_text().replace("cargo-metadata.json", "cargo-" + mark + "metadata.json", 1))
                    with self.assertRaisesRegex(mod.ProofError, "format control in sha256 filename"):
                        mod.verify(root, evidence, require_live_checkout=False)

    def test_unicode_line_separators_sha256_filename_fail_closed(self):
        for separator in ("\u0085", "\u2028", "\u2029"):
            with self.subTest(separator=hex(ord(separator))):
                with tempfile.TemporaryDirectory() as td:
                    root = Path(td)
                    evidence = self.make_bundle(root)
                    digest_path = evidence / "cargo-metadata.sha256"
                    digest_path.write_text(digest_path.read_text().replace(
                        "cargo-metadata.json", "cargo-" + separator + "metadata.json", 1
                    ))
                    with self.assertRaisesRegex(mod.ProofError, "noncanonical line separator"):
                        mod.verify(root, evidence, require_live_checkout=False)

    def test_trailing_whitespace_sha256_filename_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            digest_path = evidence / "cargo-metadata.sha256"
            digest_path.write_text(digest_path.read_text().replace("cargo-metadata.json", "cargo-metadata.json ", 1))
            with self.assertRaisesRegex(mod.ProofError, "noncanonical sha256 filename whitespace"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_source_manifest_drift_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            (root / "Cargo.toml").write_text("[workspace]\nresolver = \"3\"\n")
            with self.assertRaises(mod.ProofError):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_symlinked_source_manifest_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            source = root / "Cargo.toml"
            external = root / "external-cargo.toml"
            source.rename(external)
            source.symlink_to(external)
            with self.assertRaisesRegex(mod.ProofError, "symlink source manifest refused"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_symlinked_source_manifest_parent_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            directory = root / "docs" / "implementation"
            moved = root / "external-implementation"
            directory.rename(moved)
            directory.symlink_to(moved, target_is_directory=True)
            with self.assertRaisesRegex(mod.ProofError, "symlink source manifest refused"):
                mod.verify(root, evidence, require_live_checkout=False)

    def test_p0_source_manifest_drift_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            evidence = self.make_bundle(root)
            (root / "docs/implementation/P0_SOURCE_MANIFEST.md").write_text("changed\n")
            with self.assertRaises(mod.ProofError):
                mod.verify(root, evidence, require_live_checkout=False)


if __name__ == "__main__":
    unittest.main()
