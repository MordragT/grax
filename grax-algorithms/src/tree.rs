use std::fmt::Debug;

use grax_core::{graph::NodeAttribute, index::NodeId};

use crate::{distances::Distances, parents::Parents};

#[derive(Debug, Clone, PartialEq)]
pub struct Tree<G>
where
    G: NodeAttribute,
{
    pub root: NodeId<G::Key>,
    // pub filter: Box<dyn FnMut(&mut G)>,
    pub parents: Parents<G>,
    // pub edges: G::FixedEdgeMap<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mst<C, G>
where
    G: NodeAttribute,
{
    pub tree: Tree<G>,
    pub cost: C,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathTree<G>
where
    G: NodeAttribute,
{
    pub from: NodeId<G::Key>,
    pub parents: Parents<G>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShortestPathTree<C, G>
where
    C: Clone + Debug + PartialEq,
    G: NodeAttribute,
{
    pub from: NodeId<G::Key>,
    pub distances: Distances<C, G>,
    pub parents: Parents<G>,
}
