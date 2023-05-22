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
    pub use crate::graph::*;
    pub use crate::graph_impl::*;
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
