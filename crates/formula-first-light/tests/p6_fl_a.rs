use formula_first_light::fl_a::{FlAOracleError, fl_a_oracle, fl_a_target_digest};

fn near_miss_value(n: i128) -> i128 {
    let target = fl_a_oracle().sample(n).unwrap();
    let vanishing = (0_i128..=6).map(|i| n - i).product::<i128>();
    target + vanishing
}

#[test]
fn sealed_fl_a_oracle_exposes_only_exact_samples() {
    let oracle = fl_a_oracle();
    assert_eq!(oracle.sample(-1), Ok(1));
    assert_eq!(oracle.sample(0), Ok(1));
    assert_eq!(oracle.sample(1), Ok(127));
    assert_eq!(oracle.sample(i128::MAX), Err(FlAOracleError::Overflow));
    assert_eq!(fl_a_target_digest(), fl_a_target_digest());
}

#[test]
fn early_exact_samples_leave_the_mandatory_near_miss_visible() {
    let oracle = fl_a_oracle();
    for n in 0_i128..=6 {
        assert_eq!(near_miss_value(n), oracle.sample(n).unwrap());
    }

    let discriminating = 7_i128;
    assert_ne!(
        near_miss_value(discriminating),
        oracle.sample(discriminating).unwrap()
    );
}
