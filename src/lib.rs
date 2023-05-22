#![feature(adt_const_params)]
#![feature(test)]
#![feature(type_alias_impl_trait)]
#![feature(specialization)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(array_windows)]

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

        pub fn contains(&self, index: NodeIndex) -> bool {
            self.from == index || self.to == index
        }

        pub fn rev(&self) -> Self {
            let Self { from, to } = self;

            Self {
                from: *to,
                to: *from,
            }
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
    pub use crate::edge_list::EdgeList;
    pub use crate::error::GraphError;
    pub use crate::graph::*;
    pub use crate::indices::*;
}

#[cfg(test)]
pub mod test {
    use crate::{
        error::GraphResult,
        prelude::{EdgeList, GraphError},
    };
    use std::{fs, path::Path, str::FromStr};

    pub fn weightless_undigraph<G, P>(path: P) -> GraphResult<G>
    where
        P: AsRef<Path>,
        G: TryFrom<EdgeList<usize, (), false>, Error = GraphError>,
    {
        let content = fs::read_to_string(path)?;
        let edge_list = EdgeList::from_str(&content)?;
        let graph = G::try_from(edge_list)?;
        Ok(graph)
    }

    pub fn undigraph<G, P>(path: P) -> GraphResult<G>
    where
        P: AsRef<Path>,
        G: TryFrom<EdgeList<usize, f64, false>, Error = GraphError>,
    {
        let content = fs::read_to_string(path)?;
        let edge_list = EdgeList::from_str(&content)?;
        let graph = G::try_from(edge_list)?;
        Ok(graph)
    }

    pub fn digraph<G, P>(path: P) -> GraphResult<G>
    where
        P: AsRef<Path>,
        G: TryFrom<EdgeList<usize, f64, true>, Error = GraphError>,
    {
        let content = fs::read_to_string(path)?;
        let edge_list = EdgeList::from_str(&content)?;
        let graph = G::try_from(edge_list)?;
        Ok(graph)
    }
}
