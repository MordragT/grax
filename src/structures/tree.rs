use crate::graph::Base;

#[derive(Debug)]
pub struct MinimumSpanningTree<G: Base> {
    pub tree: G,
    pub root: G::NodeId,
}

impl<G: Base> MinimumSpanningTree<G> {
    pub fn new(tree: G, root: G::NodeId) -> Self {
        Self { tree, root }
    }
}
