pub use constraints::*;

use super::index::{EdgeId, Identifier, NodeId};
use std::fmt::Debug;

mod constraints;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub struct EdgeRefMut<'a, Id: Identifier, Weight> {
    pub edge_id: EdgeId<Id>,
    pub weight: &'a mut Weight,
}

impl<'a, Id: Identifier, Weight> EdgeRefMut<'a, Id, Weight> {
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

impl<'a, Id: Identifier, Weight: Clone> EdgeRefMut<'a, Id, Weight> {
    pub fn to_owned(&self) -> Edge<Id, Weight> {
        Edge::new(self.edge_id.clone(), self.weight.clone())
    }
}

impl<'a, Id: Identifier, Weight: Ord> Ord for EdgeRefMut<'a, Id, Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<'a, Id: Identifier, Weight: PartialOrd> PartialOrd for EdgeRefMut<'a, Id, Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<'a, Id: Identifier, Weight> From<&'a mut Edge<Id, Weight>> for EdgeRefMut<'a, Id, Weight> {
    fn from(edge: &'a mut Edge<Id, Weight>) -> Self {
        Self {
            edge_id: edge.edge_id,
            weight: &mut edge.weight,
        }
    }
}
pub trait EdgeCost: Clone + Debug {
    type Cost;

    fn cost(&self) -> &Self::Cost;
    fn cost_mut(&mut self) -> &mut Self::Cost;
}

pub trait EdgeFlow: Clone + Debug {
    type Flow;

    fn capacity(&self) -> &Self::Flow;
    fn capacity_mut(&mut self) -> &mut Self::Flow;
    fn flow(&self) -> &Self::Flow;
    fn flow_mut(&mut self) -> &mut Self::Flow;
    fn is_reverse(&self) -> bool;
    fn reverse(&mut self);
}

impl EdgeCost for f32 {
    type Cost = f32;

    fn cost(&self) -> &Self::Cost {
        &self
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        self
    }
}

impl EdgeCost for f64 {
    type Cost = f64;

    fn cost(&self) -> &Self::Cost {
        &self
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        self
    }
}

// impl EdgeFlow for f32 {
//     type Flow = f32;

//     fn flow(&self) -> &Self::Flow {
//         &0.0
//     }

//     fn flow_mut(&mut self) -> &mut Self::Flow {
//         panic!("Cannot mutate flow of cost only weight")
//     }
// }

// impl EdgeFlow for f64 {
//     type Flow = f64;

//     fn flow(&self) -> &Self::Flow {
//         &0.0
//     }

//     fn flow_mut(&mut self) -> &mut Self::Flow {
//         panic!("Cannot mutate flow of cost only weight")
//     }
// }
