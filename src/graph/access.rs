use crate::{
    error::GraphResult,
    indices::{EdgeIndex, NodeIndex},
};

use super::topology::GraphTopology;

pub trait GraphAccess<N, W> {
    fn add_node(&mut self, node: N) -> NodeIndex;
    // todo remove result just return EdgeIndex
    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: W) -> GraphResult<EdgeIndex>;

    fn node(&self, index: NodeIndex) -> &N;
    fn node_mut(&mut self, index: NodeIndex) -> &mut N;

    fn weight(&self, index: EdgeIndex) -> &W;
    fn weight_mut(&mut self, index: EdgeIndex) -> &mut W;

    fn update_node(&mut self, index: NodeIndex, node: N) -> N {
        std::mem::replace(self.node_mut(index), node)
    }
    fn update_edge(&mut self, index: EdgeIndex, weight: W) -> W {
        std::mem::replace(self.weight_mut(index), weight)
    }
}

pub trait GraphCompare<N: PartialEq, W>: GraphTopology<N, W> {
    fn contains_node(&self, node: &N) -> Option<NodeIndex> {
        for (i, other) in self.nodes().enumerate() {
            if node == other {
                return Some(NodeIndex(i));
            }
        }
        None
    }
    fn contains_edge(&self, from: NodeIndex, to: NodeIndex) -> Option<EdgeIndex> {
        let index = EdgeIndex::new(from, to);
        for edge in self.edges() {
            if edge.from == from && edge.to == to {
                return Some(index);
            }
        }
        None
    }
}
