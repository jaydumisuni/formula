use formula_core::digest::ArtifactDigest;
use formula_engine::observational::{BoolExpr, ByteExpr, FrozenExprCandidate, U8BoolGrammar};

const FL_C_TARGET_SCHEMA_V1: &str = "formula-p6-fl-c-sealed-u8-target-v1";

#[derive(Clone, Copy, Debug, Default)]
pub struct FlCOracle;

pub fn fl_c_oracle() -> FlCOracle {
    FlCOracle
}

pub fn fl_c_target_digest() -> ArtifactDigest {
    ArtifactDigest::of_bytes(FL_C_TARGET_SCHEMA_V1.as_bytes())
}

pub fn fl_c_grammar_digest() -> ArtifactDigest {
    U8BoolGrammar::minimal().digest()
}

pub fn fl_c_zero_near_miss() -> BoolExpr {
    BoolExpr::eq_zero(ByteExpr::bit_and(
        ByteExpr::x(),
        ByteExpr::sub_wrap(ByteExpr::x(), ByteExpr::one()),
    ))
}

fn sealed_expected(input: u8) -> bool {
    matches!(input, 1 | 2 | 4 | 8 | 16 | 32 | 64 | 128)
}

impl FlCOracle {
    pub fn first_counterexample(&self, candidate: &FrozenExprCandidate) -> Option<(u8, bool)> {
        for input in u8::MIN..=u8::MAX {
            let expected = sealed_expected(input);
            if candidate.eval(input) != expected {
                return Some((input, expected));
            }
        }
        None
    }
}
