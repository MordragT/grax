use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use grax_core::{
    collections::{
        EdgeCollection, EdgeCount, EdgeIter, EdgeIterMut, EdgeMap, FixedEdgeMap, GetEdge,
        GetEdgeMut, InsertEdge, Keyed, RemoveEdge,
    },
    edge::{Edge, EdgeMut, EdgeRef},
    graph::{EdgeIterAdjacent, EdgeIterAdjacentMut},
    index::{EdgeId, NodeId},
    node,
};
use more_asserts::assert_lt;
use rayon::slice::ParallelSliceMut;
use serde::{Deserialize, Serialize};

use super::EdgeStorage;

struct CsrMatrixIterator<W> {
    row_offsets: Vec<RowOffset>,
    edges: Vec<Edge<usize, W>>,
}

impl<W: Debug> CsrMatrixIterator<W> {
    pub fn new(matrix: CsrMatrix<W>) -> Self {
        let CsrMatrix {
            mut row_offsets,
            edges,
        } = matrix;

        row_offsets.reverse();

        Self { row_offsets, edges }
    }
}

impl<W: Debug> Iterator for CsrMatrixIterator<W> {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
struct RowOffset {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CsrMatrix<W> {
    row_offsets: Vec<RowOffset>,
    edges: Vec<Edge<usize, W>>,
}

impl<W: Debug> CsrMatrix<W> {
    /// Returns an iterator return edges in a row major order
    /// Does not preserve the column order
    pub fn stable_iter(&self) -> impl Iterator<Item = EdgeRef<'_, usize, W>> {
        self.row_offsets
            .iter()
            .map(|offset| &self.edges[offset.start..offset.end])
            .flatten()
            .map(Into::into)
    }

    /// Returns an iterator return edges in a row major order
    /// Does not preserve the column order
    pub fn stable_iter_mut(&mut self) -> impl Iterator<Item = EdgeMut<'_, usize, W>> {
        CsrMatrixIterMut::new(self)
            .into_iter()
            .flatten()
            .map(Into::into)
    }

    pub fn into_stable_iter(self) -> impl Iterator<Item = Edge<usize, W>> {
        CsrMatrixIterator::new(self).into_iter().flatten()
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

    pub fn row_mut(&mut self, row: usize) -> impl Iterator<Item = EdgeMut<'_, usize, W>> {
        if let Some(offset) = self.row_offsets.get(row) {
            self.edges[offset.start..offset.end]
                .iter_mut()
                .map(Into::into)
        } else {
            [].iter_mut().map(Into::into)
        }
    }
}

impl<W: Debug> IntoIterator for CsrMatrix<W> {
    type Item = Edge<usize, W>;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.edges.into_iter()
    }
}

impl<W: Debug> Keyed for CsrMatrix<W> {
    type Key = usize;
}

impl<W: Debug> EdgeCollection for CsrMatrix<W> {
    type EdgeWeight = W;

    fn edges_capacity(&self) -> usize {
        self.edges.capacity()
    }
}

impl<W: Debug> EdgeCount for CsrMatrix<W> {
    fn edge_count(&self) -> usize {
        self.nnz()
    }
}

impl<W: Debug> Index<EdgeId<usize>> for CsrMatrix<W> {
    type Output = W;

    fn index(&self, index: EdgeId<usize>) -> &Self::Output {
        self.edge(index).unwrap().weight
    }
}

impl<W: Debug> IndexMut<EdgeId<usize>> for CsrMatrix<W> {
    fn index_mut(&mut self, index: EdgeId<usize>) -> &mut Self::Output {
        self.edge_mut(index).unwrap().weight
    }
}

impl<W: Debug> GetEdge for CsrMatrix<W> {
    fn edge(&self, edge_id: EdgeId<Self::Key>) -> Option<EdgeRef<Self::Key, Self::EdgeWeight>> {
        let (from, to) = edge_id.raw();
        if let Some(offset) = self.row_offsets.get(from) {
            if let Ok(relative) = self.edges[offset.start..offset.end]
                .binary_search_by_key(&NodeId::new_unchecked(to), |edge| edge.to())
            {
                let pos = relative + offset.start;
                Some((&self.edges[pos]).into())
            } else {
                None
            }
        } else {
            None
        }
    }

    // fn has_edge(
    //     &self,
    //     from: NodeId<Self::Key>,
    //     to: NodeId<Self::Key>,
    // ) -> Option<EdgeId<Self::Key>> {
    //     let edge_id = EdgeId::new_unchecked(from, to);
    //     if self.contains_edge_id(edge_id) {
    //         Some(edge_id)
    //     } else {
    //         None
    //     }
    // }
}

impl<W: Debug> GetEdgeMut for CsrMatrix<W> {
    fn edge_mut(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<EdgeMut<Self::Key, Self::EdgeWeight>> {
        let (from, to) = edge_id.raw();
        if let Some(offset) = self.row_offsets.get(from) {
            if let Ok(relative) = self.edges[offset.start..offset.end]
                .binary_search_by_key(&NodeId::new_unchecked(to), |edge| edge.to())
            {
                let pos = relative + offset.start;
                Some((&mut self.edges[pos]).into())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<W: Debug> InsertEdge for CsrMatrix<W> {
    fn insert_edge(
        &mut self,
        from: NodeId<Self::Key>,
        to: NodeId<Self::Key>,
        weight: Self::EdgeWeight,
    ) -> EdgeId<Self::Key> {
        assert_lt!(*from, self.row_offsets.len());

        let edge_id = EdgeId::new_unchecked(from, to);
        let edge = Edge::new(edge_id, weight);

        let offset = &mut self.row_offsets[*from];

        let node_index = match self.edges[offset.start..offset.end]
            .binary_search_by_key(&to, |edge| edge.to())
        {
            Ok(index) | Err(index) => index + offset.start,
        };

        offset.end += 1;

        self.edges.insert(node_index, edge);

        for offset in &mut self.row_offsets[*from + 1..] {
            offset.start += 1;
            offset.end += 1;
        }

        edge_id
    }

    fn reserve_edges(&mut self, additional: usize) {
        self.edges.reserve(additional)
    }
}

impl<W: Debug> RemoveEdge for CsrMatrix<W> {
    fn remove_edge(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<Edge<Self::Key, Self::EdgeWeight>> {
        let (from, to) = edge_id.raw();
        if let Some(offset) = self.row_offsets.get_mut(from) {
            if let Ok(relative) = self.edges[offset.start..offset.end]
                .binary_search_by_key(&NodeId::new_unchecked(to), |edge| edge.to())
            {
                offset.end -= 1;
                let pos = relative + offset.start;

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
}

impl<W: Debug> EdgeIter for CsrMatrix<W> {
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<usize>> + 'a
    where
        Self: 'a;
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
        where
            W: 'a,
            Self: 'a;

    fn edge_ids(&self) -> Self::EdgeIds<'_> {
        self.edges.iter().map(|edge| edge.edge_id)
    }

    fn iter_edges(&self) -> Self::Edges<'_> {
        self.edges.iter().map(Into::into)
    }
}

impl<W: Debug> EdgeIterMut for CsrMatrix<W> {
    type EdgesMut<'a> = impl Iterator<Item = EdgeMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_edges_mut(&mut self) -> Self::EdgesMut<'_> {
        self.edges.iter_mut().map(Into::into)
    }
}

impl<W: Debug> EdgeIterAdjacent for CsrMatrix<W> {
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<usize>> + 'a
    where
        Self: 'a;
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
        where
            W: 'a,
            Self: 'a;

    fn adjacent_edge_ids(&self, node_id: NodeId<Self::Key>) -> Self::EdgeIds<'_> {
        self.row(*node_id).map(|edge| edge.edge_id)
    }
    fn iter_adjacent_edges(&self, node_id: NodeId<Self::Key>) -> Self::Edges<'_> {
        self.row(*node_id)
    }
}

impl<W: Debug> EdgeIterAdjacentMut for CsrMatrix<W> {
    type EdgesMut<'a> = impl Iterator<Item = EdgeMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_adjacent_edges_mut(&mut self, node_id: NodeId<Self::Key>) -> Self::EdgesMut<'_> {
        self.row_mut(*node_id)
    }
}

impl<W: Debug + Clone> FixedEdgeMap<usize, W> for CsrMatrix<W> {}

impl<W: Debug + Clone> EdgeMap<usize, W> for CsrMatrix<W> {}

impl<W: Debug + Clone + Send + Sync> EdgeStorage<usize, W> for CsrMatrix<W> {
    fn new() -> Self {
        Self {
            edges: Vec::new(),
            row_offsets: vec![RowOffset { start: 0, end: 0 }],
        }
    }

    fn with_capacity(_: usize, edge_count: usize) -> Self {
        Self {
            edges: Vec::with_capacity(edge_count),
            row_offsets: vec![RowOffset { start: 0, end: 0 }],
        }
    }

    fn with_edges(
        node_count: usize,
        _: usize,
        edges: impl IntoIterator<Item = (NodeId<usize>, NodeId<usize>, W)>,
    ) -> Self {
        let mut edges = edges
            .into_iter()
            .map(|(from, to, weight)| Edge::new(EdgeId::new_unchecked(from, to), weight))
            .collect::<Vec<_>>();
        edges.par_sort_unstable_by(|a, b| a.from().cmp(&b.from()).then(a.to().cmp(&b.to())));

        let mut row_offsets = Vec::new();

        if let Some(last) = edges.last() {
            let row_count = *last.from() + 1;
            let mut start = 0;

            for row in 0..row_count {
                if let Some(pos) = edges[start..].iter().position(|edge| *edge.from() != row) {
                    row_offsets.push(RowOffset {
                        start,
                        end: start + pos,
                    });
                    start += pos;
                } else {
                    assert_eq!(row, row_count - 1);
                    row_offsets.push(RowOffset {
                        start,
                        end: edges.len(),
                    });
                    break;
                }
            }

            // allocate for nodes currently without edges
            for _ in row_count..node_count {
                row_offsets.push(RowOffset {
                    start: edges.len(),
                    end: edges.len(),
                })
            }
        } else {
            todo!()
        }

        Self { row_offsets, edges }
    }

    fn clear(&mut self) {
        self.edges.clear();
        self.row_offsets.fill(RowOffset { start: 0, end: 0 });
    }

    fn allocate(&mut self, additional: usize) {
        let offset = self.row_offsets.last().unwrap().clone();

        for _ in 0..additional {
            self.row_offsets.push(RowOffset {
                start: offset.end,
                end: offset.end,
            });
        }
    }

    fn remove_node(&mut self, node_id: NodeId<Self::Key>) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::edges::test::{
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
