use super::{BaseGraph, EdgeIndex, NodeIndex};
use crate::{
    edge_list::EdgeList,
    graph::{
        Base, Capacity, Clear, Contains, Count, Create, Directed, EdgeIdentifier, Extend, Get,
        GetMut, Graph, Index, IndexAdjacent, Insert, Iter, IterAdjacent, IterAdjacentMut, IterMut,
        Remove, Reserve,
    },
    prelude::{EdgeRef, EdgeRefMut, WeightlessGraph},
};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct AdjacencyList<Node, Weight, const DI: bool = false> {
    pub(crate) base: BaseGraph<Node, Weight, DI>,
    pub(crate) adjacencies: Vec<Vec<NodeIndex>>,
}

impl<Node, Weight, const DI: bool> AdjacencyList<Node, Weight, DI> {
    pub fn new() -> Self {
        Self::with_capacity(0, 0)
    }
}

impl<Weight: Copy, const DI: bool> From<EdgeList<usize, Weight, DI>>
    for AdjacencyList<usize, Weight, DI>
{
    fn from(edge_list: EdgeList<usize, Weight, DI>) -> Self {
        let EdgeList {
            parents,
            children,
            weights,
            node_count,
        } = edge_list;

        let mut adj_list = Self::with_capacity(node_count, parents.len());

        for ((from, to), weight) in parents
            .into_iter()
            .zip(children.into_iter())
            .zip(weights.into_iter())
        {
            adj_list.base.nodes[from] = from;
            adj_list.base.nodes[to] = to;

            let edge_id = EdgeIndex::between(NodeIndex(from), NodeIndex(to));

            if !DI {
                adj_list.insert_edge(edge_id.rev(), weight);
            }

            adj_list.insert_edge(edge_id, weight);
        }

        adj_list
    }
}

impl<Node, Weight, const DI: bool> Base for AdjacencyList<Node, Weight, DI> {
    type EdgeId = EdgeIndex;
    type NodeId = NodeIndex;
}

impl<Node, Weight, const DI: bool> Capacity for AdjacencyList<Node, Weight, DI> {
    fn edges_capacity(&self) -> usize {
        self.base.edges_capacity()
    }

    fn nodes_capacity(&self) -> usize {
        self.base.nodes_capacity()
    }
}

impl<Node, Weight, const DI: bool> Clear for AdjacencyList<Node, Weight, DI> {
    fn clear(&mut self) {
        self.base.clear();
        self.adjacencies.clear();
    }

    fn clear_edges(&mut self) {
        self.base.clear_edges();
        self.adjacencies.clear();
    }
}

impl<Node: PartialEq, Weight, const DI: bool> Contains<Node> for AdjacencyList<Node, Weight, DI> {
    fn contains_node(&self, node: &Node) -> Option<Self::NodeId> {
        self.base.contains_node(node)
    }

    fn contains_edge(&self, from: Self::NodeId, to: Self::NodeId) -> Option<Self::EdgeId> {
        self.base.contains_edge(from, to)
    }
}

impl<Node, Weight, const DI: bool> Count for AdjacencyList<Node, Weight, DI> {
    fn edge_count(&self) -> usize {
        self.base.edge_count()
    }

    fn node_count(&self) -> usize {
        self.base.node_count()
    }
}

impl<Node, Weight, const DI: bool> Create<Node> for AdjacencyList<Node, Weight, DI> {
    fn with_capacity(nodes: usize, edges: usize) -> Self {
        let base = BaseGraph::with_capacity(nodes, edges);
        let adjacencies = Vec::with_capacity(nodes);

        Self { base, adjacencies }
    }

    fn with_nodes(nodes: impl Iterator<Item = Node>) -> Self {
        let base = BaseGraph::with_nodes(nodes);
        let adjacencies = vec![Vec::new(); base.node_count()];

        Self { base, adjacencies }
    }
}

impl<Node, Weight, const DI: bool> Directed for AdjacencyList<Node, Weight, DI> {
    fn directed(&self) -> bool {
        DI
    }
}

impl<Node, Weight, const DI: bool> Extend<Node, Weight> for AdjacencyList<Node, Weight, DI> {
    fn extend_edges(&mut self, edges: impl Iterator<Item = (Self::EdgeId, Weight)>) {
        for (edge_id, weight) in edges {
            self.insert_edge(edge_id, weight);
        }
    }

    fn extend_nodes(&mut self, nodes: impl Iterator<Item = Node>) {
        for node in nodes {
            self.add_node(node);
        }
    }
}

impl<Node, Weight, const DI: bool> Get<Node, Weight> for AdjacencyList<Node, Weight, DI> {
    fn node(&self, node_id: Self::NodeId) -> Option<&Node> {
        self.base.node(node_id)
    }
    fn weight(&self, edge_id: Self::EdgeId) -> Option<&Weight> {
        self.base.weight(edge_id)
    }
}

impl<Node, Weight, const DI: bool> GetMut<Node, Weight> for AdjacencyList<Node, Weight, DI> {
    fn node_mut(&mut self, node_id: Self::NodeId) -> Option<&mut Node> {
        self.base.node_mut(node_id)
    }
    fn weight_mut(&mut self, edge_id: Self::EdgeId) -> Option<&mut Weight> {
        self.base.weight_mut(edge_id)
    }
}

impl<Node, Weight, const DI: bool> Index for AdjacencyList<Node, Weight, DI> {
    type EdgeIds<'a> = impl Iterator<Item = EdgeIndex> + 'a
    where Self: 'a;
    type NodeIds<'a> = impl Iterator<Item = NodeIndex> + 'a
    where Self: 'a;

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.base.edge_ids()
    }

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        self.base.node_ids()
    }
}

impl<Node, Weight, const DI: bool> IndexAdjacent for AdjacencyList<Node, Weight, DI> {
    type AdjacentEdgeIds<'a> = impl Iterator<Item = EdgeIndex> + 'a
    where Self: 'a;
    type AdjacentNodeIds<'a> = impl Iterator<Item = NodeIndex> + 'a
    where Self: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentEdgeIds<'a> {
        self.adjacent_node_ids(node_id)
            .map(move |to| EdgeIndex::between(node_id, to))
    }
    fn adjacent_node_ids<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentNodeIds<'a> {
        self.adjacencies[node_id.0].iter().cloned()
    }
}

impl<Node, Weight, const DI: bool> Iter<Node, Weight> for AdjacencyList<Node, Weight, DI> {
    type Nodes<'a> = impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, Self::EdgeId, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a> {
        self.base.iter_nodes()
    }

    fn iter_edges<'a>(&'a self) -> Self::Edges<'a> {
        self.base.iter_edges()
    }
}
impl<Node, Weight, const DI: bool> IterMut<Node, Weight> for AdjacencyList<Node, Weight, DI> {
    type NodesMut<'a> = impl Iterator<Item = &'a mut Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    type EdgesMut<'a> = impl Iterator<Item = EdgeRefMut<'a, Self::EdgeId, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a> {
        self.base.iter_nodes_mut()
    }

    fn iter_edges_mut<'a>(&'a mut self) -> Self::EdgesMut<'a> {
        self.base.iter_edges_mut()
    }
}

impl<Node, Weight, const DI: bool> IterAdjacent<Node, Weight> for AdjacencyList<Node, Weight, DI> {
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
        self.adjacent_node_ids(node_id).map(move |to| {
            let edge_id = EdgeIndex::between(node_id, to);
            EdgeRef::new(edge_id, self.weight(edge_id).unwrap())
        })
    }
}
impl<Node, Weight, const DI: bool> IterAdjacentMut<Node, Weight>
    for AdjacencyList<Node, Weight, DI>
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
        self.iter_nodes_mut()
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
        let ids = self.adjacent_edge_ids(node_id).collect::<Vec<_>>();
        self.iter_edges_mut().filter_map(move |edge_ref| {
            if ids.contains(&edge_ref.edge_id) {
                Some(edge_ref)
            } else {
                None
            }
        })
    }
}

impl<Node, Weight, const DI: bool> Insert<Node, Weight> for AdjacencyList<Node, Weight, DI> {
    fn add_node(&mut self, node: Node) -> Self::NodeId {
        let index = self.base.add_node(node);
        self.adjacencies.push(Vec::new());
        index
    }
    fn insert_edge(&mut self, edge_id: Self::EdgeId, weight: Weight) -> Option<Weight> {
        self.adjacencies[edge_id.from.0].push(edge_id.to);
        self.base.insert_edge(edge_id, weight)
    }
}

impl<Node, Weight, const DI: bool> Remove<Node, Weight> for AdjacencyList<Node, Weight, DI> {
    fn remove_node(&mut self, node_id: Self::NodeId) -> Option<Node> {
        todo!()
    }

    fn remove_edge(&mut self, edge_id: Self::EdgeId) -> Option<Weight> {
        todo!()
    }
}

impl<Node, Weight, const DI: bool> Reserve for AdjacencyList<Node, Weight, DI> {
    fn reserve_edges(&mut self, additional: usize) {
        self.base.reserve_edges(additional)
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.base.reserve_nodes(additional)
    }
}

impl<Node: crate::graph::Node, Weight: crate::graph::Weight, const DI: bool> Graph<Node, Weight>
    for AdjacencyList<Node, Weight, DI>
{
}

impl<Node: crate::graph::Node, const DI: bool> WeightlessGraph<Node>
    for AdjacencyList<Node, (), DI>
{
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::AdjacencyList;
    use crate::graph::test::*;

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
