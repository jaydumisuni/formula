use crate::candidate_space::SearchAuthority;
use formula_core::digest::ArtifactDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FairRoundRobin<T> {
    items: Vec<T>,
    next_index: usize,
}

impl<T: Clone> FairRoundRobin<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            next_index: 0,
        }
    }

    pub fn next(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }
        let item = self.items[self.next_index].clone();
        self.next_index = (self.next_index + 1) % self.items.len();
        Some(item)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeuristicRanking {
    scores: Vec<(ArtifactDigest, u64)>,
}

impl HeuristicRanking {
    pub fn new(mut scores: Vec<(ArtifactDigest, u64)>) -> Self {
        scores.sort_by(|left, right| {
            right
                .1
                .cmp(&left.1)
                .then_with(|| left.0.cmp(&right.0))
        });
        scores.dedup_by_key(|entry| entry.0);
        Self { scores }
    }

    pub fn ordered_candidates(&self) -> Vec<ArtifactDigest> {
        self.scores.iter().map(|(digest, _)| *digest).collect()
    }

    pub fn authority(&self) -> SearchAuthority {
        SearchAuthority::CandidateOnly
    }
}
