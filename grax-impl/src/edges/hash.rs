use std::{collections::HashMap, fmt::Debug};

use grax_core::{
    collections::{
        EdgeCollection, EdgeCount, EdgeIter, EdgeIterMut, EdgeMap, FixedEdgeMap, GetEdge,
        GetEdgeMut, InsertEdge, Keyed, RemoveEdge,
    },
    edge::{Edge, EdgeMut, EdgeRef},
    graph::{EdgeIterAdjacent, EdgeIterAdjacentMut},
    index::{EdgeId, NodeId},
};

use super::EdgeStorage;

#[derive(Debug, Clone)]
pub struct HashStorage<W>(HashMap<EdgeId<usize>, W>);

impl<W: Debug> Keyed for HashStorage<W> {
    type Key = usize;
}

impl<W: Debug> EdgeCollection for HashStorage<W> {
    type EdgeWeight = W;

    fn edges_capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<W: Debug> EdgeCount for HashStorage<W> {
    fn edge_count(&self) -> usize {
        self.0.len()
    }
}

impl<W: Debug> GetEdge for HashStorage<W> {
    fn edge(&self, edge_id: EdgeId<Self::Key>) -> Option<EdgeRef<Self::Key, Self::EdgeWeight>> {
        self.0
            .get(&edge_id)
            .map(|weight| EdgeRef::new(edge_id, weight))
    }

    // fn has_edge(
    //     &self,
    //     from: NodeId<Self::Key>,
    //     to: NodeId<Self::Key>,
    // ) -> Option<EdgeId<Self::Key>> {
    //     let edge_id = EdgeId::new_unchecked(from, to);
    //     if self.0.contains_key(&edge_id) {
    //         Some(edge_id)
    //     } else {
    //         None
    //     }
    // }
}

impl<W: Debug> GetEdgeMut for HashStorage<W> {
    fn edge_mut(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<EdgeMut<Self::Key, Self::EdgeWeight>> {
        self.0
            .get_mut(&edge_id)
            .map(|weight| EdgeMut::new(edge_id, weight))
    }
}

impl<W: Debug> InsertEdge for HashStorage<W> {
    fn insert_edge(
        &mut self,
        from: NodeId<Self::Key>,
        to: NodeId<Self::Key>,
        weight: Self::EdgeWeight,
    ) -> EdgeId<Self::Key> {
        let edge_id = EdgeId::new_unchecked(from, to);
        self.0.insert(edge_id, weight);
        edge_id
    }

    fn reserve_edges(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    fn extend_edges(
        &mut self,
        edges: impl IntoIterator<Item = (NodeId<Self::Key>, NodeId<Self::Key>, Self::EdgeWeight)>,
    ) {
        let edges = edges.into_iter().map(|(from, to, weight)| {
            let edge_id = EdgeId::new_unchecked(from, to);
            (edge_id, weight)
        });

        self.0.extend(edges);
    }
}

impl<W: Debug> RemoveEdge for HashStorage<W> {
    fn remove_edge(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<Edge<Self::Key, Self::EdgeWeight>> {
        self.0
            .remove(&edge_id)
            .map(|weight| Edge::new(edge_id, weight))
    }
}

impl<W: Debug> EdgeIter for HashStorage<W> {
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<usize>> + 'a
    where
        Self: 'a;
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
        where
            W: 'a,
            Self: 'a;

    fn edge_ids(&self) -> Self::EdgeIds<'_> {
        self.0.keys().cloned()
    }

    fn iter_edges(&self) -> Self::Edges<'_> {
        self.0
            .iter()
            .map(|(edge_id, weight)| EdgeRef::new(*edge_id, weight))
    }
}

impl<W: Debug> EdgeIterMut for HashStorage<W> {
    type EdgesMut<'a> = impl Iterator<Item = EdgeMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_edges_mut(&mut self) -> Self::EdgesMut<'_> {
        self.0
            .iter_mut()
            .map(|(edge_id, weight)| EdgeMut::new(*edge_id, weight))
    }
}

impl<W: Debug> EdgeIterAdjacent for HashStorage<W> {
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<usize>> + 'a
    where
        Self: 'a;
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
        where
            W: 'a,
            Self: 'a;

    fn adjacent_edge_ids(&self, node_id: NodeId<Self::Key>) -> Self::EdgeIds<'_> {
        self.0
            .keys()
            .filter(move |edge_id| edge_id.from() == node_id)
            .cloned()
    }

    fn iter_adjacent_edges(&self, node_id: NodeId<Self::Key>) -> Self::Edges<'_> {
        self.0.iter().filter_map(move |(edge_id, weight)| {
            if edge_id.from() == node_id {
                Some(EdgeRef::new(*edge_id, weight))
            } else {
                None
            }
        })
    }
}

impl<W: Debug> EdgeIterAdjacentMut for HashStorage<W> {
    type EdgesMut<'a> = impl Iterator<Item = EdgeMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_adjacent_edges_mut(&mut self, node_id: NodeId<Self::Key>) -> Self::EdgesMut<'_> {
        self.0.iter_mut().filter_map(move |(edge_id, weight)| {
            if edge_id.from() == node_id {
                Some(EdgeMut::new(*edge_id, weight))
            } else {
                None
            }
        })
    }
}

impl<W: Debug> IntoIterator for HashStorage<W> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Edge<usize, W>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|(edge_id, weight)| Edge::new(edge_id, weight))
    }
}

impl<W: Debug + Clone> FixedEdgeMap<usize, W> for HashStorage<W> {}

impl<W: Debug + Clone> EdgeMap<usize, W> for HashStorage<W> {}

impl<W: Debug + Clone> EdgeStorage<usize, W> for HashStorage<W> {
    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(_: usize, edge_count: usize) -> Self {
        Self(HashMap::with_capacity(edge_count))
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn allocate(&mut self, _: usize) {}

    fn remove_node(&mut self, node_id: NodeId<Self::Key>) {
        let to_remove = self
            .0
            .keys()
            .filter(|edge_id| edge_id.contains(node_id))
            .cloned()
            .collect::<Vec<_>>();

        for edge_id in to_remove {
            self.0.remove(&edge_id);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::edges::test::{
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
