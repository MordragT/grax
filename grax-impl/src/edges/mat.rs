use std::fmt::Debug;

use grax_core::{
    collections::{
        EdgeCollection, EdgeCount, EdgeIter, EdgeIterMut, EdgeMap, FixedEdgeMap, GetEdge,
        GetEdgeMut, InsertEdge, Keyed, RemoveEdge,
    },
    edge::{Edge, EdgeMut, EdgeRef},
    graph::{EdgeIterAdjacent, EdgeIterAdjacentMut},
    index::{EdgeId, NodeId},
};
use stable_vec::StableVec;

use super::EdgeStorage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjacencyMatrix<W> {
    edges: Vec<StableVec<Edge<usize, W>>>,
}

impl<W: Debug> AdjacencyMatrix<W> {
    pub fn col(&self, col: usize) -> impl Iterator<Item = EdgeRef<'_, usize, W>> {
        self.edges.iter().flat_map(move |edges| {
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
        self.edges.iter().map(|row| row.num_elements()).sum()
    }
}

impl<W: Debug> IntoIterator for AdjacencyMatrix<W> {
    type Item = Edge<usize, W>;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.edges
            .into_iter()
            .flat_map(|neigh| neigh.into_iter().map(|(_, edge)| edge))
    }
}

impl<W: Debug> Keyed for AdjacencyMatrix<W> {
    type Key = usize;
}

impl<W: Debug> EdgeCollection for AdjacencyMatrix<W> {
    type EdgeWeight = W;

    fn edges_capacity(&self) -> usize {
        self.edges.capacity() ^ 2
    }
}

impl<W: Debug> EdgeCount for AdjacencyMatrix<W> {
    fn edge_count(&self) -> usize {
        self.nnz()
    }
}

impl<W: Debug> GetEdge for AdjacencyMatrix<W> {
    fn edge(&self, edge_id: EdgeId<Self::Key>) -> Option<EdgeRef<Self::Key, Self::EdgeWeight>> {
        let (from, to) = edge_id.raw();
        // TODO allow out of bounds
        self.edges[from].get(to).map(Into::into)
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

impl<W: Debug> GetEdgeMut for AdjacencyMatrix<W> {
    fn edge_mut(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<EdgeMut<Self::Key, Self::EdgeWeight>> {
        let (from, to) = edge_id.raw();

        // TODO allow out of bounds
        self.edges[from].get_mut(to).map(Into::into)
    }
}

impl<W: Debug> InsertEdge for AdjacencyMatrix<W> {
    fn insert_edge(
        &mut self,
        from: NodeId<Self::Key>,
        to: NodeId<Self::Key>,
        weight: Self::EdgeWeight,
    ) -> EdgeId<Self::Key> {
        assert!(*from.max(to) < self.edges.len());

        let edge_id = EdgeId::new_unchecked(from, to);
        let edge = Edge::new(edge_id, weight);
        self.edges[*from].insert(*to, edge);
        edge_id
    }

    fn reserve_edges(&mut self, additional: usize) {
        self.edges.reserve(additional)
    }
}

impl<W: Debug> RemoveEdge for AdjacencyMatrix<W> {
    fn remove_edge(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<Edge<Self::Key, Self::EdgeWeight>> {
        let (from, to) = edge_id.raw();
        if let Some(neigh) = self.edges.get_mut(from) {
            neigh.remove(to)
        } else {
            None
        }
    }
}

impl<W: Debug> EdgeIter for AdjacencyMatrix<W> {
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<usize>> + 'a
    where
        Self: 'a;
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
        where
            W: 'a,
            Self: 'a;

    fn edge_ids(&self) -> Self::EdgeIds<'_> {
        self.edges
            .iter()
            .flat_map(|edges| edges.iter().map(|(_, edge)| edge.edge_id))
    }

    fn iter_edges(&self) -> Self::Edges<'_> {
        self.edges
            .iter()
            .flat_map(|edges| edges.iter().map(|(_, edge)| edge.into()))
    }
}

impl<W: Debug> EdgeIterMut for AdjacencyMatrix<W> {
    type EdgesMut<'a> = impl Iterator<Item = EdgeMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_edges_mut(&mut self) -> Self::EdgesMut<'_> {
        self.edges
            .iter_mut()
            .flat_map(|edges| edges.iter_mut().map(|(_, edge)| edge.into()))
    }
}

impl<W: Debug> EdgeIterAdjacent for AdjacencyMatrix<W> {
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<usize>> + 'a
    where
        Self: 'a;
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
        where
            W: 'a,
            Self: 'a;

    fn adjacent_edge_ids(&self, node_id: NodeId<Self::Key>) -> Self::EdgeIds<'_> {
        self.edges[*node_id].iter().map(|(_, edge)| edge.edge_id)
    }
    fn iter_adjacent_edges(&self, node_id: NodeId<Self::Key>) -> Self::Edges<'_> {
        self.edges[*node_id].iter().map(|(_, edge)| edge.into())
    }
}

impl<W: Debug> EdgeIterAdjacentMut for AdjacencyMatrix<W> {
    type EdgesMut<'a> = impl Iterator<Item = EdgeMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_adjacent_edges_mut(&mut self, node_id: NodeId<Self::Key>) -> Self::EdgesMut<'_> {
        self.edges[*node_id].iter_mut().map(|(_, edge)| edge.into())
    }
}

impl<W: Debug + Clone> FixedEdgeMap<usize, W> for AdjacencyMatrix<W> {}

impl<W: Debug + Clone> EdgeMap<usize, W> for AdjacencyMatrix<W> {}

impl<W: Debug + Clone> EdgeStorage<usize, W> for AdjacencyMatrix<W> {
    fn new() -> Self {
        Self { edges: Vec::new() }
    }

    fn with_capacity(node_count: usize, _: usize) -> Self {
        let edges = Vec::with_capacity(node_count);

        Self { edges }
    }

    fn clear(&mut self) {
        for row in &mut self.edges {
            row.clear()
        }
    }

    fn allocate(&mut self, additional: usize) {
        let size = self.edges.len() + additional;

        for row in &mut self.edges {
            row.reserve_for(size);
        }

        self.edges.resize(size, StableVec::with_capacity(size));
    }

    fn remove_node(&mut self, node_id: NodeId<Self::Key>) {
        self.edges[*node_id].clear();

        for row in &mut self.edges {
            row.remove(*node_id);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::edges::test::{
        edge_storage_adjacent, edge_storage_capacity, edge_storage_clear, edge_storage_count,
        edge_storage_get, edge_storage_remove,
    };

    use super::AdjacencyMatrix;
    #[test]
    fn dense_edge_storage_capacity() {
        edge_storage_capacity::<AdjacencyMatrix<f32>>()
    }
    #[test]
    fn dense_edge_storage_count() {
        edge_storage_count::<AdjacencyMatrix<f32>>()
    }
    #[test]
    fn dense_edge_storage_clear() {
        edge_storage_clear::<AdjacencyMatrix<f32>>()
    }
    #[test]
    fn dense_edge_storage_remove() {
        edge_storage_remove::<AdjacencyMatrix<f32>>()
    }
    #[test]
    fn dense_edge_storage_get() {
        edge_storage_get::<AdjacencyMatrix<f32>>()
    }
    #[test]
    fn dense_edge_storage_adjacent() {
        edge_storage_adjacent::<AdjacencyMatrix<f32>>()
    }
}
