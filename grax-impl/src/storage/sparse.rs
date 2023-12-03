// each row correpsonds to the neighbours of the index into the row

use grax_core::{
    edge::{Edge, EdgeRef, EdgeRefMut},
    index::{EdgeId, NodeId},
};

use super::EdgeStorage;

/// A N*M sized sparse Matrix
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseMatrix<W> {
    edges: Vec<Edge<usize, W>>,
}

impl<W> SparseMatrix<W> {
    /// Returns the elements of a specific column
    pub fn col(&self, col: usize) -> impl Iterator<Item = EdgeRef<'_, usize, W>> {
        self.edges
            .iter()
            .filter(move |edge| edge.to() == NodeId::new_unchecked(col))
            .map(Into::into)
    }

    /// Returns the number of non-zero elements in the matrix
    pub fn nnz(&self) -> usize {
        self.edges.len()
    }
}

impl<W> IntoIterator for SparseMatrix<W> {
    type Item = Edge<usize, W>;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.edges.into_iter()
    }
}

impl<W> EdgeStorage<W> for SparseMatrix<W> {
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

    fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    fn count(&self) -> usize {
        self.nnz()
    }

    fn clear(&mut self) {
        self.edges.clear();
    }

    fn into_iter_unstable(self) -> Self::IntoIter {
        self.into_iter()
    }

    fn iter_unstable(&self) -> Self::Iter<'_> {
        self.edges.iter().map(Into::into)
    }

    fn iter_mut_unstable(&mut self) -> Self::IterMut<'_> {
        self.edges.iter_mut().map(Into::into)
    }

    fn indices(&self) -> Self::Indices<'_> {
        self.iter_unstable().map(|edge| edge.edge_id)
    }

    fn allocate(&mut self, _: usize) {}

    fn insert(&mut self, from: usize, to: usize, weight: W) -> EdgeId<usize> {
        let edge_id = EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to));
        let edge = Edge::new(edge_id, weight);
        self.edges.push(edge);
        edge_id
    }

    fn extend(&mut self, edges: impl IntoIterator<Item = (usize, usize, W)>) {
        for (from, to, weight) in edges {
            self.insert(from, to, weight);
        }
    }

    fn remove(&mut self, from: usize, to: usize) -> Option<Edge<usize, W>> {
        if let Some(pos) = self.edges.iter().position(|edge| {
            edge.from() == NodeId::new_unchecked(from) && edge.to() == NodeId::new_unchecked(to)
        }) {
            Some(self.edges.remove(pos))
        } else {
            None
        }
    }

    fn get(&self, from: usize, to: usize) -> Option<EdgeRef<'_, usize, W>> {
        self.edges
            .iter()
            .find(|edge| {
                edge.from() == NodeId::new_unchecked(from) && edge.to() == NodeId::new_unchecked(to)
            })
            .map(Into::into)
    }

    fn get_mut(
        &mut self,
        from: usize,
        to: usize,
    ) -> Option<grax_core::prelude::EdgeRefMut<'_, usize, W>> {
        self.edges
            .iter_mut()
            .find(|edge| {
                edge.from() == NodeId::new_unchecked(from) && edge.to() == NodeId::new_unchecked(to)
            })
            .map(Into::into)
    }

    fn iter_adjacent_unstable(&self, index: usize) -> Self::Adjacent<'_> {
        self.edges
            .iter()
            .filter(move |edge| edge.from() == NodeId::new_unchecked(index))
            .map(Into::into)
    }

    fn iter_adjacent_mut_unstable(&mut self, index: usize) -> Self::AdjacentMut<'_> {
        self.edges
            .iter_mut()
            .filter(move |edge| edge.from() == NodeId::new_unchecked(index))
            .map(Into::into)
    }
}

#[cfg(test)]
mod test {
    use crate::storage::test::{
        edge_storage_adjacent, edge_storage_capacity, edge_storage_clear, edge_storage_count,
        edge_storage_get, edge_storage_remove,
    };

    use super::SparseMatrix;

    #[test]
    fn sparse_edge_storage_capacity() {
        edge_storage_capacity::<SparseMatrix<f32>>()
    }

    #[test]
    fn sparse_edge_storage_count() {
        edge_storage_count::<SparseMatrix<f32>>()
    }

    #[test]
    fn sparse_edge_storage_clear() {
        edge_storage_clear::<SparseMatrix<f32>>()
    }

    #[test]
    fn sparse_edge_storage_remove() {
        edge_storage_remove::<SparseMatrix<f32>>()
    }
    #[test]
    fn sparse_edge_storage_get() {
        edge_storage_get::<SparseMatrix<f32>>()
    }
    #[test]
    fn sparse_edge_storage_adjacent() {
        edge_storage_adjacent::<SparseMatrix<f32>>()
    }
}
