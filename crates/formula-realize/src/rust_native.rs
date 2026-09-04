use formula_core::{digest::ArtifactDigest, realization::SpecializationIdentity};
use formula_engine::observational::{BoolExpr, ByteExpr};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeSourceArtifact {
    source: String,
    source_digest: ArtifactDigest,
}

impl NativeSourceArtifact {
    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn source_digest(&self) -> ArtifactDigest {
        self.source_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeGenerationError {
    SemanticTargetMismatch,
}

pub fn generate_u8_bool_rust_source(
    expression: &BoolExpr,
    specialization: &SpecializationIdentity,
) -> Result<NativeSourceArtifact, NativeGenerationError> {
    if expression.digest() != specialization.semantic_target() {
        return Err(NativeGenerationError::SemanticTargetMismatch);
    }

    let expression_source = render_bool(expression);
    let source = format!(
        "fn main() {{\n    let mut args = std::env::args();\n    let _program = args.next();\n    let Some(raw) = args.next() else {{\n        std::process::exit(2);\n    }};\n    if args.next().is_some() {{\n        std::process::exit(2);\n    }}\n    let Ok(x) = raw.parse::<u8>() else {{\n        std::process::exit(2);\n    }};\n    let result = {expression_source};\n    println!(\"{{}}\", if result {{ 1 }} else {{ 0 }});\n}}\n"
    );
    let source_digest = ArtifactDigest::of_bytes(source.as_bytes());

    Ok(NativeSourceArtifact {
        source,
        source_digest,
    })
}

fn render_bool(expression: &BoolExpr) -> String {
    match expression {
        BoolExpr::EqZero(value) => format!("({}) == 0", render_byte(value)),
        BoolExpr::NeqZero(value) => format!("({}) != 0", render_byte(value)),
        BoolExpr::And(left, right) => {
            format!("({}) && ({})", render_bool(left), render_bool(right))
        }
    }
}

fn render_byte(expression: &ByteExpr) -> String {
    match expression {
        ByteExpr::X => "x".into(),
        ByteExpr::Zero => "0u8".into(),
        ByteExpr::One => "1u8".into(),
        ByteExpr::SubWrap(left, right) => format!(
            "({}).wrapping_sub({})",
            render_byte(left),
            render_byte(right)
        ),
        ByteExpr::BitAnd(left, right) => {
            format!("({}) & ({})", render_byte(left), render_byte(right))
        }
    }
}
