use std::fmt::Debug;

use grax_core::{
    collections::{
        FixedNodeMap, GetNode, GetNodeMut, Keyed, NodeCollection, NodeCount, NodeIter, NodeIterMut,
    },
    index::NodeId,
    node::{NodeMut, NodeRef},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedNodeVec<V>(Vec<V>);

impl<V: Debug> FixedNodeVec<V> {
    pub fn new(vec: Vec<V>) -> Self {
        Self(vec)
    }
}

impl<V: Debug> Keyed for FixedNodeVec<V> {
    type Key = usize;
}

impl<V: Debug> NodeCollection for FixedNodeVec<V> {
    type NodeWeight = V;

    fn nodes_capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<V: Debug> NodeCount for FixedNodeVec<V> {
    fn node_count(&self) -> usize {
        self.0.len()
    }
}

impl<V: Debug> GetNode for FixedNodeVec<V> {
    fn node(&self, node_id: NodeId<Self::Key>) -> Option<NodeRef<Self::Key, Self::NodeWeight>> {
        self.0
            .get(*node_id)
            .map(|weight| NodeRef::new(node_id, weight))
    }
}

impl<V: Debug> GetNodeMut for FixedNodeVec<V> {
    fn node_mut(
        &mut self,
        node_id: NodeId<Self::Key>,
    ) -> Option<grax_core::prelude::NodeMut<Self::Key, Self::NodeWeight>> {
        self.0
            .get_mut(*node_id)
            .map(|weight| NodeMut::new(node_id, weight))
    }
}

impl<V: Debug> NodeIter for FixedNodeVec<V> {
    type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Key>> + 'a where Self: 'a;
    type Nodes<'a> = impl Iterator<Item = NodeRef<'a, Self::Key, Self::NodeWeight>> + 'a where V: 'a, Self: 'a;

    fn node_ids(&self) -> Self::NodeIds<'_> {
        (0..self.0.len()).map(NodeId::new_unchecked)
    }

    fn iter_nodes(&self) -> Self::Nodes<'_> {
        self.0
            .iter()
            .enumerate()
            .map(|(key, weight)| NodeRef::new(NodeId::new_unchecked(key), weight))
    }
}

impl<V: Debug> NodeIterMut for FixedNodeVec<V> {
    type NodesMut<'a> = impl Iterator<Item = NodeMut<'a, Self::Key, Self::NodeWeight>> + 'a where V: 'a, Self: 'a;

    fn iter_nodes_mut(&mut self) -> Self::NodesMut<'_> {
        self.0
            .iter_mut()
            .enumerate()
            .map(|(key, weight)| NodeMut::new(NodeId::new_unchecked(key), weight))
    }
}

impl<V: Debug + Clone> FixedNodeMap<usize, V> for FixedNodeVec<V> {}
