use grax_core::{
    edge::EdgeRef,
    index::{EdgeId, NodeId},
};

use super::EdgeStorage;

pub fn edge_storage_capacity<S: EdgeStorage<f32>>() {
    let storage = S::with_capacity(10);
    assert_eq!(storage.capacity(), 10);
}

pub fn edge_storage_count<S: EdgeStorage<f32>>() {
    let mut storage = S::with_capacity(10);

    assert!(storage.is_empty());

    storage.allocate(4);
    storage.insert(0, 1, 1.0);
    storage.insert(1, 3, 2.0);
    storage.insert(3, 2, 3.0);

    assert_eq!(storage.count(), 3);
}

pub fn edge_storage_clear<S: EdgeStorage<f32>>() {
    let mut storage = S::with_capacity(10);

    storage.allocate(4);
    storage.insert(0, 1, 1.0);
    storage.insert(1, 3, 2.0);
    storage.insert(3, 2, 3.0);

    storage.clear();
    assert!(storage.is_empty());
}

pub fn edge_storage_remove<S: EdgeStorage<f32>>() {
    let mut storage = S::with_capacity(10);

    storage.allocate(4);
    storage.insert(0, 1, 1.0);
    storage.insert(1, 3, 2.0);
    storage.insert(3, 2, 3.0);

    assert_eq!(storage.remove(0, 1).map(|edge| edge.weight), Some(1.0));
    assert_eq!(storage.remove(0, 1), None);

    assert_eq!(storage.count(), 2);
}

pub fn edge_storage_get<S: EdgeStorage<f32>>() {
    let mut storage = S::with_capacity(10);

    storage.allocate(4);
    storage.insert(0, 1, 1.0);
    storage.insert(1, 3, 2.0);
    storage.insert(3, 2, 3.0);

    assert_eq!(storage.get(0, 1).map(|edge| edge.weight), Some(&1.0));
    assert_eq!(storage.get(0, 2), None);
}

pub fn edge_storage_adjacent<S: EdgeStorage<f32>>() {
    let mut storage = S::with_capacity(10);

    storage.allocate(4);
    storage.insert(0, 1, 1.0);
    storage.insert(0, 3, 2.0);

    let nodes = storage.iter_adjacent_unstable(0).collect::<Vec<_>>();

    assert!(nodes.contains(&EdgeRef::new(
        EdgeId::new_unchecked(NodeId::new_unchecked(0), NodeId::new_unchecked(1)),
        &1.0
    )));

    assert!(!nodes.contains(&EdgeRef::new(
        EdgeId::new_unchecked(NodeId::new_unchecked(1), NodeId::new_unchecked(0)),
        &1.0
    )));
}
