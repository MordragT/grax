#![feature(adt_const_params)]
#![feature(test)]
#![feature(type_alias_impl_trait)]
#![feature(specialization)]
#![feature(let_chains)]
#![feature(array_windows)]

pub mod adjacency_list;
pub mod algorithms;
pub mod edge;
pub mod edge_list;
pub mod error;
pub mod graph;
pub mod tree;

pub mod indices {
    use crate::edge::{Edge, EdgeRef};

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    pub struct NodeIndex(pub(crate) usize);

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EdgeIndex {
        pub(crate) from: NodeIndex,
        pub(crate) to: NodeIndex,
    }

    impl EdgeIndex {
        pub(crate) fn new(from: NodeIndex, to: NodeIndex) -> Self {
            Self { from, to }
        }
    }

    impl<'a, W> From<EdgeRef<'a, W>> for EdgeIndex {
        fn from(edge: EdgeRef<'a, W>) -> Self {
            Self {
                from: edge.from,
                to: edge.to,
            }
        }
    }

    impl<W> From<Edge<W>> for EdgeIndex {
        fn from(edge: Edge<W>) -> Self {
            Self {
                from: edge.from,
                to: edge.to,
            }
        }
    }
}

pub mod prelude {
    pub use crate::adjacency_list::AdjacencyList;
    pub use crate::edge_list::EdgeList;
    pub use crate::error::GraphError;
    pub use crate::graph::*;
    pub use crate::indices::*;
}
