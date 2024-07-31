extern crate test;

use grax_core::edge::weight::Reverse;
use grax_impl::edges::EdgeStorage;
use grax_impl::nodes::NodeStorage;
use std::{fs, path::Path};
use test::Bencher;

use grax_core::parse::{ParseGrax, ParseResult};
use grax_core::prelude::*;

use grax_impl::{AdjGraph, Graph};

use crate::flow::FlowCostBundle;

pub fn id(raw: usize) -> NodeId<usize> {
    NodeId::new_unchecked(raw)
}

pub fn weightless_undigraph<NS, ES, P>(path: P) -> ParseResult<Graph<NS, ES, (), (), false>>
where
    P: AsRef<Path>,
    NS: NodeStorage<usize, ()>,
    ES: EdgeStorage<usize, ()>,
{
    let content = fs::read_to_string(path)?;
    Graph::parse_grax(&content, clone_weight_for_back_edge)
}

pub fn undigraph<NS, ES, P>(path: P) -> ParseResult<Graph<NS, ES, (), f64, false>>
where
    P: AsRef<Path>,
    NS: NodeStorage<usize, ()>,
    ES: EdgeStorage<usize, f64>,
{
    let content = fs::read_to_string(path)?;
    Graph::parse_grax(&content, clone_weight_for_back_edge)
}

pub fn digraph<NS, ES, P>(path: P) -> ParseResult<Graph<NS, ES, (), f64, true>>
where
    P: AsRef<Path>,
    NS: NodeStorage<usize, ()>,
    ES: EdgeStorage<usize, f64>,
{
    let content = fs::read_to_string(path)?;
    Graph::parse_grax(&content, clone_weight_for_back_edge)
}

pub fn bgraph<NS, ES, P>(path: P) -> ParseResult<Graph<NS, ES, f64, FlowCostBundle<f64>, true>>
where
    P: AsRef<Path>,
    NS: NodeStorage<usize, f64>,
    ES: EdgeStorage<usize, FlowCostBundle<f64>>,
{
    let content = fs::read_to_string(path)?;
    Graph::parse_grax(&content, reverse_weight_for_back_edge)
}

fn clone_weight_for_back_edge<W: Clone>(
    from: NodeId<usize>,
    to: NodeId<usize>,
    weight: W,
) -> [(NodeId<usize>, NodeId<usize>, W); 2] {
    [(from, to, weight.clone()), (to, from, weight)]
}

fn reverse_weight_for_back_edge<W: Reverse + Clone>(
    from: NodeId<usize>,
    to: NodeId<usize>,
    weight: W,
) -> [(NodeId<usize>, NodeId<usize>, W); 2] {
    [(from, to, weight.clone()), (to, from, weight.reverse())]
}

#[bench]
fn read_graph_1_2_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/G_1_2.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), f64>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}

#[bench]
fn read_graph_1_20_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/G_1_20.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), f64>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}

#[bench]
fn read_graph_1_200_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/G_1_200.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), f64>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}

#[bench]
fn read_graph_10_20_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/G_10_20.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), f64>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}

#[bench]
fn read_graph_10_200_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/G_10_200.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), f64>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}

#[bench]
fn read_graph_100_200_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/G_100_200.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), f64>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}

#[bench]
fn read_digraph_1_2_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/G_1_2.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), f64, true>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}

#[bench]
fn read_digraph_1_20_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/G_1_20.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), f64, true>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}

#[bench]
fn read_digraph_1_200_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/G_1_200.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), f64, true>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}

#[bench]
fn read_digraph_10_20_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/G_10_20.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), f64, true>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}

#[bench]
fn read_digraph_10_200_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/G_10_200.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), f64, true>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}

#[bench]
fn read_digraph_100_200_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/G_100_200.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), f64, true>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}

#[bench]
fn read_kostenminimal1_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/Kostenminimal1.txt").unwrap();

    b.iter(|| {
        AdjGraph::<f64, FlowCostBundle<f64>, true>::parse_grax(
            &edge_list,
            reverse_weight_for_back_edge,
        )
        .unwrap();
    })
}

#[bench]
fn read_kostenminimal2_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/Kostenminimal2.txt").unwrap();

    b.iter(|| {
        AdjGraph::<f64, FlowCostBundle<f64>, true>::parse_grax(
            &edge_list,
            reverse_weight_for_back_edge,
        )
        .unwrap();
    })
}

#[bench]
fn read_kostenminimal3_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/Kostenminimal3.txt").unwrap();

    b.iter(|| {
        AdjGraph::<f64, FlowCostBundle<f64>, true>::parse_grax(
            &edge_list,
            reverse_weight_for_back_edge,
        )
        .unwrap();
    })
}

#[bench]
fn read_kostenminimal4_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/Kostenminimal4.txt").unwrap();

    b.iter(|| {
        AdjGraph::<f64, FlowCostBundle<f64>, true>::parse_grax(
            &edge_list,
            reverse_weight_for_back_edge,
        )
        .unwrap();
    })
}

#[bench]
fn read_kostenminimal_gross1_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/Kostenminimal_gross1.txt").unwrap();

    b.iter(|| {
        AdjGraph::<f64, FlowCostBundle<f64>, true>::parse_grax(
            &edge_list,
            reverse_weight_for_back_edge,
        )
        .unwrap();
    })
}

#[bench]
fn read_kostenminimal_gross2_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/Kostenminimal_gross2.txt").unwrap();

    b.iter(|| {
        AdjGraph::<f64, FlowCostBundle<f64>, true>::parse_grax(
            &edge_list,
            reverse_weight_for_back_edge,
        )
        .unwrap();
    })
}

#[bench]
fn read_kostenminimal_gross3_adj_list(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/Kostenminimal_gross3.txt").unwrap();

    b.iter(|| {
        AdjGraph::<f64, FlowCostBundle<f64>, true>::parse_grax(
            &edge_list,
            reverse_weight_for_back_edge,
        )
        .unwrap();
    })
}

#[bench]
fn read_weightless_graph_gross_adj_graph(b: &mut Bencher) {
    let edge_list = fs::read_to_string("../data/Graph_gross.txt").unwrap();

    b.iter(|| {
        AdjGraph::<(), ()>::parse_grax(&edge_list, clone_weight_for_back_edge).unwrap();
    })
}
