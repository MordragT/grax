use std::fmt::Debug;

use crate::{
    collections::Keyed,
    graph::{EdgeAttribute, NodeAttribute},
    prelude::NodeId,
    view::{Distances, FilterEdgeView},
};

pub trait ShortestPath<C: Clone + Debug, G: Keyed + NodeAttribute> {
    fn shortest_path_to(
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
    ) -> Option<Distances<C, G>>;
    fn shortest_path(graph: &G, from: NodeId<G::Key>) -> Distances<C, G>;
}

#[derive(Debug, Clone)]
pub struct Mst<C: Clone + Debug, G: Keyed + EdgeAttribute> {
    pub root: NodeId<G::Key>,
    pub filter: FilterEdgeView<G>,
    pub total_cost: C,
}

pub trait MinimumSpanningTree<C: Clone + Debug, G: Keyed + EdgeAttribute> {
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
