use std::{fmt::Debug, ops::Rem};

use grax_core::{
    collections::{
        EdgeCollection, EdgeCount, EdgeIter, EdgeIterMut, FixedEdgeMap, GetEdge, GetEdgeMut, Keyed,
    },
    edge::{EdgeMut, EdgeRef},
    index::{EdgeId, NodeId},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FixedEdgeVec<V> {
    vec: Vec<V>,
    node_count: usize,
}

impl<V: Debug> FixedEdgeVec<V> {
    pub fn new(vec: Vec<V>, node_count: usize) -> Self {
        Self { vec, node_count }
    }
}

impl<V: Debug> Keyed for FixedEdgeVec<V> {
    type Key = usize;
}

impl<V: Debug> EdgeCollection for FixedEdgeVec<V> {
    type EdgeWeight = V;

    fn edges_capacity(&self) -> usize {
        self.vec.capacity()
    }
}

impl<V: Debug> EdgeCount for FixedEdgeVec<V> {
    fn edge_count(&self) -> usize {
        self.vec.len()
    }
}

impl<V: Debug> GetEdge for FixedEdgeVec<V> {
    fn edge(&self, edge_id: EdgeId<Self::Key>) -> Option<EdgeRef<Self::Key, Self::EdgeWeight>> {
        let (from, to) = edge_id.raw();
        let key = from * self.node_count + to;
        self.vec
            .get(key)
            .map(|weight| EdgeRef::new(edge_id, weight))
    }
}

impl<V: Debug> GetEdgeMut for FixedEdgeVec<V> {
    fn edge_mut(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<grax_core::prelude::EdgeMut<Self::Key, Self::EdgeWeight>> {
        let (from, to) = edge_id.raw();
        let key = from * self.node_count + to;
        self.vec
            .get_mut(key)
            .map(|weight| EdgeMut::new(edge_id, weight))
    }
}

impl<V: Debug> EdgeIter for FixedEdgeVec<V> {
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Key>> + 'a where Self: 'a;
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, Self::Key, Self::EdgeWeight>> + 'a where Self: 'a;

    fn iter_edges(&self) -> Self::Edges<'_> {
        self.vec.iter().enumerate().map(|(key, weight)| {
            let from = NodeId::new_unchecked(key.div_floor(self.node_count));
            let to = NodeId::new_unchecked(key.rem(self.node_count));
            let edge_id = EdgeId::new_unchecked(from, to);
            EdgeRef::new(edge_id, weight)
        })
    }

    fn edge_ids(&self) -> Self::EdgeIds<'_> {
        (0..self.vec.len()).map(|key| {
            let from = NodeId::new_unchecked(key.div_floor(self.node_count));
            let to = NodeId::new_unchecked(key.rem(self.node_count));
            EdgeId::new_unchecked(from, to)
        })
    }
}

impl<V: Debug> EdgeIterMut for FixedEdgeVec<V> {
    type EdgesMut<'a> = impl Iterator<Item = EdgeMut<'a, Self::Key, Self::EdgeWeight>> + 'a where Self: 'a;

    fn iter_edges_mut(&mut self) -> Self::EdgesMut<'_> {
        self.vec.iter_mut().enumerate().map(|(key, weight)| {
            let from = NodeId::new_unchecked(key.div_floor(self.node_count));
            let to = NodeId::new_unchecked(key.rem(self.node_count));
            let edge_id = EdgeId::new_unchecked(from, to);
            EdgeMut::new(edge_id, weight)
        })
    }
}

impl<V: Debug + Clone> FixedEdgeMap<usize, V> for FixedEdgeVec<V> {}
