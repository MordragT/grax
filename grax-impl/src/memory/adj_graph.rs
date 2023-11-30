use grax_core::edge::*;
use grax_core::node::NodeBalance;
use grax_core::prelude::*;
use grax_core::traits::*;
use grax_core::weight::Num;
use std::collections::HashMap;
use std::fmt::Debug;

use super::attr::{AttrHashMap, AttrVec};
use crate::edge_list::EdgeList;

type RawNodeId = NodeId<usize>;
type RawEdgeId = EdgeId<usize>;

#[derive(Debug, Clone)]
pub struct AdjGraph<N, W, const DI: bool = false> {
    pub(crate) nodes: Vec<Node<usize, N>>,
    pub(crate) edges: Vec<Vec<Edge<usize, W>>>,
}

impl<W: Clone, const DI: bool> AdjGraph<usize, W, DI> {
    pub fn with_edges(
        edges: impl IntoIterator<Item = (usize, usize, W)>,
        node_count: usize,
    ) -> Self {
        let mut adj_list = Self::with_nodes(0..node_count);
        adj_list.extend_edges(edges.into_iter().map(|(from, to, weight)| {
            (
                NodeId::new_unchecked(from),
                NodeId::new_unchecked(to),
                weight,
            )
        }));

        adj_list
    }
}

impl<N, W: Copy, const DI: bool> From<EdgeList<N, W, DI>> for AdjGraph<N, W, DI> {
    fn from(edge_list: EdgeList<N, W, DI>) -> Self {
        let EdgeList {
            nodes,
            edges,
            node_count: _,
        } = edge_list;

        let mut adj_list = Self::with_nodes(nodes.into_iter());

        for (from, to, weight) in edges.into_iter() {
            let from = NodeId::new_unchecked(from);
            let to = NodeId::new_unchecked(to);

            if !DI {
                adj_list.insert_edge(to, from, weight);
            }

            adj_list.insert_edge(from, to, weight);
        }

        adj_list
    }
}

impl<N, W, const DI: bool> Base for AdjGraph<N, W, DI> {
    type Id = usize;
    type NodeWeight = N;
    type EdgeWeight = W;
}

// impl<N, W, const DI: bool> Ref for AdjacencyList<N, W, DI> {
//     type GraphRef<'a> = AdjacencyList<&'a N, &'a W, DI>
//     where
//         N: 'a,
//         W: 'a;
// }

impl<N, W, const DI: bool> Capacity for AdjGraph<N, W, DI> {
    fn edges_capacity(&self) -> usize {
        self.edges
            .first()
            .map(|first| first.capacity())
            .unwrap_or_default()
    }

    fn nodes_capacity(&self) -> usize {
        self.nodes.capacity()
    }
}

impl<N, W, const DI: bool> Clear for AdjGraph<N, W, DI> {
    fn clear(&mut self) {
        self.nodes.clear();
        self.clear_edges();
    }

    fn clear_edges(&mut self) {
        for adj in &mut self.edges {
            adj.clear();
        }
    }
}

impl<N: PartialEq, W, const DI: bool> Contains for AdjGraph<N, W, DI> {
    fn contains_node(&self, node: &N) -> Option<RawNodeId> {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_i, other)| &other.weight == node)
            .map(|(id, _)| RawNodeId::new_unchecked(id))
    }

    fn contains_edge(&self, from: RawNodeId, to: RawNodeId) -> Option<RawEdgeId> {
        self.edges.get(from.raw()).and_then(|adj| {
            adj.iter().find_map(|edge| {
                if edge.to() == to {
                    Some(edge.edge_id)
                } else {
                    None
                }
            })
        })
    }
}

impl<N, W, const DI: bool> Count for AdjGraph<N, W, DI> {
    fn edge_count(&self) -> usize {
        let count = self.edges.iter().fold(0, |mut akku, edges| {
            akku += edges.len();
            akku
        });

        // if DI {
        //     count
        // } else {
        //     count / 2
        // }
        count
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl<N, W: Clone, const DI: bool> Create for AdjGraph<N, W, DI> {
    fn with_capacity(nodes: usize, edges: usize) -> Self {
        let nodes = Vec::with_capacity(nodes);
        let edges = Vec::with_capacity(nodes.len());

        Self { nodes, edges }
    }

    fn with_nodes(nodes: impl IntoIterator<Item = N>) -> Self {
        let nodes = nodes
            .into_iter()
            .enumerate()
            .map(|(id, weight)| Node::new(RawNodeId::new_unchecked(id), weight))
            .collect::<Vec<_>>();
        let edges = vec![Vec::new(); nodes.len()];

        Self { nodes, edges }
    }

    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl<N, W, const DI: bool> Directed for AdjGraph<N, W, DI> {
    fn directed() -> bool {
        DI
    }
}

impl<N, W, const DI: bool> Extend for AdjGraph<N, W, DI> {
    fn extend_edges(&mut self, edges: impl IntoIterator<Item = (RawNodeId, RawNodeId, W)>) {
        for (from, to, weight) in edges {
            self.insert_edge(from, to, weight);
        }
    }

    fn extend_nodes(&mut self, nodes: impl IntoIterator<Item = N>) {
        for node in nodes {
            self.insert_node(node);
        }
    }
}

impl<N, W, const DI: bool> Get for AdjGraph<N, W, DI> {
    // fn node(&self, node_id: RawNodeId) -> Option<&N> {
    // }

    fn node(&self, node_id: NodeId<Self::Id>) -> Option<NodeRef<Self::Id, Self::NodeWeight>> {
        self.nodes.get(node_id.raw()).map(Into::into)
    }

    fn edge(&self, edge_id: EdgeId<Self::Id>) -> Option<EdgeRef<Self::Id, Self::EdgeWeight>> {
        self.edges.get(edge_id.from().raw()).and_then(|adj| {
            adj.iter().find_map(|edge| {
                if edge.to() == edge_id.to() {
                    Some(edge.into())
                } else {
                    None
                }
            })
        })
    }
}

impl<N, W, const DI: bool> GetMut for AdjGraph<N, W, DI> {
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
        self.edges.get_mut(edge_id.from().raw()).and_then(|adj| {
            adj.iter_mut().find_map(|edge| {
                if edge.to() == edge_id.to() {
                    Some(edge.into())
                } else {
                    None
                }
            })
        })
    }
}

impl<N, W, const DI: bool> Index for AdjGraph<N, W, DI> {
    type EdgeIds<'a> = impl Iterator<Item = RawEdgeId> + 'a
    where Self: 'a;
    type NodeIds<'a> = impl Iterator<Item = RawNodeId> + 'a
    where Self: 'a;

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.edges
            .iter()
            .map(|adj| adj.iter().map(|edge| edge.edge_id))
            .flatten()
    }

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        (0..self.nodes.len()).map(RawNodeId::new_unchecked)
    }
}

impl<N, W, const DI: bool> IndexAdjacent for AdjGraph<N, W, DI> {
    type EdgeIds<'a> = impl Iterator<Item = RawEdgeId> + 'a
    where Self: 'a;
    type NodeIds<'a> = impl Iterator<Item = RawNodeId> + 'a
    where Self: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: RawNodeId) -> Self::EdgeIds<'a> {
        self.edges
            .get(node_id.raw())
            .map(|adj| adj.iter().map(|edge| edge.edge_id))
            .into_iter()
            .flatten()
    }
    fn adjacent_node_ids<'a>(&'a self, node_id: RawNodeId) -> Self::NodeIds<'a> {
        self.edges
            .get(node_id.raw())
            .map(|adj| adj.iter().map(|edge| edge.to()))
            .into_iter()
            .flatten()
    }
}

impl<N, W, const DI: bool> Iter for AdjGraph<N, W, DI> {
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
        self.edges
            .iter()
            .map(|adj| adj.iter())
            .flatten()
            .map(Into::into)
    }
}
impl<N, W, const DI: bool> IterMut for AdjGraph<N, W, DI> {
    type NodesMut<'a> =  impl Iterator<Item = NodeRefMut<'a, usize, N>> + 'a
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
        self.edges
            .iter_mut()
            .map(|adj| adj.iter_mut())
            .flatten()
            .map(Into::into)
    }
}

impl<N, W, const DI: bool> IterAdjacent for AdjGraph<N, W, DI> {
    type Nodes<'a> =  impl Iterator<Item = NodeRef<'a, usize, N>> + 'a
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
        self.edges
            .get(node_id.raw())
            .map(|adj| adj.iter().map(Into::into))
            .into_iter()
            .flatten()
    }
}
impl<N, W, const DI: bool> IterAdjacentMut for AdjGraph<N, W, DI> {
    type NodesMut<'a> =  impl Iterator<Item = NodeRefMut<'a, usize, N>> + 'a
    where
        N: 'a,
        Self: 'a;

    type EdgesMut<'a> = impl Iterator<Item = EdgeRefMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn iter_adjacent_nodes_mut<'a>(&'a mut self, node_id: RawNodeId) -> Self::NodesMut<'a> {
        let ids = self.adjacent_node_ids(node_id).collect::<Vec<_>>();
        self.iter_nodes_mut()
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
            .get_mut(node_id.raw())
            .map(|adj| adj.iter_mut().map(Into::into))
            .into_iter()
            .flatten()
    }
}

impl<N, W, const DI: bool> Insert for AdjGraph<N, W, DI> {
    fn insert_node(&mut self, node: N) -> RawNodeId {
        let node_id = RawNodeId::new_unchecked(self.nodes.len());
        self.nodes.push(Node::new(node_id, node));
        self.edges.push(Vec::new());
        node_id
    }

    // TODO undirected ??
    fn insert_edge(&mut self, from: RawNodeId, to: RawNodeId, weight: W) -> RawEdgeId {
        let edge_id = RawEdgeId::new_unchecked(from, to);
        let edge = Edge::new(edge_id, weight);
        self.edges[from.raw()].push(edge);
        edge_id
    }
}

impl<N: Default, W, const DI: bool> Remove for AdjGraph<N, W, DI> {
    fn remove_node(
        &mut self,
        node_id: NodeId<Self::Id>,
    ) -> Option<Node<Self::Id, Self::NodeWeight>> {
        if let Some(adj) = self.edges.get_mut(node_id.raw()) {
            adj.clear();
        }

        for adj in self.edges.iter_mut() {
            adj.retain(|edge| edge.to() != node_id);
        }

        // TODO replace with Option::None
        if let Some(node) = self.nodes.get_mut(node_id.raw()) {
            let weight = std::mem::replace(&mut node.weight, N::default());
            Some(Node::new(node_id, weight))
        } else {
            None
        }
    }

    fn remove_edge(
        &mut self,
        edge_id: EdgeId<Self::Id>,
    ) -> Option<Edge<Self::Id, Self::EdgeWeight>> {
        todo!()
    }
}

impl<N, W, const DI: bool> Reserve for AdjGraph<N, W, DI> {
    fn reserve_edges(&mut self, additional: usize) {
        todo!()
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.nodes.reserve(additional)
    }
}

impl<N, W, const DI: bool> Visitable for AdjGraph<N, W, DI> {
    type VisitMap = AttrVec<bool>;

    fn visit_map(&self) -> Self::VisitMap {
        AttrVec(vec![false; self.node_count()])
    }
}

impl<N, W, const DI: bool> Viewable for AdjGraph<N, W, DI> {
    type NodeMap<Attr: Clone + Default + Debug> = AttrVec<Attr>;
    type EdgeMap<Attr: Clone + Default + Debug> = AttrHashMap<EdgeId<Self::Id>, Attr>;

    fn node_map<Attr: Clone + Default + Debug>(&self) -> Self::NodeMap<Attr> {
        AttrVec(vec![Attr::default(); self.node_count()])
    }

    fn edge_map<Attr: Clone + Default + Debug>(&self) -> Self::EdgeMap<Attr> {
        let mut map = HashMap::with_capacity(self.edge_count());
        map.extend(self.edge_ids().map(|edge_id| (edge_id, Attr::default())));

        // for edge_id in self.edge_ids() {
        //     map.insert(edge_id, Attr::default());
        // }
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

impl<C, N, W: EdgeCost<Cost = C>, const DI: bool> Cost<C> for AdjGraph<N, W, DI> {}

impl<F, N, W: EdgeFlow<Flow = F>, const DI: bool> Flow<F> for AdjGraph<N, W, DI> {}

impl<B, N: NodeBalance<Balance = B>, W, const DI: bool> Balance<B> for AdjGraph<N, W, DI> {}

impl<N, W1, W2: Clone, const DI: bool> AdaptEdge<AdjGraph<N, W2, DI>, W2> for AdjGraph<N, W1, DI> {
    fn map_edge<F>(self, f: F) -> AdjGraph<N, W2, DI>
    where
        F: Fn(Edge<Self::Id, Self::EdgeWeight>) -> Edge<Self::Id, W2>,
    {
        let Self { nodes, edges } = self;

        let edges = edges.into_iter().flat_map(|neighs| {
            neighs.into_iter().map(|edge| {
                let edge = f(edge);
                (edge.edge_id.from(), edge.edge_id.to(), edge.weight)
            })
        });

        let mut adj_list = AdjGraph::with_nodes(nodes.into_iter().map(|node| node.weight));
        adj_list.extend_edges(edges);

        adj_list
    }

    fn split_map_edge<F>(self, f: F) -> AdjGraph<N, W2, DI>
    where
        F: Fn(Edge<Self::Id, Self::EdgeWeight>) -> Vec<Edge<Self::Id, W2>>,
    {
        let Self { nodes, edges } = self;

        let edges = edges.into_iter().flat_map(|neighs| {
            neighs.into_iter().flat_map(|edge| {
                f(edge)
                    .into_iter()
                    .map(|edge| (edge.edge_id.from(), edge.edge_id.to(), edge.weight))
            })
        });

        let mut adj_list = AdjGraph::with_nodes(nodes.into_iter().map(|node| node.weight));
        adj_list.extend_edges(edges);

        adj_list
    }
}

impl<N: Num, W: Num, const DI: bool> Graph<N, W> for AdjGraph<N, W, DI> {}

impl<N: Num, const DI: bool> WeightlessGraph<N> for AdjGraph<N, (), DI> {}

#[cfg(test)]
mod test {
    extern crate test;
    use super::AdjGraph;
    use crate::test::*;

    #[test]
    pub fn adj_list_create_with_nodes() {
        graph_create_with_nodes::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_list_create_with_capacity() {
        graph_create_with_capacity::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_list_insert_and_contains() {
        graph_insert_and_contains::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_list_clear() {
        graph_clear::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_list_get() {
        graph_get::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_list_index() {
        graph_index::<AdjGraph<_, _>>()
    }

    #[test]
    pub fn adj_list_index_adjacent() {
        graph_index_adjacent::<AdjGraph<_, _>>()
    }
}
