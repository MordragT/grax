use std::fmt::Debug;

use grax_core::{
    collections::{EdgeCollection, Keyed},
    edge::EdgeRef,
    graph::NodeAttribute,
    prelude::NodeId,
};

use crate::util::{Distances, Parents};

pub trait ShortestPathFinder<C, G>: Sized + Copy
where
    C: Clone + Debug,
    G: Keyed + NodeAttribute + EdgeCollection,
{
    /// Returns the shortest path between two nodes
    /// Returns none if no path could be found
    fn shortest_path_where<F>(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
        filter: F,
    ) -> Option<ShortestPath<C, G>>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool;

    /// Returns the shortest path tree starting from the specified node
    fn shortest_path_tree_where<F>(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        filter: F,
    ) -> ShortestPathTree<C, G>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool;

    /// Returns the shortest path between two nodes
    /// Returns none if no path could be found
    fn shortest_path(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
    ) -> Option<ShortestPath<C, G>> {
        self.shortest_path_where(graph, from, to, |_| true)
    }

    /// Returns the shortest path tree starting from the specified node
    fn shortest_path_tree(self, graph: &G, from: NodeId<G::Key>) -> ShortestPathTree<C, G> {
        self.shortest_path_tree_where(graph, from, |_| true)
    }
}

pub struct ShortestPathTree<C, G>
where
    C: Clone + Debug,
    G: NodeAttribute,
{
    pub from: NodeId<G::Key>,
    pub distances: Distances<C, G>,
    pub parents: Parents<G>,
}

pub struct ShortestPath<C, G>
where
    C: Clone + Debug,
    G: NodeAttribute,
{
    pub distance: C,
    pub from: NodeId<G::Key>,
    pub to: NodeId<G::Key>,
    pub distances: Distances<C, G>,
    pub parents: Parents<G>,
}
