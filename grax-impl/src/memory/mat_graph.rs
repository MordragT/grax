use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

use grax_core::edge::*;
use grax_core::node::*;
use grax_core::prelude::*;
use grax_core::traits::*;
use grax_core::weight::Num;

use crate::edge_list::EdgeList;
use crate::matrix::csr::CsrMatrix;
use crate::matrix::{dense::DenseMatrix, sparse::SparseMatrix, Matrix};

use super::attr::{AttrHashMap, AttrVec};

type RawNodeId = NodeId<usize>;
type RawEdgeId = EdgeId<usize>;

pub type SparseMatGraph<N, W, const DI: bool = false> = MatGraph<SparseMatrix<W>, N, W, DI>;

pub type DenseMatGraph<N, W, const DI: bool = false> = MatGraph<DenseMatrix<W>, N, W, DI>;

pub type CsrMatGraph<N, W, const DI: bool = false> = MatGraph<CsrMatrix<W>, N, W, DI>;

#[derive(Debug, Clone)]
pub struct MatGraph<M, N, W, const DI: bool = false> {
    pub(crate) nodes: Vec<Node<usize, N>>,
    pub(crate) edges: M,
    weight: PhantomData<W>,
}

impl<M: Matrix<W>, W: Clone, const DI: bool> MatGraph<M, usize, W, DI> {
    pub fn with_edges(
        edges: impl IntoIterator<Item = (usize, usize, W)>,
        node_count: usize,
    ) -> Self {
        let mut adj_mat = Self::with_nodes(0..node_count);
        adj_mat.extend_edges(edges.into_iter().map(|(from, to, weight)| {
            (
                NodeId::new_unchecked(from),
                NodeId::new_unchecked(to),
                weight,
            )
        }));

        adj_mat
    }
}

impl<M: Matrix<W>, N, W: Copy, const DI: bool> From<EdgeList<N, W, DI>> for MatGraph<M, N, W, DI> {
    fn from(edge_list: EdgeList<N, W, DI>) -> Self {
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

impl<M: Matrix<W>, N, W, const DI: bool> Base for MatGraph<M, N, W, DI> {
    type Id = usize;
    type NodeWeight = N;
    type EdgeWeight = W;
}

// impl<N, W, const DI: bool> Ref for AdjacencyMatrix<N, W, DI> {
//     type GraphRef<'a> = AdjacencyMatrix<&'a Node, &'a Weight, DI>
//     where
//         Node: 'a,
//         Weight: 'a;
// }

impl<M: Matrix<W>, N, W, const DI: bool> Capacity for MatGraph<M, N, W, DI> {
    fn edges_capacity(&self) -> usize {
        self.edges.capacity()
    }

    fn nodes_capacity(&self) -> usize {
        self.nodes.capacity()
    }
}

impl<M: Matrix<W>, N, W, const DI: bool> Clear for MatGraph<M, N, W, DI> {
    fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    fn clear_edges(&mut self) {
        self.edges.clear();
    }
}

impl<M: Matrix<W>, N: PartialEq, W, const DI: bool> Contains for MatGraph<M, N, W, DI> {
    fn contains_node(&self, node: &N) -> Option<RawNodeId> {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_i, other)| &other.weight == node)
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

impl<M: Matrix<W>, N, W, const DI: bool> Count for MatGraph<M, N, W, DI> {
    fn edge_count(&self) -> usize {
        self.edges.nnz()
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl<M: Matrix<W>, N, W, const DI: bool> Create for MatGraph<M, N, W, DI> {
    fn with_capacity(nodes: usize, _edges: usize) -> Self {
        let edges = M::with_capacity(nodes, nodes);
        let nodes = Vec::with_capacity(nodes);

        Self {
            nodes,
            edges,
            weight: PhantomData,
        }
    }

    fn with_nodes(nodes: impl IntoIterator<Item = N>) -> Self {
        let nodes = nodes
            .into_iter()
            .enumerate()
            .map(|(id, weight)| Node::new(NodeId::new_unchecked(id), weight))
            .collect::<Vec<_>>();

        let node_count = nodes.len();
        let edges = M::with_capacity(node_count, node_count);

        Self {
            nodes,
            edges,
            weight: PhantomData,
        }
    }

    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: M::new(),
            weight: PhantomData,
        }
    }
}

impl<M: Matrix<W>, N, W, const DI: bool> Directed for MatGraph<M, N, W, DI> {
    fn directed() -> bool {
        DI
    }
}

impl<M: Matrix<W>, N, W, const DI: bool> Extend for MatGraph<M, N, W, DI> {
    fn extend_edges(&mut self, edges: impl IntoIterator<Item = (RawNodeId, RawNodeId, W)>) {
        for (from, to, weight) in edges {
            self.edges.insert(from.raw(), to.raw(), weight)
        }
    }

    fn extend_nodes(&mut self, nodes: impl IntoIterator<Item = N>) {
        for node in nodes {
            self.insert_node(node);
        }
    }
}

impl<M: Matrix<W>, N, W, const DI: bool> Get for MatGraph<M, N, W, DI> {
    fn node(&self, node_id: NodeId<Self::Id>) -> Option<NodeRef<Self::Id, Self::NodeWeight>> {
        self.nodes.get(node_id.raw()).map(Into::into)
    }

    fn edge(&self, edge_id: EdgeId<Self::Id>) -> Option<EdgeRef<Self::Id, Self::EdgeWeight>> {
        self.edges
            .get(edge_id.from().raw(), edge_id.to().raw())
            .map(|weight| EdgeRef::new(edge_id, weight))
    }
}

impl<M: Matrix<W>, N, W, const DI: bool> GetMut for MatGraph<M, N, W, DI> {
    fn node_mut(
        &mut self,
        node_id: NodeId<Self::Id>,
    ) -> Option<NodeRefMut<Self::Id, Self::NodeWeight>> {
        self.nodes.get_mut(node_id.raw()).map(Into::into)
    }

    fn edge_mut(
        &mut self,
        edge_id: EdgeId<Self::Id>,
    ) -> Option<EdgeRefMut<Self::Id, Self::EdgeWeight>> {
        self.edges
            .get_mut(edge_id.from().raw(), edge_id.to().raw())
            .map(|weight| EdgeRefMut::new(edge_id, weight))
    }
}

impl<M: Matrix<W>, N, W, const DI: bool> Insert for MatGraph<M, N, W, DI> {
    fn insert_node(&mut self, node: N) -> RawNodeId {
        let node_id = RawNodeId::new_unchecked(self.nodes.len());
        self.nodes.push(Node::new(node_id, node));
        return node_id;
    }
    fn insert_edge(&mut self, from: RawNodeId, to: RawNodeId, weight: W) -> RawEdgeId {
        self.edges.insert(from.raw(), to.raw(), weight);
        RawEdgeId::new_unchecked(from, to)
    }
}

impl<M: Matrix<W>, N, W, const DI: bool> Index for MatGraph<M, N, W, DI> {
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

impl<M: Matrix<W>, N, W, const DI: bool> IndexAdjacent for MatGraph<M, N, W, DI> {
    type EdgeIds<'a> = impl Iterator<Item = RawEdgeId> + 'a
    where Self: 'a;
    type NodeIds<'a> = impl Iterator<Item = RawNodeId> + 'a
    where Self: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: RawNodeId) -> Self::EdgeIds<'a> {
        self.edges.row(node_id.raw()).map(move |(to, _)| {
            let to = RawNodeId::new_unchecked(to);
            RawEdgeId::new_unchecked(node_id, to)
        })
    }
    fn adjacent_node_ids<'a>(&'a self, node_id: RawNodeId) -> Self::NodeIds<'a> {
        self.edges
            .row(node_id.raw())
            .map(|(to, _)| RawNodeId::new_unchecked(to))
    }
}

impl<M: Matrix<W>, N, W, const DI: bool> Iter for MatGraph<M, N, W, DI> {
    type Nodes<'a> = impl Iterator<Item = NodeRef<'a, usize, N>> + 'a
    where
        N: 'a,
        Self: 'a;

    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a> {
        self.nodes.iter().map(Into::into)
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
impl<M: Matrix<W>, N, W, const DI: bool> IterMut for MatGraph<M, N, W, DI> {
    type NodesMut<'a> = impl Iterator<Item = NodeRefMut<'a, usize, N>> + 'a
    where
        N: 'a,
        Self: 'a;

    type EdgesMut<'a> = impl Iterator<Item = EdgeRefMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a> {
        self.nodes.iter_mut().map(Into::into)
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

impl<M: Matrix<W>, N, W, const DI: bool> IterAdjacent for MatGraph<M, N, W, DI> {
    type Nodes<'a> = impl Iterator<Item = NodeRef<'a, usize, N>> + 'a
    where
        N: 'a,
        Self: 'a;

    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_adjacent_nodes<'a>(&'a self, node_id: RawNodeId) -> Self::Nodes<'a> {
        self.adjacent_node_ids(node_id)
            .map(|node_id| self.node(node_id).unwrap())
    }

    fn iter_adjacent_edges<'a>(&'a self, node_id: RawNodeId) -> Self::Edges<'a> {
        self.edges.row(node_id.raw()).map(move |(to, weight)| {
            let to = RawNodeId::new_unchecked(to);
            let edge_id = RawEdgeId::new_unchecked(node_id, to);
            EdgeRef::new(edge_id, weight)
        })
    }
}
impl<M: Matrix<W>, N, W, const DI: bool> IterAdjacentMut for MatGraph<M, N, W, DI> {
    type NodesMut<'a> = impl Iterator<Item = NodeRefMut<'a, usize, N>> + 'a
    where
        N: 'a,
        Self: 'a;

    type EdgesMut<'a> = impl Iterator<Item = EdgeRefMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_adjacent_nodes_mut<'a>(&'a mut self, node_id: RawNodeId) -> Self::NodesMut<'a> {
        let ids = self.adjacent_node_ids(node_id).collect::<Vec<_>>();
        self.nodes
            .iter_mut()
            .enumerate()
            .filter_map(move |(i, node)| {
                let node_id = RawNodeId::new_unchecked(i);
                if ids.contains(&node_id) {
                    Some(node.into())
                } else {
                    None
                }
            })
    }

    fn iter_adjacent_edges_mut<'a>(&'a mut self, node_id: RawNodeId) -> Self::EdgesMut<'a> {
        self.edges.row_mut(node_id.raw()).map(move |(to, weight)| {
            let to = RawNodeId::new_unchecked(to);
            let edge_id = RawEdgeId::new_unchecked(node_id, to);
            EdgeRefMut::new(edge_id, weight)
        })
    }
}

impl<M: Matrix<W>, N: Default, W, const DI: bool> Remove for MatGraph<M, N, W, DI> {
    fn remove_node(
        &mut self,
        node_id: NodeId<Self::Id>,
    ) -> Option<Node<Self::Id, Self::NodeWeight>> {
        todo!()
    }
    fn remove_edge(
        &mut self,
        edge_id: EdgeId<Self::Id>,
    ) -> Option<Edge<Self::Id, Self::EdgeWeight>> {
        todo!()
    }
}

impl<M: Matrix<W>, N, W, const DI: bool> Reserve for MatGraph<M, N, W, DI> {
    fn reserve_edges(&mut self, additional: usize) {
        todo!()
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.nodes.reserve(additional)
    }
}

impl<M: Matrix<W>, N, W, const DI: bool> Visitable for MatGraph<M, N, W, DI> {
    type VisitMap = AttrVec<bool>;

    fn visit_map(&self) -> Self::VisitMap {
        AttrVec(vec![false; self.node_count()])
    }
}

impl<M: Matrix<W>, N, W, const DI: bool> Viewable for MatGraph<M, N, W, DI> {
    type NodeMap<Attr: Clone + Default + Debug> = AttrVec<Attr>;
    type EdgeMap<Attr: Clone + Default + Debug> = AttrHashMap<EdgeId<Self::Id>, Attr>;

    fn node_map<Attr: Clone + Default + Debug>(&self) -> Self::NodeMap<Attr> {
        AttrVec(vec![Attr::default(); self.node_count()])
    }

    fn edge_map<Attr: Clone + Default + Debug>(&self) -> Self::EdgeMap<Attr> {
        let mut map = HashMap::with_capacity(self.edge_count());
        map.extend(self.edge_ids().map(|edge_id| (edge_id, Attr::default())));
        AttrHashMap(map)
    }

    // fn update_edge_map<Attr: Clone + Debug + Default>(&self, map: &mut Self::EdgeMap<Attr>) {
    //     let subset = map.0.keys().cloned().collect::<HashSet<_>>();
    //     let supset = self.edge_ids().collect::<HashSet<_>>();

    //     for &key in supset.difference(&subset) {
    //         assert!(map.0.insert(key, Attr::default()).is_none());
    //     }
    // }

    // fn update_node_map<Attr: Clone + Debug + Default>(&self, map: &mut Self::NodeMap<Attr>) {
    //     let pos = map.0.len();

    //     // TODO Assumes that no nodes have been deleted
    //     // otherwise ids in the map are no longer valid
    //     for _ in pos..self.node_count() {
    //         map.0.push(Attr::default())
    //     }
    // }
}

impl<M1: Matrix<W1>, M2: Matrix<W2>, N, W1, W2: Clone, const DI: bool>
    AdaptEdge<MatGraph<M2, N, W2, DI>, W2> for MatGraph<M1, N, W1, DI>
{
    fn map_edge<F>(self, f: F) -> MatGraph<M2, N, W2, DI>
    where
        F: Fn(Edge<Self::Id, Self::EdgeWeight>) -> Edge<Self::Id, W2>,
    {
        let Self {
            nodes,
            edges,
            weight: _,
        } = self;

        let edges = edges.into_iter().map(|(from, to, weight)| {
            let edge = f(Edge::new(
                EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to)),
                weight,
            ));
            (edge.edge_id.from(), edge.edge_id.to(), edge.weight)
        });

        let mut adj_list = MatGraph::with_nodes(nodes.into_iter().map(|node| node.weight));
        adj_list.extend_edges(edges);

        adj_list
    }

    fn split_map_edge<F>(self, f: F) -> MatGraph<M2, N, W2, DI>
    where
        F: Fn(Edge<Self::Id, Self::EdgeWeight>) -> Vec<Edge<Self::Id, W2>>,
    {
        let Self {
            nodes,
            edges,
            weight: _,
        } = self;

        let edges = edges.into_iter().flat_map(|(from, to, weight)| {
            f(Edge::new(
                EdgeId::new_unchecked(NodeId::new_unchecked(from), NodeId::new_unchecked(to)),
                weight,
            ))
            .into_iter()
            .map(|edge| (edge.edge_id.from(), edge.edge_id.to(), edge.weight))
        });

        let mut adj_list = MatGraph::with_nodes(nodes.into_iter().map(|node| node.weight));
        adj_list.extend_edges(edges);

        adj_list
    }
}

impl<C, M: Matrix<W>, N, W: EdgeCost<Cost = C>, const DI: bool> Cost<C> for MatGraph<M, N, W, DI> {}

impl<F, M: Matrix<W>, N, W: EdgeFlow<Flow = F>, const DI: bool> Flow<F> for MatGraph<M, N, W, DI> {}

impl<B, M: Matrix<W>, N: NodeBalance<Balance = B>, W, const DI: bool> Balance<B>
    for MatGraph<M, N, W, DI>
{
}

impl<M: Matrix<W> + Clone + Debug, N: Num, W: Num, const DI: bool> Graph<N, W>
    for MatGraph<M, N, W, DI>
{
}

impl<M: Matrix<()> + Clone, N: Num, const DI: bool> WeightlessGraph<N> for MatGraph<M, N, (), DI> {}

#[cfg(test)]
mod test {
    extern crate test;
    use super::{CsrMatGraph, DenseMatGraph, SparseMatGraph};
    use crate::test::*;

    // Dense

    #[test]
    pub fn dense_mat_create_with_nodes() {
        graph_create_with_nodes::<DenseMatGraph<_, _>>()
    }

    #[test]
    pub fn dense_mat_create_with_capacity() {
        graph_create_with_capacity::<DenseMatGraph<_, _>>()
    }

    #[test]
    pub fn dense_mat_insert_and_contains() {
        graph_insert_and_contains::<DenseMatGraph<_, _>>()
    }

    #[test]
    pub fn dense_mat_clear() {
        graph_clear::<DenseMatGraph<_, _>>()
    }

    #[test]
    pub fn dense_mat_get() {
        graph_get::<DenseMatGraph<_, _>>()
    }

    #[test]
    pub fn dense_mat_index() {
        graph_index::<DenseMatGraph<_, _>>()
    }

    #[test]
    pub fn dense_mat_index_adjacent() {
        graph_index_adjacent::<DenseMatGraph<_, _>>()
    }

    // Sparse

    #[test]
    pub fn sparse_mat_create_with_nodes() {
        graph_create_with_nodes::<SparseMatGraph<_, _>>()
    }

    #[test]
    pub fn sparse_mat_create_with_capacity() {
        graph_create_with_capacity::<SparseMatGraph<_, _>>()
    }

    #[test]
    pub fn sparse_mat_insert_and_contains() {
        graph_insert_and_contains::<SparseMatGraph<_, _>>()
    }

    #[test]
    pub fn sparse_mat_clear() {
        graph_clear::<SparseMatGraph<_, _>>()
    }

    #[test]
    pub fn sparse_mat_get() {
        graph_get::<SparseMatGraph<_, _>>()
    }

    #[test]
    pub fn sparse_mat_index() {
        graph_index::<SparseMatGraph<_, _>>()
    }

    #[test]
    pub fn sparse_mat_index_adjacent() {
        graph_index_adjacent::<SparseMatGraph<_, _>>()
    }

    // Ellpack

    #[test]
    pub fn csr_mat_create_with_nodes() {
        graph_create_with_nodes::<CsrMatGraph<_, _>>()
    }

    #[test]
    pub fn csr_mat_create_with_capacity() {
        graph_create_with_capacity::<CsrMatGraph<_, _>>()
    }

    #[test]
    pub fn csr_mat_insert_and_contains() {
        graph_insert_and_contains::<CsrMatGraph<_, _>>()
    }

    #[test]
    pub fn csr_mat_clear() {
        graph_clear::<CsrMatGraph<_, _>>()
    }

    #[test]
    pub fn csr_mat_get() {
        graph_get::<CsrMatGraph<_, _>>()
    }

    #[test]
    pub fn csr_mat_index() {
        graph_index::<CsrMatGraph<_, _>>()
    }

    #[test]
    pub fn csr_mat_index_adjacent() {
        graph_index_adjacent::<CsrMatGraph<_, _>>()
    }
}
