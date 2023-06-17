#![feature(adt_const_params)]
#![feature(test)]
#![feature(type_alias_impl_trait)]
#![feature(specialization)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(array_windows)]
#![feature(associated_type_bounds)]

pub mod algorithms;
pub mod edge_list;
pub mod error;
pub mod graph;
pub mod graph_impl;
pub mod structures;

pub mod prelude {
    pub use crate::edge_list::EdgeList;
    pub use crate::error::GraphError;
    pub use crate::graph::{
        Edge, EdgeIdentifier, EdgeRef, EdgeRefMut, Graph, Node, NodeIdentifier, Weight,
        WeightlessGraph,
    };
    pub use crate::graph_impl::*;
}

#[cfg(test)]
pub mod test {
    use crate::{
        error::GraphResult,
        graph::{BalancedNode, Base, FlowWeight},
        prelude::{EdgeIdentifier, EdgeList},
    };
    use std::{fs, path::Path, str::FromStr};

    #[derive(Debug)]
    pub struct PhantomGraph;

    impl Base for PhantomGraph {
        type NodeId = usize;
        type EdgeId = (usize, usize);
    }

    impl EdgeIdentifier for (usize, usize) {
        type NodeId = usize;

        fn between(from: Self::NodeId, to: Self::NodeId) -> Self {
            (from, to)
        }

        fn contains(&self, index: usize) -> bool {
            self.0 == index || self.1 == index
        }

        fn rev(&self) -> Self {
            (self.1, self.0)
        }

        fn to(&self) -> Self::NodeId {
            self.1
        }

        fn from(&self) -> Self::NodeId {
            self.0
        }

        fn as_usize(&self) -> (usize, usize) {
            *self
        }
    }

    pub fn weightless_undigraph<G, P>(path: P) -> GraphResult<G>
    where
        P: AsRef<Path>,
        G: From<EdgeList<usize, (), false>>,
    {
        let content = fs::read_to_string(path)?;
        let edge_list = EdgeList::from_str(&content)?;
        let graph = G::from(edge_list);
        Ok(graph)
    }

    pub fn undigraph<G, P>(path: P) -> GraphResult<G>
    where
        P: AsRef<Path>,
        G: From<EdgeList<usize, f64, false>>,
    {
        let content = fs::read_to_string(path)?;
        let edge_list = EdgeList::from_str(&content)?;
        let graph = G::from(edge_list);
        Ok(graph)
    }

    pub fn digraph<G, P>(path: P) -> GraphResult<G>
    where
        P: AsRef<Path>,
        G: From<EdgeList<usize, f64, true>>,
    {
        let content = fs::read_to_string(path)?;
        let edge_list = EdgeList::from_str(&content)?;
        let graph = G::from(edge_list);
        Ok(graph)
    }

    pub fn bgraph<G, P>(path: P) -> GraphResult<G>
    where
        P: AsRef<Path>,
        G: From<EdgeList<BalancedNode<usize, f64>, FlowWeight<f64>, true>>,
    {
        let content = fs::read_to_string(path)?;
        let edge_list = EdgeList::from_str(&content)?;
        let graph = G::from(edge_list);
        Ok(graph)
    }
}
