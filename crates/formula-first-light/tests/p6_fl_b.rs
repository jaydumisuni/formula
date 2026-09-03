use formula_first_light::fl_b::{
    fl_b_direct_route_digest, fl_b_gf2_route_digest, fl_b_problem_digest, fl_b_public_problem,
    fl_b_route_contract_digest,
};

fn private_witness() -> Vec<bool> {
    (0..24).map(|index| index % 2 == 0).collect()
}

#[test]
fn public_fl_b_fixture_is_deterministic_and_satisfiable() {
    let problem = fl_b_public_problem();
    let fresh = fl_b_public_problem();
    assert_eq!(problem, fresh);
    assert_eq!(problem.width(), 24);
    assert!(!problem.rows().is_empty());
    assert_eq!(fl_b_problem_digest(), fl_b_problem_digest());

    let witness = private_witness();
    for row in problem.rows() {
        let lhs = row
            .variables()
            .iter()
            .fold(false, |acc, &variable| acc ^ witness[variable]);
        assert_eq!(lhs, row.rhs());
    }
}

#[test]
fn public_route_identities_are_distinct_from_problem_and_each_other() {
    let problem = fl_b_problem_digest();
    let direct = fl_b_direct_route_digest();
    let gf2 = fl_b_gf2_route_digest();
    let contract = fl_b_route_contract_digest();

    assert_ne!(problem, direct);
    assert_ne!(problem, gf2);
    assert_ne!(problem, contract);
    assert_ne!(direct, gf2);
    assert_ne!(direct, contract);
    assert_ne!(gf2, contract);
}
