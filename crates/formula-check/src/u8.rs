use crate::verdict::{CheckFailure, CheckVerdict};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ByteExpr {
    X,
    Const(u8),
    SubWrap(Box<ByteExpr>, Box<ByteExpr>),
    BitAnd(Box<ByteExpr>, Box<ByteExpr>),
    BitOr(Box<ByteExpr>, Box<ByteExpr>),
    BitXor(Box<ByteExpr>, Box<ByteExpr>),
    BitNot(Box<ByteExpr>),
}

impl ByteExpr {
    pub fn evaluate(&self, input: u8) -> u8 {
        match self {
            Self::X => input,
            Self::Const(value) => *value,
            Self::SubWrap(left, right) => left.evaluate(input).wrapping_sub(right.evaluate(input)),
            Self::BitAnd(left, right) => left.evaluate(input) & right.evaluate(input),
            Self::BitOr(left, right) => left.evaluate(input) | right.evaluate(input),
            Self::BitXor(left, right) => left.evaluate(input) ^ right.evaluate(input),
            Self::BitNot(value) => !value.evaluate(input),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoolExpr {
    EqZero(ByteExpr),
    NeqZero(ByteExpr),
    And(Box<BoolExpr>, Box<BoolExpr>),
    Or(Box<BoolExpr>, Box<BoolExpr>),
    Not(Box<BoolExpr>),
}

impl BoolExpr {
    pub fn evaluate(&self, input: u8) -> bool {
        match self {
            Self::EqZero(value) => value.evaluate(input) == 0,
            Self::NeqZero(value) => value.evaluate(input) != 0,
            Self::And(left, right) => left.evaluate(input) && right.evaluate(input),
            Self::Or(left, right) => left.evaluate(input) || right.evaluate(input),
            Self::Not(value) => !value.evaluate(input),
        }
    }
}

pub fn check_u8_equivalence(candidate: &BoolExpr, specification: fn(u8) -> bool) -> CheckVerdict {
    for raw in 0u16..=u8::MAX as u16 {
        let input = raw as u8;
        if candidate.evaluate(input) != specification(input) {
            return CheckVerdict::Fail(CheckFailure::U8Counterexample(input));
        }
    }

    CheckVerdict::Pass
}
