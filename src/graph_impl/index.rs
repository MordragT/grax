use std::fmt::Debug;

use crate::prelude::{EdgeIdentifier, NodeIdentifier};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawNodeId(pub(crate) usize);

impl From<usize> for RawNodeId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Into<usize> for RawNodeId {
    fn into(self) -> usize {
        self.0
    }
}

impl Debug for RawNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawEdgeId {
    pub(crate) from: RawNodeId,
    pub(crate) to: RawNodeId,
}

impl Debug for RawEdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from.0, self.to.0)
    }
}

impl EdgeIdentifier for RawEdgeId {
    type NodeId = RawNodeId;

    fn between(from: Self::NodeId, to: Self::NodeId) -> Self {
        RawEdgeId { from, to }
    }

    fn contains(&self, index: RawNodeId) -> bool {
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
