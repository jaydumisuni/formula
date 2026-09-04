from pathlib import Path

path = Path("crates/formula-store/src/authority_store.rs")
text = path.read_text()

if "mod activation_store;" not in text:
    text = text.replace("mod realization_store;\n", "mod activation_store;\nmod realization_store;\n", 1)

variant_anchor = "    RealizationGenerationMismatch {\n"
variants = """    SemanticActivationStateMismatch,\n    SemanticActivationGenerationMismatch {\n        expected: ArtifactDigest,\n        actual: ArtifactDigest,\n    },\n    SemanticActivationPrimitiveNotAdmitted(ArtifactDigest),\n    SemanticActivationPrimitiveNotRecorded(ArtifactDigest),\n    SemanticActivationEvidenceNotAuthorityBound(ArtifactDigest),\n    SemanticActivationDigestMismatch {\n        stored: ArtifactDigest,\n        reconstructed: ArtifactDigest,\n    },\n"""
if "SemanticActivationStateMismatch" not in text:
    if variant_anchor not in text:
        raise SystemExit("activation variant anchor missing")
    text = text.replace(variant_anchor, variants + variant_anchor, 1)

display_anchor = "            Self::RealizationGenerationMismatch { expected, actual } => write!(\n"
display = """            Self::SemanticActivationStateMismatch => {\n                f.write_str(\"semantic activation requires ACTIVATED state\")\n            }\n            Self::SemanticActivationGenerationMismatch { expected, actual } => write!(\n                f,\n                \"semantic activation generation mismatch: expected {}, got {}\",\n                expected.as_str(),\n                actual.as_str()\n            ),\n            Self::SemanticActivationPrimitiveNotAdmitted(digest) => write!(\n                f,\n                \"semantic activation primitive is not admitted: {}\",\n                digest.as_str()\n            ),\n            Self::SemanticActivationPrimitiveNotRecorded(digest) => write!(\n                f,\n                \"semantic activation record does not contain primitive: {}\",\n                digest.as_str()\n            ),\n            Self::SemanticActivationEvidenceNotAuthorityBound(digest) => write!(\n                f,\n                \"semantic activation evidence is not authority-bound: {}\",\n                digest.as_str()\n            ),\n            Self::SemanticActivationDigestMismatch { stored, reconstructed } => write!(\n                f,\n                \"semantic activation digest mismatch: stored {}, reconstructed {}\",\n                stored.as_str(),\n                reconstructed.as_str()\n            ),\n"""
if "Self::SemanticActivationStateMismatch" not in text:
    if display_anchor not in text:
        raise SystemExit("activation display anchor missing")
    text = text.replace(display_anchor, display + display_anchor, 1)

path.write_text(text)
