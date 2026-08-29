use formula_check::router::{
    select_certificate_route, RouteCandidate, RouteError,
};
use formula_core::{artifacts::AuthorityContract, digest::ArtifactDigest};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn exact_contract() -> AuthorityContract {
    AuthorityContract::new(
        "deterministic-proof".into(),
        vec!["POLYNOMIAL_IDENTITY".into(), "PROBABILISTIC_FILTER".into()],
        "exact".into(),
    )
}

#[test]
fn cheaper_weak_route_never_beats_exact_authority_requirement() {
    let weak = RouteCandidate::new(
        "PROBABILISTIC_FILTER".into(),
        "PROBABILISTIC".into(),
        "probabilistic".into(),
        "probabilistic".into(),
        d("checker"),
        d("trust"),
        1,
    );
    let exact = RouteCandidate::new(
        "POLYNOMIAL_IDENTITY".into(),
        "EXACT_RECOMPUTATION".into(),
        "deterministic-proof".into(),
        "exact".into(),
        d("checker"),
        d("trust"),
        100,
    );

    let selected = select_certificate_route(
        &exact_contract(),
        d("checker"),
        d("trust"),
        &[weak, exact.clone()],
    )
    .unwrap();
    assert_eq!(selected.candidate(), &exact);
}

#[test]
fn checker_and_trust_root_are_exact_route_requirements() {
    let wrong_checker = RouteCandidate::new(
        "POLYNOMIAL_IDENTITY".into(),
        "EXACT_RECOMPUTATION".into(),
        "deterministic-proof".into(),
        "exact".into(),
        d("other-checker"),
        d("trust"),
        1,
    );
    let wrong_trust = RouteCandidate::new(
        "POLYNOMIAL_IDENTITY".into(),
        "EXACT_RECOMPUTATION".into(),
        "deterministic-proof".into(),
        "exact".into(),
        d("checker"),
        d("other-trust"),
        1,
    );

    assert_eq!(
        select_certificate_route(
            &exact_contract(),
            d("checker"),
            d("trust"),
            &[wrong_checker, wrong_trust],
        ),
        Err(RouteError::NoAdmissibleRoute)
    );
}

#[test]
fn unavailable_exact_route_fails_closed() {
    let weak = RouteCandidate::new(
        "PROBABILISTIC_FILTER".into(),
        "PROBABILISTIC".into(),
        "probabilistic".into(),
        "probabilistic".into(),
        d("checker"),
        d("trust"),
        1,
    );
    assert_eq!(
        select_certificate_route(
            &exact_contract(),
            d("checker"),
            d("trust"),
            &[weak],
        ),
        Err(RouteError::NoAdmissibleRoute)
    );
}
