use std::{fmt::Debug, hash::Hash};

pub trait Identifier: Hash + PartialEq + Eq + PartialOrd + Ord + Copy + Clone + Debug {}

impl<T: Hash + PartialEq + Eq + PartialOrd + Ord + Copy + Clone + Debug> Identifier for T {}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NodeId<Id: Identifier>(Id);

impl<Id: Identifier> NodeId<Id> {
    pub fn new_unchecked(id: Id) -> Self {
        Self(id)
    }

    pub fn raw(&self) -> Id {
        self.0
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct EdgeId<Id: Identifier> {
    from: NodeId<Id>,
    to: NodeId<Id>,
}

impl<Id: Identifier> EdgeId<Id> {
    pub fn new_unchecked(from: NodeId<Id>, to: NodeId<Id>) -> Self {
        Self { from, to }
    }

    pub fn contains(&self, node_id: NodeId<Id>) -> bool {
        self.from == node_id || self.to == node_id
    }

    pub fn from(&self) -> NodeId<Id> {
        self.from
    }

    pub fn to(&self) -> NodeId<Id> {
        self.to
    }

    pub fn raw(&self) -> (Id, Id) {
        (self.from.raw(), self.to.raw())
    }

    pub fn rev(&self) -> EdgeId<Id> {
        let Self { from, to } = self;
        Self {
            from: *to,
            to: *from,
        }
    }
}
