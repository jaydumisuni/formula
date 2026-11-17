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

    def test_rejects_canonical_fixture_with_incorrect_digest(self):
        fixture = self.fixtures / "extra.json"
        fixture.write_bytes(b'{"value":1}\n')
        fixture.with_suffix(".sha256").write_text(
            "0" * 64 + "  extra.json\n", encoding="ascii"
        )
        with self.assertRaisesRegex(AssertionError, "identity changed"):
            p0.check_fixture_identity()

    def test_rejects_nested_fixture_without_digest_sidecar(self):
        nested = self.fixtures / "cases"
        nested.mkdir()
        (nested / "extra.json").write_text("{\"value\":1}\n", encoding="utf-8")
        with self.assertRaisesRegex(AssertionError, "digest"):
            p0.check_fixture_identity()


if __name__ == "__main__":
    unittest.main()
