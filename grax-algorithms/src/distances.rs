use std::fmt::Debug;

use grax_core::{collections::GetNodeMut, graph::NodeAttribute, index::NodeId};

#[derive(Debug, Clone, PartialEq)]
pub struct Distances<C, G>(G::FixedNodeMap<Option<C>>)
where
    C: Clone + Debug + PartialEq,
    G: NodeAttribute;

impl<C, G> Distances<C, G>
where
    C: Clone + Debug + PartialEq,
    G: NodeAttribute,
{
    pub fn new(graph: &G) -> Self {
        Self(graph.fixed_node_map(None))
    }

    pub fn update(&mut self, to: NodeId<G::Key>, cost: C) -> Option<C> {
        self.0.update_node(to, Some(cost)).flatten()
    }

    pub fn distance(&self, node_id: NodeId<G::Key>) -> Option<&C> {
        self.0[node_id].as_ref()
    }
}
