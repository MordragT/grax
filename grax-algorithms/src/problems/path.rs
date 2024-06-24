use grax_core::{
    collections::{EdgeCollection, Keyed},
    edge::EdgeRef,
    graph::NodeAttribute,
    index::NodeId,
};

use crate::util::Parents;

pub trait PathFinder<G>: Sized + Copy
where
    G: Keyed + NodeAttribute + EdgeCollection,
{
    /// Returns the path between two nodes
    /// Returns none if no path could be found
    fn path_where<F>(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
        filter: F,
    ) -> Option<Path<G>>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool;

    /// Returns the path tree starting from the specified node
    fn path_tree_where<F>(self, graph: &G, from: NodeId<G::Key>, filter: F) -> PathTree<G>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool;

    /// Returns the path between two nodes
    /// Returns none if no path could be found
    fn path(self, graph: &G, from: NodeId<G::Key>, to: NodeId<G::Key>) -> Option<Path<G>> {
        self.path_where(graph, from, to, |_| true)
    }

    /// Returns the path tree starting from the specified node
    fn path_tree(self, graph: &G, from: NodeId<G::Key>) -> PathTree<G> {
        self.path_tree_where(graph, from, |_| true)
    }
}

#[derive(Debug, Clone)]
pub struct PathTree<G>
where
    G: NodeAttribute,
{
    pub from: NodeId<G::Key>,
    pub parents: Parents<G>,
}

#[derive(Debug, Clone)]
pub struct Path<G>
where
    G: NodeAttribute,
{
    pub from: NodeId<G::Key>,
    pub to: NodeId<G::Key>,
    pub parents: Parents<G>,
}
