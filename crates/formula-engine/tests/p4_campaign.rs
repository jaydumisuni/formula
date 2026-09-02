use formula_core::digest::ArtifactDigest;
use formula_engine::campaign::{
    CampaignAggregation, CampaignEdge, CampaignEdgeKind, CampaignError, CampaignIR, CampaignNode,
    CampaignNodeKind,
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn nodes() -> Vec<CampaignNode> {
    vec![
        CampaignNode::new(d(10), CampaignNodeKind::Goal, d(1), d(2), Some(CampaignAggregation::Or)),
        CampaignNode::new(d(11), CampaignNodeKind::Route, d(1), d(2), Some(CampaignAggregation::And)),
        CampaignNode::new(d(12), CampaignNodeKind::Obligation, d(1), d(2), None),
    ]
}

fn edges() -> Vec<CampaignEdge> {
    vec![
        CampaignEdge::new(d(10), d(11), CampaignEdgeKind::AlternativeTo),
        CampaignEdge::new(d(11), d(12), CampaignEdgeKind::Requires),
    ]
}

#[test]
fn campaign_identity_is_insertion_order_independent() {
    let left = CampaignIR::new(d(1), d(2), nodes(), edges());
    let mut reversed_nodes = nodes();
    reversed_nodes.reverse();
    let mut reversed_edges = edges();
    reversed_edges.reverse();
    let right = CampaignIR::new(d(1), d(2), reversed_nodes, reversed_edges);

    assert_eq!(left.validate(), Ok(()));
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn campaign_rejects_dangling_edges() {
    let mut invalid_edges = edges();
    invalid_edges.push(CampaignEdge::new(d(11), d(99), CampaignEdgeKind::Produces));
    let campaign = CampaignIR::new(d(1), d(2), nodes(), invalid_edges);
    assert_eq!(campaign.validate(), Err(CampaignError::DanglingReference));
}

#[test]
fn node_generation_and_world_must_match_campaign() {
    let mut wrong_generation = nodes();
    wrong_generation.push(CampaignNode::new(
        d(20), CampaignNodeKind::FactRef, d(99), d(2), None,
    ));
    assert_eq!(
        CampaignIR::new(d(1), d(2), wrong_generation, edges()).validate(),
        Err(CampaignError::GenerationMismatch)
    );

    let mut wrong_world = nodes();
    wrong_world.push(CampaignNode::new(
        d(20), CampaignNodeKind::FactRef, d(1), d(99), None,
    ));
    assert_eq!(
        CampaignIR::new(d(1), d(2), wrong_world, edges()).validate(),
        Err(CampaignError::WorldMismatch)
    );
}

#[test]
fn only_goal_and_route_nodes_may_aggregate() {
    let mut invalid = nodes();
    invalid.push(CampaignNode::new(
        d(20),
        CampaignNodeKind::ArtifactRef,
        d(1),
        d(2),
        Some(CampaignAggregation::And),
    ));
    assert_eq!(
        CampaignIR::new(d(1), d(2), invalid, edges()).validate(),
        Err(CampaignError::IllegalAggregation)
    );
}

#[test]
fn route_requires_at_least_one_obligation() {
    let campaign = CampaignIR::new(
        d(1),
        d(2),
        nodes(),
        vec![CampaignEdge::new(d(10), d(11), CampaignEdgeKind::AlternativeTo)],
    );
    assert_eq!(
        campaign.validate(),
        Err(CampaignError::RouteWithoutObligation)
    );
}
