//! Candidate-only Formula P1 structural identity CLI.
//!
//! The CLI uses real formula-core canonical Entity identity. It provides no
//! solver/search, proof certification, promotion or execution authority.
use formula_core::Entity;
use std::process::ExitCode;

const MAX_STRUCTURE_BYTES: usize = 32 * 1024;
const MAX_KIND_BYTES: usize = 128;

fn unhex_nibble(value: u8) -> Result<u8, &'static str> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => Err("hex must contain lowercase 0-9a-f only"),
    }
}

fn decode_bounded_hex(hex: &str) -> Result<Vec<u8>, &'static str> {
    if hex.len() % 2 != 0 || hex.len() > MAX_STRUCTURE_BYTES * 2 {
        return Err("structure must contain even lowercase hex, at most 32768 bytes");
    }
    let mut out = Vec::with_capacity(hex.len() / 2);
    for pair in hex.as_bytes().chunks_exact(2) {
        out.push((unhex_nibble(pair[0])? << 4) | unhex_nibble(pair[1])?);
    }
    Ok(out)
}

fn canonical_entity_digest(kind: &str, hex: &str) -> Result<String, &'static str> {
    if kind.is_empty()
        || kind.len() > MAX_KIND_BYTES
        || !kind
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_:-./".contains(&byte))
    {
        return Err("kind must be 1-128 ASCII letters, digits, underscore, colon, dash, dot or slash");
    }
    let structure = decode_bounded_hex(hex)?;
    Ok(Entity::new(kind, structure, Vec::new(), Vec::new())
        .structural_digest()
        .to_hex())
}

fn usage() {
    eprintln!(
        "formula-cli candidate commands:\n  capabilities\n  entity-digest <kind> <lowercase-hex-structure>\n  verify-entity-digest <kind> <lowercase-hex-structure> <64-lowercase-hex-digest>\n\nNo theorem, solver, promotion, or effect authority is conferred."
    );
}

fn run(args: &[String]) -> Result<(), &'static str> {
    match args {
        [action] if action == "capabilities" => {
            println!(
                "{{\"schema\":\"formula.cli.capabilities.v1\",\"status\":\"p1_candidate\",\"operations\":[\"entity_digest\",\"verify_entity_digest\"],\"mathematical_solver_qualified\":false,\"certification_authority\":false,\"effect_authority\":false}}"
            );
            Ok(())
        }
        [action, kind, structure] if action == "entity-digest" => {
            let digest = canonical_entity_digest(kind, structure)?;
            println!(
                "{{\"schema\":\"formula.cli.entity-digest.v1\",\"status\":\"candidate_structural_identity\",\"digest\":\"{digest}\",\"certified\":false,\"effect_authority\":false}}"
            );
            Ok(())
        }
        [action, kind, structure, digest] if action == "verify-entity-digest" => {
            let actual = canonical_entity_digest(kind, structure)?;
            if digest.len() != 64
                || !digest.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            {
                return Err("expected digest must have exactly 64 lowercase hex digits");
            }
            if actual != *digest {
                return Err("structural digest mismatch");
            }
            println!(
                "{{\"schema\":\"formula.cli.identity-check.v1\",\"matched\":true,\"certification_authority\":false,\"effect_authority\":false}}"
            );
            Ok(())
        }
        [action] if action == "help" || action == "--help" || action == "-h" => {
            usage();
            Ok(())
        }
        _ => Err("unknown command or argument count"),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("formula-cli candidate refused: {message}");
            usage();
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_semantics_have_same_identity() {
        let a = canonical_entity_digest("ttg.re:artifact", "00ff1020").unwrap();
        let b = canonical_entity_digest("ttg.re:artifact", "00ff1020").unwrap();
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
    }

    #[test]
    fn kind_changes_structural_identity() {
        let a = canonical_entity_digest("ttg.re:apk", "00ff").unwrap();
        let b = canonical_entity_digest("ttg.re:elf", "00ff").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn byte_change_changes_identity() {
        assert_ne!(
            canonical_entity_digest("ttg.re:artifact", "00fe").unwrap(),
            canonical_entity_digest("ttg.re:artifact", "00ff").unwrap()
        );
    }

    #[test]
    fn unknown_and_invalid_values_fail_closed() {
        assert!(canonical_entity_digest("bad kind", "00").is_err());
        assert!(canonical_entity_digest("kind", "0").is_err());
        assert!(canonical_entity_digest("kind", "GG").is_err());
        assert!(canonical_entity_digest("kind", &"ab".repeat(MAX_STRUCTURE_BYTES + 1)).is_err());
    }

    #[test]
    fn candidate_reports_no_mathematical_authority() {
        assert!(run(&["capabilities".into()]).is_ok());
        assert!(run(&["promote".into()]).is_err());
        assert!(run(&["prove".into()]).is_err());
    }

    #[test]
    fn digest_verification_rejects_substitution() {
        let good = canonical_entity_digest("ttg.re:artifact", "aa").unwrap();
        assert!(run(&[
            "verify-entity-digest".into(), "ttg.re:artifact".into(),
            "aa".into(), good
        ]).is_ok());
        assert!(run(&[
            "verify-entity-digest".into(), "ttg.re:artifact".into(),
            "ab".into(), "0".repeat(64)
        ]).is_err());
    }
}
