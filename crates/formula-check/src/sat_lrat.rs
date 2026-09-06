use formula_core::{
    artifacts::StructuralIdentity,
    canonical::CanonicalValue,
    digest::ArtifactDigest,
};
use num_bigint::BigInt;
use std::collections::{BTreeMap, BTreeSet};

const SAT_CNF_SCHEMA_V1: &str = "formula-sat-cnf-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LratCheckError {
    MalformedDimacs,
    MissingDimacsHeader,
    DuplicateDimacsHeader,
    ClauseCountMismatch,
    UnterminatedClause,
    InvalidLiteral(i32),
    MalformedLrat,
    InvalidClauseId(u64),
    DuplicateClauseId(u64),
    UnknownClauseId(u64),
    InvalidRupStep(u64),
    UnsupportedRatStep,
    UnsupportedDeletion,
    MissingEmptyClause,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SatCnf {
    variable_count: u32,
    clauses: Vec<Vec<i32>>,
}

impl SatCnf {
    pub fn from_dimacs(input: &str) -> Result<Self, LratCheckError> {
        let mut variable_count = None;
        let mut declared_clause_count = None;
        let mut clause_tokens = Vec::new();

        for line in input.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('c') {
                continue;
            }
            if trimmed.starts_with('p') {
                if variable_count.is_some() {
                    return Err(LratCheckError::DuplicateDimacsHeader);
                }
                let parts: Vec<_> = trimmed.split_whitespace().collect();
                if parts.len() != 4 || parts[0] != "p" || parts[1] != "cnf" {
                    return Err(LratCheckError::MalformedDimacs);
                }
                variable_count = Some(
                    parts[2]
                        .parse::<u32>()
                        .map_err(|_| LratCheckError::MalformedDimacs)?,
                );
                declared_clause_count = Some(
                    parts[3]
                        .parse::<usize>()
                        .map_err(|_| LratCheckError::MalformedDimacs)?,
                );
                continue;
            }

            if variable_count.is_none() {
                return Err(LratCheckError::MissingDimacsHeader);
            }
            for token in trimmed.split_whitespace() {
                clause_tokens.push(
                    token
                        .parse::<i32>()
                        .map_err(|_| LratCheckError::MalformedDimacs)?,
                );
            }
        }

        let variable_count = variable_count.ok_or(LratCheckError::MissingDimacsHeader)?;
        let declared_clause_count =
            declared_clause_count.ok_or(LratCheckError::MissingDimacsHeader)?;
        let mut clauses = Vec::new();
        let mut current = Vec::new();
        for literal in clause_tokens {
            if literal == 0 {
                clauses.push(std::mem::take(&mut current));
                continue;
            }
            validate_literal(literal, variable_count)?;
            current.push(literal);
        }
        if !current.is_empty() {
            return Err(LratCheckError::UnterminatedClause);
        }
        if clauses.len() != declared_clause_count {
            return Err(LratCheckError::ClauseCountMismatch);
        }

        Ok(Self {
            variable_count,
            clauses,
        })
    }

    pub fn variable_count(&self) -> u32 {
        self.variable_count
    }

    pub fn clauses(&self) -> &[Vec<i32>] {
        &self.clauses
    }

    pub fn structural_digest(&self) -> ArtifactDigest {
        StructuralIdentity::structural_digest(self)
    }
}

impl StructuralIdentity for SatCnf {
    fn canonical_value(&self) -> CanonicalValue {
        let mut normalized_clauses: Vec<Vec<i32>> = self
            .clauses
            .iter()
            .map(|clause| {
                let mut clause = clause.clone();
                clause.sort_unstable();
                clause.dedup();
                clause
            })
            .collect();
        normalized_clauses.sort();

        let clauses = normalized_clauses
            .into_iter()
            .map(|clause| {
                CanonicalValue::Array(
                    clause
                        .into_iter()
                        .map(|literal| CanonicalValue::Integer(BigInt::from(literal)))
                        .collect(),
                )
            })
            .collect();

        CanonicalValue::Object(BTreeMap::from([
            (
                "kind".into(),
                CanonicalValue::String("SatCnf".into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(SAT_CNF_SCHEMA_V1.into()),
            ),
            (
                "variable_count".into(),
                CanonicalValue::Integer(BigInt::from(self.variable_count)),
            ),
            ("clauses".into(), CanonicalValue::Array(clauses)),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LratProof {
    cnf_digest: ArtifactDigest,
    evidence_digest: ArtifactDigest,
    empty_clause_id: u64,
}

impl LratProof {
    pub fn cnf_digest(&self) -> ArtifactDigest {
        self.cnf_digest
    }

    pub fn evidence_digest(&self) -> ArtifactDigest {
        self.evidence_digest
    }

    pub fn empty_clause_id(&self) -> u64 {
        self.empty_clause_id
    }
}

pub fn check_lrat_rup_unsat(cnf: &SatCnf, proof_text: &str) -> Result<LratProof, LratCheckError> {
    let mut clauses: BTreeMap<u64, Vec<i32>> = cnf
        .clauses
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, clause)| ((index + 1) as u64, clause))
        .collect();
    let mut known_ids: BTreeSet<u64> = clauses.keys().copied().collect();
    let mut empty_clause_id = None;

    for line in proof_text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('c') {
            continue;
        }
        if trimmed.starts_with('d') {
            return Err(LratCheckError::UnsupportedDeletion);
        }

        let values: Vec<i64> = trimmed
            .split_whitespace()
            .map(|token| token.parse::<i64>().map_err(|_| LratCheckError::MalformedLrat))
            .collect::<Result<_, _>>()?;
        if values.len() < 3 {
            return Err(LratCheckError::MalformedLrat);
        }

        let raw_id = values[0];
        if raw_id <= 0 {
            return Err(LratCheckError::InvalidClauseId(raw_id.unsigned_abs()));
        }
        let id = raw_id as u64;
        if !known_ids.insert(id) {
            return Err(LratCheckError::DuplicateClauseId(id));
        }

        let first_zero = values[1..]
            .iter()
            .position(|value| *value == 0)
            .map(|index| index + 1)
            .ok_or(LratCheckError::MalformedLrat)?;
        let second_zero = values[first_zero + 1..]
            .iter()
            .position(|value| *value == 0)
            .map(|index| index + first_zero + 1)
            .ok_or(LratCheckError::MalformedLrat)?;
        if second_zero != values.len() - 1 {
            return Err(LratCheckError::MalformedLrat);
        }

        let mut new_clause = Vec::new();
        for raw_literal in &values[1..first_zero] {
            let literal = i32::try_from(*raw_literal).map_err(|_| LratCheckError::MalformedLrat)?;
            validate_literal(literal, cnf.variable_count)?;
            new_clause.push(literal);
        }

        let mut hints = Vec::new();
        for raw_hint in &values[first_zero + 1..second_zero] {
            if *raw_hint < 0 {
                return Err(LratCheckError::UnsupportedRatStep);
            }
            if *raw_hint == 0 {
                return Err(LratCheckError::MalformedLrat);
            }
            let hint = u64::try_from(*raw_hint).map_err(|_| LratCheckError::MalformedLrat)?;
            if !clauses.contains_key(&hint) {
                return Err(LratCheckError::UnknownClauseId(hint));
            }
            hints.push(hint);
        }
        if hints.is_empty() || !rup_holds(&new_clause, &hints, &clauses) {
            return Err(LratCheckError::InvalidRupStep(id));
        }

        if new_clause.is_empty() {
            empty_clause_id = Some(id);
        }
        clauses.insert(id, new_clause);
    }

    let empty_clause_id = empty_clause_id.ok_or(LratCheckError::MissingEmptyClause)?;
    let cnf_digest = cnf.structural_digest();
    let evidence_digest = ArtifactDigest::of_bytes(
        format!("{}\n{}", cnf_digest.as_str(), proof_text).as_bytes(),
    );
    Ok(LratProof {
        cnf_digest,
        evidence_digest,
        empty_clause_id,
    })
}

fn validate_literal(literal: i32, variable_count: u32) -> Result<(), LratCheckError> {
    if literal == 0 || literal.unsigned_abs() > variable_count {
        return Err(LratCheckError::InvalidLiteral(literal));
    }
    Ok(())
}

fn rup_holds(
    new_clause: &[i32],
    hints: &[u64],
    clauses: &BTreeMap<u64, Vec<i32>>,
) -> bool {
    let mut assignment: BTreeMap<u32, bool> = BTreeMap::new();

    for literal in new_clause {
        let variable = literal.unsigned_abs();
        let value_that_falsifies_literal = *literal < 0;
        if let Some(existing) = assignment.insert(variable, value_that_falsifies_literal)
            && existing != value_that_falsifies_literal
        {
            return true;
        }
    }

    for hint in hints {
        let clause = &clauses[hint];
        let mut unassigned = None;
        let mut multiple_unassigned = false;
        let mut satisfied = false;

        for literal in clause {
            let variable = literal.unsigned_abs();
            match assignment.get(&variable) {
                Some(value) => {
                    let literal_is_true = if *literal > 0 { *value } else { !*value };
                    if literal_is_true {
                        satisfied = true;
                        break;
                    }
                }
                None => {
                    if unassigned.is_some() {
                        multiple_unassigned = true;
                    } else {
                        unassigned = Some(*literal);
                    }
                }
            }
        }

        if satisfied || multiple_unassigned {
            return false;
        }
        let Some(unit_literal) = unassigned else {
            return true;
        };
        let variable = unit_literal.unsigned_abs();
        let required_value = unit_literal > 0;
        if let Some(existing) = assignment.insert(variable, required_value)
            && existing != required_value
        {
            return true;
        }
    }

    false
}
