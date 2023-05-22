use super::{GraphAccess, GraphAdjacentTopology, GraphTopology, Sortable};
use crate::{
    algorithms::{AugmentedPath, Flow},
    edge::{Edge, EdgeRef},
    error::GraphResult,
    prelude::{DirectedEdgeIndex, EdgeIndex, GraphError, NodeIndex},
};
use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Debug,
    ops::{AddAssign, Sub},
};

pub struct ResidualGraph<N, W> {
    pub(crate) nodes: Vec<N>,
    pub(crate) adjacencies: Vec<Vec<NodeIndex>>,
    pub(crate) edges: BTreeMap<DirectedEdgeIndex, Flow<W>>,
}

impl<N, W: Clone + Default> ResidualGraph<N, W> {
    pub fn new(nodes: Vec<N>, edges: impl Iterator<Item = Edge<W>>) -> Self {
        let mut graph = ResidualGraph {
            nodes,
            adjacencies: Vec::new(),
            edges: BTreeMap::new(),
        };
        for Edge { from, to, weight } in edges {
            graph.add_edge(from, to, weight).unwrap();
        }

        graph
    }

    pub fn is_edge_rev(&self, index: EdgeIndex) -> bool {
        let index = DirectedEdgeIndex::Reverse(index);
        self.edges.contains_key(index)
    }
}

impl<N: PartialEq, W: Sortable + Default + Clone + Sub<W, Output = W> + AddAssign + Debug>
    ResidualGraph<N, W>
{
    fn _bfs_augmenting_path<'a>(
        &self,
        source: NodeIndex,
        sink: NodeIndex,
    ) -> Option<AugmentedPath> {
        let mut queue = VecDeque::new();
        let mut edges = Vec::new();
        let mut visited = vec![false; self.node_count()];

        queue.push_front(source);
        visited[source.0] = true;

        while let Some(from) = queue.pop_front() {
            if from == sink {
                return Some(AugmentedPath::new(edges));
            }

            for EdgeRef {
                from,
                to,
                weight: _,
            } in self.adjacent_edges(from)
            {
                let index = EdgeIndex::new(from, to);

                if !visited[to.0] {
                    let index = if backward_edges.contains(&index) {
                        DirectedEdgeIndex::Reverse(index)
                    } else if full_edges.contains(&index) {
                        continue;
                    } else {
                        DirectedEdgeIndex::Forward(index)
                    };
                    edges.push(index);

                    queue.push_back(to);
                    visited[to.0] = true;
                }
            }
        }
        None
    }
}

impl<N, W> GraphTopology<N, W> for ResidualGraph<N, W> {
    type Indices<'a> = impl Iterator<Item = NodeIndex> where Self: 'a;
    type Nodes<'a> = impl Iterator<Item = &'a N> where Self: 'a;
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, W>> where Self: 'a;

    fn indices<'a>(&self) -> Self::Indices<'a> {
        (0..self.node_count()).map(NodeIndex)
    }
    fn nodes<'a>(&'a self) -> Self::Nodes<'a> {
        self.nodes.iter()
    }
    fn edges<'a>(&'a self) -> Self::Edges<'a> {
        self.edges.iter().map(|(index, flow)| {
            let index = index.raw();
            EdgeRef::new(index.from, index.to, &flow.max)
        })
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

    fn directed(&self) -> bool {
        false
    }
}

impl<N, W: Clone + Default> GraphAdjacentTopology<N, W> for ResidualGraph<N, W> {
    type AdjacentIndices<'a> = impl Iterator<Item = NodeIndex> + 'a where Self: 'a;
    type AdjacentNodes<'a> = impl Iterator<Item = &'a N> where Self: 'a;
    type AdjacentEdges<'a> = impl Iterator<Item = EdgeRef<'a, W>> where Self: 'a;

    fn adjacent_indices<'a>(&'a self, index: NodeIndex) -> Self::AdjacentIndices<'a> {
        self.adjacencies[index.0].iter().cloned()
    }
    fn adjacent_nodes<'a>(&'a self, index: NodeIndex) -> Self::AdjacentNodes<'a> {
        self.adjacent_indices(index).map(|index| self.node(index))
    }
    fn adjacent_edges<'a>(&'a self, index: NodeIndex) -> Self::AdjacentEdges<'a> {
        self.adjacent_indices(index).map(move |child| {
            let edge_index = EdgeIndex::new(index, child);
            let weight = self.weight(edge_index);
            EdgeRef::new(index, child, weight)
        })
    }
}

impl<N, W: Clone + Default> GraphAccess<N, W> for ResidualGraph<N, W> {
    fn add_node(&mut self, node: N) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(node);
        self.adjacencies.push(Vec::new());
        NodeIndex(index)
    }

    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: W) -> GraphResult<EdgeIndex> {
        if self.adjacencies[from.0].contains(&to) {
            return Err(GraphError::EdgeAlreadyExists { from, to });
        }

        self.adjacencies[from.0].push(to);
        self.adjacencies[to.0].push(from);

        let index = EdgeIndex::new(from, to);
        assert!(self
            .edges
            .insert(DirectedEdgeIndex::Forward(index), Flow::new(weight))
            .is_none());
        assert!(self
            .edges
            .insert(DirectedEdgeIndex::Reverse(index.rev()), Flow::new(weight))
            .is_none());

        Ok(index)
    }

    fn node(&self, index: NodeIndex) -> &N {
        &self.nodes[index.0]
    }

    fn node_mut(&mut self, index: NodeIndex) -> &mut N {
        &mut self.nodes[index.0]
    }

    fn weight(&self, index: EdgeIndex) -> &W {
        let index = DirectedEdgeIndex::Forward(index);
        &self.edges[&index].max
    }

    fn weight_mut(&mut self, index: EdgeIndex) -> &mut W {
        let index = DirectedEdgeIndex::Forward(index);
        &mut self
            .edges
            .get_mut(&index)
            .expect("INTERNAL: Broken EdgeIndex: cannot get weight")
            .max
    }
}
