use grax_core::{collections::Keyed, graph::EdgeAttribute, prelude::NodeId};

pub trait MstBuilder<C, G>: Sized + Copy
where
    G: Keyed + EdgeAttribute,
{
    /// Constructs a minimal spanning tree from a graph
    /// Returns None if graph is empty
    fn mst(self, graph: &G) -> Option<Mst<C, G>>;
}

pub struct Mst<C, G>
where
    G: Keyed + EdgeAttribute,
{
    pub root: NodeId<G::Key>,
    // pub filter: Box<dyn FnMut(&mut G)>,
    // pub parents: Parents<G>,
    pub edges: G::FixedEdgeMap<bool>,
    pub total_cost: C,
}
