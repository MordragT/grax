use bitvec::vec::BitVec;
use grax_core::{
    collections::{
        FixedNodeMap, GetNode, GetNodeMut, Keyed, NodeCollection, NodeCount, NodeIter, VisitNodeMap,
    },
    index::NodeId,
    node::{NodeMut, NodeRef},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeBoolVec(BitVec);

impl NodeBoolVec {
    pub fn new(vec: BitVec) -> Self {
        Self(vec)
    }
}

impl Keyed for NodeBoolVec {
    type Key = usize;
}

impl NodeCollection for NodeBoolVec {
    type NodeWeight = bool;

    fn nodes_capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl NodeCount for NodeBoolVec {
    fn node_count(&self) -> usize {
        self.0.len()
    }
}

// impl GetNode for NodeBoolVec {
//     fn node(&self, node_id: NodeId<Self::Key>) -> Option<NodeRef<Self::Key, Self::NodeWeight>> {
//         self.0
//             .get(*node_id)
//             .map(|weight| NodeRef::new(node_id, weight.as_ref()))
//     }
// }

// impl GetNodeMut for NodeBoolVec {
//     fn node_mut(
//         &mut self,
//         node_id: NodeId<Self::Key>,
//     ) -> Option<grax_core::prelude::NodeMut<Self::Key, Self::NodeWeight>> {
//         self.0
//             .get_mut(*node_id)
//             .map(|mut weight| NodeMut::new(node_id, weight.as_mut()))
//     }
// }

// impl NodeIter for NodeBoolVec {
//     type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Key>> + 'a where Self: 'a;
//     type Nodes<'a> = impl Iterator<Item = NodeRef<'a, Self::Key, Self::NodeWeight>> + 'a where Self: 'a;

//     fn iter_nodes(&self) -> Self::Nodes<'_> {
//         self.0
//             .iter()
//             .enumerate()
//             .map(|(key, weight)| NodeRef::new(NodeId::new_unchecked(key), weight.as_ref()))
//     }

//     fn node_ids(&self) -> Self::NodeIds<'_> {
//         (0..self.0.len()).map(NodeId::new_unchecked)
//     }
// }

impl VisitNodeMap<usize> for NodeBoolVec {
    fn is_visited(&self, node_id: NodeId<Self::Key>) -> bool {
        if let Some(weight) = self.0.get(*node_id) {
            *weight
        } else {
            false
        }
    }

    fn visit(&mut self, node_id: NodeId<Self::Key>) {
        self.0.set(*node_id, true)
    }

    fn unvisit(&mut self, node_id: NodeId<Self::Key>) {
        self.0.set(*node_id, false)
    }

    fn all_visited(&self) -> bool {
        self.0.all()
    }
}
