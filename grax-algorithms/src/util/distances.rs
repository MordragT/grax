use grax_core::{
    collections::{FixedNodeMap, GetNodeMut},
    graph::NodeAttribute,
    prelude::NodeId,
};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Distances<C: Clone + Debug, G: NodeAttribute>(G::FixedNodeMap<Option<C>>);

impl<C, G> Distances<C, G>
where
    C: Clone + Debug,
    G: NodeAttribute,
{
    pub fn new(graph: &G) -> Self {
        Self(graph.fixed_node_map(None))
    }

    pub fn update(&mut self, to: NodeId<G::Key>, cost: C) -> Option<C> {
        self.0.update_node(to, Some(cost)).flatten()
    }

    pub fn distance(&self, node_id: NodeId<G::Key>) -> Option<&C> {
        self.0.get(node_id).as_ref()
    }
}
