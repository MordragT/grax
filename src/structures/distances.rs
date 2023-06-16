use crate::prelude::NodeIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Distances<NodeId: NodeIdentifier, Weight> {
    pub distances: Vec<Option<Weight>>,
    pub from: NodeId,
}

impl<NodeId: NodeIdentifier, Weight> Distances<NodeId, Weight> {
    pub fn new(from: NodeId, distances: Vec<Option<Weight>>) -> Self {
        Self { distances, from }
    }

    pub fn to(&self, to: NodeId) -> Option<&Weight> {
        self.distances[to.as_usize()].as_ref()
    }
}
