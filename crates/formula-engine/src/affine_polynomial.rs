use crate::candidate_space::{
    CandidatePolarity, CandidateSpaceContext, CompletenessClass, FrozenCandidate,
    FrozenCandidateSpace,
};
use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const SPACE_STATE_SCHEMA_V1: &str = "formula-affine-polynomial-space-v1";
const CANDIDATE_SCHEMA_V1: &str = "formula-affine-polynomial-candidate-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rational128 {
    numerator: i128,
    denominator: i128,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RationalError {
    ZeroDenominator,
    Overflow,
    DivisionByZero,
}

impl Rational128 {
    pub fn new(numerator: i128, denominator: i128) -> Result<Self, RationalError> {
        if denominator == 0 {
            return Err(RationalError::ZeroDenominator);
        }
        if numerator == 0 {
            return Ok(Self::integer(0));
        }

        let (numerator, denominator) = if denominator < 0 {
            (
                numerator.checked_neg().ok_or(RationalError::Overflow)?,
                denominator.checked_neg().ok_or(RationalError::Overflow)?,
            )
        } else {
            (numerator, denominator)
        };

        let gcd = gcd_u128(numerator.unsigned_abs(), denominator.unsigned_abs());
        let gcd = i128::try_from(gcd).map_err(|_| RationalError::Overflow)?;
        Ok(Self {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        })
    }

    pub const fn integer(value: i128) -> Self {
        Self {
            numerator: value,
            denominator: 1,
        }
    }

    fn is_zero(self) -> bool {
        self.numerator == 0
    }

    fn checked_add(self, rhs: Self) -> Result<Self, RationalError> {
        let left = self
            .numerator
            .checked_mul(rhs.denominator)
            .ok_or(RationalError::Overflow)?;
        let right = rhs
            .numerator
            .checked_mul(self.denominator)
            .ok_or(RationalError::Overflow)?;
        let numerator = left.checked_add(right).ok_or(RationalError::Overflow)?;
        let denominator = self
            .denominator
            .checked_mul(rhs.denominator)
            .ok_or(RationalError::Overflow)?;
        Self::new(numerator, denominator)
    }

    fn checked_sub(self, rhs: Self) -> Result<Self, RationalError> {
        let neg = rhs
            .numerator
            .checked_neg()
            .ok_or(RationalError::Overflow)?;
        self.checked_add(Self::new(neg, rhs.denominator)?)
    }

    fn checked_mul(self, rhs: Self) -> Result<Self, RationalError> {
        let numerator = self
            .numerator
            .checked_mul(rhs.numerator)
            .ok_or(RationalError::Overflow)?;
        let denominator = self
            .denominator
            .checked_mul(rhs.denominator)
            .ok_or(RationalError::Overflow)?;
        Self::new(numerator, denominator)
    }

    fn checked_div(self, rhs: Self) -> Result<Self, RationalError> {
        if rhs.is_zero() {
            return Err(RationalError::DivisionByZero);
        }
        let numerator = self
            .numerator
            .checked_mul(rhs.denominator)
            .ok_or(RationalError::Overflow)?;
        let denominator = self
            .denominator
            .checked_mul(rhs.numerator)
            .ok_or(RationalError::Overflow)?;
        Self::new(numerator, denominator)
    }

    fn canonical_value(self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "denominator".into(),
                CanonicalValue::String(self.denominator.to_string()),
            ),
            (
                "numerator".into(),
                CanonicalValue::String(self.numerator.to_string()),
            ),
        ]))
    }
}

fn gcd_u128(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AffinePolynomialError {
    Arithmetic(RationalError),
    EmptySpace,
    NotUnique,
}

impl From<RationalError> for AffinePolynomialError {
    fn from(value: RationalError) -> Self {
        Self::Arithmetic(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AffinePolynomialCandidate {
    frozen: FrozenCandidate,
    coefficients: Vec<Rational128>,
}

impl AffinePolynomialCandidate {
    pub fn coefficients(&self) -> &[Rational128] {
        &self.coefficients
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.frozen.digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AffinePolynomialSpace {
    context: CandidateSpaceContext,
    degree_limit: usize,
    samples: Vec<(i128, i128)>,
}

impl AffinePolynomialSpace {
    pub fn new(context: CandidateSpaceContext, degree_limit: usize) -> Self {
        Self {
            context,
            degree_limit,
            samples: Vec::new(),
        }
    }

    pub fn add_exact_sample(&mut self, x: i128, y: i128) -> Result<(), AffinePolynomialError> {
        if !self.samples.contains(&(x, y)) {
            self.samples.push((x, y));
            self.samples.sort_unstable();
        }
        Ok(())
    }

    pub fn restrict_degree(&mut self, degree_limit: usize) -> Result<(), AffinePolynomialError> {
        self.degree_limit = self.degree_limit.min(degree_limit);
        Ok(())
    }

    pub fn affine_dimension(&self) -> Result<usize, AffinePolynomialError> {
        let solved = self.solve(self.degree_limit)?;
        if solved.inconsistent {
            return Err(AffinePolynomialError::EmptySpace);
        }
        Ok(self.degree_limit + 1 - solved.rank)
    }

    pub fn empty(&self) -> Result<bool, AffinePolynomialError> {
        Ok(self.solve(self.degree_limit)?.inconsistent)
    }

    pub fn extract_min_degree_unique(
        &self,
    ) -> Result<AffinePolynomialCandidate, AffinePolynomialError> {
        for degree in 0..=self.degree_limit {
            let solved = self.solve(degree)?;
            if solved.inconsistent {
                continue;
            }
            if solved.rank != degree + 1 {
                continue;
            }
            let mut coefficients = solved
                .solution
                .ok_or(AffinePolynomialError::NotUnique)?;
            while coefficients.len() > 1 && coefficients.last() == Some(&Rational128::integer(0)) {
                coefficients.pop();
            }
            let candidate_digest = candidate_state_digest(&coefficients);
            let frozen_space = self.freeze();
            let frozen = FrozenCandidate::new(
                frozen_space.digest(),
                candidate_digest,
                coefficients.len() as u64,
            );
            return Ok(AffinePolynomialCandidate {
                frozen,
                coefficients,
            });
        }

        if self.empty()? {
            Err(AffinePolynomialError::EmptySpace)
        } else {
            Err(AffinePolynomialError::NotUnique)
        }
    }

    pub fn freeze(&self) -> FrozenCandidateSpace {
        FrozenCandidateSpace::new(
            self.context.clone(),
            "affine-polynomial-v1",
            CandidatePolarity::Exact,
            CompletenessClass::CompleteWithinBound,
            self.state_digest(),
        )
    }

    fn state_digest(&self) -> ArtifactDigest {
        let samples = self
            .samples
            .iter()
            .map(|(x, y)| {
                CanonicalValue::Object(BTreeMap::from([
                    ("x".into(), CanonicalValue::String(x.to_string())),
                    ("y".into(), CanonicalValue::String(y.to_string())),
                ]))
            })
            .collect();
        CanonicalValue::Object(BTreeMap::from([
            (
                "degree_limit".into(),
                CanonicalValue::String(self.degree_limit.to_string()),
            ),
            ("samples".into(), CanonicalValue::Array(samples)),
            (
                "schema".into(),
                CanonicalValue::String(SPACE_STATE_SCHEMA_V1.into()),
            ),
        ]))
        .digest()
    }

    fn solve(&self, degree: usize) -> Result<SolvedSystem, AffinePolynomialError> {
        let variables = degree + 1;
        if self.samples.is_empty() {
            return Ok(SolvedSystem {
                rank: 0,
                inconsistent: false,
                solution: None,
            });
        }

        let mut matrix = Vec::with_capacity(self.samples.len());
        for (x, y) in &self.samples {
            let mut row = Vec::with_capacity(variables + 1);
            let mut power = Rational128::integer(1);
            let x = Rational128::integer(*x);
            for _ in 0..variables {
                row.push(power);
                power = power.checked_mul(x)?;
            }
            row.push(Rational128::integer(*y));
            matrix.push(row);
        }

        let mut pivot_row = 0usize;
        let mut pivot_columns = Vec::new();
        for column in 0..variables {
            let Some(found_row) = (pivot_row..matrix.len())
                .find(|row| !matrix[*row][column].is_zero())
            else {
                continue;
            };
            matrix.swap(pivot_row, found_row);

            let pivot = matrix[pivot_row][column];
            for entry in column..=variables {
                matrix[pivot_row][entry] = matrix[pivot_row][entry].checked_div(pivot)?;
            }

            for row in 0..matrix.len() {
                if row == pivot_row {
                    continue;
                }
                let factor = matrix[row][column];
                if factor.is_zero() {
                    continue;
                }
                for entry in column..=variables {
                    let scaled = factor.checked_mul(matrix[pivot_row][entry])?;
                    matrix[row][entry] = matrix[row][entry].checked_sub(scaled)?;
                }
            }

            pivot_columns.push(column);
            pivot_row += 1;
            if pivot_row == matrix.len() {
                break;
            }
        }

        let inconsistent = matrix.iter().any(|row| {
            row[..variables].iter().all(|value| value.is_zero()) && !row[variables].is_zero()
        });
        if inconsistent {
            return Ok(SolvedSystem {
                rank: pivot_columns.len(),
                inconsistent: true,
                solution: None,
            });
        }

        let solution = if pivot_columns.len() == variables {
            let mut coefficients = vec![Rational128::integer(0); variables];
            for (row, column) in pivot_columns.iter().copied().enumerate() {
                coefficients[column] = matrix[row][variables];
            }
            Some(coefficients)
        } else {
            None
        };

        Ok(SolvedSystem {
            rank: pivot_columns.len(),
            inconsistent: false,
            solution,
        })
    }
}

struct SolvedSystem {
    rank: usize,
    inconsistent: bool,
    solution: Option<Vec<Rational128>>,
}

fn candidate_state_digest(coefficients: &[Rational128]) -> ArtifactDigest {
    CanonicalValue::Object(BTreeMap::from([
        (
            "coefficients".into(),
            CanonicalValue::Array(
                coefficients
                    .iter()
                    .copied()
                    .map(Rational128::canonical_value)
                    .collect(),
            ),
        ),
        (
            "schema".into(),
            CanonicalValue::String(CANDIDATE_SCHEMA_V1.into()),
        ),
    ]))
    .digest()
}
