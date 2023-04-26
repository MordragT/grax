use crate::NodeIndex;

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

impl<'a, W: ToOwned<Owned = W>> From<EdgeRef<'a, W>> for Edge<W> {
    fn from(edge_ref: EdgeRef<'a, W>) -> Self {
        Self {
            from: edge_ref.from,
            to: edge_ref.to,
            weight: edge_ref.weight.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EdgeRef<'a, W> {
    pub from: NodeIndex,
    pub to: NodeIndex,
    pub weight: &'a W,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinOrderEdge<'a, W> {
    to: NodeIndex,
    weight: &'a W,
}

impl<'a, W> MinOrderEdge<'a, W> {
    pub fn new(to: NodeIndex, weight: &'a W) -> Self {
        Self { to, weight }
    }
}

impl<'a, W: Ord> Ord for MinOrderEdge<'a, W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse order
        other.weight.cmp(&self.weight)
    }
}

impl<'a, W: PartialOrd> PartialOrd for MinOrderEdge<'a, W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // reverse order
        other.weight.partial_cmp(&self.weight)
    }
}

impl<'a, W> From<&'a Edge<W>> for MinOrderEdge<'a, W> {
    fn from(edge: &'a Edge<W>) -> Self {
        Self {
            to: edge.to,
            weight: &edge.weight,
        }
    }
}

impl<'a, W> From<EdgeRef<'a, W>> for MinOrderEdge<'a, W> {
    fn from(edge: EdgeRef<'a, W>) -> Self {
        Self {
            to: edge.to,
            weight: edge.weight,
        }
    }
}
