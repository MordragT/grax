use super::AdjacencyOptions;
use crate::{
    edge::EdgeRef,
    error::GraphResult,
    prelude::{
        EdgeIndex, EdgeList, Graph, GraphAccess, GraphAdjacentTopology, GraphCompare, GraphError,
        GraphTopology, Node, NodeIndex, Weight, WeightlessGraph,
    },
};

#[derive(Debug, Default, Clone)]
pub struct AdjacencyMatrix<N, W, const DIRECTED: bool = false> {
    pub(crate) nodes: Vec<N>,
    pub(crate) adjacencies: Vec<Vec<Option<W>>>,
}

impl<N, W, const DIRECTED: bool> AdjacencyMatrix<N, W, DIRECTED> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            adjacencies: Vec::new(),
        }
    }
}

impl<N, W: Clone, const DIRECTED: bool> AdjacencyMatrix<N, W, DIRECTED> {
    pub fn with(options: AdjacencyOptions<N>) -> Self {
        let nodes = if let Some(nodes) = options.nodes {
            nodes
        } else {
            Vec::new()
        };

        let adjacencies = vec![vec![None; nodes.len()]; nodes.len()];

        Self { nodes, adjacencies }
    }
}

impl<W: Copy, const DIRECTED: bool> TryFrom<EdgeList<usize, W, DIRECTED>>
    for AdjacencyMatrix<usize, W, DIRECTED>
{
    type Error = GraphError;

    fn try_from(edge_list: EdgeList<usize, W, DIRECTED>) -> Result<Self, Self::Error> {
        let EdgeList {
            parents,
            children,
            weights,
            node_count,
        } = edge_list;

        let options = AdjacencyOptions {
            nodes: Some(vec![0; node_count]),
        };
        let mut adj_mat = Self::with(options);

        for ((from, to), weight) in parents
            .into_iter()
            .zip(children.into_iter())
            .zip(weights.into_iter())
        {
            adj_mat.nodes[from] = from;
            adj_mat.nodes[to] = to;

            let from_idx = NodeIndex(from);
            let to_idx = NodeIndex(to);

            if !DIRECTED {
                adj_mat.add_edge(to_idx, from_idx, weight)?;
            }

            adj_mat.add_edge(from_idx, to_idx, weight)?;
        }

        Ok(adj_mat)
    }
}

impl<N, W, const DIRECTED: bool> GraphTopology<N, W> for AdjacencyMatrix<N, W, DIRECTED> {
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
        self.adjacencies
            .iter()
            .enumerate()
            .map(|(from, neigh)| {
                neigh
                    .into_iter()
                    .enumerate()
                    .filter_map(move |(to, weight)| {
                        if let Some(weight) = weight {
                            Some(EdgeRef::new(NodeIndex(from), NodeIndex(to), weight))
                        } else {
                            None
                        }
                    })
            })
            .flatten()
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn edge_count(&self) -> usize {
        todo!()
    }

    fn directed(&self) -> bool {
        DIRECTED
    }
}

impl<N, W: Clone, const DIRECTED: bool> GraphAdjacentTopology<N, W>
    for AdjacencyMatrix<N, W, DIRECTED>
{
    type AdjacentIndices<'a> = impl Iterator<Item = NodeIndex> + 'a where Self: 'a;
    type AdjacentNodes<'a> = impl Iterator<Item = &'a N> where Self: 'a;
    type AdjacentEdges<'a> = impl Iterator<Item = EdgeRef<'a, W>> where Self: 'a;

    fn adjacent_indices<'a>(&'a self, index: NodeIndex) -> Self::AdjacentIndices<'a> {
        self.adjacencies[index.0]
            .iter()
            .enumerate()
            .filter_map(|(to, weight)| {
                if weight.is_some() {
                    Some(NodeIndex(to))
                } else {
                    None
                }
            })
    }
    fn adjacent_nodes<'a>(&'a self, index: NodeIndex) -> Self::AdjacentNodes<'a> {
        self.adjacent_indices(index).map(|index| self.node(index))
    }
    fn adjacent_edges<'a>(&'a self, from: NodeIndex) -> Self::AdjacentEdges<'a> {
        self.adjacencies[from.0]
            .iter()
            .enumerate()
            .filter_map(move |(to, weight)| {
                if let Some(weight) = weight {
                    Some(EdgeRef::new(from, NodeIndex(to), weight))
                } else {
                    None
                }
            })
    }
}

impl<N, W: Clone, const DIRECTED: bool> GraphAccess<N, W> for AdjacencyMatrix<N, W, DIRECTED> {
    fn add_node(&mut self, node: N) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(node);
        self.adjacencies.push(vec![None; index]);
        NodeIndex(index)
    }

    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: W) -> GraphResult<EdgeIndex> {
        if self.adjacencies[from.0][to.0].is_some() {
            return Err(GraphError::EdgeAlreadyExists { from, to });
        }

        self.adjacencies[from.0][to.0] = Some(weight);

        Ok(EdgeIndex::new(from, to))
    }

    fn node(&self, index: NodeIndex) -> &N {
        &self.nodes[index.0]
    }

    fn node_mut(&mut self, index: NodeIndex) -> &mut N {
        &mut self.nodes[index.0]
    }

    fn weight(&self, index: EdgeIndex) -> &W {
        self.adjacencies[index.from.0][index.to.0]
            .as_ref()
            .expect("INTERNAL: Broken EdgeIndex: cannot get weight")
    }

    fn weight_mut(&mut self, index: EdgeIndex) -> &mut W {
        self.adjacencies[index.from.0][index.to.0]
            .as_mut()
            .expect("INTERNAL: Broken EdgeIndex: cannot get weight")
    }
}

impl<N: PartialEq, W, const DIRECTED: bool> GraphCompare<N, W> for AdjacencyMatrix<N, W, DIRECTED> {
    fn contains_edge(&self, from: NodeIndex, to: NodeIndex) -> Option<EdgeIndex> {
        if self.adjacencies[from.0][to.0].is_some() {
            Some(EdgeIndex { from, to })
        } else {
            None
        }
    }
}

impl<N: Node + Clone, W: Weight, const DIRECTED: bool> Graph<N, W>
    for AdjacencyMatrix<N, W, DIRECTED>
{
}

impl<N> WeightlessGraph<N> for AdjacencyMatrix<N, ()> {}
