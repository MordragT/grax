#![feature(adt_const_params)]
#![feature(test)]
#![feature(type_alias_impl_trait)]
#![feature(specialization)]
#![feature(let_chains)]

pub use graph::{Graph, GraphKind};

pub mod adjacency_list;
pub mod edge;
pub mod edge_list;
pub mod error;
pub mod graph;
pub mod tree;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct NodeIndex(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeIndex {
    pub(crate) from: NodeIndex,
    pub(crate) to: NodeIndex,
}

impl EdgeIndex {
    fn new(from: NodeIndex, to: NodeIndex) -> Self {
        Self { from, to }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    /// left -> right
    Outgoing,
    /// left <- right
    Incoming,
}
