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
use serde::{Deserialize, Serialize};

use super::EdgeStorage;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AdjacencyList<W> {
    edges: Vec<Vec<Edge<usize, W>>>,
}

impl<W: Debug> AdjacencyList<W> {
    pub fn search(&self, from: usize, to: usize) -> Result<usize, usize> {
        self.edges[from].binary_search_by(|edge| edge.edge_id.to().cmp(&NodeId::new_unchecked(to)))
    }
}

impl<W: Debug> Keyed for AdjacencyList<W> {
    type Key = usize;
}

impl<W: Debug> EdgeCollection for AdjacencyList<W> {
    type EdgeWeight = W;

    fn edges_capacity(&self) -> usize {
        self.edges.capacity()
    }
}

impl<W: Debug> EdgeCount for AdjacencyList<W> {
    fn edge_count(&self) -> usize {
        let count = self.edges.iter().fold(0, |mut akku, edges| {
            akku += edges.len();
            akku
        });
        count
    }
}

impl<W: Debug> GetEdge for AdjacencyList<W> {
    fn edge(&self, edge_id: EdgeId<Self::Key>) -> Option<EdgeRef<Self::Key, Self::EdgeWeight>> {
        let (from, to) = edge_id.raw();
        if let Some(neigh) = self.edges.get(from) {
            if let Ok(pos) =
                neigh.binary_search_by(|edge| edge.edge_id.to().cmp(&NodeId::new_unchecked(to)))
            {
                Some((&neigh[pos]).into())
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

impl<W: Debug> GetEdgeMut for AdjacencyList<W> {
    fn edge_mut(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<EdgeMut<Self::Key, Self::EdgeWeight>> {
        let (from, to) = edge_id.raw();
        if let Some(neigh) = self.edges.get_mut(from) {
            if let Ok(pos) =
                neigh.binary_search_by(|edge| edge.edge_id.to().cmp(&NodeId::new_unchecked(to)))
            {
                Some((&mut neigh[pos]).into())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<W: Debug> EdgeIter for AdjacencyList<W> {
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<usize>> + 'a
    where
        Self: 'a;
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn edge_ids(&self) -> Self::EdgeIds<'_> {
        self.iter_edges().map(|edge| edge.edge_id)
    }

    fn iter_edges(&self) -> Self::Edges<'_> {
        self.edges.iter().flatten().map(Into::into)
    }
}

impl<W: Debug> EdgeIterMut for AdjacencyList<W> {
    type EdgesMut<'a> = impl Iterator<Item = EdgeMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_edges_mut(&mut self) -> Self::EdgesMut<'_> {
        self.edges.iter_mut().flatten().map(Into::into)
    }
}

impl<W: Debug> EdgeIterAdjacent for AdjacencyList<W> {
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<usize>> + 'a
    where
        Self: 'a;
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn adjacent_edge_ids(&self, node_id: NodeId<Self::Key>) -> Self::EdgeIds<'_> {
        self.iter_adjacent_edges(node_id).map(|edge| edge.edge_id)
    }

    fn iter_adjacent_edges(&self, node_id: NodeId<Self::Key>) -> Self::Edges<'_> {
        self.edges
            .get(*node_id)
            .into_iter()
            .flat_map(|adj| adj.iter().map(Into::into))
    }
}

impl<W: Debug> EdgeIterAdjacentMut for AdjacencyList<W> {
    type EdgesMut<'a> = impl Iterator<Item = EdgeMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_adjacent_edges_mut(&mut self, node_id: NodeId<Self::Key>) -> Self::EdgesMut<'_> {
        self.edges
            .get_mut(*node_id)
            .into_iter()
            .flat_map(|adj| adj.iter_mut().map(Into::into))
    }
}

impl<W: Debug> InsertEdge for AdjacencyList<W> {
    fn insert_edge(
        &mut self,
        from: NodeId<Self::Key>,
        to: NodeId<Self::Key>,
        weight: Self::EdgeWeight,
    ) -> EdgeId<Self::Key> {
        let edge_id = EdgeId::new_unchecked(from, to);
        let edge = Edge::new(edge_id, weight);

        let (from, to) = edge_id.raw();

        match self.search(from, to) {
            Ok(idx) | Err(idx) => self.edges[from].insert(idx, edge),
        }

        edge_id
    }

    fn reserve_edges(&mut self, additional: usize) {
        self.edges.reserve(additional)
    }
}

impl<W: Debug> RemoveEdge for AdjacencyList<W> {
    fn remove_edge(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<Edge<Self::Key, Self::EdgeWeight>> {
        let (from, to) = edge_id.raw();
        if let Ok(pos) = self.search(from, to) {
            Some(self.edges[from].remove(pos))
        } else {
            None
        }
    }
}

impl<W: Debug> IntoIterator for AdjacencyList<W> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Edge<usize, W>;

    fn into_iter(self) -> Self::IntoIter {
        self.edges.into_iter().flatten()
    }
}

impl<W: Debug + Clone> FixedEdgeMap<usize, W> for AdjacencyList<W> {}

impl<W: Debug + Clone> EdgeMap<usize, W> for AdjacencyList<W> {}

impl<W: Debug + Clone> EdgeStorage<usize, W> for AdjacencyList<W> {
    fn new() -> Self {
        Self { edges: Vec::new() }
    }

    fn with_capacity(node_count: usize, _: usize) -> Self {
        Self {
            edges: Vec::with_capacity(node_count),
        }
    }

    fn with_edges(
        node_count: usize,
        edge_count: usize,
        edges: impl IntoIterator<Item = (NodeId<usize>, NodeId<usize>, W)>,
    ) -> Self {
        let mut storage = Self::with_capacity(node_count, edge_count);
        storage.allocate(node_count);
        storage.extend_edges(edges);
        storage
    }

    fn clear(&mut self) {
        self.edges.clear();
    }

    fn allocate(&mut self, additional: usize) {
        let size = self.edges.len() + additional;
        self.edges.resize_with(size, || Vec::new());
    }

    fn remove_node(&mut self, node_id: NodeId<usize>) {
        self.edges[*node_id].clear();

        // for row in &mut self.edges {
        //     row.remove(node_id.raw());
        // }
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::edges::test::{
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
