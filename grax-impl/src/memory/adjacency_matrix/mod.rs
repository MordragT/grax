use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;

use grax_core::edge::*;
use grax_core::node::*;
use grax_core::prelude::*;
use grax_core::traits::*;
use matrix::SparseMatrix;

use crate::edge_list::EdgeList;

use super::attr::{AttrHashMap, AttrVec};

mod matrix;

type RawNodeId = NodeId<usize>;
type RawEdgeId = EdgeId<usize>;

#[derive(Debug, Clone)]
pub struct AdjacencyMatrix<Node, Weight, const DI: bool = false> {
    pub(crate) nodes: Vec<Node>,
    pub(crate) edges: SparseMatrix<Weight>,
}

impl<Weight: Clone, const DI: bool> AdjacencyMatrix<usize, Weight, DI> {
    pub fn with_edges(
        edges: impl IntoIterator<Item = (usize, usize, Weight)>,
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
    type Node = Node;
    type Weight = Weight;
}

// impl<Node, Weight, const DI: bool> Ref for AdjacencyMatrix<Node, Weight, DI> {
//     type GraphRef<'a> = AdjacencyMatrix<&'a Node, &'a Weight, DI>
//     where
//         Node: 'a,
//         Weight: 'a;
// }

impl<Node, Weight, const DI: bool> Capacity for AdjacencyMatrix<Node, Weight, DI> {
    fn edges_capacity(&self) -> usize {
        self.edges.capacity()
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

impl<Node: PartialEq, Weight, const DI: bool> Contains for AdjacencyMatrix<Node, Weight, DI> {
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

impl<Node, Weight, const DI: bool> Create for AdjacencyMatrix<Node, Weight, DI> {
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

    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: SparseMatrix::new(),
        }
    }
}

impl<Node, Weight, const DI: bool> Directed for AdjacencyMatrix<Node, Weight, DI> {
    fn directed() -> bool {
        DI
    }
}

impl<Node, Weight, const DI: bool> Extend for AdjacencyMatrix<Node, Weight, DI> {
    fn extend_edges(&mut self, edges: impl IntoIterator<Item = (RawNodeId, RawNodeId, Weight)>) {
        for (from, to, weight) in edges {
            self.edges.insert(from.raw(), to.raw(), weight)
        }
    }

    fn extend_nodes(&mut self, nodes: impl IntoIterator<Item = Node>) {
        self.nodes.extend(nodes);
    }
}

impl<Node, Weight, const DI: bool> Get for AdjacencyMatrix<Node, Weight, DI> {
    fn node(&self, node_id: RawNodeId) -> Option<&Node> {
        self.nodes.get(node_id.raw())
    }
    fn weight(&self, edge_id: RawEdgeId) -> Option<&Weight> {
        self.edges.get(edge_id.from().raw(), edge_id.to().raw())
    }
}

impl<Node, Weight, const DI: bool> GetMut for AdjacencyMatrix<Node, Weight, DI> {
    fn node_mut(&mut self, node_id: RawNodeId) -> Option<&mut Node> {
        self.nodes.get_mut(node_id.raw())
    }
    fn weight_mut(&mut self, edge_id: RawEdgeId) -> Option<&mut Weight> {
        self.edges.get_mut(edge_id.from().raw(), edge_id.to().raw())
    }
}

impl<Node, Weight, const DI: bool> Insert for AdjacencyMatrix<Node, Weight, DI> {
    fn insert_node(&mut self, node: Node) -> RawNodeId {
        let node_id = RawNodeId::new_unchecked(self.nodes.len());
        self.nodes.push(node);
        return node_id;
    }
    fn insert_edge(&mut self, from: RawNodeId, to: RawNodeId, weight: Weight) -> RawEdgeId {
        self.edges.insert(from.raw(), to.raw(), weight);
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
        self.edges.row(node_id.raw()).map(move |(to, _)| {
            let to = RawNodeId::new_unchecked(to);
            RawEdgeId::new_unchecked(node_id, to)
        })
    }
    fn adjacent_node_ids<'a>(&'a self, node_id: RawNodeId) -> Self::AdjacentNodeIds<'a> {
        self.edges
            .row(node_id.raw())
            .map(|(to, _)| RawNodeId::new_unchecked(to))
    }
}

impl<Node, Weight, const DI: bool> Iter for AdjacencyMatrix<Node, Weight, DI> {
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
impl<Node, Weight, const DI: bool> IterMut for AdjacencyMatrix<Node, Weight, DI> {
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

impl<Node, Weight, const DI: bool> IterAdjacent for AdjacencyMatrix<Node, Weight, DI> {
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
        self.edges.row(node_id.raw()).map(move |(to, weight)| {
            let to = RawNodeId::new_unchecked(to);
            let edge_id = RawEdgeId::new_unchecked(node_id, to);
            EdgeRef::new(edge_id, weight)
        })
    }
}
impl<Node, Weight, const DI: bool> IterAdjacentMut for AdjacencyMatrix<Node, Weight, DI> {
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
        self.edges.row_mut(node_id.raw()).map(move |(to, weight)| {
            let to = RawNodeId::new_unchecked(to);
            let edge_id = RawEdgeId::new_unchecked(node_id, to);
            EdgeRefMut::new(edge_id, weight)
        })
    }
}

impl<Node: Default, Weight, const DI: bool> Remove for AdjacencyMatrix<Node, Weight, DI> {
    fn remove_node(&mut self, node_id: RawNodeId) -> Option<Node> {
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

impl<Node, Weight, const DI: bool> Visitable for AdjacencyMatrix<Node, Weight, DI> {
    type VisitMap = AttrVec<bool>;

    fn visit_map(&self) -> Self::VisitMap {
        AttrVec(vec![false; self.node_count()])
    }
}

impl<Node, Weight, const DI: bool> Viewable for AdjacencyMatrix<Node, Weight, DI> {
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
    for AdjacencyMatrix<Node, Weight, DI>
{
    type EdgeCost = Weight;

    fn cost(&self, edge_id: EdgeId<Self::Id>) -> Option<&Self::EdgeCost> {
        self.weight(edge_id)
    }

    fn cost_mut(&mut self, edge_id: EdgeId<Self::Id>) -> Option<&mut Self::EdgeCost> {
        self.weight_mut(edge_id)
    }
}

impl<N: Node, W: Debug + Clone, const DI: bool> Graph<N, W> for AdjacencyMatrix<N, W, DI> {}

impl<N: Node, const DI: bool> WeightlessGraph<N> for AdjacencyMatrix<N, (), DI> {}

#[cfg(test)]
mod test {
    extern crate test;
    use super::AdjacencyMatrix;
    use crate::test::*;

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
