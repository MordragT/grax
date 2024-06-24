use serde::{Deserialize, Serialize};

use super::index::{EdgeId, Identifier, NodeId};
use std::fmt::Debug;

pub mod weight;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edge<Id: Identifier, Weight> {
    pub edge_id: EdgeId<Id>,
    pub weight: Weight,
}

impl<Id: Identifier, Weight> Edge<Id, Weight> {
    pub fn new(edge_id: EdgeId<Id>, weight: Weight) -> Self {
        Self { edge_id, weight }
    }

    pub fn from(&self) -> NodeId<Id> {
        self.edge_id.from()
    }

    pub fn to(&self) -> NodeId<Id> {
        self.edge_id.to()
    }
}

impl<Id: Identifier, Weight: Ord> Ord for Edge<Id, Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<Id: Identifier, Weight: PartialOrd> PartialOrd for Edge<Id, Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EdgeRef<'a, Id: Identifier, Weight> {
    pub edge_id: EdgeId<Id>,
    pub weight: &'a Weight,
}

impl<'a, Id: Identifier, Weight> EdgeRef<'a, Id, Weight> {
    pub fn new(edge_id: EdgeId<Id>, weight: &'a Weight) -> Self {
        Self { edge_id, weight }
    }

    pub fn from(&self) -> NodeId<Id> {
        self.edge_id.from()
    }

    pub fn to(&self) -> NodeId<Id> {
        self.edge_id.to()
    }
}

impl<'a, Id: Identifier, Weight> From<&'a Edge<Id, Weight>> for EdgeRef<'a, Id, Weight> {
    fn from(edge: &'a Edge<Id, Weight>) -> Self {
        Self {
            edge_id: edge.edge_id,
            weight: &edge.weight,
        }
    }
}

impl<'a, Id: Identifier, Weight: Clone> EdgeRef<'a, Id, Weight> {
    pub fn to_owned(&self) -> Edge<Id, Weight> {
        Edge::new(self.edge_id.clone(), self.weight.clone())
    }
}

impl<'a, Id: Identifier, Weight: Ord> Ord for EdgeRef<'a, Id, Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(other.weight)
    }
}

impl<'a, Id: Identifier, Weight: PartialOrd> PartialOrd for EdgeRef<'a, Id, Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(other.weight)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EdgeMut<'a, Id: Identifier, Weight> {
    pub edge_id: EdgeId<Id>,
    pub weight: &'a mut Weight,
}

impl<'a, Id: Identifier, Weight> EdgeMut<'a, Id, Weight> {
    pub fn new(edge_id: EdgeId<Id>, weight: &'a mut Weight) -> Self {
        Self { edge_id, weight }
    }
    pub fn from(&self) -> NodeId<Id> {
        self.edge_id.from()
    }

    pub fn to(&self) -> NodeId<Id> {
        self.edge_id.to()
    }
}

impl<'a, Id: Identifier, Weight: Clone> EdgeMut<'a, Id, Weight> {
    pub fn to_owned(&self) -> Edge<Id, Weight> {
        Edge::new(self.edge_id.clone(), self.weight.clone())
    }
}

impl<'a, Id: Identifier, Weight: Ord> Ord for EdgeMut<'a, Id, Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<'a, Id: Identifier, Weight: PartialOrd> PartialOrd for EdgeMut<'a, Id, Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<'a, Id: Identifier, Weight> From<&'a mut Edge<Id, Weight>> for EdgeMut<'a, Id, Weight> {
    fn from(edge: &'a mut Edge<Id, Weight>) -> Self {
        Self {
            edge_id: edge.edge_id,
            weight: &mut edge.weight,
        }
    }
}
