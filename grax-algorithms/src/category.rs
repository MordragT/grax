use std::fmt::Debug;

use grax_core::{
    collections::{Keyed, RemoveEdge, RemoveNode},
    graph::{EdgeAttribute, NodeAttribute},
    prelude::NodeId,
};

use crate::util::Distances;

pub trait ShortestPath<C: Clone + Debug, G: Keyed + NodeAttribute> {
    fn shortest_path_to(
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
    ) -> (Option<C>, Distances<C, G>);
    fn shortest_path(graph: &G, from: NodeId<G::Key>) -> Distances<C, G>;
}

pub trait TravelingSalesman {}

pub trait MaximumFlow {}

pub trait MinimumCostFlow {}

pub trait ConnectedComponents {}

pub trait Partition {}

pub trait ConstraintSatisfaction {}
