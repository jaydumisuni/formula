use crate::candidate_space::{
    CandidatePolarity, CandidateSpaceContext, CompletenessClass, FrozenCandidate,
    FrozenCandidateSpace,
};
use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const BYTE_EXPR_SCHEMA_V1: &str = "formula-p5-byte-expr-v1";
const BOOL_EXPR_SCHEMA_V1: &str = "formula-p5-bool-expr-v1";
const GRAMMAR_SCHEMA_V1: &str = "formula-p5-u8-bool-grammar-v1";
const OBSERVATIONAL_SPACE_SCHEMA_V1: &str = "formula-p5-observational-space-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ByteExpr {
    X,
    Zero,
    One,
    SubWrap(Box<ByteExpr>, Box<ByteExpr>),
    BitAnd(Box<ByteExpr>, Box<ByteExpr>),
}

impl ByteExpr {
    pub fn x() -> Self {
        Self::X
    }

    pub fn zero() -> Self {
        Self::Zero
    }

    pub fn one() -> Self {
        Self::One
    }

    pub fn sub_wrap(left: Self, right: Self) -> Self {
        Self::SubWrap(Box::new(left), Box::new(right))
    }

    pub fn bit_and(left: Self, right: Self) -> Self {
        let (left, right) = canonical_pair(left, right);
        Self::BitAnd(Box::new(left), Box::new(right))
    }

    pub fn eval(&self, x: u8) -> u8 {
        match self {
            Self::X => x,
            Self::Zero => 0,
            Self::One => 1,
            Self::SubWrap(left, right) => left.eval(x).wrapping_sub(right.eval(x)),
            Self::BitAnd(left, right) => left.eval(x) & right.eval(x),
        }
    }

    pub fn cost(&self) -> u64 {
        match self {
            Self::X | Self::Zero | Self::One => 1,
            Self::SubWrap(left, right) | Self::BitAnd(left, right) => {
                1 + left.cost() + right.cost()
            }
        }
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }

    fn canonical_value(&self) -> CanonicalValue {
        match self {
            Self::X => leaf_value(BYTE_EXPR_SCHEMA_V1, "X"),
            Self::Zero => leaf_value(BYTE_EXPR_SCHEMA_V1, "ZERO"),
            Self::One => leaf_value(BYTE_EXPR_SCHEMA_V1, "ONE"),
            Self::SubWrap(left, right) => binary_value(
                BYTE_EXPR_SCHEMA_V1,
                "SUB_WRAP",
                left.canonical_value(),
                right.canonical_value(),
            ),
            Self::BitAnd(left, right) => binary_value(
                BYTE_EXPR_SCHEMA_V1,
                "BIT_AND",
                left.canonical_value(),
                right.canonical_value(),
            ),
        }
    }
}

fn canonical_pair(left: ByteExpr, right: ByteExpr) -> (ByteExpr, ByteExpr) {
    if left.digest() <= right.digest() {
        (left, right)
    } else {
        (right, left)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoolExpr {
    EqZero(ByteExpr),
    NeqZero(ByteExpr),
    And(Box<BoolExpr>, Box<BoolExpr>),
}

impl BoolExpr {
    pub fn eq_zero(value: ByteExpr) -> Self {
        Self::EqZero(value)
    }

    pub fn neq_zero(value: ByteExpr) -> Self {
        Self::NeqZero(value)
    }

    pub fn and(left: Self, right: Self) -> Self {
        let (left, right) = if left.digest() <= right.digest() {
            (left, right)
        } else {
            (right, left)
        };
        Self::And(Box::new(left), Box::new(right))
    }

    pub fn eval(&self, x: u8) -> bool {
        match self {
            Self::EqZero(value) => value.eval(x) == 0,
            Self::NeqZero(value) => value.eval(x) != 0,
            Self::And(left, right) => left.eval(x) && right.eval(x),
        }
    }

    pub fn cost(&self) -> u64 {
        match self {
            Self::EqZero(value) | Self::NeqZero(value) => 1 + value.cost(),
            Self::And(left, right) => 1 + left.cost() + right.cost(),
        }
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }

    fn canonical_value(&self) -> CanonicalValue {
        match self {
            Self::EqZero(value) => CanonicalValue::Object(BTreeMap::from([
                (
                    "argument".into(),
                    value.canonical_value(),
                ),
                ("kind".into(), CanonicalValue::String("EQ_ZERO".into())),
                (
                    "schema".into(),
                    CanonicalValue::String(BOOL_EXPR_SCHEMA_V1.into()),
                ),
            ])),
            Self::NeqZero(value) => CanonicalValue::Object(BTreeMap::from([
                (
                    "argument".into(),
                    value.canonical_value(),
                ),
                ("kind".into(), CanonicalValue::String("NEQ_ZERO".into())),
                (
                    "schema".into(),
                    CanonicalValue::String(BOOL_EXPR_SCHEMA_V1.into()),
                ),
            ])),
            Self::And(left, right) => binary_value(
                BOOL_EXPR_SCHEMA_V1,
                "AND",
                left.canonical_value(),
                right.canonical_value(),
            ),
        }
    }
}

fn leaf_value(schema: &str, kind: &str) -> CanonicalValue {
    CanonicalValue::Object(BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        ("schema".into(), CanonicalValue::String(schema.into())),
    ]))
}

fn binary_value(
    schema: &str,
    kind: &str,
    left: CanonicalValue,
    right: CanonicalValue,
) -> CanonicalValue {
    CanonicalValue::Object(BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        ("left".into(), left),
        ("right".into(), right),
        ("schema".into(), CanonicalValue::String(schema.into())),
    ]))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct U8BoolGrammar {
    allow_sub_wrap: bool,
    allow_bit_and: bool,
    allow_and: bool,
}

impl U8BoolGrammar {
    pub fn minimal() -> Self {
        Self {
            allow_sub_wrap: true,
            allow_bit_and: true,
            allow_and: true,
        }
    }

    pub fn digest(&self) -> ArtifactDigest {
        CanonicalValue::Object(BTreeMap::from([
            ("allow_and".into(), CanonicalValue::Bool(self.allow_and)),
            (
                "allow_bit_and".into(),
                CanonicalValue::Bool(self.allow_bit_and),
            ),
            (
                "allow_sub_wrap".into(),
                CanonicalValue::Bool(self.allow_sub_wrap),
            ),
            (
                "schema".into(),
                CanonicalValue::String(GRAMMAR_SCHEMA_V1.into()),
            ),
        ]))
        .digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BehaviorRepresentative {
    behavior: Vec<bool>,
    expression: BoolExpr,
}

impl BehaviorRepresentative {
    pub fn behavior(&self) -> &[bool] {
        &self.behavior
    }

    pub fn expression(&self) -> &BoolExpr {
        &self.expression
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenExprCandidate {
    expression: BoolExpr,
    frozen: FrozenCandidate,
}

impl FrozenExprCandidate {
    pub fn expression(&self) -> &BoolExpr {
        &self.expression
    }

    pub fn eval(&self, input: u8) -> bool {
        self.expression.eval(input)
    }

    pub fn frozen(&self) -> &FrozenCandidate {
        &self.frozen
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.frozen.digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationalExprSpace {
    context: CandidateSpaceContext,
    grammar: U8BoolGrammar,
    max_cost: u64,
    samples: Vec<(u8, bool)>,
}

impl ObservationalExprSpace {
    pub fn new(context: CandidateSpaceContext, grammar: U8BoolGrammar, max_cost: u64) -> Self {
        Self {
            context,
            grammar,
            max_cost,
            samples: Vec::new(),
        }
    }

    pub fn restrict_exact_sample(&mut self, input: u8, expected: bool) {
        if !self.samples.contains(&(input, expected)) {
            self.samples.push((input, expected));
            self.samples.sort_unstable();
        }
    }

    pub fn refine_counterexample(&mut self, input: u8, expected: bool) {
        self.restrict_exact_sample(input, expected);
    }

    pub fn empty(&self) -> bool {
        self.behavior_representatives().is_empty()
    }

    pub fn behavior_representatives(&self) -> Vec<BehaviorRepresentative> {
        let expressions = self.generated_bool_expressions();
        let mut by_behavior: BTreeMap<Vec<bool>, BoolExpr> = BTreeMap::new();
        for expression in expressions {
            let behavior: Vec<bool> = self
                .samples
                .iter()
                .map(|(input, _)| expression.eval(*input))
                .collect();
            if self
                .samples
                .iter()
                .zip(behavior.iter())
                .any(|((_, expected), actual)| expected != actual)
            {
                continue;
            }
            match by_behavior.get(&behavior) {
                Some(existing)
                    if (existing.cost(), existing.digest())
                        <= (expression.cost(), expression.digest()) => {}
                _ => {
                    by_behavior.insert(behavior, expression);
                }
            }
        }
        by_behavior
            .into_iter()
            .map(|(behavior, expression)| BehaviorRepresentative {
                behavior,
                expression,
            })
            .collect()
    }

    pub fn partition_by_behavior(&self) -> BTreeMap<Vec<bool>, ArtifactDigest> {
        self.behavior_representatives()
            .into_iter()
            .map(|entry| (entry.behavior, entry.expression.digest()))
            .collect()
    }

    pub fn extract_min_cost(&self) -> Option<FrozenExprCandidate> {
        let mut expressions: Vec<BoolExpr> = self
            .behavior_representatives()
            .into_iter()
            .map(|entry| entry.expression)
            .collect();
        expressions.sort_by_key(|expression| (expression.cost(), expression.digest()));
        let expression = expressions.into_iter().next()?;
        let frozen_space = self.freeze();
        let frozen = FrozenCandidate::new(
            frozen_space.digest(),
            expression.digest(),
            expression.cost(),
        );
        Some(FrozenExprCandidate { expression, frozen })
    }

    pub fn freeze(&self) -> FrozenCandidateSpace {
        FrozenCandidateSpace::new(
            self.context.clone(),
            "observational-u8-bool-v1",
            CandidatePolarity::Exact,
            CompletenessClass::CompleteWithinBound,
            self.state_digest(),
        )
    }

    fn state_digest(&self) -> ArtifactDigest {
        CanonicalValue::Object(BTreeMap::from([
            (
                "grammar_digest".into(),
                CanonicalValue::Digest(self.grammar.digest()),
            ),
            (
                "max_cost".into(),
                CanonicalValue::String(self.max_cost.to_string()),
            ),
            (
                "samples".into(),
                CanonicalValue::Array(
                    self.samples
                        .iter()
                        .map(|(input, expected)| {
                            CanonicalValue::Object(BTreeMap::from([
                                (
                                    "expected".into(),
                                    CanonicalValue::Bool(*expected),
                                ),
                                (
                                    "input".into(),
                                    CanonicalValue::String(input.to_string()),
                                ),
                            ]))
                        })
                        .collect(),
                ),
            ),
            (
                "schema".into(),
                CanonicalValue::String(OBSERVATIONAL_SPACE_SCHEMA_V1.into()),
            ),
        ]))
        .digest()
    }

    fn generated_bool_expressions(&self) -> Vec<BoolExpr> {
        let max_cost = usize::try_from(self.max_cost).unwrap_or(usize::MAX);
        let mut byte_by_cost: Vec<Vec<ByteExpr>> = vec![Vec::new(); max_cost.saturating_add(1)];
        let mut bool_by_cost: Vec<Vec<BoolExpr>> = vec![Vec::new(); max_cost.saturating_add(1)];

        if max_cost >= 1 {
            byte_by_cost[1] = vec![ByteExpr::x(), ByteExpr::zero(), ByteExpr::one()];
        }

        for cost in 2..=max_cost {
            let mut bytes: BTreeMap<ArtifactDigest, ByteExpr> = BTreeMap::new();
            if cost >= 3 {
                for left_cost in 1..cost - 1 {
                    let right_cost = cost - 1 - left_cost;
                    if right_cost == 0 {
                        continue;
                    }
                    for left in &byte_by_cost[left_cost] {
                        for right in &byte_by_cost[right_cost] {
                            if self.grammar.allow_sub_wrap {
                                let expression = ByteExpr::sub_wrap(left.clone(), right.clone());
                                bytes.entry(expression.digest()).or_insert(expression);
                            }
                            if self.grammar.allow_bit_and {
                                let expression = ByteExpr::bit_and(left.clone(), right.clone());
                                bytes.entry(expression.digest()).or_insert(expression);
                            }
                        }
                    }
                }
            }
            byte_by_cost[cost] = bytes.into_values().collect();

            let mut bools: BTreeMap<ArtifactDigest, BoolExpr> = BTreeMap::new();
            if cost >= 2 {
                for byte in &byte_by_cost[cost - 1] {
                    let eq = BoolExpr::eq_zero(byte.clone());
                    bools.entry(eq.digest()).or_insert(eq);
                    let neq = BoolExpr::neq_zero(byte.clone());
                    bools.entry(neq.digest()).or_insert(neq);
                }
            }
            if self.grammar.allow_and && cost >= 5 {
                for left_cost in 2..cost - 2 {
                    let right_cost = cost - 1 - left_cost;
                    if right_cost < 2 {
                        continue;
                    }
                    for left in &bool_by_cost[left_cost] {
                        for right in &bool_by_cost[right_cost] {
                            let expression = BoolExpr::and(left.clone(), right.clone());
                            bools.entry(expression.digest()).or_insert(expression);
                        }
                    }
                }
            }
            bool_by_cost[cost] = bools.into_values().collect();
        }

        let mut all: Vec<BoolExpr> = bool_by_cost.into_iter().flatten().collect();
        all.sort_by_key(|expression| (expression.cost(), expression.digest()));
        all
    }
}
