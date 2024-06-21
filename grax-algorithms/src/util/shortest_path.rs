use std::fmt::Debug;

use grax_core::{
    collections::{EdgeCollection, Keyed},
    edge::EdgeRef,
    graph::NodeAttribute,
    prelude::NodeId,
};

use crate::util::Distances;

use super::Parents;

pub trait ShortestPathFinder<C, G>: Sized + Copy
where
    C: Clone + Debug,
    G: Keyed + NodeAttribute + EdgeCollection,
{
    fn shortest_path_to_where<F>(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
        filter: F,
    ) -> ShortestPathTo<C, G>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool;

    fn shortest_path_were<F>(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        filter: F,
    ) -> ShortestPath<C, G>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool;

    fn shortest_path_to(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
    ) -> ShortestPathTo<C, G> {
        self.shortest_path_to_where(graph, from, to, |_| true)
    }

    fn shortest_path(self, graph: &G, from: NodeId<G::Key>) -> ShortestPath<C, G> {
        self.shortest_path_were(graph, from, |_| true)
    }
}

pub struct ShortestPath<C, G>
where
    C: Clone + Debug,
    G: NodeAttribute,
{
    pub distances: Distances<C, G>,
    pub parents: Parents<G>,
}

pub struct ShortestPathTo<C, G>
where
    C: Clone + Debug,
    G: NodeAttribute,
{
    pub distance: Option<C>,
    pub distances: Distances<C, G>,
    pub parents: Parents<G>,
}
