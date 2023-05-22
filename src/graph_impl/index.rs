use crate::prelude::{EdgeId, NodeId};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct NodeIndex(pub(crate) usize);

impl NodeId for NodeIndex {
    fn as_usize(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeIndex {
    pub(crate) from: NodeIndex,
    pub(crate) to: NodeIndex,
}

impl EdgeId for EdgeIndex {
    type NodeId = NodeIndex;

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
}

impl EdgeIndex {
    pub(crate) fn new(from: NodeIndex, to: NodeIndex) -> Self {
        Self { from, to }
    }
}
