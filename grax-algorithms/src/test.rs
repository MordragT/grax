use std::str::FromStr;
use std::{fs, path::Path};

use grax_core::adaptor::flow::FlowBundle;
use grax_core::prelude::*;
use grax_impl::error::GraphResult;
use grax_impl::flow::BalancedNode;
use grax_impl::EdgeList;

// #[derive(Debug)]
// pub struct PhantomGraph;

// impl Base for PhantomGraph {
//     type Id = usize;
//     type Node = usize;
//     type Weight = f32;
// }

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
    G: From<EdgeList<BalancedNode<usize, f64>, FlowBundle<f64, f64>, true>>,
{
    let content = fs::read_to_string(path)?;
    let edge_list = EdgeList::from_str(&content)?;
    let graph = G::from(edge_list);
    Ok(graph)
}
