extern crate test;

use crate::prelude::*;
use std::str::FromStr;

#[test]
fn add_node() {
    let mut graph = AdjacencyList::<u32, ()>::new();
    let _idx1 = graph.add_node(1);
    let _idx2 = graph.add_node(2);
    let _idx3 = graph.add_node(3);

    graph.contains_node(&1).unwrap();
    graph.contains_node(&2).unwrap();
    graph.contains_node(&3).unwrap();

    assert!(graph.contains_node(&100).is_none());
}

#[test]
fn update_node() {
    let mut graph = AdjacencyList::<u32, ()>::new();
    let idx1 = graph.add_node(1);

    assert_eq!(graph.update_node(idx1, 5), 1);

    graph.contains_node(&5).unwrap();
    assert!(graph.contains_node(&1).is_none());
}

#[test]
fn add_edge() {
    let mut graph = AdjacencyList::<u32, ()>::new();
    let idx1 = graph.add_node(1);
    let idx2 = graph.add_node(2);
    let _idx3 = graph.add_node(3);

    let _ = graph.add_edge(idx1, idx2, ()).unwrap();

    graph.contains_edge(idx1, idx2).unwrap();
    //graph.contains_edge(idx2, idx1).unwrap();

    assert!(graph.contains_edge(idx2, idx1).is_none());
}

#[test]
fn update_edge() {
    let mut graph = AdjacencyList::<u32, u32>::new();
    let idx1 = graph.add_node(1);
    let idx2 = graph.add_node(2);

    let edge = graph.add_edge(idx1, idx2, 2).unwrap();

    assert_eq!(graph.update_edge(edge, 5), 2);
    assert_eq!(graph.weight(edge), &5);
}

#[test]
fn from_edge_list() {
    let edge_list = "4
        0 2
        1 2
        2 3
        3 1";
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::try_from(edge_list).unwrap();

    assert_eq!(graph.node_count(), 4);

    let idx0 = graph.contains_node(&0).unwrap();
    let idx1 = graph.contains_node(&1).unwrap();
    let idx2 = graph.contains_node(&2).unwrap();
    let idx3 = graph.contains_node(&3).unwrap();

    graph.contains_edge(idx0, idx2).unwrap();
    graph.contains_edge(idx1, idx2).unwrap();
    graph.contains_edge(idx2, idx3).unwrap();
    graph.contains_edge(idx3, idx1).unwrap();

    graph.contains_edge(idx2, idx0).unwrap();
    graph.contains_edge(idx2, idx1).unwrap();
    graph.contains_edge(idx3, idx2).unwrap();
    graph.contains_edge(idx1, idx3).unwrap();

    assert!(graph.contains_edge(idx1, idx0).is_none());
}

#[test]
fn djikstra() {
    let edge_list = EdgeList::with(
        [
            (0, 1, 1.0),
            (0, 2, 3.0),
            (1, 2, 1.0),
            (2, 3, 4.0),
            (3, 0, 1.5),
        ]
        .into_iter(),
        4,
    );

    let graph = AdjacencyList::<usize, f32>::try_from(edge_list).unwrap();
    let dist = graph.dijkstra_between(NodeIndex(0), NodeIndex(2));

    assert_eq!(dist, Some(2.0));
}
