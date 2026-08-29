use formula_check::{
    gf2::{
        check_gf2_witness, BooleanXorRow, BooleanXorSystem, Gf2Row, Gf2System,
    },
    verdict::CheckVerdict,
};

fn problem() -> BooleanXorSystem {
    BooleanXorSystem::new(
        3,
        vec![
            BooleanXorRow::new(vec![0, 1], true),
            BooleanXorRow::new(vec![1, 2], false),
        ],
    )
}

fn exact_translation() -> Gf2System {
    Gf2System::new(
        3,
        vec![
            // Order and even duplicate occurrences are non-semantic over GF(2).
            Gf2Row::new(vec![1, 2, 0, 2], true),
            Gf2Row::new(vec![2, 1], false),
        ],
    )
}

#[test]
fn exact_translation_and_valid_witness_pass() {
    let witness = [true, false, false];
    assert_eq!(
        check_gf2_witness(&problem(), &exact_translation(), &witness),
        CheckVerdict::Pass
    );
}

#[test]
fn changed_rhs_translation_fails() {
    let translated = Gf2System::new(
        3,
        vec![
            Gf2Row::new(vec![0, 1], false),
            Gf2Row::new(vec![1, 2], false),
        ],
    );
    assert_ne!(
        check_gf2_witness(&problem(), &translated, &[false, false, false]),
        CheckVerdict::Pass
    );
}

#[test]
fn missing_variable_in_translated_row_fails() {
    let translated = Gf2System::new(
        3,
        vec![
            Gf2Row::new(vec![0], true),
            Gf2Row::new(vec![1, 2], false),
        ],
    );
    assert_ne!(
        check_gf2_witness(&problem(), &translated, &[true, false, false]),
        CheckVerdict::Pass
    );
}

#[test]
fn witness_bit_outside_declared_width_fails() {
    assert_ne!(
        check_gf2_witness(
            &problem(),
            &exact_translation(),
            &[true, false, false, true],
        ),
        CheckVerdict::Pass
    );
}

#[test]
fn witness_that_satisfies_claimed_gf2_but_not_original_boolean_problem_fails() {
    let translated = Gf2System::new(
        3,
        vec![
            Gf2Row::new(vec![0, 1], false),
            Gf2Row::new(vec![1, 2], false),
        ],
    );
    let translated_only_witness = [false, false, false];
    assert_ne!(
        check_gf2_witness(&problem(), &translated, &translated_only_witness),
        CheckVerdict::Pass
    );
}
