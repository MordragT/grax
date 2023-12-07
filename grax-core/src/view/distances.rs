use super::Parents;
use crate::{
    collections::{FixedNodeMap, GetNode, GetNodeMut},
    graph::{Cost, NodeAttribute},
    prelude::NodeId,
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Distances<C: Clone + Debug, G: Cost<C> + NodeAttribute> {
    pub distances: G::FixedNodeMap<Option<C>>,
    pub parents: Parents<G>,
}

impl<C: Clone + Debug, G: Cost<C> + NodeAttribute> Distances<C, G> {
    pub fn new(graph: &G) -> Self {
        let distances = graph.fixed_node_map(None);
        let parents = Parents::new(graph);

        Self { distances, parents }
    }

    pub fn insert(&mut self, from: NodeId<G::Key>, to: NodeId<G::Key>, cost: C) {
        self.parents.insert(from, to);
        self.distances.update_node(to, Some(cost));
    }

    pub fn update_cost(&mut self, to: NodeId<G::Key>, cost: C) -> Option<C> {
        self.distances.update_node(to, Some(cost)).flatten()
    }

    pub fn distance(&self, node_id: NodeId<G::Key>) -> Option<&C> {
        self.distances.get(node_id).as_ref()
    }
}
