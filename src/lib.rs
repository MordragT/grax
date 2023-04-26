#![feature(adt_const_params)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(type_alias_impl_trait)]

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
    pub(crate) to: NodeIndex,
    pub(crate) from: NodeIndex,
    pub(crate) depth: u32,
}

impl EdgeIndex {
    fn new(parent: NodeIndex, child: NodeIndex, depth: u32) -> Self {
        Self {
            to: parent,
            from: child,
            depth,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    /// left -> right
    Outgoing,
    /// left <- right
    Incoming,
}
