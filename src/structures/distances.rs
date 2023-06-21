use crate::prelude::{Identifier, NodeId};

use super::Parents;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Distances<Id: Identifier, Weight> {
    pub distances: Vec<Option<Weight>>,
    // pub parents: Parents<G>,
    pub from: NodeId<Id>,
}

impl<Id: Identifier, Weight> Distances<Id, Weight> {
    pub fn new(from: NodeId<Id>, distances: Vec<Option<Weight>>) -> Self {
        Self { distances, from }
    }

    pub fn to(&self, to: NodeId<Id>) -> Option<&Weight> {
        self.distances[to.as_usize()].as_ref()
    }
}
