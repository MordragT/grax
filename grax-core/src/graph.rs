// use std::fmt::Debug;

use std::fmt::Debug;

use crate::{
    collections::{EdgeMap, FixedEdgeMap, FixedNodeMap, NodeMap},
    edge::{Edge, EdgeMut, EdgeRef},
    index::{EdgeId, NodeId},
    node::{Node, NodeMut, NodeRef},
};

use super::collections::{
    EdgeCollection, EdgeCount, EdgeIter, EdgeIterMut, GetEdge, GetEdgeMut, GetNode, GetNodeMut,
    InsertEdge, InsertNode, Keyed, NodeCollection, NodeCount, NodeIter, NodeIterMut, RemoveEdge,
    RemoveNode,
};

/// Immutable Graph
pub trait ImGraph:
    Keyed
    + Directed
    + Create
    + EdgeCollection
    + NodeCollection
    + EdgeCount
    + NodeCount
    + GetEdge
    + GetNode
    + EdgeIter
    + NodeIter
    + EdgeIterAdjacent
    + NodeIterAdjacent
    + EdgeAttribute
    + NodeAttribute
{
}
// Mutable Graph
pub trait MutGraph:
    ImGraph
    + Clear
    + GetEdgeMut
    + GetNodeMut
    + EdgeIterMut
    + NodeIterMut
    + EdgeIterAdjacentMut
    + NodeIterAdjacentMut
    + InsertEdge
    + InsertNode
    + RemoveEdge
    + RemoveNode
{
}

pub trait Clear {
    /// Clears the Graph completely
    fn clear(&mut self);
    fn clear_edges(&mut self);
}

pub trait Directed {
    fn directed() -> bool;
}

pub trait Root: NodeCollection + Keyed {
    fn root_id(&self) -> NodeId<Self::Key>;
    fn root(&self) -> NodeRef<Self::Key, Self::NodeWeight>;
}

/// Creatable Graph
pub trait Create: NodeCollection + Keyed {
    fn new() -> Self;
    fn with_capacity(node_count: usize, edge_count: usize) -> Self;
    fn with_nodes(node_count: usize, nodes: impl IntoIterator<Item = Self::NodeWeight>) -> Self;
    // fn with_edges(
    //     node_count: usize,
    //     edge_count: usize,
    //     edges: impl IntoIterator<Item = (Self::NodeWeight, Self::NodeWeight, Self::EdgeWeight)>,
    // ) -> Self;
}

pub trait AdaptNodes<G, N>: NodeCollection + Keyed + Sized
where
    G: NodeCollection<NodeWeight = N>,
{
    type Iterator: Iterator<Item = Node<Self::Key, Self::NodeWeight>>;

    fn adapt_nodes<F, O>(self, f: F) -> G
    where
        F: Fn(Self::Iterator) -> O,
        O: Iterator<Item = Node<Self::Key, N>>;

    fn map_nodes<F>(self, f: F) -> G
    where
        F: Fn(Node<Self::Key, Self::NodeWeight>) -> Node<Self::Key, N>,
    {
        self.adapt_nodes(|nodes| nodes.map(&f))
    }
}

pub trait AdaptEdges<G, W>: EdgeCollection + Keyed + Sized
where
    G: EdgeCollection<EdgeWeight = W>,
{
    type Iterator: Iterator<Item = Edge<Self::Key, Self::EdgeWeight>>;

    fn adapt_edges<F, O>(self, f: F) -> G
    where
        F: Fn(Self::Iterator) -> O,
        O: Iterator<Item = Edge<Self::Key, W>>;

    fn map_edges<F>(self, f: F) -> G
    where
        F: Fn(Edge<Self::Key, Self::EdgeWeight>) -> Edge<Self::Key, W>,
    {
        self.adapt_edges(|edges| edges.map(&f))
    }
}

pub trait NodeIterAdjacent: NodeCollection + Keyed {
    type NodeIds<'a>: Iterator<Item = NodeId<Self::Key>> + 'a
    where
        Self: 'a;
    type Nodes<'a>: Iterator<Item = NodeRef<'a, Self::Key, Self::NodeWeight>> + 'a
    where
        Self::NodeWeight: 'a,
        Self: 'a;

    fn adjacent_node_ids(&self, node_id: NodeId<Self::Key>) -> Self::NodeIds<'_>;
    /// This returns an iterator over all nodes adjacent to the specified node in the graph.
    fn iter_adjacent_nodes(&self, node_id: NodeId<Self::Key>) -> Self::Nodes<'_>;
}

pub trait EdgeIterAdjacent: EdgeCollection + Keyed {
    type EdgeIds<'a>: Iterator<Item = EdgeId<Self::Key>> + 'a
    where
        Self: 'a;
    type Edges<'a>: Iterator<Item = EdgeRef<'a, Self::Key, Self::EdgeWeight>> + 'a
    where
        Self::EdgeWeight: 'a,
        Self: 'a;

    fn adjacent_edge_ids(&self, node_id: NodeId<Self::Key>) -> Self::EdgeIds<'_>;
    /// This returns an iterator over all edges adjacent to the specified node in the graph.
    fn iter_adjacent_edges(&self, node_id: NodeId<Self::Key>) -> Self::Edges<'_>;
}

pub trait NodeIterAdjacentMut: NodeCollection + Keyed {
    type NodesMut<'a>: Iterator<Item = NodeMut<'a, Self::Key, Self::NodeWeight>> + 'a
    where
        Self::NodeWeight: 'a,
        Self: 'a;

    fn iter_adjacent_nodes_mut(&mut self, node_id: NodeId<Self::Key>) -> Self::NodesMut<'_>;
}

pub trait EdgeIterAdjacentMut: EdgeCollection + Keyed {
    type EdgesMut<'a>: Iterator<Item = EdgeMut<'a, Self::Key, Self::EdgeWeight>> + 'a
    where
        Self::EdgeWeight: 'a,
        Self: 'a;

    fn iter_adjacent_edges_mut(&mut self, node_id: NodeId<Self::Key>) -> Self::EdgesMut<'_>;
}

pub trait NodeAttribute: Keyed {
    type FixedNodeMap<V: Debug + Clone>: FixedNodeMap<Self::Key, V>;
    type NodeMap<V: Debug + Clone>: NodeMap<Self::Key, V>;

    // implement by stable vec
    fn fixed_node_map<V: Debug + Clone>(&self, fill: V) -> Self::FixedNodeMap<V>;
    fn node_map<V: Debug + Clone>(&self) -> Self::NodeMap<V>;

    fn visit_node_map(&self) -> Self::FixedNodeMap<bool> {
        self.fixed_node_map(false)
    }
}

pub trait EdgeAttribute: Keyed {
    type FixedEdgeMap<V: Debug + Clone>: FixedEdgeMap<Self::Key, V>;
    type EdgeMap<V: Debug + Clone>: EdgeMap<Self::Key, V>;

    fn fixed_edge_map<V: Debug + Clone>(&self, fill: V) -> Self::FixedEdgeMap<V>;
    fn edge_map<V: Debug + Clone>(&self) -> Self::EdgeMap<V>;

    fn visit_edge_map(&self) -> Self::FixedEdgeMap<bool> {
        self.fixed_edge_map(false)
    }
}
