use crate::{edge::EdgeRef, error::GraphResult, Direction, EdgeIndex, NodeIndex};

pub trait GraphDataProvider<N, W> {
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

    fn indices<'a>(&self) -> Self::Indices<'a>;
    fn nodes<'a>(&'a self) -> Self::Nodes<'a>;
    fn edges<'a>(&'a self) -> Self::Edges<'a>;

    fn adjacent_indices<'a>(&'a self, index: NodeIndex) -> Self::AdjacentIndices<'a>;
    fn adjacent_nodes<'a>(&'a self, index: NodeIndex) -> Self::AdjacentNodes<'a>;
    fn adjacent_edges<'a>(&'a self, index: NodeIndex) -> Self::AdjacentEdges<'a>;

    fn add_node(&mut self, node: N) -> NodeIndex;
    fn add_edge(
        &mut self,
        left: NodeIndex,
        right: NodeIndex,
        weight: W,
        direction: Direction,
    ) -> GraphResult<EdgeIndex>;

    fn get(&self, index: NodeIndex) -> &N;
    fn get_mut(&mut self, index: NodeIndex) -> &mut N;

    fn weight(&self, index: EdgeIndex) -> &W;
    fn weight_mut(&mut self, index: EdgeIndex) -> &mut W;

    fn node_count(&self) -> usize;
    fn edge_count(&self) -> usize;

    fn update_node(&mut self, index: NodeIndex, node: N) -> N {
        std::mem::replace(self.get_mut(index), node)
    }
    fn update_edge(&mut self, index: EdgeIndex, weight: W) -> W {
        std::mem::replace(self.weight_mut(index), weight)
    }
}

pub trait GraphDataProviderExt<N: PartialEq, W: PartialEq>: GraphDataProvider<N, W> {
    fn contains_node(&self, node: &N) -> Option<NodeIndex> {
        for (i, other) in self.nodes().enumerate() {
            if node == other {
                return Some(NodeIndex(i));
            }
        }
        None
    }
    fn contains_edge(&self, left: NodeIndex, right: NodeIndex) -> Option<EdgeIndex>;
}
