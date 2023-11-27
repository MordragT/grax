use grax_core::edge::*;
use grax_core::node::*;
use grax_core::prelude::*;
use grax_core::traits::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;

use super::attr::{AttrHashMap, AttrVec};
use crate::edge_list::EdgeList;

type RawNodeId = NodeId<usize>;
type RawEdgeId = EdgeId<usize>;

#[derive(Debug, Clone)]
pub struct AdjacencyList<Node, Weight, const DI: bool = false> {
    pub(crate) nodes: Vec<Node>,
    pub(crate) edges: Vec<Vec<Edge<usize, Weight>>>,
}

impl<Weight: Clone, const DI: bool> AdjacencyList<usize, Weight, DI> {
    pub fn with_edges(
        edges: impl IntoIterator<Item = (usize, usize, Weight)>,
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

impl<Node, Weight: Copy, const DI: bool> From<EdgeList<Node, Weight, DI>>
    for AdjacencyList<Node, Weight, DI>
{
    fn from(edge_list: EdgeList<Node, Weight, DI>) -> Self {
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

impl<Node, Weight, const DI: bool> Base for AdjacencyList<Node, Weight, DI> {
    type Id = usize;
    type Node = Node;
    type Weight = Weight;
}

// impl<Node, Weight, const DI: bool> Ref for AdjacencyList<Node, Weight, DI> {
//     type GraphRef<'a> = AdjacencyList<&'a Node, &'a Weight, DI>
//     where
//         Node: 'a,
//         Weight: 'a;
// }

impl<Node, Weight, const DI: bool> Capacity for AdjacencyList<Node, Weight, DI> {
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

impl<Node, Weight, const DI: bool> Clear for AdjacencyList<Node, Weight, DI> {
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

impl<Node: PartialEq, Weight, const DI: bool> Contains for AdjacencyList<Node, Weight, DI> {
    fn contains_node(&self, node: &Node) -> Option<RawNodeId> {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_i, other)| *other == node)
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

impl<Node, Weight, const DI: bool> Count for AdjacencyList<Node, Weight, DI> {
    fn edge_count(&self) -> usize {
        let count = self.edges.iter().fold(0, |mut akku, edges| {
            akku += edges.len();
            akku
        });

        if DI {
            count
        } else {
            count / 2
        }
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl<Node, Weight: Clone, const DI: bool> Create for AdjacencyList<Node, Weight, DI> {
    fn with_capacity(nodes: usize, edges: usize) -> Self {
        let nodes = Vec::with_capacity(nodes);
        let edges = Vec::with_capacity(nodes.len());

        Self { nodes, edges }
    }

    fn with_nodes(nodes: impl IntoIterator<Item = Node>) -> Self {
        let nodes = nodes.into_iter().collect::<Vec<_>>();
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

impl<Node, Weight, const DI: bool> Directed for AdjacencyList<Node, Weight, DI> {
    fn directed() -> bool {
        DI
    }
}

impl<Node, Weight, const DI: bool> Extend for AdjacencyList<Node, Weight, DI> {
    fn extend_edges(&mut self, edges: impl IntoIterator<Item = (RawNodeId, RawNodeId, Weight)>) {
        for (from, to, weight) in edges {
            self.insert_edge(from, to, weight);
        }
    }

    fn extend_nodes(&mut self, nodes: impl IntoIterator<Item = Node>) {
        for node in nodes {
            self.insert_node(node);
        }
    }
}

impl<Node, Weight, const DI: bool> Get for AdjacencyList<Node, Weight, DI> {
    fn node(&self, node_id: RawNodeId) -> Option<&Node> {
        self.nodes.get(node_id.raw())
    }

    fn weight(&self, edge_id: RawEdgeId) -> Option<&Weight> {
        self.edges.get(edge_id.from().raw()).and_then(|adj| {
            adj.iter().find_map(|edge| {
                if edge.to() == edge_id.to() {
                    Some(&edge.weight)
                } else {
                    None
                }
            })
        })
    }
}

impl<Node, Weight, const DI: bool> GetMut for AdjacencyList<Node, Weight, DI> {
    fn node_mut(&mut self, node_id: RawNodeId) -> Option<&mut Node> {
        self.nodes.get_mut(node_id.raw())
    }

    fn weight_mut(&mut self, edge_id: RawEdgeId) -> Option<&mut Weight> {
        self.edges.get_mut(edge_id.from().raw()).and_then(|adj| {
            adj.iter_mut().find_map(|edge| {
                if edge.to() == edge_id.to() {
                    Some(&mut edge.weight)
                } else {
                    None
                }
            })
        })
    }
}

impl<Node, Weight, const DI: bool> Index for AdjacencyList<Node, Weight, DI> {
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

impl<Node, Weight, const DI: bool> IndexAdjacent for AdjacencyList<Node, Weight, DI> {
    type AdjacentEdgeIds<'a> = impl Iterator<Item = RawEdgeId> + 'a
    where Self: 'a;
    type AdjacentNodeIds<'a> = impl Iterator<Item = RawNodeId> + 'a
    where Self: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: RawNodeId) -> Self::AdjacentEdgeIds<'a> {
        self.edges
            .get(node_id.raw())
            .map(|adj| adj.iter().map(|edge| edge.edge_id))
            .into_iter()
            .flatten()
    }
    fn adjacent_node_ids<'a>(&'a self, node_id: RawNodeId) -> Self::AdjacentNodeIds<'a> {
        self.edges
            .get(node_id.raw())
            .map(|adj| adj.iter().map(|edge| edge.to()))
            .into_iter()
            .flatten()
    }
}

impl<Node, Weight, const DI: bool> Iter for AdjacencyList<Node, Weight, DI> {
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
        self.edges
            .iter()
            .map(|adj| adj.iter())
            .flatten()
            .map(Into::into)
    }
}
impl<Node, Weight, const DI: bool> IterMut for AdjacencyList<Node, Weight, DI> {
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
        self.edges
            .iter_mut()
            .map(|adj| adj.iter_mut())
            .flatten()
            .map(Into::into)
    }
}

impl<Node, Weight, const DI: bool> IterAdjacent for AdjacencyList<Node, Weight, DI> {
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
        self.edges
            .get(node_id.raw())
            .map(|adj| adj.iter().map(Into::into))
            .into_iter()
            .flatten()
    }
}
impl<Node, Weight, const DI: bool> IterAdjacentMut for AdjacencyList<Node, Weight, DI> {
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

impl<Node, Weight, const DI: bool> Insert for AdjacencyList<Node, Weight, DI> {
    fn insert_node(&mut self, node: Node) -> RawNodeId {
        let node_id = RawNodeId::new_unchecked(self.nodes.len());
        self.nodes.push(node);
        self.edges.push(Vec::new());
        node_id
    }

    // TODO undirected ??
    fn insert_edge(&mut self, from: RawNodeId, to: RawNodeId, weight: Weight) -> RawEdgeId {
        let edge_id = RawEdgeId::new_unchecked(from, to);
        let edge = Edge::new(edge_id, weight);
        self.edges[from.raw()].push(edge);
        edge_id
    }
}

impl<Node: Default, Weight, const DI: bool> Remove for AdjacencyList<Node, Weight, DI> {
    fn remove_node(&mut self, node_id: RawNodeId) -> Option<Node> {
        if let Some(adj) = self.edges.get_mut(node_id.raw()) {
            adj.clear();
        }

        for adj in self.edges.iter_mut() {
            adj.retain(|edge| edge.to() != node_id);
        }

        // TODO replace with Option::None
        if let Some(node) = self.nodes.get_mut(node_id.raw()) {
            Some(std::mem::replace(node, Node::default()))
        } else {
            None
        }
    }

    fn remove_edge(&mut self, edge_id: RawEdgeId) -> Option<Weight> {
        // self.edges[edge_id.from().raw()].
        todo!()
    }
}

impl<Node, Weight, const DI: bool> Reserve for AdjacencyList<Node, Weight, DI> {
    fn reserve_edges(&mut self, additional: usize) {
        todo!()
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.nodes.reserve(additional)
    }
}

impl<Node, Weight, const DI: bool> Visitable for AdjacencyList<Node, Weight, DI> {
    type VisitMap = AttrVec<bool>;

    fn visit_map(&self) -> Self::VisitMap {
        AttrVec(vec![false; self.node_count()])
    }
}

impl<Node, Weight, const DI: bool> Viewable for AdjacencyList<Node, Weight, DI> {
    type NodeMap<Attr: Clone + Default + Debug> = AttrVec<Attr>;
    type EdgeMap<Attr: Clone + Default + Debug> = AttrHashMap<EdgeId<Self::Id>, Attr>;

    fn node_map<Attr: Clone + Default + Debug>(&self) -> Self::NodeMap<Attr> {
        AttrVec(vec![Attr::default(); self.node_count()])
    }

    fn edge_map<Attr: Clone + Default + Debug>(&self) -> Self::EdgeMap<Attr> {
        let mut map = HashMap::new();
        for edge_id in self.edge_ids() {
            map.insert(edge_id, Attr::default());
        }
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

impl<C, Node, Weight: EdgeCost<Cost = C>, const DI: bool> Cost<C>
    for AdjacencyList<Node, Weight, DI>
{
    type EdgeCost = Weight;

    fn cost(&self, edge_id: EdgeId<Self::Id>) -> Option<&Self::EdgeCost> {
        self.weight(edge_id)
    }

    fn cost_mut(&mut self, edge_id: EdgeId<Self::Id>) -> Option<&mut Self::EdgeCost> {
        self.weight_mut(edge_id)
    }
}

impl<N: Node, W: Debug + Clone, const DI: bool> Graph<N, W> for AdjacencyList<N, W, DI> {}

impl<N: Node, const DI: bool> WeightlessGraph<N> for AdjacencyList<N, (), DI> {}

#[cfg(test)]
mod test {
    extern crate test;
    use super::AdjacencyList;
    use crate::test::*;

    #[test]
    pub fn adj_list_create_with_nodes() {
        graph_create_with_nodes::<AdjacencyList<_, _>>()
    }

    #[test]
    pub fn adj_list_create_with_capacity() {
        graph_create_with_capacity::<AdjacencyList<_, _>>()
    }

    #[test]
    pub fn adj_list_insert_and_contains() {
        graph_insert_and_contains::<AdjacencyList<_, _>>()
    }

    #[test]
    pub fn adj_list_clear() {
        graph_clear::<AdjacencyList<_, _>>()
    }

    #[test]
    pub fn adj_list_get() {
        graph_get::<AdjacencyList<_, _>>()
    }

    #[test]
    pub fn adj_list_index() {
        graph_index::<AdjacencyList<_, _>>()
    }

    #[test]
    pub fn adj_list_index_adjacent() {
        graph_index_adjacent::<AdjacencyList<_, _>>()
    }
}
