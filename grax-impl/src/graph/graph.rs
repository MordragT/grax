use std::fmt::Debug;
use std::marker::PhantomData;

use bitvec::bitvec;
use bitvec::vec::BitVec;
use grax_core::{collections::*, edge::*, graph::*, node::*, prelude::*};

use crate::edges::boolean::EdgeBoolVec;
use crate::edges::fixed::FixedEdgeVec;
use crate::edges::EdgeStorage;
use crate::edges::{adj::AdjacencyList, csr::CsrMatrix, hash::HashStorage, mat::AdjacencyMatrix};
use crate::nodes::{FixedNodeVec, StableNodeVec, UnstableNodeVec};
use crate::nodes::{NodeBoolVec, NodeStorage};

pub type MatGraph<N, W, const DI: bool = false> =
    Graph<UnstableNodeVec<N>, AdjacencyMatrix<W>, N, W, DI>;

pub type CsrGraph<N, W, const DI: bool = false> = Graph<UnstableNodeVec<N>, CsrMatrix<W>, N, W, DI>;

pub type AdjGraph<N, W, const DI: bool = false> =
    Graph<UnstableNodeVec<N>, AdjacencyList<W>, N, W, DI>;

pub type HashGraph<N, W, const DI: bool = false> =
    Graph<UnstableNodeVec<N>, HashStorage<W>, N, W, DI>;

pub type StableMatGraph<N, W, const DI: bool = false> =
    Graph<StableNodeVec<N>, AdjacencyMatrix<W>, N, W, DI>;

pub type StableCsrGraph<N, W, const DI: bool = false> =
    Graph<StableNodeVec<N>, CsrMatrix<W>, N, W, DI>;

pub type StableAdjGraph<N, W, const DI: bool = false> =
    Graph<StableNodeVec<N>, AdjacencyList<W>, N, W, DI>;

pub type StableHashGraph<N, W, const DI: bool = false> =
    Graph<StableNodeVec<N>, HashStorage<W>, N, W, DI>;

#[derive(Debug, Clone)]
pub struct Graph<NS, ES, N: Debug, W: Debug, const DI: bool = false> {
    pub(crate) nodes: NS,
    pub(crate) edges: ES,
    pub(crate) edge_weight: PhantomData<W>,
    pub(crate) node_weight: PhantomData<N>,
}

impl<
        NS: NodeStorage<usize, usize>,
        ES: EdgeStorage<usize, W>,
        W: Debug + Clone,
        const DI: bool,
    > Graph<NS, ES, usize, W, DI>
{
    pub fn with_edges(
        edges: impl IntoIterator<Item = (usize, usize, W)>,
        node_count: usize,
    ) -> Self {
        let mut graph = Self::with_nodes(node_count, 0..node_count);
        graph.extend_edges(edges.into_iter().map(|(from, to, weight)| {
            (
                NodeId::new_unchecked(from),
                NodeId::new_unchecked(to),
                weight,
            )
        }));

        graph
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool> Clear
    for Graph<NS, ES, N, W, DI>
{
    fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    fn clear_edges(&mut self) {
        self.edges.clear();
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    Create for Graph<NS, ES, N, W, DI>
{
    fn with_capacity(node_count: usize, edge_count: usize) -> Self {
        let edges = ES::with_capacity(node_count, edge_count);
        let nodes = NS::with_capacity(node_count);

        Self {
            nodes,
            edges,
            edge_weight: PhantomData,
            node_weight: PhantomData,
        }
    }

    fn with_nodes(node_count: usize, nodes: impl IntoIterator<Item = N>) -> Self {
        let nodes = NS::with_nodes(node_count, nodes);
        let mut edges = ES::with_capacity(node_count, node_count * 2);
        edges.allocate(node_count);

        Self {
            nodes,
            edges,
            edge_weight: PhantomData,
            node_weight: PhantomData,
        }
    }

    fn new() -> Self {
        Self {
            nodes: NS::new(),
            edges: ES::new(),
            edge_weight: PhantomData,
            node_weight: PhantomData,
        }
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    Directed for Graph<NS, ES, N, W, DI>
{
    fn directed() -> bool {
        DI
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool> Keyed
    for Graph<NS, ES, N, W, DI>
{
    type Key = usize;
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    EdgeCollection for Graph<NS, ES, N, W, DI>
{
    type EdgeWeight = W;

    fn edges_capacity(&self) -> usize {
        self.edges.edges_capacity()
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    NodeCollection for Graph<NS, ES, N, W, DI>
{
    type NodeWeight = N;

    fn nodes_capacity(&self) -> usize {
        self.nodes.nodes_capacity()
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    NodeCount for Graph<NS, ES, N, W, DI>
{
    fn node_count(&self) -> usize {
        self.nodes.node_count()
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    EdgeCount for Graph<NS, ES, N, W, DI>
{
    fn edge_count(&self) -> usize {
        self.edges.edge_count()
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    GetNode for Graph<NS, ES, N, W, DI>
{
    fn node(&self, node_id: NodeId<Self::Key>) -> Option<NodeRef<Self::Key, Self::NodeWeight>> {
        self.nodes.node(node_id)
    }

    // fn has_node_weight(&self, weight: &Self::NodeWeight) -> Option<NodeId<usize>> {
    //     self.nodes
    //         .iter()
    //         .find(|other| &other.weight == weight)
    //         .map(|node| node.node_id)
    // }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    GetEdge for Graph<NS, ES, N, W, DI>
{
    fn edge(&self, edge_id: EdgeId<Self::Key>) -> Option<EdgeRef<Self::Key, Self::EdgeWeight>> {
        self.edges.edge(edge_id)
    }

    // fn has_edge(
    //     &self,
    //     from: NodeId<Self::Key>,
    //     to: NodeId<Self::Key>,
    // ) -> Option<EdgeId<Self::Key>> {
    //     let edge_id = EdgeId::new_unchecked(from, to);
    //     if self.contains_edge_id(edge_id) {
    //         Some(edge_id)
    //     } else {
    //         None
    //     }
    // }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    GetNodeMut for Graph<NS, ES, N, W, DI>
{
    fn node_mut(
        &mut self,
        node_id: NodeId<Self::Key>,
    ) -> Option<NodeMut<Self::Key, Self::NodeWeight>> {
        self.nodes.node_mut(node_id)
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    GetEdgeMut for Graph<NS, ES, N, W, DI>
{
    fn edge_mut(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<EdgeMut<Self::Key, Self::EdgeWeight>> {
        self.edges.edge_mut(edge_id)
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    InsertNode for Graph<NS, ES, N, W, DI>
{
    fn insert_node(&mut self, weight: Self::NodeWeight) -> NodeId<Self::Key> {
        let node_id = self.nodes.insert_node(weight);
        self.edges.allocate(1);
        return node_id;
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.nodes.reserve_nodes(additional)
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    InsertEdge for Graph<NS, ES, N, W, DI>
{
    fn insert_edge(
        &mut self,
        from: NodeId<Self::Key>,
        to: NodeId<Self::Key>,
        weight: W,
    ) -> EdgeId<Self::Key> {
        self.edges.insert_edge(from, to, weight)
    }

    fn reserve_edges(&mut self, additional: usize) {
        self.edges.reserve_edges(additional)
    }

    fn extend_edges(
        &mut self,
        edges: impl IntoIterator<Item = (NodeId<Self::Key>, NodeId<Self::Key>, Self::EdgeWeight)>,
    ) {
        self.edges.extend_edges(edges)
    }
}
impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    RemoveNode for Graph<NS, ES, N, W, DI>
{
    fn remove_node(
        &mut self,
        node_id: NodeId<Self::Key>,
    ) -> Option<Node<Self::Key, Self::NodeWeight>> {
        self.nodes.remove_node(node_id)
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    RemoveEdge for Graph<NS, ES, N, W, DI>
{
    fn remove_edge(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<Edge<Self::Key, Self::EdgeWeight>> {
        self.edges.remove_edge(edge_id)
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    NodeIter for Graph<NS, ES, N, W, DI>
{
    type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Key>> + 'a
    where Self: 'a;

    type Nodes<'a> = impl Iterator<Item = NodeRef<'a, usize, N>> + 'a
    where
        N: 'a,
        Self: 'a;

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        self.nodes.node_ids()
    }

    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a> {
        self.nodes.iter_nodes()
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    EdgeIter for Graph<NS, ES, N, W, DI>
{
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Key>> + 'a
    where Self: 'a;

    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.edges.edge_ids()
    }

    fn iter_edges<'a>(&'a self) -> Self::Edges<'a> {
        self.edges.iter_edges()
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    NodeIterMut for Graph<NS, ES, N, W, DI>
{
    type NodesMut<'a> = impl Iterator<Item = NodeMut<'a, usize, N>> + 'a
    where
        N: 'a,
        Self: 'a;

    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a> {
        self.nodes.iter_nodes_mut()
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    EdgeIterMut for Graph<NS, ES, N, W, DI>
{
    type EdgesMut<'a> = impl Iterator<Item = EdgeMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_edges_mut<'a>(&'a mut self) -> Self::EdgesMut<'a> {
        self.edges.iter_edges_mut()
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    NodeIterAdjacent for Graph<NS, ES, N, W, DI>
{
    type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Key>> + 'a
    where Self: 'a;
    type Nodes<'a> = impl Iterator<Item = NodeRef<'a, usize, N>> + 'a
    where
        N: 'a,
        Self: 'a;

    fn adjacent_node_ids<'a>(&'a self, node_id: NodeId<Self::Key>) -> Self::NodeIds<'a> {
        self.edges
            .adjacent_edge_ids(node_id)
            .map(|edge_id| edge_id.to())
    }

    fn iter_adjacent_nodes<'a>(&'a self, node_id: NodeId<Self::Key>) -> Self::Nodes<'a> {
        self.adjacent_node_ids(node_id)
            .map(|node_id| self.node(node_id).unwrap())
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    EdgeIterAdjacent for Graph<NS, ES, N, W, DI>
{
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Key>> + 'a
    where Self: 'a;

    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: NodeId<Self::Key>) -> Self::EdgeIds<'a> {
        self.edges.adjacent_edge_ids(node_id)
    }

    fn iter_adjacent_edges<'a>(&'a self, node_id: NodeId<Self::Key>) -> Self::Edges<'a> {
        self.edges.iter_adjacent_edges(node_id)
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    NodeIterAdjacentMut for Graph<NS, ES, N, W, DI>
{
    type NodesMut<'a> = impl Iterator<Item = NodeMut<'a, usize, N>> + 'a
    where
        N: 'a,
        Self: 'a;
    fn iter_adjacent_nodes_mut<'a>(&'a mut self, node_id: NodeId<Self::Key>) -> Self::NodesMut<'a> {
        let indices = self.adjacent_node_ids(node_id).collect::<Vec<_>>();
        self.nodes.iter_indexed_nodes_mut(indices)
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    EdgeIterAdjacentMut for Graph<NS, ES, N, W, DI>
{
    type EdgesMut<'a> = impl Iterator<Item = EdgeMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_adjacent_edges_mut<'a>(&'a mut self, node_id: NodeId<Self::Key>) -> Self::EdgesMut<'a> {
        self.edges.iter_adjacent_edges_mut(node_id)
    }
}

// TODO replace HashStorage with Vec implementations

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    NodeAttribute for Graph<NS, ES, N, W, DI>
{
    type VisitNodeMap = NodeBoolVec;
    type FixedNodeMap<V: Debug + Clone> = FixedNodeVec<V>;
    type NodeMap<V: Debug + Clone> = StableNodeVec<V>;

    fn visit_node_map(&self) -> Self::VisitNodeMap {
        let buffer = vec![0_usize; self.node_count() / std::mem::size_of::<usize>() + 1];
        NodeBoolVec::new(BitVec::from_vec(buffer))
    }

    fn fixed_node_map<V: Debug + Clone>(&self, fill: V) -> Self::FixedNodeMap<V> {
        FixedNodeVec::new(vec![fill; self.node_count()])
    }

    fn node_map<V: Debug + Clone>(&self) -> Self::NodeMap<V> {
        StableNodeVec::with_capacity(self.node_count())
    }
}

impl<NS: NodeStorage<usize, N>, ES: EdgeStorage<usize, W>, N: Debug, W: Debug, const DI: bool>
    EdgeAttribute for Graph<NS, ES, N, W, DI>
{
    type VisitEdgeMap = EdgeBoolVec;
    type FixedEdgeMap<V: Debug + Clone> = FixedEdgeVec<V>;
    type EdgeMap<V: Debug + Clone> = HashStorage<V>;

    fn visit_edge_map(&self) -> Self::VisitEdgeMap {
        let buffer = vec![0_usize; self.edge_count() / std::mem::size_of::<usize>() + 1];
        EdgeBoolVec::new(BitVec::from_vec(buffer), self.node_count())
    }

    fn fixed_edge_map<V: Debug + Clone>(&self, fill: V) -> Self::FixedEdgeMap<V> {
        FixedEdgeVec::new(vec![fill; self.edge_count()], self.node_count())
    }

    fn edge_map<V: Debug + Clone>(&self) -> Self::EdgeMap<V> {
        HashStorage::with_capacity(self.node_count(), self.edge_count())
    }
}

impl<
        NS: NodeStorage<usize, N>,
        M1: EdgeStorage<usize, W1>,
        M2: EdgeStorage<usize, W2>,
        N: Debug,
        W1: Debug,
        W2: Clone + Debug,
        const DI: bool,
    > AdaptEdge<Graph<NS, M2, N, W2, DI>, W2> for Graph<NS, M1, N, W1, DI>
{
    fn map_edge<F>(self, f: F) -> Graph<NS, M2, N, W2, DI>
    where
        F: Fn(Edge<Self::Key, Self::EdgeWeight>) -> Edge<Self::Key, W2>,
    {
        let node_count = self.node_count();

        let Self {
            nodes,
            edges,
            edge_weight: _,
            node_weight: _,
        } = self;

        let edges = edges.into_iter().map(|edge| {
            let edge = f(edge);
            (edge.edge_id.from(), edge.edge_id.to(), edge.weight)
        });

        let mut graph = Graph::with_nodes(node_count, nodes.into_iter().map(|node| node.weight));
        graph.extend_edges(edges);

        graph
    }

    fn split_map_edge<F>(self, f: F) -> Graph<NS, M2, N, W2, DI>
    where
        F: Fn(Edge<Self::Key, Self::EdgeWeight>) -> Vec<Edge<Self::Key, W2>>,
    {
        let node_count = self.node_count();

        let Self {
            nodes,
            edges,
            edge_weight: _,
            node_weight: _,
        } = self;

        let edges = edges.into_iter().flat_map(|edge| {
            f(edge)
                .into_iter()
                .map(|edge| (edge.edge_id.from(), edge.edge_id.to(), edge.weight))
        });

        let mut graph = Graph::with_nodes(node_count, nodes.into_iter().map(|node| node.weight));
        graph.extend_edges(edges);

        graph
    }
}

impl<
        C,
        NS: NodeStorage<usize, N>,
        ES: EdgeStorage<usize, W>,
        N: Debug,
        W: EdgeCost<Cost = C>,
        const DI: bool,
    > Cost<C> for Graph<NS, ES, N, W, DI>
{
}

impl<
        F,
        NS: NodeStorage<usize, N>,
        ES: EdgeStorage<usize, W>,
        N: Debug,
        W: EdgeFlow<Flow = F>,
        const DI: bool,
    > Flow<F> for Graph<NS, ES, N, W, DI>
{
}

impl<
        B,
        NS: NodeStorage<usize, N>,
        ES: EdgeStorage<usize, W>,
        N: NodeBalance<Balance = B> + Debug,
        W: Debug,
        const DI: bool,
    > Balance<B> for Graph<NS, ES, N, W, DI>
{
}

impl<
        NS: NodeStorage<usize, N>,
        ES: EdgeStorage<usize, W>,
        N: Debug + Clone,
        W: Debug + Clone,
        const DI: bool,
    > ImGraph for Graph<NS, ES, N, W, DI>
{
}

impl<
        NS: NodeStorage<usize, N>,
        ES: EdgeStorage<usize, W>,
        N: Debug + Clone,
        W: Debug + Clone,
        const DI: bool,
    > MutGraph for Graph<NS, ES, N, W, DI>
{
}

#[cfg(test)]
mod test {
    use super::{AdjGraph, CsrGraph, HashGraph, MatGraph};
    use crate::graph::test::*;

    // adj

    #[test]
    pub fn adj_graph_create_with_nodes() {
        graph_create_with_nodes::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_graph_create_with_capacity() {
        graph_create_with_capacity::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_graph_insert_and_contains() {
        graph_insert_and_contains::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_graph_clear() {
        graph_clear::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_graph_get() {
        graph_get::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_graph_index() {
        graph_index::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_graph_index_adjacent() {
        graph_index_adjacent::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_graph_iter_adjacent() {
        graph_iter_adjacent::<AdjGraph<_, _>>()
    }

    // hash

    #[test]
    pub fn hash_graph_create_with_nodes() {
        graph_create_with_nodes::<HashGraph<_, _>>()
    }

    #[test]
    pub fn hash_graph_create_with_capacity() {
        graph_create_with_capacity::<HashGraph<_, _>>()
    }

    #[test]
    pub fn hash_graph_insert_and_contains() {
        graph_insert_and_contains::<HashGraph<_, _>>()
    }

    #[test]
    pub fn hash_graph_clear() {
        graph_clear::<HashGraph<_, _>>()
    }

    #[test]
    pub fn hash_graph_get() {
        graph_get::<HashGraph<_, _>>()
    }

    #[test]
    pub fn hash_graph_index() {
        graph_index::<HashGraph<_, _>>()
    }

    #[test]
    pub fn hash_graph_index_adjacent() {
        graph_index_adjacent::<HashGraph<_, _>>()
    }

    #[test]
    pub fn hash_graph_iter_adjacent() {
        graph_iter_adjacent::<HashGraph<_, _>>()
    }

    // dense

    #[test]
    pub fn dense_graph_create_with_nodes() {
        graph_create_with_nodes::<MatGraph<_, _>>()
    }

    #[test]
    pub fn dense_graph_create_with_capacity() {
        graph_create_with_capacity::<MatGraph<_, _>>()
    }

    #[test]
    pub fn dense_graph_insert_and_contains() {
        graph_insert_and_contains::<MatGraph<_, _>>()
    }

    #[test]
    pub fn dense_graph_clear() {
        graph_clear::<MatGraph<_, _>>()
    }

    #[test]
    pub fn dense_graph_get() {
        graph_get::<MatGraph<_, _>>()
    }

    #[test]
    pub fn dense_graph_index() {
        graph_index::<MatGraph<_, _>>()
    }

    #[test]
    pub fn dense_graph_index_adjacent() {
        graph_index_adjacent::<MatGraph<_, _>>()
    }

    #[test]
    pub fn dense_graph_iter_adjacent() {
        graph_iter_adjacent::<MatGraph<_, _>>()
    }

    // csr

    #[test]
    pub fn csr_graph_create_with_nodes() {
        graph_create_with_nodes::<CsrGraph<_, _>>()
    }

    #[test]
    pub fn csr_graph_create_with_capacity() {
        graph_create_with_capacity::<CsrGraph<_, _>>()
    }

    #[test]
    pub fn csr_graph_insert_and_contains() {
        graph_insert_and_contains::<CsrGraph<_, _>>()
    }

    #[test]
    pub fn csr_graph_clear() {
        graph_clear::<CsrGraph<_, _>>()
    }

    #[test]
    pub fn csr_graph_get() {
        graph_get::<CsrGraph<_, _>>()
    }

    #[test]
    pub fn csr_graph_index() {
        graph_index::<CsrGraph<_, _>>()
    }

    #[test]
    pub fn csr_graph_iter_adjacent() {
        graph_iter_adjacent::<CsrGraph<_, _>>()
    }

    #[test]
    pub fn csr_graph_index_adjacent() {
        graph_index_adjacent::<CsrGraph<_, _>>()
    }
}
