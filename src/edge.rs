use crate::{indices::NodeIndex, prelude::EdgeIndex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge<W> {
    pub from: NodeIndex,
    pub to: NodeIndex,
    pub weight: W,
}

impl<W> Edge<W> {
    pub fn new(from: NodeIndex, to: NodeIndex, weight: W) -> Self {
        Self { from, to, weight }
    }
}

impl<W> Edge<W> {
    pub fn rev(self) -> Self {
        let Self { from, to, weight } = self;

        Self {
            from: to,
            to: from,
            weight,
        }
    }

    pub fn index(&self) -> EdgeIndex {
        EdgeIndex::new(self.from, self.to)
    }
}

impl<W: Ord> Ord for Edge<W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<W: PartialOrd> PartialOrd for Edge<W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<'a, W: Copy> From<EdgeRef<'a, W>> for Edge<W> {
    fn from(edge_ref: EdgeRef<'a, W>) -> Self {
        Self {
            from: edge_ref.from,
            to: edge_ref.to,
            weight: *edge_ref.weight,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EdgeRef<'a, W> {
    pub from: NodeIndex,
    pub to: NodeIndex,
    pub weight: &'a W,
}

impl<'a, W> EdgeRef<'a, W> {
    pub fn rev(self) -> Self {
        let Self { from, to, weight } = self;

        Self {
            from: to,
            to: from,
            weight,
        }
    }
}

impl<'a, W> EdgeRef<'a, W> {
    pub fn new(from: NodeIndex, to: NodeIndex, weight: &'a W) -> Self {
        Self { from, to, weight }
    }
}

impl<'a, W: Ord> Ord for EdgeRef<'a, W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(other.weight)
    }
}

impl<'a, W: PartialOrd> PartialOrd for EdgeRef<'a, W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(other.weight)
    }
}
