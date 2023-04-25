use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::{
    error::{GraphError, GraphResult},
    Direction, EdgeIndex, EdgeRef, GraphKind, NodeIndex,
};

pub trait GraphDataProvider<N, W>: Default {
    type AdjacentEdges<'a>: IntoIterator<Item = EdgeRef<'a, W>> + 'a
    where
        W: 'a,
        Self: 'a;
    type AdjacentIndices<'a>: IntoIterator<Item = &'a NodeIndex> + 'a
    where
        Self: 'a;
    type Nodes<'a>: IntoIterator<Item = &'a N> + 'a
    where
        N: 'a,
        Self: 'a;

    fn node_indices(&self) -> Vec<NodeIndex> {
        (0..self.node_count()).map(NodeIndex).collect()
    }
    fn nodes<'a>(&'a self) -> Self::Nodes<'a>;
    fn adjacent_indices<'a>(&'a self, index: NodeIndex) -> Self::AdjacentIndices<'a>;
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
        for (i, other) in self.nodes().into_iter().enumerate() {
            if node == other {
                return Some(NodeIndex(i));
            }
        }
        None
    }
    fn contains_edge(&self, left: NodeIndex, right: NodeIndex) -> Option<EdgeIndex>;
}

#[derive(Debug, Default)]
pub struct AdjacencyList<const KIND: GraphKind, N, W> {
    pub(crate) nodes: Vec<N>,
    pub(crate) adjacencies: Vec<HashSet<NodeIndex>>,
    pub(crate) edges: HashMap<EdgeIndex, W>,
}

impl<const KIND: GraphKind, N: PartialEq + Default, W: PartialEq + Default>
    GraphDataProviderExt<N, W> for AdjacencyList<KIND, N, W>
{
    fn contains_edge(&self, left: NodeIndex, right: NodeIndex) -> Option<EdgeIndex> {
        let index = EdgeIndex::new(left, right, 0);
        if self.edges.contains_key(&index) {
            Some(index)
        } else {
            None
        }
    }
}

impl<const KIND: GraphKind, N: Default, W: Default> GraphDataProvider<N, W>
    for AdjacencyList<KIND, N, W>
{
    type Nodes<'a> = &'a Vec<N> where Self: 'a;
    type AdjacentIndices<'a> = &'a HashSet<NodeIndex> where Self: 'a;
    type AdjacentEdges<'a> = Vec<EdgeRef<'a, W>> where Self: 'a;

    fn nodes<'a>(&'a self) -> Self::Nodes<'a> {
        &self.nodes
    }

    fn adjacent_indices<'a>(&'a self, index: NodeIndex) -> Self::AdjacentIndices<'a> {
        &self.adjacencies[index.0]
    }
    fn adjacent_edges<'a>(&'a self, index: NodeIndex) -> Self::AdjacentEdges<'a> {
        self.adjacent_indices(index)
            .into_iter()
            .map(|child| {
                let index = EdgeIndex::new(index, *child, 0);
                let weight = self.weight(index);
                EdgeRef::new(*child, weight)
            })
            .collect::<Vec<_>>()
    }

    fn add_node(&mut self, node: N) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(node);
        self.adjacencies.push(HashSet::new());
        NodeIndex(index)
    }

    fn add_edge(
        &mut self,
        left: NodeIndex,
        right: NodeIndex,
        weight: W,
        direction: Direction,
    ) -> GraphResult<EdgeIndex> {
        let (parent, child) = match direction {
            Direction::Incoming => (right, left),
            Direction::Outgoing => (left, right),
        };

        if self.adjacencies[parent.0].contains(&child) {
            return Err(GraphError::EdgeAlreadyExists {
                left: parent,
                right: child,
            });
        }

        assert!(self.adjacencies[parent.0].insert(child));

        let index = EdgeIndex::new(parent, child, 0);

        assert!(self.edges.insert(index, weight).is_none());

        Ok(index)
    }

    fn get(&self, index: NodeIndex) -> &N {
        &self.nodes[index.0]
    }

    fn get_mut(&mut self, index: NodeIndex) -> &mut N {
        &mut self.nodes[index.0]
    }

    fn weight(&self, index: EdgeIndex) -> &W {
        &self.edges[&index]
    }

    fn weight_mut(&mut self, index: EdgeIndex) -> &mut W {
        self.edges
            .get_mut(&index)
            .expect("INTERNAL: Broken EdgeIndex: cannot get weight")
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn edge_count(&self) -> usize {
        self.adjacencies
            .iter()
            .map(|adjs| adjs.len())
            .fold(0, |a, b| a + b)
    }
}
