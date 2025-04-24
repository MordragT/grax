use grax_core::{
    collections::VisitNodeMap,
    graph::NodeAttribute,
    index::{EdgeId, NodeId},
};
use itertools::Itertools;

use crate::parents::Parents;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CycleDetected;

#[derive(Debug, Clone, PartialEq)]
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
    pub fn detect(graph: &G, parents: Parents<G>) -> Self {
        let member = parents
            .node_ids()
            .find_map(|from| {
                let mut visited = graph.visit_node_map();
                visited.visit(from);

                for parent in parents.iter(from) {
                    if parent == from {
                        return Some(from);
                    } else if visited.is_visited(parent) {
                        return None;
                    } else {
                        visited.visit(parent);
                    }
                }
                None
            })
            .unwrap();

        Self { parents, member }
    }

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

pub struct TspCycle<C, G>
where
    G: NodeAttribute,
{
    pub cost: C,
    pub cycle: Cycle<G>,
}
