use std::str::FromStr;
use std::{fs, path::Path};

use grax_core::edge::weight::FlowCostBundle;
use grax_core::prelude::*;
use grax_impl::error::{GraphError, GraphResult};
use grax_impl::Graph;

pub fn id(raw: usize) -> NodeId<usize> {
    NodeId::new_unchecked(raw)
}

pub fn weightless_undigraph<NS, ES, P>(path: P) -> GraphResult<Graph<NS, ES, usize, (), false>>
where
    P: AsRef<Path>,
    Graph<NS, ES, usize, (), false>: FromStr<Err = GraphError>,
{
    let content = fs::read_to_string(path)?;
    Graph::from_str(&content)
}

pub fn undigraph<NS, ES, P>(path: P) -> GraphResult<Graph<NS, ES, usize, f64, false>>
where
    P: AsRef<Path>,
    Graph<NS, ES, usize, f64, false>: FromStr<Err = GraphError>,
{
    let content = fs::read_to_string(path)?;
    Graph::from_str(&content)
}

pub fn digraph<NS, ES, P>(path: P) -> GraphResult<Graph<NS, ES, usize, f64, true>>
where
    P: AsRef<Path>,
    Graph<NS, ES, usize, f64, true>: FromStr<Err = GraphError>,
{
    let content = fs::read_to_string(path)?;
    Graph::from_str(&content)
}

pub fn bgraph<NS, ES, P>(path: P) -> GraphResult<Graph<NS, ES, f64, FlowCostBundle<f64>, true>>
where
    P: AsRef<Path>,
    Graph<NS, ES, f64, FlowCostBundle<f64>, true>: FromStr<Err = GraphError>,
{
    let content = fs::read_to_string(path)?;
    Graph::from_str(&content)
}
