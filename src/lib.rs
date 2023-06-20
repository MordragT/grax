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
        Edge, EdgeId, EdgeRef, EdgeRefMut, Graph, Identifier, Node, NodeId, Weight, WeightlessGraph,
    };
    pub use crate::graph_impl::*;
}

#[cfg(test)]
pub mod test {
    use crate::{
        error::GraphResult,
        graph::{BalancedNode, Base, FlowWeight},
        prelude::{EdgeList, NodeId},
    };
    use std::{fs, path::Path, str::FromStr};

    #[derive(Debug)]
    pub struct PhantomGraph;

    impl Base for PhantomGraph {
        type Id = usize;
    }

    pub fn id(raw: usize) -> NodeId<usize> {
        NodeId::new_unchecked(raw)
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
