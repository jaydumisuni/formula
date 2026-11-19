import importlib.util
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("check_p0_boundaries.py")
spec = importlib.util.spec_from_file_location("p0_boundaries", SCRIPT)
p0 = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(p0)


class FixtureIdentityTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        self.previous_root = p0.ROOT
        p0.ROOT = self.root
        self.fixtures = self.root / "tests" / "authority-boundary" / "fixtures"
        self.fixtures.mkdir(parents=True)

    def tearDown(self):
        p0.ROOT = self.previous_root
        self.tmp.cleanup()

    def test_rejects_orphan_digest_sidecar(self):
        (self.fixtures / "orphan.sha256").write_text("0" * 64 + "  orphan.json\n", encoding="ascii")
        with self.assertRaisesRegex(AssertionError, "sidecar has no fixture"):
            p0.check_fixture_identity()

    def test_rejects_fixture_without_digest_sidecar(self):
        (self.fixtures / "extra.json").write_text("{\"value\":1}\n", encoding="utf-8")
        with self.assertRaisesRegex(AssertionError, "digest"):
            p0.check_fixture_identity()

    def test_rejects_digest_sidecar_with_extra_fields(self):
        import hashlib
        fixture = self.fixtures / "extra.json"
        raw = b'{"value":1}\n'
        fixture.write_bytes(raw)
        fixture.with_suffix(".sha256").write_text(
            hashlib.sha256(raw).hexdigest() + "  wrong.json\n", encoding="ascii"
        )
        with self.assertRaisesRegex(AssertionError, "canonical SHA-256"):
            p0.check_fixture_identity()

    def test_rejects_uppercase_digest_sidecar(self):
        import hashlib
        fixture = self.fixtures / "extra.json"
        raw = b'{"value":1}\n'
        fixture.write_bytes(raw)
        fixture.with_suffix(".sha256").write_text(
            hashlib.sha256(raw).hexdigest().upper() + "  extra.json\n", encoding="ascii"
        )
        with self.assertRaisesRegex(AssertionError, "canonical SHA-256"):
            p0.check_fixture_identity()

    def test_accepts_canonical_fixture_with_matching_digest(self):
        import hashlib
        fixture = self.fixtures / "extra.json"
        raw = b'{"value":1}\n'
        fixture.write_bytes(raw)
        fixture.with_suffix(".sha256").write_text(
            hashlib.sha256(raw).hexdigest() + "  extra.json\n", encoding="ascii"
        )
        p0.check_fixture_identity()

    def test_rejects_nonfinite_json_constant_even_with_matching_digest(self):
        import hashlib
        fixture = self.fixtures / "extra.json"
        raw = b'{"value":NaN}\n'
        fixture.write_bytes(raw)
        fixture.with_suffix(".sha256").write_text(
            hashlib.sha256(raw).hexdigest() + "  extra.json\n", encoding="ascii"
        )
        with self.assertRaisesRegex(AssertionError, "nonfinite JSON constant"):
            p0.check_fixture_identity()

    def test_rejects_canonical_fixture_with_incorrect_digest(self):
        fixture = self.fixtures / "extra.json"
        fixture.write_bytes(b'{"value":1}\n')
        fixture.with_suffix(".sha256").write_text(
            "0" * 64 + "  extra.json\n", encoding="ascii"
        )
        with self.assertRaisesRegex(AssertionError, "identity changed"):
            p0.check_fixture_identity()

    def test_rejects_symlinked_fixture(self):
        import hashlib
        external = self.root / "external.json"
        raw = b'{"value":1}\n'
        external.write_bytes(raw)
        fixture = self.fixtures / "extra.json"
        fixture.symlink_to(external)
        fixture.with_suffix(".sha256").write_text(
            hashlib.sha256(raw).hexdigest() + "  extra.json\n", encoding="ascii"
        )
        with self.assertRaisesRegex(AssertionError, "fixture tree must not contain symlinks"):
            p0.check_fixture_identity()

    def test_rejects_symlinked_digest_sidecar(self):
        import hashlib
        fixture = self.fixtures / "extra.json"
        raw = b'{"value":1}\n'
        fixture.write_bytes(raw)
        external = self.root / "external.sha256"
        external.write_text(
            hashlib.sha256(raw).hexdigest() + "  extra.json\n", encoding="ascii"
        )
        fixture.with_suffix(".sha256").symlink_to(external)
        with self.assertRaisesRegex(AssertionError, "fixture tree must not contain symlinks"):
            p0.check_fixture_identity()

    def test_rejects_symlinked_fixture_root_directory(self):
        import hashlib
        real = self.root / "real-fixtures"
        real.mkdir()
        raw = b'{"value":1}\n'
        (real / "extra.json").write_bytes(raw)
        (real / "extra.sha256").write_text(
            hashlib.sha256(raw).hexdigest() + "  extra.json\n", encoding="ascii"
        )
        self.fixtures.rmdir()
        self.fixtures.symlink_to(real, target_is_directory=True)
        with self.assertRaisesRegex(AssertionError, "fixture directory must not be a symlink"):
            p0.check_fixture_identity()

    def test_rejects_symlinked_fixture_parent_directory(self):
        import hashlib
        real = self.root / "real-authority"
        real.mkdir()
        fixture_root = real / "fixtures"
        fixture_root.mkdir()
        raw = b'{"value":1}\n'
        (fixture_root / "extra.json").write_bytes(raw)
        (fixture_root / "extra.sha256").write_text(
            hashlib.sha256(raw).hexdigest() + "  extra.json\n", encoding="ascii"
        )
        self.fixtures.rmdir()
        parent = self.root / "tests" / "authority-boundary"
        parent.rmdir()
        parent.symlink_to(real, target_is_directory=True)
        with self.assertRaisesRegex(AssertionError, "fixture directory ancestor must not be a symlink"):
            p0.check_fixture_identity()

    def test_rejects_symlinked_nested_fixture_directory(self):
        real = self.root / "external-cases"
        real.mkdir()
        (self.fixtures / "cases").symlink_to(real, target_is_directory=True)
        with self.assertRaisesRegex(AssertionError, "fixture tree must not contain symlinks"):
            p0.check_fixture_identity()

    def test_rejects_unrelated_symlink_in_fixture_tree(self):
        external = self.root / "outside.txt"
        external.write_text("external", encoding="utf-8")
        (self.fixtures / "notes.txt").symlink_to(external)
        with self.assertRaisesRegex(AssertionError, "fixture tree must not contain symlinks"):
            p0.check_fixture_identity()

    def test_rejects_unsealed_regular_file_in_fixture_tree(self):
        (self.fixtures / "untracked-notes.txt").write_text("hidden fixture input", encoding="utf-8")
        with self.assertRaisesRegex(AssertionError, "unexpected file"):
            p0.check_fixture_identity()

    def test_rejects_nested_unsealed_regular_file(self):
        nested = self.fixtures / "cases"
        nested.mkdir()
        (nested / "payload.bin").write_bytes(b"unsealed")
        with self.assertRaisesRegex(AssertionError, "unexpected file"):
            p0.check_fixture_identity()

    def test_rejects_nonfixture_fifo_in_sealed_tree(self):
        import os
        os.mkfifo(self.fixtures / "notes.pipe")
        with self.assertRaisesRegex(AssertionError, "nonregular entry"):
            p0.check_fixture_identity()

    def test_rejects_fifo_fixture_before_read(self):
        import os
        fifo = self.fixtures / "pipe.json"
        os.mkfifo(fifo)
        with self.assertRaisesRegex(AssertionError, "nonregular entry"):
            p0.check_fixture_identity()

    def test_rejects_hardlinked_fixture_even_with_matching_digest(self):
        import hashlib
        import os
        external = self.root / "external.json"
        raw = b'{"value":1}\n'
        external.write_bytes(raw)
        fixture = self.fixtures / "extra.json"
        os.link(external, fixture)
        fixture.with_suffix(".sha256").write_text(
            hashlib.sha256(raw).hexdigest() + "  extra.json\n", encoding="ascii"
        )
        with self.assertRaisesRegex(AssertionError, "fixture must not be hardlinked"):
            p0.check_fixture_identity()

    def test_rejects_hardlinked_digest_sidecar_even_with_matching_digest(self):
        import hashlib
        import os
        raw = b'{"value":1}\n'
        fixture = self.fixtures / "extra.json"
        fixture.write_bytes(raw)
        external = self.root / "external.sha256"
        external.write_text(
            hashlib.sha256(raw).hexdigest() + "  extra.json\n", encoding="ascii"
        )
        os.link(external, fixture.with_suffix(".sha256"))
        with self.assertRaisesRegex(AssertionError, "sidecar must not be hardlinked"):
            p0.check_fixture_identity()

    def test_rejects_nested_hardlinked_digest_sidecar(self):
        import hashlib
        import os
        nested = self.fixtures / "cases"
        nested.mkdir()
        raw = b'{"value":1}\n'
        (nested / "extra.json").write_bytes(raw)
        external = self.root / "external.sha256"
        external.write_text(
            hashlib.sha256(raw).hexdigest() + "  extra.json\n", encoding="ascii"
        )
        os.link(external, nested / "extra.sha256")
        with self.assertRaisesRegex(AssertionError, "sidecar must not be hardlinked"):
            p0.check_fixture_identity()

    def test_rejects_nested_fixture_without_digest_sidecar(self):
        nested = self.fixtures / "cases"
        nested.mkdir()
        (nested / "extra.json").write_text("{\"value\":1}\n", encoding="utf-8")
        with self.assertRaisesRegex(AssertionError, "digest"):
            p0.check_fixture_identity()


if __name__ == "__main__":
    unittest.main()
