#![feature(adt_const_params)]
#![feature(test)]
#![feature(type_alias_impl_trait)]
#![feature(specialization)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(array_windows)]

pub mod algorithms;
pub mod edge_list;
pub mod error;
pub mod graph;
pub mod graph_impl;
pub mod utils;

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
        graph::{BalancedNode, CapacityWeight},
        prelude::EdgeList,
    };
    use std::{fs, path::Path, str::FromStr};

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
        G: From<EdgeList<BalancedNode<usize, f64>, CapacityWeight<f64>>>,
    {
        let content = fs::read_to_string(path)?;
        let edge_list = EdgeList::from_str(&content)?;
        let graph = G::from(edge_list);
        Ok(graph)
    }
}
