use grax_core::{
    edge::{Edge, EdgeRef, EdgeRefMut},
    index::{EdgeId, NodeId},
};
use stable_vec::StableVec;

use super::EdgeStorage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenseMatrix<W> {
    mat: Vec<StableVec<Edge<usize, W>>>,
}

impl<W> DenseMatrix<W> {
    pub fn col(&self, col: usize) -> impl Iterator<Item = EdgeRef<'_, usize, W>> {
        self.mat.iter().flat_map(move |edges| {
            edges.iter().filter_map(
                move |(to, edge)| {
                    if to == col {
                        Some(edge.into())
                    } else {
                        None
                    }
                },
            )
        })
    }

    /// Returns the number of non-zero elements in the matrix
    pub fn nnz(&self) -> usize {
        self.mat.iter().map(|row| row.num_elements()).sum()
    }
}

impl<W> IntoIterator for DenseMatrix<W> {
    type Item = Edge<usize, W>;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.mat
            .into_iter()
            .flat_map(|neigh| neigh.into_iter().map(|(_, edge)| edge))
    }
}

impl<W: Clone> EdgeStorage<W> for DenseMatrix<W> {
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
        Self { mat: Vec::new() }
    }

    fn with_capacity(edge_count: usize) -> Self {
        let count = edge_count / 2;
        let mat = vec![StableVec::with_capacity(count); count];

        Self { mat }
    }

    fn capacity(&self) -> usize {
        self.mat.capacity() * 2
    }

    fn count(&self) -> usize {
        self.nnz()
    }

    fn is_empty(&self) -> bool {
        self.mat.iter().all(|row| row.is_empty())
    }

    fn clear(&mut self) {
        for row in &mut self.mat {
            row.clear()
        }
    }

    fn into_iter_unstable(self) -> Self::IntoIter {
        self.into_iter()
    }

    fn iter_unstable(&self) -> Self::Iter<'_> {
        self.mat
            .iter()
            .flat_map(|edges| edges.iter().map(|(_, edge)| edge.into()))
    }

    fn iter_mut_unstable(&mut self) -> Self::IterMut<'_> {
        self.mat
            .iter_mut()
            .flat_map(|edges| edges.iter_mut().map(|(_, edge)| edge.into()))
    }

    fn indices(&self) -> Self::Indices<'_> {
        self.mat.iter().enumerate().flat_map(|(from, edges)| {
            edges.indices().map(move |to| {
                EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to))
            })
        })
    }

    fn allocate(&mut self, additional: usize) {
        let size = self.mat.len() + additional;

        for row in &mut self.mat {
            row.reserve_for(size);
        }

        self.mat.resize(size, StableVec::with_capacity(size));
    }

    fn insert(&mut self, from: usize, to: usize, weight: W) -> EdgeId<usize> {
        assert!(from.max(to) < self.mat.len());

        let edge_id = EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to));
        let edge = Edge::new(edge_id, weight);
        self.mat[from].insert(to, edge);
        edge_id
    }

    fn extend(&mut self, edges: impl IntoIterator<Item = (usize, usize, W)>) {
        for (from, to, weight) in edges {
            self.insert(from, to, weight);
        }
    }

    fn remove(&mut self, from: usize, to: usize) -> Option<Edge<usize, W>> {
        if let Some(neigh) = self.mat.get_mut(from) {
            neigh.remove(to)
        } else {
            None
        }
    }

    fn get(&self, from: usize, to: usize) -> Option<EdgeRef<'_, usize, W>> {
        // TODO allow out of bounds
        self.mat[from].get(to).map(Into::into)
    }

    fn get_mut(&mut self, from: usize, to: usize) -> Option<EdgeRefMut<'_, usize, W>> {
        // TODO allow out of bounds
        self.mat[from].get_mut(to).map(Into::into)
    }

    fn iter_adjacent_unstable(&self, node_id: usize) -> Self::Adjacent<'_> {
        self.mat[node_id].iter().map(|(_, edge)| edge.into())
    }

    fn iter_adjacent_mut_unstable(&mut self, node_id: usize) -> Self::AdjacentMut<'_> {
        self.mat[node_id].iter_mut().map(|(_, edge)| edge.into())
    }
}

#[cfg(test)]
mod test {
    use crate::storage::test::{
        edge_storage_adjacent, edge_storage_capacity, edge_storage_clear, edge_storage_count,
        edge_storage_get, edge_storage_remove,
    };

    use super::DenseMatrix;
    #[test]
    fn dense_edge_storage_capacity() {
        edge_storage_capacity::<DenseMatrix<f32>>()
    }
    #[test]
    fn dense_edge_storage_count() {
        edge_storage_count::<DenseMatrix<f32>>()
    }
    #[test]
    fn dense_edge_storage_clear() {
        edge_storage_clear::<DenseMatrix<f32>>()
    }
    #[test]
    fn dense_edge_storage_remove() {
        edge_storage_remove::<DenseMatrix<f32>>()
    }
    #[test]
    fn dense_edge_storage_get() {
        edge_storage_get::<DenseMatrix<f32>>()
    }
    #[test]
    fn dense_edge_storage_adjacent() {
        edge_storage_adjacent::<DenseMatrix<f32>>()
    }
}
