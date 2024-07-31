use std::{
    fmt::{self, Debug},
    hash::Hash,
    ops::Deref,
};

use serde::{Deserialize, Serialize};

pub trait Identifier:
    Hash + PartialEq + Eq + PartialOrd + Ord + Copy + Clone + Debug + Send + Sync
{
}

impl<T: Hash + PartialEq + Eq + PartialOrd + Ord + Copy + Clone + Debug + Send + Sync> Identifier
    for T
{
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct NodeId<Id: Identifier>(Id);

impl<Id: Identifier> NodeId<Id> {
    pub fn new_unchecked(id: Id) -> Self {
        Self(id)
    }
}

impl<Id: Identifier> Deref for NodeId<Id> {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Id: Identifier> fmt::Display for NodeId<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Serialize, Deserialize)]
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
        (*self.from, *self.to)
    }

    pub fn reverse(&self) -> EdgeId<Id> {
        let Self { from, to } = self;
        Self {
            from: *to,
            to: *from,
        }
    }
}

impl<Id: Identifier> fmt::Display for EdgeId<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}
