use formula_check::router::{select_certificate_route, RouteCandidate, RouteError};
use formula_core::{artifacts::AuthorityContract, digest::ArtifactDigest};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn exact_authority_request_rejects_only_weak_routes_even_when_they_are_cheaper() {
    let contract = AuthorityContract::new(
        "deterministic-proof".into(),
        vec!["PROBABILISTIC_FILTER".into()],
        "exact".into(),
    );
    let weak = RouteCandidate::new(
        "PROBABILISTIC_FILTER".into(),
        "PROBABILISTIC".into(),
        "probabilistic".into(),
        "probabilistic".into(),
        d("checker"),
        d("trust"),
        0,
    );

    assert_eq!(
        select_certificate_route(&contract, d("checker"), d("trust"), &[weak]),
        Err(RouteError::NoAdmissibleRoute)
    );
}
