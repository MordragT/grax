use crate::{
    graph::{
        Base, Capacity, Clear, Contains, Count, Create, Directed, EdgeRef, Extend, Get, GetMut,
        Index, Insert, Iter, IterMut, Reserve,
    },
    prelude::EdgeRefMut,
};

use super::{EdgeIndex, NodeIndex};
use std::collections::HashMap;

/// This Graph only implements a subset of the Graph traits.
/// It can be used by other Graph implementations to ease
/// the process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseGraph<Node, Weight, const DI: bool> {
    pub(crate) nodes: Vec<Node>,
    pub(crate) edges: HashMap<EdgeIndex, Weight>,
}

impl<Node, Weight, const DI: bool> BaseGraph<Node, Weight, DI> {}

impl<Node, Weight, const DI: bool> Base for BaseGraph<Node, Weight, DI> {
    type EdgeId = EdgeIndex;
    type NodeId = NodeIndex;
}

impl<Node, Weight, const DI: bool> Capacity for BaseGraph<Node, Weight, DI> {
    fn edges_capacity(&self) -> usize {
        self.edges.capacity()
    }

    fn nodes_capacity(&self) -> usize {
        self.nodes.capacity()
    }
}

impl<Node, Weight, const DI: bool> Clear for BaseGraph<Node, Weight, DI> {
    fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }
}

impl<Node: PartialEq, Weight, const DI: bool> Contains<Node> for BaseGraph<Node, Weight, DI> {
    fn contains_node(&self, node: &Node) -> Option<Self::NodeId> {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_i, other)| *other == node)
            .map(|(id, _)| NodeIndex(id))
    }

    fn contains_edge(&self, from: Self::NodeId, to: Self::NodeId) -> Option<Self::EdgeId> {
        let edge_id = EdgeIndex::new(from, to);
        if self.contains_edge_id(edge_id) {
            Some(edge_id)
        } else {
            None
        }
    }
}

impl<Node, Weight, const DI: bool> Count for BaseGraph<Node, Weight, DI> {
    fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl<Node, Weight, const DI: bool> Create<Node> for BaseGraph<Node, Weight, DI> {
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

impl<Node, Weight, const DI: bool> Directed for BaseGraph<Node, Weight, DI> {
    fn directed(&self) -> bool {
        DI
    }
}

impl<Node, Weight, const DI: bool> Extend<Node, Weight> for BaseGraph<Node, Weight, DI> {
    fn extend_edges(&mut self, edges: impl Iterator<Item = (Self::EdgeId, Weight)>) {
        self.edges.extend(edges)
    }

    fn extend_nodes(&mut self, nodes: impl Iterator<Item = Node>) {
        self.nodes.extend(nodes)
    }
}

impl<Node, Weight, const DI: bool> Get<Node, Weight> for BaseGraph<Node, Weight, DI> {
    fn node(&self, node_id: Self::NodeId) -> Option<&Node> {
        self.nodes.get(node_id.0)
    }

    fn weight(&self, edge_id: Self::EdgeId) -> Option<&Weight> {
        self.edges.get(&edge_id)
    }
}

impl<Node, Weight, const DI: bool> GetMut<Node, Weight> for BaseGraph<Node, Weight, DI> {
    fn node_mut(&mut self, node_id: Self::NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(node_id.0)
    }

    fn weight_mut(&mut self, edge_id: Self::EdgeId) -> Option<&mut Weight> {
        self.edges.get_mut(&edge_id)
    }
}

impl<Node, Weight, const DI: bool> Index for BaseGraph<Node, Weight, DI> {
    type EdgeIds<'a> = impl Iterator<Item = EdgeIndex> + 'a
    where Self: 'a;
    type NodeIds<'a> = impl Iterator<Item = NodeIndex> + 'a
    where Self: 'a;

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.edges.keys().cloned()
    }

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        (0..self.nodes.len()).map(NodeIndex)
    }
}

impl<Node, Weight, const DI: bool> Insert<Node, Weight> for BaseGraph<Node, Weight, DI> {
    fn add_node(&mut self, node: Node) -> Self::NodeId {
        let node_id = NodeIndex(self.nodes.len());
        self.nodes.push(node);
        return node_id;
    }

    fn insert_edge(&mut self, edge_id: Self::EdgeId, weight: Weight) -> Option<Weight> {
        self.edges.insert(edge_id, weight)
    }
}

impl<Node, Weight, const DI: bool> Iter<Node, Weight> for BaseGraph<Node, Weight, DI> {
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
        self.edges
            .iter()
            .map(|(id, weight)| EdgeRef::new(*id, weight))
    }
}

impl<Node, Weight, const DI: bool> IterMut<Node, Weight> for BaseGraph<Node, Weight, DI> {
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
        self.edges
            .iter_mut()
            .map(|(id, weight)| EdgeRefMut::new(*id, weight))
    }
}

impl<Node, Weight, const DI: bool> Reserve for BaseGraph<Node, Weight, DI> {
    fn reserve_edges(&mut self, additional: usize) {
        self.edges.reserve(additional)
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.nodes.reserve(additional)
    }
}
