use crate::prelude::{Identifier, NodeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node<Id: Identifier, Weight> {
    pub node_id: NodeId<Id>,
    pub weight: Weight,
}

impl<Id: Identifier, Weight> Node<Id, Weight> {
    pub fn new(node_id: NodeId<Id>, weight: Weight) -> Self {
        Self { node_id, weight }
    }
}

impl<Id: Identifier, Weight: Ord> Ord for Node<Id, Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<Id: Identifier, Weight: PartialOrd> PartialOrd for Node<Id, Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct NodeRef<'a, Id: Identifier, Weight> {
    pub node_id: NodeId<Id>,
    pub weight: &'a Weight,
}

impl<'a, Id: Identifier, Weight> NodeRef<'a, Id, Weight> {
    pub fn new(node_id: NodeId<Id>, weight: &'a Weight) -> Self {
        Self { node_id, weight }
    }
}

impl<'a, Id: Identifier, Weight> From<&'a Node<Id, Weight>> for NodeRef<'a, Id, Weight> {
    fn from(node: &'a Node<Id, Weight>) -> Self {
        Self {
            node_id: node.node_id,
            weight: &node.weight,
        }
    }
}

impl<'a, Id: Identifier, Weight: Clone> NodeRef<'a, Id, Weight> {
    pub fn to_owned(&self) -> Node<Id, Weight> {
        Node::new(self.node_id.clone(), self.weight.clone())
    }
}

impl<'a, Id: Identifier, Weight: Ord> Ord for NodeRef<'a, Id, Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(other.weight)
    }
}

impl<'a, Id: Identifier, Weight: PartialOrd> PartialOrd for NodeRef<'a, Id, Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(other.weight)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct NodeMut<'a, Id: Identifier, Weight> {
    pub node_id: NodeId<Id>,
    pub weight: &'a mut Weight,
}

impl<'a, Id: Identifier, Weight> NodeMut<'a, Id, Weight> {
    pub fn new(node_id: NodeId<Id>, weight: &'a mut Weight) -> Self {
        Self { node_id, weight }
    }
}

impl<'a, Id: Identifier, Weight: Clone> NodeMut<'a, Id, Weight> {
    pub fn to_owned(&self) -> Node<Id, Weight> {
        Node::new(self.node_id.clone(), self.weight.clone())
    }
}

impl<'a, Id: Identifier, Weight: Ord> Ord for NodeMut<'a, Id, Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<'a, Id: Identifier, Weight: PartialOrd> PartialOrd for NodeMut<'a, Id, Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<'a, Id: Identifier, Weight> From<&'a mut Node<Id, Weight>> for NodeMut<'a, Id, Weight> {
    fn from(node: &'a mut Node<Id, Weight>) -> Self {
        Self {
            node_id: node.node_id,
            weight: &mut node.weight,
        }
    }
}

pub trait NodeBalance {
    type Balance;

    fn balance(&self) -> &Self::Balance;
    fn balance_mut(&mut self) -> &mut Self::Balance;
}

// pub trait Node: Default + PartialEq + Clone + Debug {}

// impl<T: Default + PartialEq + Clone + Debug> Node for T {}
