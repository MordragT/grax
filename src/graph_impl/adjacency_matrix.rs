use super::{EdgeIndex, NodeIndex};
use crate::{
    edge_list::EdgeList,
    graph::{Clear, Extend, Get, GetMut, IndexAdjacent, Insert, Remove},
    prelude::{
        Base, Capacity, Count, Create, Directed, EdgeRef, Index, IterEdges, IterNodes,
        IterNodesMut, Reserve,
    },
    utils::SparseMatrix,
};

#[derive(Debug, Clone)]
pub struct AdjacencyMatrix<Node, Weight, const Di: bool = false> {
    pub(crate) nodes: Vec<Node>,
    pub(crate) edges: SparseMatrix<Weight>,
}

impl<W: Copy, const Di: bool> From<EdgeList<usize, W, Di>> for AdjacencyMatrix<usize, W, Di> {
    fn from(edge_list: EdgeList<usize, W, Di>) -> Self {
        let EdgeList {
            parents,
            children,
            weights,
            node_count,
        } = edge_list;

        let mut adj_mat = Self::with_capacity(node_count, parents.len());

        for ((from, to), weight) in parents
            .into_iter()
            .zip(children.into_iter())
            .zip(weights.into_iter())
        {
            adj_mat.nodes[from] = from;
            adj_mat.nodes[to] = to;

            let edge_id = EdgeIndex::new(NodeIndex(from), NodeIndex(to));

            if !Di {
                adj_mat.insert_edge(edge_id.rev(), weight);
            }

            adj_mat.insert_edge(edge_id, weight);
        }

        adj_mat
    }
}

impl<Node, Weight, const Di: bool> Base for AdjacencyMatrix<Node, Weight, Di> {
    type EdgeId = EdgeIndex;
    type NodeId = NodeIndex;
}

impl<Node, Weight, const Di: bool> Capacity for AdjacencyMatrix<Node, Weight, Di> {
    fn edges_capacity(&self) -> usize {
        todo!()
    }

    fn nodes_capacity(&self) -> usize {
        self.nodes.capacity()
    }
}

impl<Node, Weight, const Di: bool> Clear for AdjacencyMatrix<Node, Weight, Di> {
    fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }
}

impl<Node, Weight, const Di: bool> Count for AdjacencyMatrix<Node, Weight, Di> {
    fn edge_count(&self) -> usize {
        todo!()
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl<Node, Weight, const Di: bool> Create<Node> for AdjacencyMatrix<Node, Weight, Di> {
    fn with_capacity(nodes: usize, _edges: usize) -> Self {
        let edges = SparseMatrix::with_capacity(nodes, nodes);
        let nodes = Vec::with_capacity(nodes);

        Self { nodes, edges }
    }

    fn with_nodes(nodes: impl Iterator<Item = Node>) -> Self {
        let nodes: Vec<Node> = nodes.collect();
        let node_count = nodes.len();
        let edges = SparseMatrix::with_capacity(node_count, node_count);

        Self { nodes, edges }
    }
}

impl<Node, Weight, const Di: bool> Directed for AdjacencyMatrix<Node, Weight, Di> {
    fn directed(&self) -> bool {
        Di
    }
}

impl<Node, Weight, const Di: bool> Extend<Node, Weight> for AdjacencyMatrix<Node, Weight, Di> {
    fn extend_edges(&mut self, edges: impl Iterator<Item = (Self::EdgeId, Weight)>) {
        for (EdgeIndex { from, to }, weight) in edges {
            self.edges.insert(from.0, to.0, weight)
        }
    }

    fn extend_nodes(&mut self, nodes: impl Iterator<Item = Node>) {
        self.nodes.extend(nodes);
    }
}

impl<Node, Weight, const Di: bool> Get<Node, Weight> for AdjacencyMatrix<Node, Weight, Di> {
    fn node(&self, node_id: Self::NodeId) -> Option<&Node> {
        self.nodes.get(node_id.0)
    }
    fn weight(&self, edge_id: Self::EdgeId) -> Option<&Weight> {
        self.edges.get(edge_id.from.0, edge_id.to.0)
    }
}

impl<Node, Weight, const Di: bool> GetMut<Node, Weight> for AdjacencyMatrix<Node, Weight, Di> {
    fn node_mut(&mut self, node_id: Self::NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(node_id.0)
    }
    fn weight_mut(&mut self, edge_id: Self::EdgeId) -> Option<&mut Weight> {
        self.edges.get_mut(edge_id.from.0, edge_id.to.0)
    }
}

impl<Node, Weight, const Di: bool> Index for AdjacencyMatrix<Node, Weight, Di> {
    type EdgeIndices<'a> = impl Iterator<Item = EdgeIndex> + 'a
    where Self: 'a;
    type NodeIndices<'a> = impl Iterator<Item = NodeIndex> + 'a
    where Self: 'a;

    fn edge_indices<'a>(&'a self) -> Self::EdgeIndices<'a> {
        self.edges
            .iter()
            .map(|(from, to, _)| EdgeIndex::new(NodeIndex(from), NodeIndex(to)))
    }

    fn node_indices<'a>(&'a self) -> Self::NodeIndices<'a> {
        (0..self.nodes.len()).map(NodeIndex)
    }
}

impl<Node, Weight, const Di: bool> IndexAdjacent for AdjacencyMatrix<Node, Weight, Di> {
    type AdjacentEdgeIndices<'a> = impl Iterator<Item = EdgeIndex> + 'a
    where Self: 'a;
    type AdjacentNodeIndices<'a> = impl Iterator<Item = NodeIndex> + 'a
    where Self: 'a;

    fn adjacent_edge_indices<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentEdgeIndices<'a> {
        self.edges
            .row(node_id.0)
            .map(move |(to, _)| EdgeIndex::new(node_id, NodeIndex(to)))
    }
    fn adjacent_node_indices<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentNodeIndices<'a> {
        self.edges.row(node_id.0).map(|(to, _)| NodeIndex(to))
    }
}

impl<Node, Weight, const Di: bool> Insert<Node, Weight> for AdjacencyMatrix<Node, Weight, Di> {
    fn add_node(&mut self, node: Node) -> Self::NodeId {
        let node_id = NodeIndex(self.nodes.len());
        self.nodes.push(node);
        return node_id;
    }
    fn insert_edge(&mut self, edge_id: Self::EdgeId, weight: Weight) -> Option<Weight> {
        self.edges.insert(edge_id.from.0, edge_id.to.0, weight);
        None
    }
}

impl<Node, Weight, const Di: bool> IterEdges<Weight> for AdjacencyMatrix<Node, Weight, Di> {
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, EdgeIndex, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_edges<'a>(&'a self) -> Self::Edges<'a> {
        self.edges.iter().map(|(from, to, weight)| {
            EdgeRef::new(EdgeIndex::new(NodeIndex(from), NodeIndex(to)), weight)
        })
    }
}

impl<Node, Weight, const Di: bool> IterNodes<Node> for AdjacencyMatrix<Node, Weight, Di> {
    type Nodes<'a> = impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a> {
        self.nodes.iter()
    }
}

impl<Node, Weight, const Di: bool> IterNodesMut<Node> for AdjacencyMatrix<Node, Weight, Di> {
    type NodesMut<'a> = impl Iterator<Item = &'a mut Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a> {
        self.nodes.iter_mut()
    }
}

impl<Node, Weight, const Di: bool> Remove<Node, Weight> for AdjacencyMatrix<Node, Weight, Di> {
    fn remove_node(&mut self, node_id: Self::NodeId) -> Option<Node> {
        todo!()
    }

    fn remove_edge(&mut self, edge_id: Self::EdgeId) -> Option<Weight> {
        todo!()
    }
}

impl<Node, Weight, const Di: bool> Reserve for AdjacencyMatrix<Node, Weight, Di> {
    fn reserve_edges(&mut self, additional: usize) {
        todo!()
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.nodes.reserve(additional)
    }
}
