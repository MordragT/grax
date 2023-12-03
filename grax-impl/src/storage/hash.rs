use std::collections::HashMap;

use grax_core::{
    edge::{Edge, EdgeRef, EdgeRefMut},
    index::{EdgeId, NodeId},
};

use super::EdgeStorage;

pub type HashStorage<W> = HashMap<EdgeId<usize>, W>;

impl<W> EdgeStorage<W> for HashStorage<W> {
    type IntoIter = impl Iterator<Item = Edge<usize, W>>;

    type Iter<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    type IterMut<'a> = impl Iterator<Item = EdgeRefMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    type Adjacent<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
        where
            W: 'a,
            Self: 'a;

    type AdjacentMut<'a> = impl Iterator<Item = EdgeRefMut<'a, usize, W>> + 'a
        where
            W: 'a,
            Self: 'a;

    type Indices<'a> = impl Iterator<Item = EdgeId<usize>> + 'a
            where
                Self: 'a;

    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(edge_count: usize) -> Self {
        Self::with_capacity(edge_count)
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn count(&self) -> usize {
        self.len()
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn into_iter_unstable(self) -> Self::IntoIter {
        self.into_iter()
            .map(|(edge_id, weight)| Edge::new(edge_id, weight))
    }

    fn iter_unstable(&self) -> Self::Iter<'_> {
        self.iter()
            .map(|(edge_id, weight)| EdgeRef::new(*edge_id, weight))
    }

    fn iter_mut_unstable(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
            .map(|(edge_id, weight)| EdgeRefMut::new(*edge_id, weight))
    }

    fn indices(&self) -> Self::Indices<'_> {
        self.keys().cloned()
    }

    fn allocate(&mut self, _: usize) {}

    fn insert(&mut self, from: usize, to: usize, weight: W) -> EdgeId<usize> {
        let edge_id = EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to));
        self.insert(edge_id, weight);
        edge_id
    }

    fn extend(&mut self, edges: impl IntoIterator<Item = (usize, usize, W)>) {
        let edges = edges.into_iter().map(|(from, to, weight)| {
            let edge_id =
                EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to));
            (edge_id, weight)
        });

        std::iter::Extend::extend(self, edges);
    }

    fn remove(&mut self, from: usize, to: usize) -> Option<Edge<usize, W>> {
        let edge_id = EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to));
        self.remove(&edge_id)
            .map(|weight| Edge::new(edge_id, weight))
    }

    fn get(&self, from: usize, to: usize) -> Option<EdgeRef<'_, usize, W>> {
        let edge_id = EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to));
        self.get(&edge_id)
            .map(|weight| EdgeRef::new(edge_id, weight))
    }

    fn get_mut(&mut self, from: usize, to: usize) -> Option<EdgeRefMut<'_, usize, W>> {
        let edge_id = EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to));
        self.get_mut(&edge_id)
            .map(|weight| EdgeRefMut::new(edge_id, weight))
    }

    fn iter_adjacent_unstable(&self, index: usize) -> Self::Adjacent<'_> {
        self.iter().filter_map(move |(edge_id, weight)| {
            if edge_id.from() == NodeId::new_unchecked(index) {
                Some(EdgeRef::new(*edge_id, weight))
            } else {
                None
            }
        })
    }

    fn iter_adjacent_mut_unstable(&mut self, index: usize) -> Self::AdjacentMut<'_> {
        self.iter_mut().filter_map(move |(edge_id, weight)| {
            if edge_id.from() == NodeId::new_unchecked(index) {
                Some(EdgeRefMut::new(*edge_id, weight))
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::storage::test::{
        edge_storage_adjacent, edge_storage_capacity, edge_storage_clear, edge_storage_count,
        edge_storage_get, edge_storage_remove,
    };

    use super::HashStorage;
    #[test]
    fn hash_edge_storage_capacity() {
        edge_storage_capacity::<HashStorage<f32>>()
    }
    #[test]
    fn hash_edge_storage_count() {
        edge_storage_count::<HashStorage<f32>>()
    }
    #[test]
    fn hash_edge_storage_clear() {
        edge_storage_clear::<HashStorage<f32>>()
    }
    #[test]
    fn hash_edge_storage_remove() {
        edge_storage_remove::<HashStorage<f32>>()
    }
    #[test]
    fn hash_edge_storage_get() {
        edge_storage_get::<HashStorage<f32>>()
    }
    #[test]
    fn hash_edge_storage_adjacent() {
        edge_storage_adjacent::<HashStorage<f32>>()
    }
}
