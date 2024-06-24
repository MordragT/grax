use grax_core::{graph::EdgeAttribute, index::NodeId};

#[derive(Debug, Clone)]
pub struct Tree<G>
where
    G: EdgeAttribute,
{
    pub root: NodeId<G::Key>,
    // pub filter: Box<dyn FnMut(&mut G)>,
    // pub parents: Parents<G>,
    pub edges: G::FixedEdgeMap<bool>,
}
