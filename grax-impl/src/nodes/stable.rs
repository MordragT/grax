use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use grax_core::{
    collections::{
        FixedNodeMap, GetNode, GetNodeMut, InsertNode, Keyed, NodeCollection, NodeCount, NodeIter,
        NodeIterMut, NodeMap, RemoveNode,
    },
    index::NodeId,
    node::{Node, NodeMut, NodeRef},
};

// vec maybeuninit, bool vec mask
// also ordered
// so mix of ordered and option vec
// doesnt as option vec use option in favor of T

use stable_vec::StableVec;

use super::NodeStorage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableNodeVec<V>(StableVec<V>);

impl<V: Debug> Keyed for StableNodeVec<V> {
    type Key = usize;
}

impl<V: Debug> NodeCollection for StableNodeVec<V> {
    type NodeWeight = V;

    fn nodes_capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<V: Debug> NodeCount for StableNodeVec<V> {
    fn node_count(&self) -> usize {
        self.0.num_elements()
    }
}

impl<V: Debug> Index<NodeId<usize>> for StableNodeVec<V> {
    type Output = V;

    fn index(&self, index: NodeId<usize>) -> &Self::Output {
        &self.0[*index]
    }
}

impl<V: Debug> IndexMut<NodeId<usize>> for StableNodeVec<V> {
    fn index_mut(&mut self, index: NodeId<usize>) -> &mut Self::Output {
        &mut self.0[*index]
    }
}

impl<V: Debug> GetNode for StableNodeVec<V> {
    fn node(&self, node_id: NodeId<Self::Key>) -> Option<NodeRef<Self::Key, Self::NodeWeight>> {
        self.0
            .get(*node_id)
            .map(|weight| NodeRef::new(node_id, weight))
    }
}

impl<V: Debug> GetNodeMut for StableNodeVec<V> {
    fn node_mut(
        &mut self,
        node_id: NodeId<Self::Key>,
    ) -> Option<NodeMut<Self::Key, Self::NodeWeight>> {
        self.0
            .get_mut(*node_id)
            .map(|weight| NodeMut::new(node_id, weight))
    }
}

impl<V: Debug> NodeIter for StableNodeVec<V> {
    type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Key>> + 'a where Self: 'a;
    type Nodes<'a> = impl Iterator<Item = NodeRef<'a, Self::Key, Self::NodeWeight>> + 'a where V: 'a, Self: 'a;

    fn node_ids(&self) -> Self::NodeIds<'_> {
        (0..self.0.num_elements()).map(NodeId::new_unchecked)
    }

    fn iter_nodes(&self) -> Self::Nodes<'_> {
        self.0
            .iter()
            .enumerate()
            .map(|(key, (_, weight))| NodeRef::new(NodeId::new_unchecked(key), weight))
    }
}

impl<V: Debug + Clone> FixedNodeMap<usize, V> for StableNodeVec<V> {}

// NodeIterMut + GetNodeMut + InsertNode + RemoveNode

impl<V: Debug> NodeIterMut for StableNodeVec<V> {
    type NodesMut<'a> = impl Iterator<Item = NodeMut<'a, Self::Key, Self::NodeWeight>> + 'a where V: 'a, Self: 'a;

    fn iter_nodes_mut(&mut self) -> Self::NodesMut<'_> {
        self.0
            .iter_mut()
            .enumerate()
            .map(|(key, (_, weight))| NodeMut::new(NodeId::new_unchecked(key), weight))
    }
}

impl<V: Debug> InsertNode for StableNodeVec<V> {
    fn insert_node(&mut self, weight: Self::NodeWeight) -> NodeId<Self::Key> {
        let node_id = NodeId::new_unchecked(self.0.next_push_index());
        self.0.push(weight);
        node_id
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.0.reserve(additional)
    }
}

impl<V: Debug> RemoveNode for StableNodeVec<V> {
    fn remove_node(
        &mut self,
        node_id: NodeId<Self::Key>,
    ) -> Option<Node<Self::Key, Self::NodeWeight>> {
        self.0
            .remove(*node_id)
            .map(|weight| Node::new(node_id, weight))
    }
}

impl<V: Debug + Clone> NodeMap<usize, V> for StableNodeVec<V> {}

impl<V: Debug> IntoIterator for StableNodeVec<V> {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = Node<usize, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|(key, weight)| Node::new(NodeId::new_unchecked(key), weight))
    }
}

impl<V: Debug + Clone> NodeStorage<usize, V> for StableNodeVec<V> {
    type IndexedNodesMut<'a> = impl Iterator<Item = NodeMut<'a, usize, V>> where Self: 'a, V: 'a;

    fn new() -> Self {
        Self(StableVec::new())
    }

    fn with_capacity(node_count: usize) -> Self {
        Self(StableVec::with_capacity(node_count))
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn iter_indexed_nodes_mut(
        &mut self,
        node_ids: Vec<NodeId<Self::Key>>,
    ) -> Self::IndexedNodesMut<'_> {
        // TODO very inefficient

        self.0
            .iter_mut()
            .enumerate()
            .filter_map(move |(key, (_, weight))| {
                let node_id = NodeId::new_unchecked(key);
                if node_ids.contains(&node_id) {
                    Some(NodeMut::new(node_id, weight))
                } else {
                    None
                }
            })
    }
}
