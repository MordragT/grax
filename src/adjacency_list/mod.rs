use crate::{
    edge::EdgeRef,
    error::{GraphError, GraphResult},
    graph::{GraphAccess, GraphAdjacentTopology, GraphCompare, GraphTopology},
    indices::{EdgeIndex, NodeIndex},
    prelude::{EdgeList, Graph, Node, Weight},
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

#[cfg(test)]
mod test;

#[derive(Default)]
pub struct AdjacencyOptions<N> {
    pub nodes: Option<Vec<N>>,
    pub directed: bool,
}

#[derive(Debug, Default, Clone)]
pub struct AdjacencyList<N, W> {
    pub(crate) nodes: Vec<N>,
    pub(crate) adjacencies: Vec<Vec<NodeIndex>>,
    pub(crate) edges: HashMap<EdgeIndex, W>,
    pub(crate) directed: bool,
}

impl<N, W> AdjacencyList<N, W> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            adjacencies: Vec::new(),
            edges: HashMap::new(),
            directed: false,
        }
    }

    pub fn with(options: AdjacencyOptions<N>) -> Self {
        let nodes = if let Some(nodes) = options.nodes {
            nodes
        } else {
            Vec::new()
        };

        let adjacencies = vec![Vec::new(); nodes.len()];

        Self {
            nodes,
            adjacencies,
            edges: HashMap::new(),
            directed: options.directed,
        }
    }
}

impl<W: Clone> AdjacencyList<usize, W> {
    pub fn from_edge_list(edge_list: EdgeList<usize, W>, directed: bool) -> GraphResult<Self> {
        let EdgeList {
            parents,
            children,
            weights,
            node_count,
        } = edge_list;

        let options = AdjacencyOptions {
            nodes: Some(vec![0; node_count]),
            directed,
        };
        let mut adj_list = Self::with(options);

        for ((from, to), weight) in parents
            .into_iter()
            .zip(children.into_iter())
            .zip(weights.into_iter())
        {
            adj_list.nodes[from] = from;
            adj_list.nodes[to] = to;

            let from_idx = NodeIndex(from);
            let to_idx = NodeIndex(to);

            if !directed {
                adj_list.add_edge(to_idx, from_idx, weight.clone())?;
            }

            adj_list.add_edge(from_idx, to_idx, weight)?;
        }

        Ok(adj_list)
    }
}

impl<N, W> GraphTopology<N, W> for AdjacencyList<N, W> {
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
        self.edges
            .iter()
            .map(|(index, weight)| EdgeRef::new(index.from, index.to, weight))
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
        self.directed
    }
}

impl<N, W: Clone> GraphAdjacentTopology<N, W> for AdjacencyList<N, W> {
    type AdjacentIndices<'a> = impl Iterator<Item = NodeIndex> + 'a where Self: 'a;
    type AdjacentNodes<'a> = impl Iterator<Item = &'a N> where Self: 'a;
    type AdjacentEdges<'a> = impl Iterator<Item = EdgeRef<'a, W>> where Self: 'a;

    fn adjacent_indices<'a>(&'a self, index: NodeIndex) -> Self::AdjacentIndices<'a> {
        self.adjacencies[index.0].iter().cloned()
    }
    fn adjacent_nodes<'a>(&'a self, index: NodeIndex) -> Self::AdjacentNodes<'a> {
        self.adjacent_indices(index).map(|index| self.get(index))
    }
    fn adjacent_edges<'a>(&'a self, index: NodeIndex) -> Self::AdjacentEdges<'a> {
        self.adjacent_indices(index).map(move |child| {
            let edge_index = EdgeIndex::new(index, child);
            let weight = self.weight(edge_index);
            EdgeRef::new(index, child, weight)
        })
    }
}

impl<N, W: Clone> GraphAccess<N, W> for AdjacencyList<N, W> {
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

        let index = EdgeIndex::new(from, to);

        assert!(self.edges.insert(index, weight).is_none());

        Ok(index)
    }

    fn retain_nodes(&mut self, nodes: impl Iterator<Item = NodeIndex>) {
        todo!()
    }

    fn retain_edges(&mut self, edges: impl Iterator<Item = EdgeIndex>) {
        let retain = edges.collect::<HashSet<_>>();

        self.edges.retain(|index, _weight| retain.contains(index));
        for (from, adj_nodes) in self.adjacencies.iter_mut().enumerate() {
            adj_nodes.retain(|to| {
                let index = EdgeIndex::new(NodeIndex(from), *to);
                retain.contains(&index)
            })
        }
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
}

impl<N: PartialEq, W> GraphCompare<N, W> for AdjacencyList<N, W> {
    fn contains_edge(&self, from: NodeIndex, to: NodeIndex) -> Option<EdgeIndex> {
        let index = EdgeIndex::new(from, to);
        if self.edges.contains_key(&index) {
            Some(index)
        } else {
            None
        }
    }
}

impl<N: Node + Clone, W: Weight> Graph<N, W> for AdjacencyList<N, W> {}
