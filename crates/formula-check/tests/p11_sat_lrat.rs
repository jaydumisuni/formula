use formula_check::sat_lrat::{LratCheckError, SatCnf, check_lrat_rup_unsat};

fn contradictory_unit_cnf() -> SatCnf {
    SatCnf::from_dimacs("p cnf 1 2\n1 0\n-1 0\n").expect("valid DIMACS")
}

#[test]
fn valid_lrat_rup_empty_clause_establishes_unsat() {
    let cnf = contradictory_unit_cnf();
    let proof = check_lrat_rup_unsat(&cnf, "3 0 1 2 0\n").expect("valid RUP proof");

    assert_eq!(proof.empty_clause_id(), 3);
    assert_eq!(proof.cnf_digest(), cnf.structural_digest());
    assert_ne!(proof.evidence_digest(), cnf.structural_digest());
}

#[test]
fn forged_lrat_hint_is_rejected() {
    let cnf = contradictory_unit_cnf();
    assert_eq!(
        check_lrat_rup_unsat(&cnf, "3 0 1 99 0\n"),
        Err(LratCheckError::UnknownClauseId(99))
    );
}

#[test]
fn proof_without_empty_clause_does_not_establish_unsat() {
    let cnf = contradictory_unit_cnf();
    assert_eq!(
        check_lrat_rup_unsat(&cnf, "3 1 0 1 2 0\n"),
        Err(LratCheckError::MissingEmptyClause)
    );
}

#[test]
fn unsupported_rat_hints_fail_closed() {
    let cnf = contradictory_unit_cnf();
    assert_eq!(
        check_lrat_rup_unsat(&cnf, "3 0 -1 2 0\n"),
        Err(LratCheckError::UnsupportedRatStep)
    );
}

#[test]
fn deletion_lines_fail_closed_in_the_p11_subset() {
    let cnf = contradictory_unit_cnf();
    assert_eq!(
        check_lrat_rup_unsat(&cnf, "d 1 0\n"),
        Err(LratCheckError::UnsupportedDeletion)
    );
}

#[test]
fn dimacs_rejects_literal_outside_declared_variable_domain() {
    assert_eq!(
        SatCnf::from_dimacs("p cnf 1 1\n2 0\n"),
        Err(LratCheckError::InvalidLiteral(2))
    );
}
