use super::EdgeIdentifier;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge<EdgeId, Weight> {
    pub edge_id: EdgeId,
    pub weight: Weight,
}

impl<EdgeId: EdgeIdentifier, Weight> Edge<EdgeId, Weight> {
    pub fn new(edge_id: EdgeId, weight: Weight) -> Self {
        Self { edge_id, weight }
    }

    pub fn from(&self) -> EdgeId::NodeId {
        self.edge_id.from()
    }

    pub fn to(&self) -> EdgeId::NodeId {
        self.edge_id.to()
    }
}

impl<EdgeId: Eq, Weight: Ord> Ord for Edge<EdgeId, Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<EdgeId: Eq, Weight: PartialOrd> PartialOrd for Edge<EdgeId, Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EdgeRef<'a, Id, Weight> {
    pub edge_id: Id,
    pub weight: &'a Weight,
}

impl<'a, EdgeId: EdgeIdentifier, Weight> EdgeRef<'a, EdgeId, Weight> {
    pub fn new(edge_id: EdgeId, weight: &'a Weight) -> Self {
        Self { edge_id, weight }
    }

    pub fn from(&self) -> EdgeId::NodeId {
        self.edge_id.from()
    }

    pub fn to(&self) -> EdgeId::NodeId {
        self.edge_id.to()
    }
}

impl<'a, EdgeId: EdgeIdentifier + Clone, Weight: Clone> EdgeRef<'a, EdgeId, Weight> {
    pub fn to_owned(&self) -> Edge<EdgeId, Weight> {
        Edge::new(self.edge_id.clone(), self.weight.clone())
    }
}

impl<'a, EdgeId: Eq, Weight: Ord> Ord for EdgeRef<'a, EdgeId, Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(other.weight)
    }
}

impl<'a, EdgeId: Eq, Weight: PartialOrd> PartialOrd for EdgeRef<'a, EdgeId, Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(other.weight)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EdgeRefMut<'a, Id, Weight> {
    pub edge_id: Id,
    pub weight: &'a mut Weight,
}

impl<'a, EdgeId: EdgeIdentifier, Weight> EdgeRefMut<'a, EdgeId, Weight> {
    pub fn new(edge_id: EdgeId, weight: &'a mut Weight) -> Self {
        Self { edge_id, weight }
    }
    pub fn from(&self) -> EdgeId::NodeId {
        self.edge_id.from()
    }

    pub fn to(&self) -> EdgeId::NodeId {
        self.edge_id.to()
    }
}

impl<'a, EdgeId: EdgeIdentifier + Clone, Weight: Clone> EdgeRefMut<'a, EdgeId, Weight> {
    pub fn to_owned(&self) -> Edge<EdgeId, Weight> {
        Edge::new(self.edge_id.clone(), self.weight.clone())
    }
}

impl<'a, EdgeId: Eq, Weight: Ord> Ord for EdgeRefMut<'a, EdgeId, Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<'a, EdgeId: Eq, Weight: PartialOrd> PartialOrd for EdgeRefMut<'a, EdgeId, Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}
