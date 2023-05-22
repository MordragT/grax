use crate::{
    graph::{Clear, Extend, Get, GetMut, Insert},
    prelude::{
        Base, Capacity, Count, Create, Directed, EdgeRef, Index, IterEdges, IterNodes,
        IterNodesMut, Reserve,
    },
};

use super::{EdgeIndex, NodeIndex};
use std::collections::HashMap;

/// This Graph only implements a subset of the Graph traits.
/// It can be used by other Graph implementations to ease
/// the process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseGraph<Node, Weight, const Di: bool> {
    pub(crate) nodes: Vec<Node>,
    pub(crate) edges: HashMap<EdgeIndex, Weight>,
}

impl<Node, Weight, const Di: bool> BaseGraph<Node, Weight, Di> {}

impl<Node, Weight, const Di: bool> Base for BaseGraph<Node, Weight, Di> {
    type EdgeId = EdgeIndex;
    type NodeId = NodeIndex;
}

impl<Node, Weight, const Di: bool> Capacity for BaseGraph<Node, Weight, Di> {
    fn edges_capacity(&self) -> usize {
        self.edges.capacity()
    }

    fn nodes_capacity(&self) -> usize {
        self.nodes.capacity()
    }
}

impl<Node, Weight, const Di: bool> Clear for BaseGraph<Node, Weight, Di> {
    fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }
}

impl<Node, Weight, const Di: bool> Count for BaseGraph<Node, Weight, Di> {
    fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl<Node, Weight, const Di: bool> Create<Node> for BaseGraph<Node, Weight, Di> {
    fn with_capacity(nodes: usize, edges: usize) -> Self {
        let nodes = Vec::with_capacity(nodes);
        let edges = HashMap::with_capacity(edges);

        Self { nodes, edges }
    }

    fn with_nodes(nodes: impl Iterator<Item = Node>) -> Self {
        let nodes = nodes.collect();
        let edges = HashMap::new();

        Self { nodes, edges }
    }
}

impl<Node, Weight, const Di: bool> Directed for BaseGraph<Node, Weight, Di> {
    fn directed(&self) -> bool {
        Di
    }
}

impl<Node, Weight, const Di: bool> Extend<Node, Weight> for BaseGraph<Node, Weight, Di> {
    fn extend_edges(&mut self, edges: impl Iterator<Item = (Self::EdgeId, Weight)>) {
        self.edges.extend(edges)
    }

    fn extend_nodes(&mut self, nodes: impl Iterator<Item = Node>) {
        self.nodes.extend(nodes)
    }
}

impl<Node, Weight, const Di: bool> Get<Node, Weight> for BaseGraph<Node, Weight, Di> {
    fn node(&self, node_id: Self::NodeId) -> Option<&Node> {
        self.nodes.get(node_id.0)
    }

    fn weight(&self, edge_id: Self::EdgeId) -> Option<&Weight> {
        self.edges.get(&edge_id)
    }
}

impl<Node, Weight, const Di: bool> GetMut<Node, Weight> for BaseGraph<Node, Weight, Di> {
    fn node_mut(&mut self, node_id: Self::NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(node_id.0)
    }

    fn weight_mut(&mut self, edge_id: Self::EdgeId) -> Option<&mut Weight> {
        self.edges.get_mut(&edge_id)
    }
}

impl<Node, Weight, const Di: bool> Index for BaseGraph<Node, Weight, Di> {
    type EdgeIndices<'a> = impl Iterator<Item = EdgeIndex> + 'a
    where Self: 'a;
    type NodeIndices<'a> = impl Iterator<Item = NodeIndex> + 'a
    where Self: 'a;

    fn edge_indices<'a>(&'a self) -> Self::EdgeIndices<'a> {
        self.edges.keys().cloned()
    }

    fn node_indices<'a>(&'a self) -> Self::NodeIndices<'a> {
        (0..self.nodes.len()).map(NodeIndex)
    }
}

impl<Node, Weight, const Di: bool> Insert<Node, Weight> for BaseGraph<Node, Weight, Di> {
    fn add_node(&mut self, node: Node) -> Self::NodeId {
        let node_id = NodeIndex(self.nodes.len());
        self.nodes.push(node);
        return node_id;
    }

    fn insert_edge(&mut self, edge_id: Self::EdgeId, weight: Weight) -> Option<Weight> {
        self.edges.insert(edge_id, weight)
    }
}

impl<Node, Weight, const Di: bool> IterEdges<Weight> for BaseGraph<Node, Weight, Di> {
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, EdgeIndex, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_edges<'a>(&'a self) -> Self::Edges<'a> {
        self.edges
            .iter()
            .map(|(id, weight)| EdgeRef::new(*id, weight))
    }
}

impl<Node, Weight, const Di: bool> IterNodes<Node> for BaseGraph<Node, Weight, Di> {
    type Nodes<'a> = impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a> {
        self.nodes.iter()
    }
}

impl<Node, Weight, const Di: bool> IterNodesMut<Node> for BaseGraph<Node, Weight, Di> {
    type NodesMut<'a> = impl Iterator<Item = &'a mut Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a> {
        self.nodes.iter_mut()
    }
}

impl<Node, Weight, const Di: bool> Reserve for BaseGraph<Node, Weight, Di> {
    fn reserve_edges(&mut self, additional: usize) {
        self.edges.reserve(additional)
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.nodes.reserve(additional)
    }
}
