use super::Parents;
use crate::{
    collections::{FixedNodeMap, GetNode, GetNodeMut},
    graph::NodeAttribute,
    prelude::NodeId,
    weight::Sortable,
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Distances<C: Clone + Debug, G: NodeAttribute> {
    pub distances: G::FixedNodeMap<Option<C>>,
    pub parents: Parents<G>,
}

impl<C: Clone + Debug, G: NodeAttribute> Distances<C, G> {
    pub fn new(graph: &G) -> Self {
        let distances = graph.fixed_node_map(None);
        let parents = Parents::new(graph);

        Self { distances, parents }
    }

    pub fn insert(&mut self, from: NodeId<G::Key>, to: NodeId<G::Key>, cost: C) {
        self.parents.insert(from, to);
        self.distances.update_node(to, Some(cost));
    }

    pub fn update(&mut self, to: NodeId<G::Key>, cost: C) -> Option<C> {
        self.distances.update_node(to, Some(cost)).flatten()
    }

    pub fn distance(&self, node_id: NodeId<G::Key>) -> Option<&C> {
        self.distances.get(node_id).as_ref()
    }
}

impl<C: Copy + Clone + Debug + Sortable, G: NodeAttribute> Distances<C, G> {
    /// Returns true if distance was replaced
    pub fn replace_min(&mut self, from: NodeId<G::Key>, to: NodeId<G::Key>, cost: C) -> bool {
        if let Some(c) = self.distances.get_mut(to) {
            if cost < *c {
                *c = cost;
                self.parents.insert(from, to);
                return true;
            }
        } else {
            self.insert(from, to, cost);
            return true;
        }
        false
    }
}

// TODO decouple parents from distances
