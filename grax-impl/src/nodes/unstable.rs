use std::fmt::Debug;

use grax_core::{
    collections::{
        FixedNodeMap, GetNode, GetNodeMut, InsertNode, Keyed, NodeCollection, NodeCount, NodeIter,
        NodeIterMut, NodeMap, RemoveNode,
    },
    index::NodeId,
    node::{Node, NodeMut, NodeRef},
};
use rayon::slice::ParallelSliceMut;
use serde::{Deserialize, Serialize};

use super::NodeStorage;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnstableNodeVec<V>(Vec<V>);

impl<V: Debug> UnstableNodeVec<V> {
    pub fn new(vec: Vec<V>) -> Self {
        Self(vec)
    }
}

impl<V: Debug> Keyed for UnstableNodeVec<V> {
    type Key = usize;
}

impl<V: Debug> NodeCollection for UnstableNodeVec<V> {
    type NodeWeight = V;

    fn nodes_capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<V: Debug> NodeCount for UnstableNodeVec<V> {
    fn node_count(&self) -> usize {
        self.0.len()
    }
}

impl<V: Debug> GetNode for UnstableNodeVec<V> {
    fn node(&self, node_id: NodeId<Self::Key>) -> Option<NodeRef<Self::Key, Self::NodeWeight>> {
        self.0
            .get(*node_id)
            .map(|weight| NodeRef::new(node_id, weight))
    }
}

impl<V: Debug> GetNodeMut for UnstableNodeVec<V> {
    fn node_mut(
        &mut self,
        node_id: NodeId<Self::Key>,
    ) -> Option<NodeMut<Self::Key, Self::NodeWeight>> {
        self.0
            .get_mut(*node_id)
            .map(|weight| NodeMut::new(node_id, weight))
    }
}

impl<V: Debug> NodeIter for UnstableNodeVec<V> {
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

impl<V: Debug + Clone> FixedNodeMap<usize, V> for UnstableNodeVec<V> {}

// NodeIterMut + GetNodeMut + InsertNode + RemoveNode

impl<V: Debug> NodeIterMut for UnstableNodeVec<V> {
    type NodesMut<'a> = impl Iterator<Item = NodeMut<'a, Self::Key, Self::NodeWeight>> + 'a where V: 'a, Self: 'a;

    fn iter_nodes_mut(&mut self) -> Self::NodesMut<'_> {
        self.0
            .iter_mut()
            .enumerate()
            .map(|(key, weight)| NodeMut::new(NodeId::new_unchecked(key), weight))
    }
}

impl<V: Debug> InsertNode for UnstableNodeVec<V> {
    fn insert_node(&mut self, weight: Self::NodeWeight) -> NodeId<Self::Key> {
        let node_id = NodeId::new_unchecked(self.0.len());
        self.0.push(weight);
        node_id
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.0.reserve(additional)
    }
}

impl<V: Debug> RemoveNode for UnstableNodeVec<V> {
    fn remove_node(
        &mut self,
        node_id: NodeId<Self::Key>,
    ) -> Option<Node<Self::Key, Self::NodeWeight>> {
        if self.contains_node_id(node_id) {
            let weight = self.0.remove(*node_id);
            Some(Node::new(node_id, weight))
        } else {
            None
        }
    }
}

impl<V: Debug + Clone> NodeMap<usize, V> for UnstableNodeVec<V> {}

impl<V: Debug> IntoIterator for UnstableNodeVec<V> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Node<usize, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .enumerate()
            .map(|(key, weight)| Node::new(NodeId::new_unchecked(key), weight))
    }
}

pub struct IndexedNodeIterMut<'a, N> {
    remaining: &'a mut [N],
    offset: usize,
    indices: Vec<NodeId<usize>>,
}

impl<'a, N> IndexedNodeIterMut<'a, N> {
    pub fn new(nodes: &'a mut [N], mut indices: Vec<NodeId<usize>>) -> Self {
        // TODO measure if faster than sequentiell sorting
        indices.par_sort_unstable_by(|a, b| b.cmp(a));

        Self {
            remaining: nodes,
            offset: 0,
            indices,
        }
    }
}

impl<'a, N> Iterator for IndexedNodeIterMut<'a, N> {
    type Item = NodeMut<'a, usize, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node_id) = self.indices.pop() {
            let nodes = std::mem::take(&mut self.remaining);
            let (left, right) = nodes.split_at_mut(*node_id + 1 - self.offset);
            self.remaining = right;
            let weight = &mut left[*node_id - self.offset];
            self.offset += *node_id + 1;
            Some(NodeMut::new(node_id, weight))
        } else {
            None
        }
    }
}

impl<V: Debug + Clone> NodeStorage<usize, V> for UnstableNodeVec<V> {
    type IndexedNodesMut<'a> = impl Iterator<Item = NodeMut<'a, usize, V>> where Self: 'a, V: 'a;

    fn new() -> Self {
        Self(Vec::new())
    }

    fn with_capacity(node_count: usize) -> Self {
        Self(Vec::with_capacity(node_count))
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn iter_indexed_nodes_mut(
        &mut self,
        node_ids: Vec<NodeId<Self::Key>>,
    ) -> Self::IndexedNodesMut<'_> {
        IndexedNodeIterMut::new(&mut self.0, node_ids)
    }
}
