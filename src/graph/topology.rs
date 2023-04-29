use crate::{edge::EdgeRef, indices::NodeIndex};

pub trait GraphTopology<N, W> {
    type Indices<'a>: Iterator<Item = NodeIndex> + 'a
    where
        Self: 'a;
    type Nodes<'a>: Iterator<Item = &'a N> + 'a
    where
        N: 'a,
        Self: 'a;
    type Edges<'a>: Iterator<Item = EdgeRef<'a, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn indices<'a>(&self) -> Self::Indices<'a>;
    fn nodes<'a>(&'a self) -> Self::Nodes<'a>;
    fn edges<'a>(&'a self) -> Self::Edges<'a>;

    fn node_count(&self) -> usize;
    fn edge_count(&self) -> usize;
}

pub trait GraphAdjacentTopology<N, W> {
    type AdjacentIndices<'a>: Iterator<Item = NodeIndex> + 'a
    where
        Self: 'a;
    type AdjacentNodes<'a>: Iterator<Item = &'a N> + 'a
    where
        N: 'a,
        Self: 'a;
    type AdjacentEdges<'a>: Iterator<Item = EdgeRef<'a, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn adjacent_indices<'a>(&'a self, index: NodeIndex) -> Self::AdjacentIndices<'a>;
    fn adjacent_nodes<'a>(&'a self, index: NodeIndex) -> Self::AdjacentNodes<'a>;
    fn adjacent_edges<'a>(&'a self, index: NodeIndex) -> Self::AdjacentEdges<'a>;
}
