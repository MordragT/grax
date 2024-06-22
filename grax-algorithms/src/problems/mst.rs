use grax_core::graph::EdgeAttribute;

use crate::util::Tree;

pub trait MstBuilder<C, G>: Sized + Copy
where
    G: EdgeAttribute,
{
    /// Constructs a minimal spanning tree from a graph
    /// Returns none if such tree cannot be created
    fn mst(self, graph: &G) -> Option<Mst<C, G>>;
}

pub struct Mst<C, G>
where
    G: EdgeAttribute,
{
    pub tree: Tree<G>,
    pub cost: C,
}
