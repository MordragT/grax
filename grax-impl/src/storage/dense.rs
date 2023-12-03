use grax_core::{
    edge::{Edge, EdgeRef, EdgeRefMut},
    index::{EdgeId, NodeId},
};

use super::EdgeStorage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenseMatrix<W> {
    mat: Vec<Vec<Option<Edge<usize, W>>>>,
}

impl<W> DenseMatrix<W> {
    pub fn col(&self, col: usize) -> impl Iterator<Item = EdgeRef<'_, usize, W>> {
        self.mat.iter().flat_map(move |edges| {
            edges.iter().filter_map(move |edge| {
                if let Some(edge) = edge && edge.to() == NodeId::new_unchecked(col) {
                    Some(edge.into())
                } else {
                    None
                }
            })
        })
    }

    /// Returns the number of non-zero elements in the matrix
    pub fn nnz(&self) -> usize {
        self.mat
            .iter()
            .filter(|rows| !rows.iter().all(|item| item.is_none()))
            .count()
    }
}

impl<W> IntoIterator for DenseMatrix<W> {
    type Item = Edge<usize, W>;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.mat
            .into_iter()
            .flat_map(|neigh| neigh.into_iter().filter_map(|edge| edge))
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
        let mat = vec![vec![None; count]; count];

        Self { mat }
    }

    fn capacity(&self) -> usize {
        self.mat.capacity() * 2
    }

    fn count(&self) -> usize {
        self.nnz()
    }

    fn is_empty(&self) -> bool {
        self.mat
            .iter()
            .all(|rows| rows.iter().all(|item| item.is_none()))
    }

    fn clear(&mut self) {
        for row in &mut self.mat {
            row.fill(None)
        }
    }

    fn into_iter_unstable(self) -> Self::IntoIter {
        self.into_iter()
    }

    fn iter_unstable(&self) -> Self::Iter<'_> {
        self.mat.iter().flat_map(|edges| {
            edges
                .iter()
                .filter_map(|edge| edge.as_ref().map(Into::into))
        })
    }

    fn iter_mut_unstable(&mut self) -> Self::IterMut<'_> {
        self.mat.iter_mut().flat_map(|edges| {
            edges
                .iter_mut()
                .filter_map(|edge| edge.as_mut().map(Into::into))
        })
    }

    fn indices(&self) -> Self::Indices<'_> {
        self.iter_unstable().map(|edge| edge.edge_id)
    }

    fn allocate(&mut self, additional: usize) {
        let size = self.mat.len() + additional;

        for row in &mut self.mat {
            row.resize(size, None)
        }

        self.mat.resize(size, vec![None; size]);
    }

    fn insert(&mut self, from: usize, to: usize, weight: W) -> EdgeId<usize> {
        assert!(from.max(to) < self.mat.len());

        let edge_id = EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to));
        let edge = Edge::new(edge_id, weight);
        self.mat[from][to] = Some(edge);
        edge_id
    }

    fn extend(&mut self, edges: impl IntoIterator<Item = (usize, usize, W)>) {
        for (from, to, weight) in edges {
            self.insert(from, to, weight);
        }
    }

    fn remove(&mut self, from: usize, to: usize) -> Option<Edge<usize, W>> {
        if let Some(neigh) = self.mat.get_mut(from) {
            if let Some(edge) = neigh.get_mut(to) {
                std::mem::replace(edge, None)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<EdgeRef<'_, usize, W>> {
        // TODO allow out of bounds
        self.mat[row][col].as_ref().map(Into::into)
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<EdgeRefMut<'_, usize, W>> {
        // TODO allow out of bounds
        self.mat[row][col].as_mut().map(Into::into)
    }

    fn iter_adjacent_unstable(&self, index: usize) -> Self::Adjacent<'_> {
        self.mat[index]
            .iter()
            .filter_map(|edge| edge.as_ref().map(Into::into))
    }

    fn iter_adjacent_mut_unstable(&mut self, index: usize) -> Self::AdjacentMut<'_> {
        self.mat[index]
            .iter_mut()
            .filter_map(|edge| edge.as_mut().map(Into::into))
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
