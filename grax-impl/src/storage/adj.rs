use grax_core::{
    edge::{Edge, EdgeRef, EdgeRefMut},
    index::{EdgeId, NodeId},
};

use super::EdgeStorage;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AdjacencyList<W> {
    edges: Vec<Vec<Edge<usize, W>>>,
}

impl<W> EdgeStorage<W> for AdjacencyList<W> {
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
        Self { edges: Vec::new() }
    }

    fn with_capacity(edge_count: usize) -> Self {
        Self {
            edges: Vec::with_capacity(edge_count),
        }
    }

    fn capacity(&self) -> usize {
        self.edges.capacity()
    }

    fn count(&self) -> usize {
        let count = self.edges.iter().fold(0, |mut akku, edges| {
            akku += edges.len();
            akku
        });
        count
    }

    fn clear(&mut self) {
        self.edges.clear();
    }

    fn into_iter_unstable(self) -> Self::IntoIter {
        self.edges.into_iter().flatten()
    }

    fn iter_unstable(&self) -> Self::Iter<'_> {
        self.edges.iter().flatten().map(Into::into)
    }

    fn iter_mut_unstable(&mut self) -> Self::IterMut<'_> {
        self.edges.iter_mut().flatten().map(Into::into)
    }

    fn iter_adjacent_unstable(&self, index: usize) -> Self::Adjacent<'_> {
        self.edges
            .get(index)
            .into_iter()
            .flat_map(|adj| adj.iter().map(Into::into))
    }

    fn iter_adjacent_mut_unstable(&mut self, index: usize) -> Self::AdjacentMut<'_> {
        self.edges
            .get_mut(index)
            .into_iter()
            .flat_map(|adj| adj.iter_mut().map(Into::into))
    }

    fn indices(&self) -> Self::Indices<'_> {
        self.iter_unstable().map(|edge| edge.edge_id)
    }

    fn allocate(&mut self, additional: usize) {
        let size = self.edges.len() + additional;
        self.edges.resize_with(size, || Vec::new());
    }

    fn insert(&mut self, from: usize, to: usize, weight: W) -> EdgeId<usize> {
        let edge_id = EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to));
        self.edges[from].push(Edge::new(edge_id, weight));
        edge_id
    }

    fn extend(&mut self, edges: impl IntoIterator<Item = (usize, usize, W)>) {
        for (from, to, weight) in edges {
            self.insert(from, to, weight);
        }
    }

    fn remove(&mut self, from: usize, to: usize) -> Option<Edge<usize, W>> {
        if let Some(pos) = self.edges[from]
            .iter()
            .position(|edge| edge.edge_id.to() == NodeId::new_unchecked(to))
        {
            Some(self.edges[from].remove(pos))
        } else {
            None
        }
    }

    fn get(&self, from: usize, to: usize) -> Option<EdgeRef<'_, usize, W>> {
        if let Some(adj) = self.edges.get(from) {
            adj.iter().find_map(|edge| {
                if edge.edge_id.to() == NodeId::new_unchecked(to) {
                    Some(edge.into())
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    fn get_mut(&mut self, from: usize, to: usize) -> Option<EdgeRefMut<'_, usize, W>> {
        if let Some(adj) = self.edges.get_mut(from) {
            adj.iter_mut().find_map(|edge| {
                if edge.edge_id.to() == NodeId::new_unchecked(to) {
                    Some(edge.into())
                } else {
                    None
                }
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::storage::test::{
        edge_storage_adjacent, edge_storage_capacity, edge_storage_clear, edge_storage_count,
        edge_storage_get, edge_storage_remove,
    };

    use super::AdjacencyList;
    #[test]
    fn adj_edge_storage_capacity() {
        edge_storage_capacity::<AdjacencyList<f32>>()
    }
    #[test]
    fn adj_edge_storage_count() {
        edge_storage_count::<AdjacencyList<f32>>()
    }
    #[test]
    fn adj_edge_storage_clear() {
        edge_storage_clear::<AdjacencyList<f32>>()
    }
    #[test]
    fn adj_edge_storage_remove() {
        edge_storage_remove::<AdjacencyList<f32>>()
    }
    #[test]
    fn adj_edge_storage_get() {
        edge_storage_get::<AdjacencyList<f32>>()
    }
    #[test]
    fn adj_edge_storage_adjacent() {
        edge_storage_adjacent::<AdjacencyList<f32>>()
    }
}
