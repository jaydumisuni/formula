import importlib.util
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("check_p0_boundaries.py")
spec = importlib.util.spec_from_file_location("p0_boundaries", SCRIPT)
p0 = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(p0)


class CheckerIsolationTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        crate = self.root / "crates" / "formula-check"
        crate.mkdir(parents=True)
        self.manifest = crate / "Cargo.toml"
        self.previous_root = p0.ROOT
        p0.ROOT = self.root

    def tearDown(self):
        p0.ROOT = self.previous_root
        self.tmp.cleanup()

    def test_rejects_aliased_formula_engine_dependency(self):
        self.manifest.write_text(
            "[dependencies]\nsearch-backend = { package = \"formula-engine\", path = \"../formula-engine\" }\n",
            encoding="utf-8",
        )
        with self.assertRaisesRegex(AssertionError, "formula-engine"):
            p0.check_checker_isolation()

    def test_rejects_target_specific_formula_engine_dependency(self):
        self.manifest.write_text(
            "[target.\"cfg(unix)\".dependencies]\nformula-engine = { path = \"../formula-engine\" }\n",
            encoding="utf-8",
        )
        with self.assertRaisesRegex(AssertionError, "formula-engine"):
            p0.check_checker_isolation()


if __name__ == "__main__":
    unittest.main()
