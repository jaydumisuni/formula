import importlib.util
import os
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("check_p0_boundaries.py")
spec = importlib.util.spec_from_file_location("p0_boundaries", SCRIPT)
p0 = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(p0)


class SealedFixtureIsolationTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        self.previous_root = p0.ROOT
        p0.ROOT = self.root
        (self.root / "Cargo.toml").write_text(
            "[workspace]\nmembers = []\n",
            encoding="utf-8",
        )

    def tearDown(self):
        p0.ROOT = self.previous_root
        self.tmp.cleanup()

    def write_crate(self, name: str, manifest_body: str = "", source: str = "") -> None:
        crate = self.root / "crates" / name
        (crate / "src").mkdir(parents=True)
        (crate / "Cargo.toml").write_text(
            f"[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2021\"\n" + manifest_body,
            encoding="utf-8",
        )
        (crate / "src" / "lib.rs").write_text(source, encoding="utf-8")

    def test_rejects_symlinked_discovery_source_file(self):
        self.write_crate("formula-engine")
        self.write_crate("formula-packages")
        outside = self.root / "outside.rs"
        outside.write_text("pub fn external() {}\n", encoding="utf-8")
        (self.root / "crates" / "formula-engine" / "src" / "external.rs").symlink_to(outside)
        with self.assertRaisesRegex(AssertionError, "source tree must not contain symlinks"):
            p0.check_sealed_fixture_isolation()

    def test_rejects_hardlinked_discovery_source_file(self):
        self.write_crate("formula-engine")
        self.write_crate("formula-packages")
        source = self.root / "crates" / "formula-engine" / "src" / "lib.rs"
        os.link(source, source.with_name("alias.rs"))
        with self.assertRaisesRegex(AssertionError, "hardlinked file"):
            p0.check_sealed_fixture_isolation()

    @unittest.skipUnless(hasattr(os, "mkfifo"), "POSIX FIFO required")
    def test_rejects_fifo_discovery_source_entry(self):
        self.write_crate("formula-engine")
        self.write_crate("formula-packages")
        os.mkfifo(self.root / "crates" / "formula-engine" / "src" / "pipe.rs")
        with self.assertRaisesRegex(AssertionError, "nonregular entry"):
            p0.check_sealed_fixture_isolation()

    def test_rejects_transitive_formula_first_light_dependency(self):
        self.write_crate(
            "formula-engine",
            "[dependencies]\nformula-core = { path = \"../formula-core\" }\n",
        )
        self.write_crate(
            "formula-core",
            "[dependencies]\nformula-first-light = { path = \"../formula-first-light\" }\n",
        )
        self.write_crate("formula-packages")
        self.write_crate("formula-first-light")

        with self.assertRaisesRegex(AssertionError, "formula-first-light"):
            p0.check_sealed_fixture_isolation()

    def test_rejects_workspace_inherited_alias_to_formula_first_light(self):
        (self.root / "Cargo.toml").write_text(
            "[workspace]\nmembers = []\n\n[workspace.dependencies]\nsealed = { package = \"formula-first-light\", path = \"crates/formula-first-light\" }\n",
            encoding="utf-8",
        )
        self.write_crate(
            "formula-engine",
            "[dependencies]\nformula-core = { path = \"../formula-core\" }\n",
        )
        self.write_crate(
            "formula-core",
            "[dependencies]\nsealed.workspace = true\n",
        )
        self.write_crate("formula-packages")
        self.write_crate("formula-first-light")

        with self.assertRaisesRegex(AssertionError, "formula-first-light"):
            p0.check_sealed_fixture_isolation()


    def test_rejects_transitive_dependency_when_directory_differs_from_package_name(self):
        self.write_crate(
            "formula-engine",
            "[dependencies]\nrenamed = { package = \"formula-core\", path = \"../odd-directory\" }\n",
        )
        self.write_crate("odd-directory", "[dependencies]\nformula-first-light = { path = \"../formula-first-light\" }\n")
        self.write_crate("formula-packages")
        self.write_crate("formula-first-light")

        with self.assertRaisesRegex(AssertionError, "formula-first-light"):
            p0.check_sealed_fixture_isolation()


if __name__ == "__main__":
    unittest.main()
