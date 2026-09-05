from pathlib import Path

path = Path("crates/formula-store/src/authority_store.rs")
text = path.read_text()
old = "pub use realization_store::AdmittedRealization;"
new = "pub use realization_store::{AdmittedRealization, RealizationUpgradeError};"
if text.count(old) != 1:
    raise SystemExit(f"expected exactly one re-export anchor, got {text.count(old)}")
path.write_text(text.replace(old, new, 1))
