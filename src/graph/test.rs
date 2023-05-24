use super::{EdgeIdentifier, Graph};
use more_asserts::*;

// + Extend<N, W>
// + GetMut<N, W>
// + IndexAdjacent
// + Iter<N, W>
// + IterMut<N, W>
// + IterAdjacent<N, W>
// + IterAdjacentMut<N, W>
// + Remove<N, W>
// + Reserve

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
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    let three = graph.add_node(3);

    let one_two = G::EdgeId::between(one, two);
    let two_three = G::EdgeId::between(two, three);
    graph.insert_edge(one_two, 2.0);
    graph.insert_edge(two_three, 3.0);

    let one_three = G::EdgeId::between(one, three);

    assert!(graph.contains_node(&1).is_some());
    assert!(graph.contains_node(&4).is_none());
    assert!(graph.contains_node_id(two));
    assert!(graph.contains_edge_id(one_two));
    assert!(graph.contains_edge_id(one_three) == false);
    assert!(graph.contains_edge(two, three).is_some());
}

pub fn graph_clear<G: Graph<usize, f32>>() {
    let mut graph = G::with_capacity(3, 2);
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    let three = graph.add_node(3);

    let one_two = G::EdgeId::between(one, two);
    let two_three = G::EdgeId::between(two, three);
    graph.insert_edge(one_two, 2.0);
    graph.insert_edge(two_three, 3.0);

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
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    let three = graph.add_node(3);

    let one_two = G::EdgeId::between(one, two);
    let two_three = G::EdgeId::between(two, three);
    graph.insert_edge(one_two, 2.0);
    graph.insert_edge(two_three, 3.0);

    assert_eq!(graph.node(one), Some(&1));
    assert_eq!(graph.node(three), Some(&3));
    assert_eq!(graph.weight(one_two), Some(&2.0));
    assert_eq!(graph.weight(two_three), Some(&3.0));

    let one_three = G::EdgeId::between(one, three);

    assert!(graph.weight(one_three).is_none());
}

pub fn graph_index<G: Graph<usize, f32>>() {
    let mut graph = G::with_capacity(3, 2);
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    let three = graph.add_node(3);

    let one_two = G::EdgeId::between(one, two);
    let two_three = G::EdgeId::between(two, three);
    graph.insert_edge(one_two, 2.0);
    graph.insert_edge(two_three, 3.0);

    let nodes = graph
        .node_ids()
        .map(|node_id| graph.node(node_id))
        .collect::<Vec<_>>();

    let edges = graph
        .edge_ids()
        .map(|edge_id| graph.weight(edge_id))
        .collect::<Vec<_>>();

    assert_eq!(nodes, vec![Some(&1), Some(&2), Some(&3)]);
    assert!(edges.contains(&Some(&2.0)));
    assert!(edges.contains(&Some(&3.0)));
    assert!(edges.contains(&Some(&1.0)) == false);
}

pub fn graph_index_adjacent<G: Graph<usize, f32>>() {
    let mut graph = G::with_capacity(3, 2);
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    let three = graph.add_node(3);

    let one_two = G::EdgeId::between(one, two);
    let one_three = G::EdgeId::between(one, three);
    graph.insert_edge(one_two, 2.0);
    graph.insert_edge(one_three, 3.0);

    let one_adj_ids = graph.adjacent_node_ids(one).collect::<Vec<_>>();
    assert_eq!(one_adj_ids, vec![two, three]);
}
