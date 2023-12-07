use grax_core::{
    edge::EdgeRef,
    index::{EdgeId, NodeId},
};

use super::EdgeStorage;

fn id(id: usize) -> NodeId<usize> {
    NodeId::new_unchecked(id)
}

fn edge_id(from: usize, to: usize) -> EdgeId<usize> {
    EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to))
}

pub fn edge_storage_capacity<S: EdgeStorage<usize, f32>>() {
    let storage = S::with_capacity(5, 10);
    let cap = storage.edges_capacity();
    assert!(cap >= 10);
    assert!(cap <= 5 * 5);
}

pub fn edge_storage_count<S: EdgeStorage<usize, f32>>() {
    let mut storage = S::with_capacity(5, 10);

    assert!(storage.edges_empty());

    let zero = id(0);
    let one = id(1);
    let two = id(2);
    let three = id(3);

    storage.allocate(4);
    storage.insert_edge(zero, one, 1.0);
    storage.insert_edge(one, three, 2.0);
    storage.insert_edge(three, two, 3.0);

    assert_eq!(storage.edge_count(), 3);
}

pub fn edge_storage_clear<S: EdgeStorage<usize, f32>>() {
    let mut storage = S::with_capacity(5, 10);

    let zero = id(0);
    let one = id(1);
    let two = id(2);
    let three = id(3);

    storage.allocate(4);
    storage.insert_edge(zero, one, 1.0);
    storage.insert_edge(one, three, 2.0);
    storage.insert_edge(three, two, 3.0);

    storage.clear();
    assert!(storage.edges_empty());
}

pub fn edge_storage_remove<S: EdgeStorage<usize, f32>>() {
    let mut storage = S::with_capacity(5, 10);

    let zero = id(0);
    let one = id(1);
    let two = id(2);
    let three = id(3);

    storage.allocate(4);
    storage.insert_edge(zero, one, 1.0);
    storage.insert_edge(one, three, 2.0);
    storage.insert_edge(three, two, 3.0);

    assert_eq!(
        storage.remove_edge(edge_id(0, 1)).map(|edge| edge.weight),
        Some(1.0)
    );
    assert_eq!(storage.remove_edge(edge_id(0, 1)), None);

    assert_eq!(storage.edge_count(), 2);
}

pub fn edge_storage_get<S: EdgeStorage<usize, f32>>() {
    let mut storage = S::with_capacity(5, 10);

    let zero = id(0);
    let one = id(1);
    let two = id(2);
    let three = id(3);

    storage.allocate(4);
    storage.insert_edge(zero, one, 1.0);
    storage.insert_edge(one, three, 2.0);
    storage.insert_edge(three, two, 3.0);

    let zero_one = assert_eq!(
        storage.edge(edge_id(0, 1)).map(|edge| edge.weight),
        Some(&1.0)
    );
    assert_eq!(storage.edge(edge_id(0, 2)), None);
}

pub fn edge_storage_adjacent<S: EdgeStorage<usize, f32>>() {
    let mut storage = S::with_capacity(5, 10);

    let zero = id(0);
    let one = id(1);
    let two = id(2);
    let three = id(3);

    storage.allocate(4);
    storage.insert_edge(zero, one, 1.0);
    storage.insert_edge(zero, three, 2.0);

    let nodes = storage.iter_adjacent_edges(zero).collect::<Vec<_>>();

    assert!(nodes.contains(&EdgeRef::new(edge_id(0, 1), &1.0)));

    assert!(!nodes.contains(&EdgeRef::new(edge_id(1, 0), &1.0)));
}
