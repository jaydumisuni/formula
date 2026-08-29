use crate::verdict::{CheckFailure, CheckVerdict};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BooleanXorRow {
    variables: Vec<usize>,
    rhs: bool,
}

impl BooleanXorRow {
    pub fn new(variables: Vec<usize>, rhs: bool) -> Self {
        Self { variables, rhs }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gf2Row {
    variables: Vec<usize>,
    rhs: bool,
}

impl Gf2Row {
    pub fn new(variables: Vec<usize>, rhs: bool) -> Self {
        Self { variables, rhs }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BooleanXorSystem {
    width: usize,
    rows: Vec<BooleanXorRow>,
}

impl BooleanXorSystem {
    pub fn new(width: usize, rows: Vec<BooleanXorRow>) -> Self {
        Self { width, rows }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gf2System {
    width: usize,
    rows: Vec<Gf2Row>,
}

impl Gf2System {
    pub fn new(width: usize, rows: Vec<Gf2Row>) -> Self {
        Self { width, rows }
    }
}

fn canonical_variables(
    variables: &[usize],
    width: usize,
) -> Result<Vec<usize>, CheckFailure> {
    if variables.iter().any(|&index| index >= width) {
        return Err(CheckFailure::InvalidConstraint);
    }

    let mut sorted = variables.to_vec();
    sorted.sort_unstable();
    let mut canonical = Vec::new();
    let mut index = 0;
    while index < sorted.len() {
        let variable = sorted[index];
        let mut count = 1usize;
        index += 1;
        while index < sorted.len() && sorted[index] == variable {
            count += 1;
            index += 1;
        }
        if count % 2 == 1 {
            canonical.push(variable);
        }
    }
    Ok(canonical)
}

fn canonical_boolean_rows(
    system: &BooleanXorSystem,
) -> Result<Vec<(Vec<usize>, bool)>, CheckFailure> {
    let mut rows = system
        .rows
        .iter()
        .map(|row| {
            Ok((
                canonical_variables(&row.variables, system.width)?,
                row.rhs,
            ))
        })
        .collect::<Result<Vec<_>, CheckFailure>>()?;
    rows.sort_unstable();
    rows.dedup();
    Ok(rows)
}

fn canonical_gf2_rows(system: &Gf2System) -> Result<Vec<(Vec<usize>, bool)>, CheckFailure> {
    let mut rows = system
        .rows
        .iter()
        .map(|row| {
            Ok((
                canonical_variables(&row.variables, system.width)?,
                row.rhs,
            ))
        })
        .collect::<Result<Vec<_>, CheckFailure>>()?;
    rows.sort_unstable();
    rows.dedup();
    Ok(rows)
}

pub fn check_gf2_witness(
    problem: &BooleanXorSystem,
    translated: &Gf2System,
    witness: &[bool],
) -> CheckVerdict {
    if problem.width != translated.width || witness.len() != problem.width {
        return CheckVerdict::Fail(CheckFailure::WitnessWidthMismatch);
    }

    let original = match canonical_boolean_rows(problem) {
        Ok(rows) => rows,
        Err(failure) => return CheckVerdict::Fail(failure),
    };
    let claimed = match canonical_gf2_rows(translated) {
        Ok(rows) => rows,
        Err(failure) => return CheckVerdict::Fail(failure),
    };

    if original != claimed {
        return CheckVerdict::Fail(CheckFailure::TranslationMismatch);
    }

    for (variables, rhs) in &original {
        let lhs = variables
            .iter()
            .fold(false, |acc, &variable| acc ^ witness[variable]);
        if lhs != *rhs {
            return CheckVerdict::Fail(CheckFailure::WitnessMismatch);
        }
    }

    CheckVerdict::Pass
}
