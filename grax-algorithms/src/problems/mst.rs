use grax_core::graph::NodeAttribute;

use crate::util::Tree;

pub trait MstBuilder<C, G>: Sized + Copy
where
    G: NodeAttribute,
{
    /// Constructs a minimal spanning tree from a graph
    /// Returns none if such tree cannot be created
    fn mst(self, graph: &G) -> Option<Mst<C, G>>;
}

#[derive(Debug, Clone)]
pub struct Mst<C, G>
where
    G: NodeAttribute,
{
    pub tree: Tree<G>,
    pub cost: C,
}
