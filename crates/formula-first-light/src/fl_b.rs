use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const FL_B_PROBLEM_SCHEMA_V1: &str = "formula-p6-fl-b-public-xor-system-v1";
const FL_B_DIRECT_ROUTE_SCHEMA_V1: &str = "formula-p6-fl-b-direct-boolean-route-v1";
const FL_B_GF2_ROUTE_SCHEMA_V1: &str = "formula-p6-fl-b-gf2-route-v1";
const FL_B_ROUTE_CONTRACT_SCHEMA_V1: &str =
    "formula-p6-fl-b-boolean-xor-gf2-exact-preserving-route-contract-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicXorRow {
    variables: Vec<usize>,
    rhs: bool,
}

impl PublicXorRow {
    fn new(mut variables: Vec<usize>, rhs: bool) -> Self {
        variables.sort_unstable();
        variables.dedup();
        Self { variables, rhs }
    }

    pub fn variables(&self) -> &[usize] {
        &self.variables
    }

    pub fn rhs(&self) -> bool {
        self.rhs
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "rhs".into(),
                CanonicalValue::String(if self.rhs { "1" } else { "0" }.into()),
            ),
            (
                "variables".into(),
                CanonicalValue::Array(
                    self.variables
                        .iter()
                        .map(|variable| CanonicalValue::String(variable.to_string()))
                        .collect(),
                ),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicXorSystem {
    width: usize,
    rows: Vec<PublicXorRow>,
}

impl PublicXorSystem {
    fn new(width: usize, mut rows: Vec<PublicXorRow>) -> Self {
        rows.sort_by_key(|row| row.canonical_value().digest());
        Self { width, rows }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn rows(&self) -> &[PublicXorRow] {
        &self.rows
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "rows".into(),
                CanonicalValue::Array(
                    self.rows
                        .iter()
                        .map(PublicXorRow::canonical_value)
                        .collect(),
                ),
            ),
            (
                "schema".into(),
                CanonicalValue::String(FL_B_PROBLEM_SCHEMA_V1.into()),
            ),
            (
                "width".into(),
                CanonicalValue::String(self.width.to_string()),
            ),
        ]))
    }

    fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}

pub fn fl_b_public_problem() -> PublicXorSystem {
    PublicXorSystem::new(
        24,
        vec![
            PublicXorRow::new(vec![0, 1, 2], false),
            PublicXorRow::new(vec![3, 4, 5], true),
            PublicXorRow::new(vec![6, 7, 8, 9], false),
            PublicXorRow::new(vec![10, 11, 12], false),
            PublicXorRow::new(vec![13, 14, 15], true),
            PublicXorRow::new(vec![16, 17, 18, 19], false),
            PublicXorRow::new(vec![20, 21, 22, 23], false),
            PublicXorRow::new(vec![0, 5, 10, 15, 20], true),
            PublicXorRow::new(vec![2, 7, 12, 17, 22], true),
            PublicXorRow::new(vec![4, 9, 14, 19, 23], false),
        ],
    )
}

pub fn fl_b_problem_digest() -> ArtifactDigest {
    fl_b_public_problem().digest()
}

pub fn fl_b_direct_route_digest() -> ArtifactDigest {
    ArtifactDigest::of_bytes(FL_B_DIRECT_ROUTE_SCHEMA_V1.as_bytes())
}

pub fn fl_b_gf2_route_digest() -> ArtifactDigest {
    ArtifactDigest::of_bytes(FL_B_GF2_ROUTE_SCHEMA_V1.as_bytes())
}

pub fn fl_b_route_contract_digest() -> ArtifactDigest {
    ArtifactDigest::of_bytes(FL_B_ROUTE_CONTRACT_SCHEMA_V1.as_bytes())
}
