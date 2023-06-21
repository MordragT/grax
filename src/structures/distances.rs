use super::Parents;
use crate::{
    graph::{Base, EdgeCost},
    prelude::NodeId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Distances<C, G: Base<Weight: EdgeCost<Cost = C>>> {
    pub distances: Vec<Option<C>>,
    pub parents: Parents<G>,
}

impl<C: Clone, G: Base<Weight: EdgeCost<Cost = C>>> Distances<C, G> {
    pub fn with_count(count: usize) -> Self {
        Self {
            parents: Parents::with_count(count),
            distances: vec![None; count],
        }
    }

    pub fn insert(&mut self, from: NodeId<G::Id>, to: NodeId<G::Id>, cost: C) {
        self.parents.insert(from, to);
        self.distances[to.as_usize()] = Some(cost);
    }

    pub fn add_cost(&mut self, to: NodeId<G::Id>, cost: C) {
        self.distances[to.as_usize()] = Some(cost);
    }

    pub fn distance(&self, node: NodeId<G::Id>) -> Option<&C> {
        self.distances[node.as_usize()].as_ref()
    }
}
