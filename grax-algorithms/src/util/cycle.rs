use std::fmt::Debug;

use grax_core::{
    graph::NodeAttribute,
    index::{EdgeId, NodeId},
};
use itertools::Itertools;

use super::Parents;

#[derive(Debug)]
pub struct Cycle<G>
where
    G: NodeAttribute,
{
    pub member: NodeId<G::Key>,
    pub parents: Parents<G>,
}

impl<G> Cycle<G>
where
    G: NodeAttribute,
{
    pub fn iter(&self) -> impl Iterator<Item = NodeId<G::Key>> + '_ {
        self.parents
            .iter(self.member)
            .take_while_inclusive(|node_id| node_id != &self.member)
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = EdgeId<G::Key>> + '_ {
        self.parents
            .iter_edges(self.member)
            .take_while_inclusive(|edge_id| edge_id.from() != self.member)
    }

    pub fn is_empty(&self) -> bool {
        self.parents.is_empty()
    }
}