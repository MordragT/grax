use grax_core::{
    edge::{Edge, EdgeRef, EdgeRefMut},
    index::{EdgeId, NodeId},
};

use super::EdgeStorage;

struct CsrMatrixIterator<W> {
    row_offsets: Vec<RowOffset>,
    edges: Vec<Edge<usize, W>>,
}

impl<W> CsrMatrixIterator<W> {
    pub fn new(matrix: CsrMatrix<W>) -> Self {
        let CsrMatrix {
            mut row_offsets,
            edges,
        } = matrix;

        row_offsets.reverse();

        Self { row_offsets, edges }
    }
}

impl<W> Iterator for CsrMatrixIterator<W> {
    type Item = Vec<Edge<usize, W>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(offset) = self.row_offsets.pop() {
            let range = 0..offset.end - offset.start;
            // TODO check if preallocated buffering might be faster
            let mut nodes = self.edges.drain(range).collect::<Vec<_>>();
            nodes.sort_unstable_by(|a, b| a.to().cmp(&b.to()));
            Some(nodes)
        } else {
            None
        }
    }
}

struct CsrMatrixIterMut<'a, W> {
    row_offsets: Vec<RowOffset>,
    edges: &'a mut Vec<Edge<usize, W>>,
}

impl<'a, W> CsrMatrixIterMut<'a, W> {
    pub fn new(matrix: &'a mut CsrMatrix<W>) -> Self {
        let mut row_offsets = matrix.row_offsets.clone();
        row_offsets.reverse();

        Self {
            row_offsets,
            edges: &mut matrix.edges,
        }
    }
}

impl<'a, W: 'a> Iterator for CsrMatrixIterMut<'a, W> {
    type Item = &'a mut [Edge<usize, W>];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(offset) = self.row_offsets.pop() {
            let range = offset.start..offset.end;
            let row_nodes = &mut self.edges[range] as *mut _;
            Some(unsafe { &mut *row_nodes })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RowOffset {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CsrMatrix<W> {
    row_offsets: Vec<RowOffset>,
    edges: Vec<Edge<usize, W>>,
}

impl<W> CsrMatrix<W> {
    /// Returns an iterator return edges in a row major order
    /// Does not preserve the column order
    pub fn iter(&self) -> impl Iterator<Item = EdgeRef<'_, usize, W>> {
        self.row_offsets
            .iter()
            .map(|offset| &self.edges[offset.start..offset.end])
            .flatten()
            .map(Into::into)
    }

    /// Returns an iterator return edges in a row major order
    /// Does not preserve the column order
    pub fn iter_mut(&mut self) -> impl Iterator<Item = EdgeRefMut<'_, usize, W>> {
        CsrMatrixIterMut::new(self)
            .into_iter()
            .flatten()
            .map(Into::into)
    }

    /// Returns the number of non-zero elements in the matrix
    pub fn nnz(&self) -> usize {
        self.edges.len()
    }

    pub fn col(&self, col: usize) -> impl Iterator<Item = EdgeRef<'_, usize, W>> {
        self.edges
            .iter()
            .filter(move |node| node.to() == NodeId::new_unchecked(col))
            .map(Into::into)
    }

    pub fn row(&self, row: usize) -> impl Iterator<Item = EdgeRef<'_, usize, W>> {
        self.row_offsets
            .get(row)
            .into_iter()
            .flat_map(|offset| self.edges[offset.start..offset.end].iter().map(Into::into))
    }

    pub fn row_mut(&mut self, row: usize) -> impl Iterator<Item = EdgeRefMut<'_, usize, W>> {
        if let Some(offset) = self.row_offsets.get(row) {
            Some(
                self.edges[offset.start..offset.end]
                    .iter_mut()
                    .map(Into::into),
            )
            .into_iter()
            .flatten()
        } else {
            None.into_iter().flatten()
        }
    }
}

impl<W> IntoIterator for CsrMatrix<W> {
    type Item = Edge<usize, W>;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        CsrMatrixIterator::new(self).into_iter().flatten()
    }
}

impl<W> EdgeStorage<W> for CsrMatrix<W> {
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
        Self {
            edges: Vec::new(),
            row_offsets: vec![RowOffset { start: 0, end: 0 }],
        }
    }

    fn with_capacity(edge_count: usize) -> Self {
        Self {
            edges: Vec::with_capacity(edge_count),
            row_offsets: vec![RowOffset { start: 0, end: 0 }],
        }
    }

    fn capacity(&self) -> usize {
        self.edges.capacity()
    }

    fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    fn clear(&mut self) {
        self.edges.clear();
        self.row_offsets.fill(RowOffset { start: 0, end: 0 });
    }

    fn count(&self) -> usize {
        self.nnz()
    }

    fn into_iter_unstable(self) -> Self::IntoIter {
        self.edges.into_iter()
    }

    fn iter_unstable(&self) -> Self::Iter<'_> {
        self.edges.iter().map(Into::into)
    }

    fn iter_mut_unstable(&mut self) -> Self::IterMut<'_> {
        self.edges.iter_mut().map(Into::into)
    }

    fn iter_adjacent_unstable(&self, index: usize) -> Self::Adjacent<'_> {
        self.row(index)
    }

    fn iter_adjacent_mut_unstable(&mut self, index: usize) -> Self::AdjacentMut<'_> {
        self.row_mut(index)
    }

    fn indices(&self) -> Self::Indices<'_> {
        self.iter_unstable().map(|edge| edge.edge_id)
    }

    fn allocate(&mut self, additional: usize) {
        let offset = self.row_offsets.last().unwrap().clone();

        for _ in 0..additional {
            self.row_offsets.push(offset.clone());
        }
    }

    fn insert(&mut self, from: usize, to: usize, weight: W) -> EdgeId<usize> {
        assert!(from < self.row_offsets.len());

        let edge_id = EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to));
        let edge = Edge::new(edge_id, weight);

        let offset = &mut self.row_offsets[from];
        let node_index = offset.end;
        offset.end += 1;

        self.edges.insert(node_index, edge);

        for offset in &mut self.row_offsets[from + 1..] {
            offset.start += 1;
            offset.end += 1;
        }

        edge_id
    }

    fn extend(&mut self, edges: impl IntoIterator<Item = (usize, usize, W)>) {
        for (from, to, weight) in edges {
            self.insert(from, to, weight);
        }
    }

    fn remove(&mut self, from: usize, to: usize) -> Option<Edge<usize, W>> {
        if let Some(offset) = self.row_offsets.get_mut(from) {
            if let Some(pos) = self.edges[offset.start..offset.end]
                .iter()
                .position(|edge| edge.to() == NodeId::new_unchecked(to))
            {
                offset.end -= 1;

                for offset in &mut self.row_offsets[from + 1..] {
                    offset.start -= 1;
                    offset.end -= 1;
                }

                Some(self.edges.remove(pos))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get(&self, from: usize, to: usize) -> Option<EdgeRef<'_, usize, W>> {
        if let Some(offset) = self.row_offsets.get(from) {
            self.edges[offset.start..offset.end]
                .iter()
                .find_map(|node| {
                    if node.to() == NodeId::new_unchecked(to) {
                        Some(node.into())
                    } else {
                        None
                    }
                })
        } else {
            None
        }
    }

    fn get_mut(&mut self, from: usize, to: usize) -> Option<EdgeRefMut<'_, usize, W>> {
        if let Some(offset) = self.row_offsets.get(from) {
            self.edges[offset.start..offset.end]
                .iter_mut()
                .find_map(|node| {
                    if node.to() == NodeId::new_unchecked(to) {
                        Some(node.into())
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

    use super::CsrMatrix;
    #[test]
    fn csr_edge_storage_capacity() {
        edge_storage_capacity::<CsrMatrix<f32>>()
    }
    #[test]
    fn csr_edge_storage_count() {
        edge_storage_count::<CsrMatrix<f32>>()
    }
    #[test]
    fn csr_edge_storage_clear() {
        edge_storage_clear::<CsrMatrix<f32>>()
    }
    #[test]
    fn csr_edge_storage_remove() {
        edge_storage_remove::<CsrMatrix<f32>>()
    }
    #[test]
    fn csr_edge_storage_get() {
        edge_storage_get::<CsrMatrix<f32>>()
    }
    #[test]
    fn csr_edge_storage_adjacent() {
        edge_storage_adjacent::<CsrMatrix<f32>>()
    }
}
