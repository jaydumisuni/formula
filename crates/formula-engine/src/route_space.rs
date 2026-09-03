use crate::{
    candidate_space::{
        CandidatePolarity, CandidateSpaceContext, CompletenessClass, FrozenCandidate,
        FrozenCandidateSpace,
    },
    query::RequestedResultClass,
};
use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::{BTreeMap, BTreeSet};

const ROUTE_SCHEMA_V1: &str = "formula-route-candidate-v1";
const ROUTE_SPACE_SCHEMA_V1: &str = "formula-reduction-route-space-v1";
const ROUTE_FAILURE_SCHEMA_V1: &str = "formula-scoped-route-failure-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteCandidate {
    route_digest: ArtifactDigest,
    source_semantics: String,
    target_semantics: String,
    preserved_result_classes: Vec<RequestedResultClass>,
    required_capabilities: Vec<ArtifactDigest>,
    exact: bool,
    cost: u64,
}

impl RouteCandidate {
    pub fn new(
        route_digest: ArtifactDigest,
        source_semantics: impl Into<String>,
        target_semantics: impl Into<String>,
        mut preserved_result_classes: Vec<RequestedResultClass>,
        mut required_capabilities: Vec<ArtifactDigest>,
        exact: bool,
        cost: u64,
    ) -> Self {
        preserved_result_classes.sort_unstable();
        preserved_result_classes.dedup();
        required_capabilities.sort_unstable();
        required_capabilities.dedup();
        Self {
            route_digest,
            source_semantics: source_semantics.into(),
            target_semantics: target_semantics.into(),
            preserved_result_classes,
            required_capabilities,
            exact,
            cost,
        }
    }

    pub fn route_digest(&self) -> ArtifactDigest {
        self.route_digest
    }

    pub fn target_semantics(&self) -> &str {
        &self.target_semantics
    }

    pub fn cost(&self) -> u64 {
        self.cost
    }

    fn preserves(&self, requested: RequestedResultClass) -> bool {
        self.exact
            && self
                .preserved_result_classes
                .binary_search(&requested)
                .is_ok()
    }

    fn capabilities_available(&self, available: &BTreeSet<ArtifactDigest>) -> bool {
        self.required_capabilities
            .iter()
            .all(|capability| available.contains(capability))
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            ("cost".into(), CanonicalValue::String(self.cost.to_string())),
            ("exact".into(), CanonicalValue::Bool(self.exact)),
            (
                "preserved_result_classes".into(),
                CanonicalValue::Array(
                    self.preserved_result_classes
                        .iter()
                        .map(|class| CanonicalValue::String(class.as_str().into()))
                        .collect(),
                ),
            ),
            (
                "required_capabilities".into(),
                CanonicalValue::Array(
                    self.required_capabilities
                        .iter()
                        .copied()
                        .map(CanonicalValue::Digest)
                        .collect(),
                ),
            ),
            (
                "route_digest".into(),
                CanonicalValue::Digest(self.route_digest),
            ),
            (
                "schema".into(),
                CanonicalValue::String(ROUTE_SCHEMA_V1.into()),
            ),
            (
                "source_semantics".into(),
                CanonicalValue::String(self.source_semantics.clone()),
            ),
            (
                "target_semantics".into(),
                CanonicalValue::String(self.target_semantics.clone()),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopedRouteFailure {
    failure_digest: ArtifactDigest,
    applicability: Vec<ArtifactDigest>,
}

impl ScopedRouteFailure {
    pub fn new(failure_digest: ArtifactDigest, mut applicability: Vec<ArtifactDigest>) -> Self {
        applicability.sort_unstable();
        applicability.dedup();
        Self {
            failure_digest,
            applicability,
        }
    }

    fn applies_to(&self, route: ArtifactDigest) -> bool {
        self.applicability.binary_search(&route).is_ok()
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "applicability".into(),
                CanonicalValue::Array(
                    self.applicability
                        .iter()
                        .copied()
                        .map(CanonicalValue::Digest)
                        .collect(),
                ),
            ),
            (
                "failure_digest".into(),
                CanonicalValue::Digest(self.failure_digest),
            ),
            (
                "schema".into(),
                CanonicalValue::String(ROUTE_FAILURE_SCHEMA_V1.into()),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReductionRouteSpace {
    context: CandidateSpaceContext,
    routes: Vec<RouteCandidate>,
    requested_result_class: Option<RequestedResultClass>,
    available_capabilities: Option<Vec<ArtifactDigest>>,
    failures: Vec<ScopedRouteFailure>,
}

impl ReductionRouteSpace {
    pub fn new(context: CandidateSpaceContext, mut routes: Vec<RouteCandidate>) -> Self {
        routes.sort_by_key(RouteCandidate::route_digest);
        routes.dedup_by_key(|route| route.route_digest());
        Self {
            context,
            routes,
            requested_result_class: None,
            available_capabilities: None,
            failures: Vec::new(),
        }
    }

    pub fn restrict_result_class(&mut self, requested: RequestedResultClass) {
        self.requested_result_class = Some(requested);
    }

    pub fn restrict_capabilities(&mut self, available: &[ArtifactDigest]) {
        let mut available = available.to_vec();
        available.sort_unstable();
        available.dedup();
        self.available_capabilities = Some(available);
    }

    pub fn subtract_scoped_failure(&mut self, failure: &ScopedRouteFailure) {
        if !self.failures.contains(failure) {
            self.failures.push(failure.clone());
            self.failures.sort_by_key(|entry| entry.failure_digest);
        }
    }

    pub fn contains(&self, route_digest: ArtifactDigest) -> bool {
        self.active_routes()
            .iter()
            .any(|route| route.route_digest() == route_digest)
    }

    pub fn empty(&self) -> bool {
        self.active_routes().is_empty()
    }

    pub fn extract_min_cost(&self) -> Option<RouteCandidate> {
        let mut routes = self.active_routes();
        routes.sort_by_key(|route| (route.cost(), route.route_digest()));
        routes.first().map(|route| (*route).clone())
    }

    pub fn partition_by_target(&self) -> BTreeMap<String, Vec<ArtifactDigest>> {
        let mut partitions: BTreeMap<String, Vec<ArtifactDigest>> = BTreeMap::new();
        for route in self.active_routes() {
            partitions
                .entry(route.target_semantics().to_owned())
                .or_default()
                .push(route.route_digest());
        }
        for routes in partitions.values_mut() {
            routes.sort_unstable();
            routes.dedup();
        }
        partitions
    }

    pub fn freeze(&self) -> FrozenCandidateSpace {
        FrozenCandidateSpace::new(
            self.context.clone(),
            "reduction-route-v1",
            CandidatePolarity::Exact,
            CompletenessClass::CompleteWithinBound,
            self.state_digest(),
        )
    }

    pub fn freeze_candidate(&self, route: &RouteCandidate) -> FrozenCandidate {
        FrozenCandidate::new(self.freeze().digest(), route.route_digest(), route.cost())
    }

    fn active_routes(&self) -> Vec<&RouteCandidate> {
        let available = self
            .available_capabilities
            .as_ref()
            .map(|values| values.iter().copied().collect::<BTreeSet<_>>());
        self.routes
            .iter()
            .filter(|route| {
                self.requested_result_class
                    .map(|requested| route.preserves(requested))
                    .unwrap_or(route.exact)
            })
            .filter(|route| {
                available
                    .as_ref()
                    .map(|capabilities| route.capabilities_available(capabilities))
                    .unwrap_or(true)
            })
            .filter(|route| {
                !self
                    .failures
                    .iter()
                    .any(|failure| failure.applies_to(route.route_digest()))
            })
            .collect()
    }

    fn state_digest(&self) -> ArtifactDigest {
        let active = self.active_routes();
        CanonicalValue::Object(BTreeMap::from([
            (
                "active_routes".into(),
                CanonicalValue::Array(active.iter().map(|route| route.canonical_value()).collect()),
            ),
            (
                "available_capabilities".into(),
                self.available_capabilities
                    .as_ref()
                    .map(|values| {
                        CanonicalValue::Array(
                            values.iter().copied().map(CanonicalValue::Digest).collect(),
                        )
                    })
                    .unwrap_or(CanonicalValue::Null),
            ),
            (
                "failures".into(),
                CanonicalValue::Array(
                    self.failures
                        .iter()
                        .map(ScopedRouteFailure::canonical_value)
                        .collect(),
                ),
            ),
            (
                "requested_result_class".into(),
                self.requested_result_class
                    .map(|value| CanonicalValue::String(value.as_str().into()))
                    .unwrap_or(CanonicalValue::Null),
            ),
            (
                "schema".into(),
                CanonicalValue::String(ROUTE_SPACE_SCHEMA_V1.into()),
            ),
        ]))
        .digest()
    }
}
