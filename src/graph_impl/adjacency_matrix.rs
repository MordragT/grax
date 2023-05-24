use super::{EdgeIndex, NodeIndex};
use crate::{
    edge_list::EdgeList,
    graph::{
        Base, Capacity, Clear, Contains, Count, Create, Directed, EdgeIdentifier, Extend, Get,
        GetMut, Graph, Index, IndexAdjacent, Insert, Iter, IterAdjacent, IterAdjacentMut, IterMut,
        Remove, Reserve,
    },
    prelude::{EdgeRef, EdgeRefMut, WeightlessGraph},
    utils::SparseMatrix,
};

#[derive(Debug, Clone)]
pub struct AdjacencyMatrix<Node, Weight, const DI: bool = false> {
    pub(crate) nodes: Vec<Node>,
    pub(crate) edges: SparseMatrix<Weight>,
}

impl<W: Copy, const DI: bool> From<EdgeList<usize, W, DI>> for AdjacencyMatrix<usize, W, DI> {
    fn from(edge_list: EdgeList<usize, W, DI>) -> Self {
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

            let edge_id = EdgeIndex::between(NodeIndex(from), NodeIndex(to));

            if !DI {
                adj_mat.insert_edge(edge_id.rev(), weight);
            }

            adj_mat.insert_edge(edge_id, weight);
        }

        adj_mat
    }
}

impl<Node, Weight, const DI: bool> Base for AdjacencyMatrix<Node, Weight, DI> {
    type EdgeId = EdgeIndex;
    type NodeId = NodeIndex;
}

impl<Node, Weight, const DI: bool> Capacity for AdjacencyMatrix<Node, Weight, DI> {
    fn edges_capacity(&self) -> usize {
        self.edges.row_count() * self.edges.col_count()
    }

    fn nodes_capacity(&self) -> usize {
        self.nodes.capacity()
    }
}

impl<Node, Weight, const DI: bool> Clear for AdjacencyMatrix<Node, Weight, DI> {
    fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    fn clear_edges(&mut self) {
        self.edges.clear();
    }
}

impl<Node: PartialEq, Weight, const DI: bool> Contains<Node> for AdjacencyMatrix<Node, Weight, DI> {
    fn contains_node(&self, node: &Node) -> Option<Self::NodeId> {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_i, other)| *other == node)
            .map(|(id, _)| NodeIndex(id))
    }

    fn contains_edge(&self, from: Self::NodeId, to: Self::NodeId) -> Option<Self::EdgeId> {
        let edge_id = EdgeIndex::between(from, to);
        if self.contains_edge_id(edge_id) {
            Some(edge_id)
        } else {
            None
        }
    }
}

impl<Node, Weight, const DI: bool> Count for AdjacencyMatrix<Node, Weight, DI> {
    fn edge_count(&self) -> usize {
        self.edges.nnz()
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl<Node, Weight, const DI: bool> Create<Node> for AdjacencyMatrix<Node, Weight, DI> {
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

impl<Node, Weight, const DI: bool> Directed for AdjacencyMatrix<Node, Weight, DI> {
    fn directed(&self) -> bool {
        DI
    }
}

impl<Node, Weight, const DI: bool> Extend<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
    fn extend_edges(&mut self, edges: impl Iterator<Item = (Self::EdgeId, Weight)>) {
        for (EdgeIndex { from, to }, weight) in edges {
            self.edges.insert(from.0, to.0, weight)
        }
    }

    fn extend_nodes(&mut self, nodes: impl Iterator<Item = Node>) {
        self.nodes.extend(nodes);
    }
}

impl<Node, Weight, const DI: bool> Get<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
    fn node(&self, node_id: Self::NodeId) -> Option<&Node> {
        self.nodes.get(node_id.0)
    }
    fn weight(&self, edge_id: Self::EdgeId) -> Option<&Weight> {
        self.edges.get(edge_id.from.0, edge_id.to.0)
    }
}

impl<Node, Weight, const DI: bool> GetMut<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
    fn node_mut(&mut self, node_id: Self::NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(node_id.0)
    }
    fn weight_mut(&mut self, edge_id: Self::EdgeId) -> Option<&mut Weight> {
        self.edges.get_mut(edge_id.from.0, edge_id.to.0)
    }
}

impl<Node, Weight, const DI: bool> Insert<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
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

impl<Node, Weight, const DI: bool> Index for AdjacencyMatrix<Node, Weight, DI> {
    type EdgeIds<'a> = impl Iterator<Item = EdgeIndex> + 'a
    where Self: 'a;
    type NodeIds<'a> = impl Iterator<Item = NodeIndex> + 'a
    where Self: 'a;

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.edges
            .iter()
            .map(|(from, to, _)| EdgeIndex::between(NodeIndex(from), NodeIndex(to)))
    }

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        (0..self.nodes.len()).map(NodeIndex)
    }
}

impl<Node, Weight, const DI: bool> IndexAdjacent for AdjacencyMatrix<Node, Weight, DI> {
    type AdjacentEdgeIds<'a> = impl Iterator<Item = EdgeIndex> + 'a
    where Self: 'a;
    type AdjacentNodeIds<'a> = impl Iterator<Item = NodeIndex> + 'a
    where Self: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentEdgeIds<'a> {
        self.edges
            .row(node_id.0)
            .map(move |(to, _)| EdgeIndex::between(node_id, NodeIndex(to)))
    }
    fn adjacent_node_ids<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentNodeIds<'a> {
        self.edges.row(node_id.0).map(|(to, _)| NodeIndex(to))
    }
}

impl<Node, Weight, const DI: bool> Iter<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
    type Nodes<'a> = impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, Self::EdgeId, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a> {
        self.nodes.iter()
    }
    fn iter_edges<'a>(&'a self) -> Self::Edges<'a> {
        self.edges.iter().map(|(from, to, weight)| {
            EdgeRef::new(EdgeIndex::between(NodeIndex(from), NodeIndex(to)), weight)
        })
    }
}
impl<Node, Weight, const DI: bool> IterMut<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
    type NodesMut<'a> = impl Iterator<Item = &'a mut Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    type EdgesMut<'a> = impl Iterator<Item = EdgeRefMut<'a, Self::EdgeId, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a> {
        self.nodes.iter_mut()
    }

    fn iter_edges_mut<'a>(&'a mut self) -> Self::EdgesMut<'a> {
        self.edges.iter_mut().map(|(from, to, weight)| {
            EdgeRefMut::new(EdgeIndex::between(NodeIndex(from), NodeIndex(to)), weight)
        })
    }
}

impl<Node, Weight, const DI: bool> IterAdjacent<Node, Weight>
    for AdjacencyMatrix<Node, Weight, DI>
{
    type Nodes<'a> = impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, Self::EdgeId, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_adjacent_nodes<'a>(&'a self, node_id: Self::NodeId) -> Self::Nodes<'a> {
        self.adjacent_node_ids(node_id)
            .map(|node_id| self.node(node_id).unwrap())
    }

    fn iter_adjacent_edges<'a>(&'a self, node_id: Self::NodeId) -> Self::Edges<'a> {
        self.edges.row(node_id.0).map(move |(to, weight)| {
            let edge_id = EdgeIndex::between(node_id, NodeIndex(to));
            EdgeRef::new(edge_id, weight)
        })
    }
}
impl<Node, Weight, const DI: bool> IterAdjacentMut<Node, Weight>
    for AdjacencyMatrix<Node, Weight, DI>
{
    type NodesMut<'a> = impl Iterator<Item = &'a mut Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    type EdgesMut<'a> = impl Iterator<Item = EdgeRefMut<'a, Self::EdgeId, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_adjacent_nodes_mut<'a>(&'a mut self, node_id: Self::NodeId) -> Self::NodesMut<'a> {
        let ids = self.adjacent_node_ids(node_id).collect::<Vec<_>>();
        self.nodes
            .iter_mut()
            .enumerate()
            .filter_map(move |(i, node)| {
                if ids.contains(&NodeIndex(i)) {
                    Some(node)
                } else {
                    None
                }
            })
    }

    fn iter_adjacent_edges_mut<'a>(&'a mut self, node_id: Self::NodeId) -> Self::EdgesMut<'a> {
        self.edges.row_mut(node_id.0).map(move |(to, weight)| {
            let edge_id = EdgeIndex::between(node_id, NodeIndex(to));
            EdgeRefMut::new(edge_id, weight)
        })
    }
}

impl<Node, Weight, const DI: bool> Remove<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
    fn remove_node(&mut self, node_id: Self::NodeId) -> Option<Node> {
        todo!()
    }

    fn remove_edge(&mut self, edge_id: Self::EdgeId) -> Option<Weight> {
        todo!()
    }
}

impl<Node, Weight, const DI: bool> Reserve for AdjacencyMatrix<Node, Weight, DI> {
    fn reserve_edges(&mut self, additional: usize) {
        todo!()
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.nodes.reserve(additional)
    }
}

impl<Node: crate::graph::Node, Weight: crate::graph::Weight, const DI: bool> Graph<Node, Weight>
    for AdjacencyMatrix<Node, Weight, DI>
{
}

impl<Node: crate::graph::Node, const DI: bool> WeightlessGraph<Node>
    for AdjacencyMatrix<Node, (), DI>
{
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::AdjacencyMatrix;
    use crate::graph::test::*;

    #[test]
    pub fn adj_mat_create_with_nodes() {
        graph_create_with_nodes::<AdjacencyMatrix<_, _>>()
    }

    #[test]
    pub fn adj_mat_create_with_capacity() {
        graph_create_with_capacity::<AdjacencyMatrix<_, _>>()
    }

    #[test]
    pub fn adj_mat_insert_and_contains() {
        graph_insert_and_contains::<AdjacencyMatrix<_, _>>()
    }

    #[test]
    pub fn adj_mat_clear() {
        graph_clear::<AdjacencyMatrix<_, _>>()
    }

    #[test]
    pub fn adj_mat_get() {
        graph_get::<AdjacencyMatrix<_, _>>()
    }

    #[test]
    pub fn adj_mat_index() {
        graph_index::<AdjacencyMatrix<_, _>>()
    }

    #[test]
    pub fn adj_mat_index_adjacent() {
        graph_index_adjacent::<AdjacencyMatrix<_, _>>()
    }
}
