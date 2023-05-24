use std::fmt::Debug;

use crate::prelude::{EdgeIdentifier, NodeIdentifier};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeIndex(pub(crate) usize);

impl NodeIdentifier for NodeIndex {
    fn as_usize(&self) -> usize {
        self.0
    }
}

impl Debug for NodeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeIndex {
    pub(crate) from: NodeIndex,
    pub(crate) to: NodeIndex,
}

impl Debug for EdgeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from.0, self.to.0)
    }
}

impl EdgeIdentifier for EdgeIndex {
    type NodeId = NodeIndex;

    fn between(from: Self::NodeId, to: Self::NodeId) -> Self {
        EdgeIndex { from, to }
    }

    fn contains(&self, index: NodeIndex) -> bool {
        self.from == index || self.to == index
    }

    fn rev(&self) -> Self {
        let Self { from, to } = self;

        Self {
            from: *to,
            to: *from,
        }
    }

    fn to(&self) -> Self::NodeId {
        self.to
    }

    fn from(&self) -> Self::NodeId {
        self.from
    }

    fn as_usize(&self) -> (usize, usize) {
        (self.from.as_usize(), self.to.as_usize())
    }
}
