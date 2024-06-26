use grax_core::{graph::NodeAttribute, index::NodeId};

use super::Parents;

#[derive(Debug, Clone)]
pub struct Tree<G>
where
    G: NodeAttribute,
{
    pub root: NodeId<G::Key>,
    // pub filter: Box<dyn FnMut(&mut G)>,
    pub parents: Parents<G>,
    // pub edges: G::FixedEdgeMap<bool>,
}
