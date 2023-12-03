use super::{AdjGraph, CsrGraph, DenseGraph, HashGraph, SparseGraph};
use grax_core::Graph;
use more_asserts::*;

// adj

#[test]
pub fn adj_graph_create_with_nodes() {
    graph_create_with_nodes::<AdjGraph<_, _>>()
}

#[test]
pub fn adj_graph_create_with_capacity() {
    graph_create_with_capacity::<AdjGraph<_, _>>()
}

#[test]
pub fn adj_graph_insert_and_contains() {
    graph_insert_and_contains::<AdjGraph<_, _>>()
}

#[test]
pub fn adj_graph_clear() {
    graph_clear::<AdjGraph<_, _>>()
}

#[test]
pub fn adj_graph_get() {
    graph_get::<AdjGraph<_, _>>()
}

#[test]
pub fn adj_graph_index() {
    graph_index::<AdjGraph<_, _>>()
}

#[test]
pub fn adj_graph_index_adjacent() {
    graph_index_adjacent::<AdjGraph<_, _>>()
}

// hash

#[test]
pub fn hash_graph_create_with_nodes() {
    graph_create_with_nodes::<HashGraph<_, _>>()
}

#[test]
pub fn hash_graph_create_with_capacity() {
    graph_create_with_capacity::<HashGraph<_, _>>()
}

#[test]
pub fn hash_graph_insert_and_contains() {
    graph_insert_and_contains::<HashGraph<_, _>>()
}

#[test]
pub fn hash_graph_clear() {
    graph_clear::<HashGraph<_, _>>()
}

#[test]
pub fn hash_graph_get() {
    graph_get::<HashGraph<_, _>>()
}

#[test]
pub fn hash_graph_index() {
    graph_index::<HashGraph<_, _>>()
}

#[test]
pub fn hash_graph_index_adjacent() {
    graph_index_adjacent::<HashGraph<_, _>>()
}

// dense

#[test]
pub fn dense_graph_create_with_nodes() {
    graph_create_with_nodes::<DenseGraph<_, _>>()
}

#[test]
pub fn dense_graph_create_with_capacity() {
    graph_create_with_capacity::<DenseGraph<_, _>>()
}

#[test]
pub fn dense_graph_insert_and_contains() {
    graph_insert_and_contains::<DenseGraph<_, _>>()
}

#[test]
pub fn dense_graph_clear() {
    graph_clear::<DenseGraph<_, _>>()
}

#[test]
pub fn dense_graph_get() {
    graph_get::<DenseGraph<_, _>>()
}

#[test]
pub fn dense_graph_index() {
    graph_index::<DenseGraph<_, _>>()
}

#[test]
pub fn dense_graph_index_adjacent() {
    graph_index_adjacent::<DenseGraph<_, _>>()
}

// Sparse

#[test]
pub fn sparse_graph_create_with_nodes() {
    graph_create_with_nodes::<SparseGraph<_, _>>()
}

#[test]
pub fn sparse_graph_create_with_capacity() {
    graph_create_with_capacity::<SparseGraph<_, _>>()
}

#[test]
pub fn sparse_graph_insert_and_contains() {
    graph_insert_and_contains::<SparseGraph<_, _>>()
}

#[test]
pub fn sparse_graph_clear() {
    graph_clear::<SparseGraph<_, _>>()
}

#[test]
pub fn sparse_graph_get() {
    graph_get::<SparseGraph<_, _>>()
}

#[test]
pub fn sparse_graph_index() {
    graph_index::<SparseGraph<_, _>>()
}

#[test]
pub fn sparse_graph_index_adjacent() {
    graph_index_adjacent::<SparseGraph<_, _>>()
}

// csr

#[test]
pub fn csr_graph_create_with_nodes() {
    graph_create_with_nodes::<CsrGraph<_, _>>()
}

#[test]
pub fn csr_graph_create_with_capacity() {
    graph_create_with_capacity::<CsrGraph<_, _>>()
}

#[test]
pub fn csr_graph_insert_and_contains() {
    graph_insert_and_contains::<CsrGraph<_, _>>()
}

#[test]
pub fn csr_graph_clear() {
    graph_clear::<CsrGraph<_, _>>()
}

#[test]
pub fn csr_graph_get() {
    graph_get::<CsrGraph<_, _>>()
}

#[test]
pub fn csr_graph_index() {
    graph_index::<CsrGraph<_, _>>()
}

#[test]
pub fn csr_graph_index_adjacent() {
    graph_index_adjacent::<CsrGraph<_, _>>()
}

pub fn graph_create_with_nodes<G: Graph<usize, f32>>() {
    let nodes = [1, 4, 8, 3, 5];
    let graph = G::with_nodes(nodes.into_iter());
    assert_eq!(graph.node_count(), 5);
    assert_eq!(graph.edge_count(), 0);
}

pub fn graph_create_with_capacity<G: Graph<usize, f32>>() {
    let graph = G::with_capacity(10, 20);
    assert_ge!(graph.nodes_capacity(), 10);
    assert_ge!(graph.edges_capacity(), 20);
}

pub fn graph_insert_and_contains<G: Graph<usize, f32>>() {
    let mut graph = G::with_capacity(3, 2);
    let one = graph.insert_node(1);
    let two = graph.insert_node(2);
    let three = graph.insert_node(3);

    graph.insert_edge(one, two, 2.0);
    graph.insert_edge(two, three, 3.0);

    assert!(graph.contains_node(&1).is_some());
    assert!(graph.contains_node(&4).is_none());
    assert!(graph.contains_node_id(two));
    assert!(graph.contains_edge(one, two).is_some());
    assert!(graph.contains_edge(one, three).is_none());
    assert!(graph.contains_edge(two, three).is_some());
}

pub fn graph_clear<G: Graph<usize, f32>>() {
    let mut graph = G::with_capacity(3, 2);
    let one = graph.insert_node(1);
    let two = graph.insert_node(2);
    let three = graph.insert_node(3);

    let one_two = graph.insert_edge(one, two, 2.0);
    graph.insert_edge(two, three, 3.0);

    graph.clear_edges();

    assert!(graph.contains_edge_id(one_two) == false);
    assert!(graph.contains_edge(one, two).is_none());
    assert!(graph.contains_node_id(one));
    assert!(graph.contains_node(&1).is_some());

    graph.clear();

    assert!(graph.contains_node_id(one) == false);
    assert!(graph.contains_node(&1).is_none());
}

pub fn graph_get<G: Graph<usize, f32>>() {
    let mut graph = G::with_capacity(3, 2);
    let one = graph.insert_node(1);
    let two = graph.insert_node(2);
    let three = graph.insert_node(3);

    let one_two = graph.insert_edge(one, two, 2.0);
    let two_three = graph.insert_edge(two, three, 3.0);

    // TODO
    // assert_eq!(graph.node(one), Some(&1));
    // assert_eq!(graph.node(three), Some(&3));
    // assert_eq!(graph.edge(one_two), Some(&2.0));
    // assert_eq!(graph.edge(two_three), Some(&3.0));

    assert!(graph.contains_edge(one, three).is_none());
}

pub fn graph_index<G: Graph<usize, f32>>() {
    let mut graph = G::with_capacity(3, 2);
    let one = graph.insert_node(1);
    let two = graph.insert_node(2);
    let three = graph.insert_node(3);

    graph.insert_edge(one, two, 2.0);
    graph.insert_edge(two, three, 3.0);

    let nodes = graph
        .node_ids()
        .filter_map(|node_id| graph.node(node_id))
        .map(|node| node.weight)
        .collect::<Vec<_>>();

    let edges = graph
        .edge_ids()
        .filter_map(|edge_id| graph.edge(edge_id))
        .map(|edge| *edge.weight)
        .collect::<Vec<_>>();

    assert_eq!(nodes, vec![&1, &2, &3]);
    assert!(edges.contains(&2.0));
    assert!(edges.contains(&3.0));
    assert!(edges.contains(&1.0) == false);
}

pub fn graph_index_adjacent<G: Graph<usize, f32>>() {
    let mut graph = G::with_capacity(3, 2);
    let one = graph.insert_node(1);
    let two = graph.insert_node(2);
    let three = graph.insert_node(3);

    graph.insert_edge(one, two, 2.0);
    graph.insert_edge(one, three, 3.0);

    let one_adj_ids = graph.adjacent_node_ids(one).collect::<Vec<_>>();
    assert_eq!(one_adj_ids, vec![two, three]);
}
