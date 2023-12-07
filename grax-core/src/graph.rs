// use std::fmt::Debug;

use std::fmt::Debug;

use crate::{
    collections::{EdgeMap, FixedEdgeMap, FixedNodeMap, NodeMap, VisitEdgeMap, VisitNodeMap},
    edge::{Edge, EdgeCost, EdgeFlow, EdgeMut, EdgeRef},
    index::{EdgeId, NodeId},
    node::{Node, NodeBalance, NodeMut, NodeRef},
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

pub trait AdaptNode<G, N>: NodeCollection + Keyed
where
    G: NodeCollection<NodeWeight = N>,
{
    fn map_node<F>(self, f: F) -> G
    where
        F: Fn(Node<Self::Key, Self::NodeWeight>) -> Node<Self::Key, N>;
}

pub trait AdaptEdge<G, W>: EdgeCollection + Keyed
where
    G: EdgeCollection<EdgeWeight = W>,
{
    fn map_edge<F>(self, f: F) -> G
    where
        F: Fn(Edge<Self::Key, Self::EdgeWeight>) -> Edge<Self::Key, W>;

    fn split_map_edge<F>(self, f: F) -> G
    where
        F: Fn(Edge<Self::Key, Self::EdgeWeight>) -> Vec<Edge<Self::Key, W>>;
}

pub trait Cost<C>: EdgeCollection<EdgeWeight: EdgeCost<Cost = C>> {}

pub trait Flow<F>: EdgeCollection<EdgeWeight: EdgeFlow<Flow = F>> {}

pub trait Balance<B>: NodeCollection<NodeWeight: NodeBalance<Balance = B>> {}

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
    type VisitNodeMap: VisitNodeMap<Self::Key>;
    type NodeMap<V: Debug + Clone>: NodeMap<Self::Key, V>;

    // implement by stable vec
    fn fixed_node_map<V: Debug + Clone>(&self, fill: V) -> Self::FixedNodeMap<V>;
    fn visit_node_map(&self) -> Self::VisitNodeMap;
    fn node_map<V: Debug + Clone>(&self) -> Self::NodeMap<V>;
}

pub trait EdgeAttribute: Keyed {
    type FixedEdgeMap<V: Debug + Clone>: FixedEdgeMap<Self::Key, V>;
    type VisitEdgeMap: VisitEdgeMap<Self::Key>;
    type EdgeMap<V: Debug + Clone>: EdgeMap<Self::Key, V>;

    fn fixed_edge_map<V: Debug + Clone>(&self, fill: V) -> Self::FixedEdgeMap<V>;
    fn visit_edge_map(&self) -> Self::VisitEdgeMap;
    fn edge_map<V: Debug + Clone>(&self) -> Self::EdgeMap<V>;
}

// pub trait Attribute: Base {
//     type NodeMap<Attr>: NodeAttrMap<Self::Key, Attr>;
//     type EdgeMap<Attr>: EdgeAttrMap<Self::Key, Attr>;

//     fn node_map<Attr>(&self) -> Self::NodeMap<Attr>;
//     fn edge_map<Attr>(&self) -> Self::EdgeMap<Attr>;

//     // TODO not necessary any more ? since insert safe
//     // fn update_node_map<Attr: Clone + Debug + Default>(&self, map: &mut Self::NodeMap<Attr>);
//     // fn update_edge_map<Attr: Clone + Debug + Default>(&self, map: &mut Self::EdgeMap<Attr>);

//     fn parents(&self) -> Parents<Self> {
//         Parents::new(self.node_map())
//     }

//     fn distances<C>(&self) -> Distances<C, Self>
//     where
//         C: Clone + Debug,
//         Self: Cost<C>,
//     {
//         let distances = self.node_map();
//         let parents = self.parents();

//         Distances::new(distances, parents)
//     }

//     fn union_find(&self) -> UnionFind<Self> {
//         let parents = self.parents();
//         let rank = self.node_map();

//         UnionFind::new(parents, rank)
//     }
// }
