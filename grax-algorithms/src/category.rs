use std::fmt::Debug;

use grax_core::{
    collections::{Keyed, RemoveEdge, RemoveNode},
    graph::{EdgeAttribute, NodeAttribute},
    prelude::NodeId,
};

use crate::utility::Distances;

pub trait ShortestPath<C: Clone + Debug, G: Keyed + NodeAttribute> {
    fn shortest_path_to(
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
    ) -> (Option<C>, Distances<C, G>);
    fn shortest_path(graph: &G, from: NodeId<G::Key>) -> Distances<C, G>;
}

pub struct Mst<C: Clone + Debug, G: Keyed + RemoveEdge + RemoveNode> {
    pub root: NodeId<G::Key>,
    pub filter: Box<dyn FnMut(&mut G)>,
    pub total_cost: C,
}

pub trait MinimumSpanningTree<C, G>
where
    C: Clone + Debug,
    G: Keyed + RemoveEdge + RemoveNode,
{
    /// Constructs a minimal spanning tree from a graph
    /// Returns None if graph is empty
    fn minimum_spanning_tree(graph: &G) -> Option<Mst<C, G>>;
}

pub trait TravelingSalesman {}

pub trait MaximumFlow {}

pub trait MinimumCostFlow {}

pub trait ConnectedComponents {}

pub trait Partition {}

pub trait ConstraintSatisfaction {}
