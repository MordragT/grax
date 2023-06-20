use crate::{
    edge_list::EdgeList,
    graph::{
        Base, Capacity, Clear, Contains, Count, Create, Directed, Extend, Get, GetMut, Graph,
        Index, IndexAdjacent, Insert, Iter, IterAdjacent, IterAdjacentMut, IterMut, Remove,
        Reserve,
    },
    prelude::{EdgeId, EdgeRef, EdgeRefMut, NodeId, WeightlessGraph},
    structures::SparseMatrix,
};

type RawNodeId = NodeId<usize>;
type RawEdgeId = EdgeId<usize>;

#[derive(Debug, Clone)]
pub struct AdjacencyMatrix<Node, Weight, const DI: bool = false> {
    pub(crate) nodes: Vec<Node>,
    pub(crate) edges: SparseMatrix<Weight>,
}

impl<Node, W: Copy, const DI: bool> From<EdgeList<Node, W, DI>> for AdjacencyMatrix<Node, W, DI> {
    fn from(edge_list: EdgeList<Node, W, DI>) -> Self {
        let EdgeList {
            nodes,
            edges,
            node_count: _,
        } = edge_list;

        let mut adj_mat = Self::with_nodes(nodes.into_iter());

        for (from, to, weight) in edges.into_iter() {
            let from = NodeId::new_unchecked(from);
            let to = NodeId::new_unchecked(to);

            if !DI {
                adj_mat.insert_edge(to, from, weight);
            }

            adj_mat.insert_edge(from, to, weight);
        }

        adj_mat
    }
}

impl<Node, Weight, const DI: bool> Base for AdjacencyMatrix<Node, Weight, DI> {
    type Id = usize;
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
    fn contains_node(&self, node: &Node) -> Option<RawNodeId> {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_i, other)| *other == node)
            .map(|(id, _)| RawNodeId::new_unchecked(id))
    }

    fn contains_edge(&self, from: RawNodeId, to: RawNodeId) -> Option<RawEdgeId> {
        let edge_id = RawEdgeId::new_unchecked(from, to);
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

    fn with_nodes(nodes: impl IntoIterator<Item = Node>) -> Self {
        let nodes: Vec<Node> = nodes.into_iter().collect();
        let node_count = nodes.len();
        let edges = SparseMatrix::with_capacity(node_count, node_count);

        Self { nodes, edges }
    }
}

impl<Node, Weight, const DI: bool> Directed for AdjacencyMatrix<Node, Weight, DI> {
    fn directed() -> bool {
        DI
    }
}

impl<Node, Weight, const DI: bool> Extend<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
    fn extend_edges(&mut self, edges: impl Iterator<Item = (RawNodeId, RawNodeId, Weight)>) {
        for (from, to, weight) in edges {
            self.edges.insert(from.as_usize(), to.as_usize(), weight)
        }
    }

    fn extend_nodes(&mut self, nodes: impl Iterator<Item = Node>) {
        self.nodes.extend(nodes);
    }
}

impl<Node, Weight, const DI: bool> Get<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
    fn node(&self, node_id: RawNodeId) -> Option<&Node> {
        self.nodes.get(node_id.as_usize())
    }
    fn weight(&self, edge_id: RawEdgeId) -> Option<&Weight> {
        self.edges
            .get(edge_id.from().as_usize(), edge_id.to().as_usize())
    }
}

impl<Node, Weight, const DI: bool> GetMut<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
    fn node_mut(&mut self, node_id: RawNodeId) -> Option<&mut Node> {
        self.nodes.get_mut(node_id.as_usize())
    }
    fn weight_mut(&mut self, edge_id: RawEdgeId) -> Option<&mut Weight> {
        self.edges
            .get_mut(edge_id.from().as_usize(), edge_id.to().as_usize())
    }
}

impl<Node, Weight, const DI: bool> Insert<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
    fn insert_node(&mut self, node: Node) -> RawNodeId {
        let node_id = RawNodeId::new_unchecked(self.nodes.len());
        self.nodes.push(node);
        return node_id;
    }
    fn insert_edge(&mut self, from: RawNodeId, to: RawNodeId, weight: Weight) -> RawEdgeId {
        self.edges.insert(from.as_usize(), to.as_usize(), weight);
        RawEdgeId::new_unchecked(from, to)
    }
}

impl<Node, Weight, const DI: bool> Index for AdjacencyMatrix<Node, Weight, DI> {
    type EdgeIds<'a> = impl Iterator<Item = RawEdgeId> + 'a
    where Self: 'a;
    type NodeIds<'a> = impl Iterator<Item = RawNodeId> + 'a
    where Self: 'a;

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.edges.iter().map(|(from, to, _)| {
            let from = RawNodeId::new_unchecked(from);
            let to = RawNodeId::new_unchecked(to);
            RawEdgeId::new_unchecked(from, to)
        })
    }

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        (0..self.nodes.len()).map(RawNodeId::new_unchecked)
    }
}

impl<Node, Weight, const DI: bool> IndexAdjacent for AdjacencyMatrix<Node, Weight, DI> {
    type AdjacentEdgeIds<'a> = impl Iterator<Item = RawEdgeId> + 'a
    where Self: 'a;
    type AdjacentNodeIds<'a> = impl Iterator<Item = RawNodeId> + 'a
    where Self: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: RawNodeId) -> Self::AdjacentEdgeIds<'a> {
        self.edges.row(node_id.as_usize()).map(move |(to, _)| {
            let to = RawNodeId::new_unchecked(to);
            RawEdgeId::new_unchecked(node_id, to)
        })
    }
    fn adjacent_node_ids<'a>(&'a self, node_id: RawNodeId) -> Self::AdjacentNodeIds<'a> {
        self.edges
            .row(node_id.as_usize())
            .map(|(to, _)| RawNodeId::new_unchecked(to))
    }
}

impl<Node, Weight, const DI: bool> Iter<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
    type Nodes<'a> = impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a> {
        self.nodes.iter()
    }
    fn iter_edges<'a>(&'a self) -> Self::Edges<'a> {
        self.edges.iter().map(|(from, to, weight)| {
            let from = RawNodeId::new_unchecked(from);
            let to = RawNodeId::new_unchecked(to);
            let edge_id = RawEdgeId::new_unchecked(from, to);
            EdgeRef::new(edge_id, weight)
        })
    }
}
impl<Node, Weight, const DI: bool> IterMut<Node, Weight> for AdjacencyMatrix<Node, Weight, DI> {
    type NodesMut<'a> = impl Iterator<Item = &'a mut Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    type EdgesMut<'a> = impl Iterator<Item = EdgeRefMut<'a, usize, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a> {
        self.nodes.iter_mut()
    }

    fn iter_edges_mut<'a>(&'a mut self) -> Self::EdgesMut<'a> {
        self.edges.iter_mut().map(|(from, to, weight)| {
            let from = RawNodeId::new_unchecked(from);
            let to = RawNodeId::new_unchecked(to);
            let edge_id = RawEdgeId::new_unchecked(from, to);
            EdgeRefMut::new(edge_id, weight)
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

    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_adjacent_nodes<'a>(&'a self, node_id: RawNodeId) -> Self::Nodes<'a> {
        self.adjacent_node_ids(node_id)
            .map(|node_id| self.node(node_id).unwrap())
    }

    fn iter_adjacent_edges<'a>(&'a self, node_id: RawNodeId) -> Self::Edges<'a> {
        self.edges.row(node_id.as_usize()).map(move |(to, weight)| {
            let to = RawNodeId::new_unchecked(to);
            let edge_id = RawEdgeId::new_unchecked(node_id, to);
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

    type EdgesMut<'a> = impl Iterator<Item = EdgeRefMut<'a, usize, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_adjacent_nodes_mut<'a>(&'a mut self, node_id: RawNodeId) -> Self::NodesMut<'a> {
        let ids = self.adjacent_node_ids(node_id).collect::<Vec<_>>();
        self.nodes
            .iter_mut()
            .enumerate()
            .filter_map(move |(i, node)| {
                let node_id = RawNodeId::new_unchecked(i);
                if ids.contains(&node_id) {
                    Some(node)
                } else {
                    None
                }
            })
    }

    fn iter_adjacent_edges_mut<'a>(&'a mut self, node_id: RawNodeId) -> Self::EdgesMut<'a> {
        self.edges
            .row_mut(node_id.as_usize())
            .map(move |(to, weight)| {
                let to = RawNodeId::new_unchecked(to);
                let edge_id = RawEdgeId::new_unchecked(node_id, to);
                EdgeRefMut::new(edge_id, weight)
            })
    }
}

impl<Node: Default, Weight, const DI: bool> Remove<Node, Weight>
    for AdjacencyMatrix<Node, Weight, DI>
{
    fn remove_node(&mut self, node_id: RawNodeId) -> Node {
        todo!()
    }

    fn remove_edge(&mut self, edge_id: RawEdgeId) -> Option<Weight> {
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
