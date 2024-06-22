use grax_core::{
    collections::{EdgeCollection, Keyed},
    edge::EdgeRef,
    graph::NodeAttribute,
    index::NodeId,
};

use super::Parents;

pub trait PathFinder<G>: Sized + Copy
where
    G: Keyed + NodeAttribute + EdgeCollection,
{
    fn path_to_where<F>(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
        filter: F,
    ) -> Option<Path<G>>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool;

    fn path_where<F>(self, graph: &G, from: NodeId<G::Key>, filter: F) -> Path<G>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool;

    fn path(self, graph: &G, from: NodeId<G::Key>) -> Path<G> {
        self.path_where(graph, from, |_| true)
    }
    fn path_to(self, graph: &G, from: NodeId<G::Key>, to: NodeId<G::Key>) -> Option<Path<G>> {
        self.path_to_where(graph, from, to, |_| true)
    }
}

pub struct Path<G>
where
    G: NodeAttribute,
{
    pub parents: Parents<G>,
}

impl<G> Path<G>
where
    G: NodeAttribute,
{
    pub fn new(graph: &G) -> Self {
        Self {
            parents: Parents::new(graph),
        }
    }
}
