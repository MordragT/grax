use std::fmt::Debug;

use grax_core::{graph::NodeAttribute, index::NodeId};

use crate::{distances::Distances, parents::Parents};

#[derive(Debug, Clone)]
pub struct Path<G>
where
    G: NodeAttribute,
{
    pub from: NodeId<G::Key>,
    pub to: NodeId<G::Key>,
    pub parents: Parents<G>,
}

#[derive(Debug, Clone)]
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