use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

use grax_core::edge::*;
use grax_core::node::*;
use grax_core::prelude::*;
use grax_core::traits::*;
use grax_core::weight::Num;

use crate::edge_list::EdgeList;
use crate::storage::EdgeStorage;
use crate::storage::{
    adj::AdjacencyList, csr::CsrMatrix, dense::DenseMatrix, hash::HashStorage, sparse::SparseMatrix,
};
use attr::{AttrHashMap, AttrVec};

pub mod attr;
#[cfg(test)]
mod test;

type RawNodeId = NodeId<usize>;
type RawEdgeId = EdgeId<usize>;

pub type SparseGraph<N, W, const DI: bool = false> = Graph<SparseMatrix<W>, N, W, DI>;

pub type DenseGraph<N, W, const DI: bool = false> = Graph<DenseMatrix<W>, N, W, DI>;

pub type CsrGraph<N, W, const DI: bool = false> = Graph<CsrMatrix<W>, N, W, DI>;

pub type AdjGraph<N, W, const DI: bool = false> = Graph<AdjacencyList<W>, N, W, DI>;

pub type HashGraph<N, W, const DI: bool = false> = Graph<HashStorage<W>, N, W, DI>;

#[derive(Debug, Clone)]
pub struct Graph<S, N, W, const DI: bool = false> {
    pub(crate) nodes: Vec<Node<usize, N>>,
    pub(crate) edges: S,
    weight: PhantomData<W>,
}

impl<S: EdgeStorage<W>, W: Clone, const DI: bool> Graph<S, usize, W, DI> {
    pub fn with_edges(
        edges: impl IntoIterator<Item = (usize, usize, W)>,
        node_count: usize,
    ) -> Self {
        let mut graph = Self::with_nodes(0..node_count);
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

impl<S: EdgeStorage<W>, N, W: Copy, const DI: bool> From<EdgeList<N, W, DI>>
    for Graph<S, N, W, DI>
{
    fn from(edge_list: EdgeList<N, W, DI>) -> Self {
        let EdgeList {
            nodes,
            edges,
            node_count: _,
        } = edge_list;

        let mut graph = Self::with_nodes(nodes.into_iter());

        for (from, to, weight) in edges.into_iter() {
            let from = NodeId::new_unchecked(from);
            let to = NodeId::new_unchecked(to);

            if !DI {
                graph.insert_edge(to, from, weight);
            }

            graph.insert_edge(from, to, weight);
        }

        graph
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Base for Graph<S, N, W, DI> {
    type Id = usize;
    type NodeWeight = N;
    type EdgeWeight = W;
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Capacity for Graph<S, N, W, DI> {
    fn edges_capacity(&self) -> usize {
        self.edges.capacity()
    }

    fn nodes_capacity(&self) -> usize {
        self.nodes.capacity()
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Clear for Graph<S, N, W, DI> {
    fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    fn clear_edges(&mut self) {
        self.edges.clear();
    }
}

impl<S: EdgeStorage<W>, N: PartialEq, W, const DI: bool> Contains for Graph<S, N, W, DI> {
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

impl<S: EdgeStorage<W>, N, W, const DI: bool> Count for Graph<S, N, W, DI> {
    fn edge_count(&self) -> usize {
        self.edges.count()
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Create for Graph<S, N, W, DI> {
    fn with_capacity(node_count: usize, edge_count: usize) -> Self {
        let edges = S::with_capacity(edge_count);
        let nodes = Vec::with_capacity(node_count);

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
        let mut edges = S::with_capacity(node_count * 2);

        edges.allocate(node_count);

        Self {
            nodes,
            edges,
            weight: PhantomData,
        }
    }

    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: S::new(),
            weight: PhantomData,
        }
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Directed for Graph<S, N, W, DI> {
    fn directed() -> bool {
        DI
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Extend for Graph<S, N, W, DI> {
    fn extend_edges(&mut self, edges: impl IntoIterator<Item = (RawNodeId, RawNodeId, W)>) {
        self.edges.extend(
            edges
                .into_iter()
                .map(|(from, to, weight)| (from.raw(), to.raw(), weight)),
        )
    }

    fn extend_nodes(&mut self, nodes: impl IntoIterator<Item = N>) {
        for node in nodes {
            self.insert_node(node);
        }
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Get for Graph<S, N, W, DI> {
    fn node(&self, node_id: NodeId<Self::Id>) -> Option<NodeRef<Self::Id, Self::NodeWeight>> {
        self.nodes.get(node_id.raw()).map(Into::into)
    }

    fn edge(&self, edge_id: EdgeId<Self::Id>) -> Option<EdgeRef<Self::Id, Self::EdgeWeight>> {
        self.edges.get(edge_id.from().raw(), edge_id.to().raw())
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> GetMut for Graph<S, N, W, DI> {
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
        self.edges.get_mut(edge_id.from().raw(), edge_id.to().raw())
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Insert for Graph<S, N, W, DI> {
    fn insert_node(&mut self, node: N) -> RawNodeId {
        let node_id = RawNodeId::new_unchecked(self.nodes.len());
        self.nodes.push(Node::new(node_id, node));
        self.edges.allocate(1);
        return node_id;
    }
    fn insert_edge(&mut self, from: RawNodeId, to: RawNodeId, weight: W) -> RawEdgeId {
        self.edges.insert(from.raw(), to.raw(), weight)
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Index for Graph<S, N, W, DI> {
    type EdgeIds<'a> = impl Iterator<Item = RawEdgeId> + 'a
    where Self: 'a;
    type NodeIds<'a> = impl Iterator<Item = RawNodeId> + 'a
    where Self: 'a;

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.edges.indices()
    }

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        (0..self.nodes.len()).map(RawNodeId::new_unchecked)
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> IndexAdjacent for Graph<S, N, W, DI> {
    type EdgeIds<'a> = impl Iterator<Item = RawEdgeId> + 'a
    where Self: 'a;
    type NodeIds<'a> = impl Iterator<Item = RawNodeId> + 'a
    where Self: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: RawNodeId) -> Self::EdgeIds<'a> {
        self.edges
            .iter_adjacent_unstable(node_id.raw())
            .map(|edge| edge.edge_id)
    }
    fn adjacent_node_ids<'a>(&'a self, node_id: RawNodeId) -> Self::NodeIds<'a> {
        self.edges
            .iter_adjacent_unstable(node_id.raw())
            .map(|edge| edge.edge_id.to())
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Iter for Graph<S, N, W, DI> {
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
        self.edges.iter_unstable()
    }
}
impl<S: EdgeStorage<W>, N, W, const DI: bool> IterMut for Graph<S, N, W, DI> {
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
        self.edges.iter_mut_unstable()
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> IterAdjacent for Graph<S, N, W, DI> {
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
        self.edges.iter_adjacent_unstable(node_id.raw())
    }
}
impl<S: EdgeStorage<W>, N, W, const DI: bool> IterAdjacentMut for Graph<S, N, W, DI> {
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
        self.edges.iter_adjacent_mut_unstable(node_id.raw())
    }
}

impl<S: EdgeStorage<W>, N: Default, W, const DI: bool> Remove for Graph<S, N, W, DI> {
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
        self.edges.remove(edge_id.from().raw(), edge_id.to().raw())
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Reserve for Graph<S, N, W, DI> {
    fn reserve_edges(&mut self, additional: usize) {
        todo!()
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.nodes.reserve(additional)
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Visitable for Graph<S, N, W, DI> {
    type VisitMap = AttrVec<bool>;

    fn visit_map(&self) -> Self::VisitMap {
        AttrVec(vec![false; self.node_count()])
    }
}

impl<S: EdgeStorage<W>, N, W, const DI: bool> Viewable for Graph<S, N, W, DI> {
    type NodeMap<Attr: Clone + Default + Debug> = AttrVec<Attr>;
    type EdgeMap<Attr: Clone + Default + Debug> = AttrHashMap<EdgeId<Self::Id>, Attr>;

    fn node_map<Attr: Clone + Default + Debug>(&self) -> Self::NodeMap<Attr> {
        AttrVec(vec![Attr::default(); self.node_count()])
    }

    fn edge_map<Attr: Clone + Default + Debug>(&self) -> Self::EdgeMap<Attr> {
        let mut map = HashMap::with_capacity(self.edge_count());
        std::iter::Extend::extend(
            &mut map,
            self.edge_ids().map(|edge_id| (edge_id, Attr::default())),
        );
        AttrHashMap(map)
    }
}

impl<M1: EdgeStorage<W1>, M2: EdgeStorage<W2>, N, W1, W2: Clone, const DI: bool>
    AdaptEdge<Graph<M2, N, W2, DI>, W2> for Graph<M1, N, W1, DI>
{
    fn map_edge<F>(self, f: F) -> Graph<M2, N, W2, DI>
    where
        F: Fn(Edge<Self::Id, Self::EdgeWeight>) -> Edge<Self::Id, W2>,
    {
        let Self {
            nodes,
            edges,
            weight: _,
        } = self;

        let edges = edges.into_iter_unstable().map(|edge| {
            let edge = f(edge);
            (edge.edge_id.from(), edge.edge_id.to(), edge.weight)
        });

        let mut adj_list = Graph::with_nodes(nodes.into_iter().map(|node| node.weight));
        adj_list.extend_edges(edges);

        adj_list
    }

    fn split_map_edge<F>(self, f: F) -> Graph<M2, N, W2, DI>
    where
        F: Fn(Edge<Self::Id, Self::EdgeWeight>) -> Vec<Edge<Self::Id, W2>>,
    {
        let Self {
            nodes,
            edges,
            weight: _,
        } = self;

        let edges = edges.into_iter_unstable().flat_map(|edge| {
            f(edge)
                .into_iter()
                .map(|edge| (edge.edge_id.from(), edge.edge_id.to(), edge.weight))
        });

        let mut adj_list = Graph::with_nodes(nodes.into_iter().map(|node| node.weight));
        adj_list.extend_edges(edges);

        adj_list
    }
}

impl<C, S: EdgeStorage<W>, N, W: EdgeCost<Cost = C>, const DI: bool> Cost<C>
    for Graph<S, N, W, DI>
{
}

impl<F, S: EdgeStorage<W>, N, W: EdgeFlow<Flow = F>, const DI: bool> Flow<F>
    for Graph<S, N, W, DI>
{
}

impl<B, S: EdgeStorage<W>, N: NodeBalance<Balance = B>, W, const DI: bool> Balance<B>
    for Graph<S, N, W, DI>
{
}

impl<S: EdgeStorage<W> + Clone + Debug, N: Num, W: Num, const DI: bool> grax_core::Graph<N, W>
    for Graph<S, N, W, DI>
{
}

impl<S: EdgeStorage<()> + Clone, N: Num, const DI: bool> WeightlessGraph<N>
    for Graph<S, N, (), DI>
{
}
