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

    def test_rejects_nested_fixture_without_digest_sidecar(self):
        nested = self.fixtures / "cases"
        nested.mkdir()
        (nested / "extra.json").write_text("{\"value\":1}\n", encoding="utf-8")
        with self.assertRaisesRegex(AssertionError, "digest"):
            p0.check_fixture_identity()


if __name__ == "__main__":
    unittest.main()
