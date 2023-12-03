use std::fmt::Debug;

use crate::{
    edge::{Edge, EdgeCost, EdgeFlow},
    node::{Node, NodeBalance, NodeRef, NodeRefMut},
    view::{AttrMap, Distances, Parents, UnionFind, VisitMap},
};

use super::{
    edge::{EdgeRef, EdgeRefMut},
    index::{EdgeId, Identifier, NodeId},
};

// delegate: Capacity, Clear, Contains, Count,  Extend, Get, GetMut, Insert, Remove, Reserve
// Viewable, Visitable, Index, IndexAdjacent, Iter, IterMut, IterAdjacent, IterAdjacentMut, Balance, Flow, Cost, Create, Base, Directed

/// A Base trait for graphs.
/// Must be implemented first to implement all the other Graph traits.
pub trait Base: Sized {
    type Id: Identifier;
    type NodeWeight;
    type EdgeWeight;
}

// pub trait Ref: Base {
//     type GraphRef<'a>: Base<Id = Self::Id, Node = &'a Self::Node, Weight = &'a Self::Weight>
//     where
//         Self::Node: 'a,
//         Self::Weight: 'a;
// }
// impl<'a, T: Base> Base for &'a T {
//     type Id = T::Id;
//     type Node = &'a T::Node;
//     type Weight = &'a T::Weight;
// }

pub trait Root: Base {
    fn root(&self) -> NodeId<Self::Id>;
}

pub trait Capacity {
    fn nodes_capacity(&self) -> usize;
    fn edges_capacity(&self) -> usize;
}

// impl<T: Capacity> Capacity for &T {
//     fn edges_capacity(&self) -> usize {
//         T::edges_capacity(self)
//     }

//     fn nodes_capacity(&self) -> usize {
//         T::nodes_capacity(self)
//     }
// }

pub trait Clear {
    /// Clears the Graph completely
    fn clear(&mut self);
    fn clear_edges(&mut self);
}

// impl<T: Clear> Clear for &mut T {
//     fn clear(&mut self) {
//         T::clear(self)
//     }

//     fn clear_edges(&mut self) {
//         T::clear_edges(self)
//     }
// }

pub trait Contains: Base {
    fn contains_node(&self, node: &Self::NodeWeight) -> Option<NodeId<Self::Id>>;
    fn contains_edge(
        &self,
        from: NodeId<Self::Id>,
        to: NodeId<Self::Id>,
    ) -> Option<EdgeId<Self::Id>>;
}

// impl<T: Contains> Contains for &T {
//     fn contains_node(&self, node: &Self::Node) -> Option<NodeId<Self::Id>> {
//         T::contains_node(self, node)
//     }

//     fn contains_edge(
//         &self,
//         from: NodeId<Self::Id>,
//         to: NodeId<Self::Id>,
//     ) -> Option<EdgeId<Self::Id>> {
//         T::contains_edge(self, from, to)
//     }
// }

pub trait Count {
    fn node_count(&self) -> usize;
    fn edge_count(&self) -> usize;

    fn nodes_empty(&self) -> bool {
        self.node_count() == 0
    }

    fn edges_empty(&self) -> bool {
        self.edge_count() == 0
    }
}

// impl<T: Count> Count for &T {
//     fn node_count(&self) -> usize {
//         T::node_count(self)
//     }

//     fn edge_count(&self) -> usize {
//         T::edge_count(self)
//     }
// }

/// Creatable Graph
pub trait Create: Base {
    fn new() -> Self;
    fn with_capacity(nodes: usize, edges: usize) -> Self;
    fn with_nodes(nodes: impl IntoIterator<Item = Self::NodeWeight>) -> Self;
}

pub trait Directed {
    fn directed() -> bool;
}

pub trait Extend: Base {
    fn extend_nodes(&mut self, nodes: impl IntoIterator<Item = Self::NodeWeight>);
    /// Is allowed to panic if the specified nodes are not within the graph
    fn extend_edges(
        &mut self,
        edges: impl IntoIterator<Item = (NodeId<Self::Id>, NodeId<Self::Id>, Self::EdgeWeight)>,
    );
}

pub trait Get: Base {
    fn node(&self, node_id: NodeId<Self::Id>) -> Option<NodeRef<Self::Id, Self::NodeWeight>>;
    fn edge(&self, edge_id: EdgeId<Self::Id>) -> Option<EdgeRef<Self::Id, Self::EdgeWeight>>;

    fn contains_node_id(&self, node_id: NodeId<Self::Id>) -> bool {
        self.node(node_id).is_some()
    }

    fn contains_edge_id(&self, edge_id: EdgeId<Self::Id>) -> bool {
        self.edge(edge_id).is_some()
    }
}

// impl<T: Get> Get for &T {
//     fn node(&self, node_id: NodeId<Self::Id>) -> Option<&Self::Node> {
//         T::node(self, node_id).map(|node| &node)
//     }

//     fn weight(&self, edge_id: EdgeId<Self::Id>) -> Option<&Self::Weight> {
//         T::weight(self, edge_id).as_ref()
//     }
// }

pub trait GetMut: Base {
    fn node_mut(
        &mut self,
        node_id: NodeId<Self::Id>,
    ) -> Option<NodeRefMut<Self::Id, Self::NodeWeight>>;
    fn edge_mut(
        &mut self,
        edge_id: EdgeId<Self::Id>,
    ) -> Option<EdgeRefMut<Self::Id, Self::EdgeWeight>>;

    fn update_node(
        &mut self,
        node_id: NodeId<Self::Id>,
        node: Self::NodeWeight,
    ) -> Option<Self::NodeWeight> {
        match self.node_mut(node_id) {
            Some(dest) => Some(std::mem::replace(dest.weight, node)),
            None => None,
        }
    }
    fn update_edge(
        &mut self,
        edge_id: EdgeId<Self::Id>,
        weight: Self::EdgeWeight,
    ) -> Option<Self::EdgeWeight> {
        match self.edge_mut(edge_id) {
            Some(dest) => Some(std::mem::replace(dest.weight, weight)),
            None => None,
        }
    }
}

pub trait AdaptNode<G, N>: Base
where
    G: Base<NodeWeight = N>,
{
    fn map_node<F>(self, f: F) -> G
    where
        F: Fn(Node<Self::Id, Self::NodeWeight>) -> Node<Self::Id, N>;
}

pub trait AdaptEdge<G, W>: Base
where
    G: Base<EdgeWeight = W>,
{
    fn map_edge<F>(self, f: F) -> G
    where
        F: Fn(Edge<Self::Id, Self::EdgeWeight>) -> Edge<Self::Id, W>;

    fn split_map_edge<F>(self, f: F) -> G
    where
        F: Fn(Edge<Self::Id, Self::EdgeWeight>) -> Vec<Edge<Self::Id, W>>;
}

pub trait Cost<C>: Base<EdgeWeight: EdgeCost<Cost = C>> {}

pub trait Flow<F>: Base<EdgeWeight: EdgeFlow<Flow = F>> {}

pub trait Balance<B>: Base<NodeWeight: NodeBalance<Balance = B>> {}

pub trait Index: Base {
    type EdgeIds<'a>: Iterator<Item = EdgeId<Self::Id>> + 'a
    where
        Self: 'a;
    type NodeIds<'a>: Iterator<Item = NodeId<Self::Id>> + 'a
    where
        Self: 'a;

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a>;
    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a>;
}

pub trait Iter: Base {
    type Nodes<'a>: Iterator<Item = NodeRef<'a, Self::Id, Self::NodeWeight>> + 'a
    where
        Self::NodeWeight: 'a,
        Self: 'a;
    type Edges<'a>: Iterator<Item = EdgeRef<'a, Self::Id, Self::EdgeWeight>> + 'a
    where
        Self::EdgeWeight: 'a,
        Self: 'a;

    /// This returns an iterator over all nodes in the graph.
    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a>;

    /// This returns an iterator over all edges in the graph.
    fn iter_edges<'a>(&'a self) -> Self::Edges<'a>;
}

pub trait IterMut: Base {
    type NodesMut<'a>: Iterator<Item = NodeRefMut<'a, Self::Id, Self::NodeWeight>> + 'a
    where
        Self::NodeWeight: 'a,
        Self: 'a;
    type EdgesMut<'a>: Iterator<Item = EdgeRefMut<'a, Self::Id, Self::EdgeWeight>> + 'a
    where
        Self::EdgeWeight: 'a,
        Self: 'a;

    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a>;
    fn iter_edges_mut<'a>(&'a mut self) -> Self::EdgesMut<'a>;
}

pub trait IndexAdjacent: Base {
    type NodeIds<'a>: Iterator<Item = NodeId<Self::Id>> + 'a
    where
        Self: 'a;
    type EdgeIds<'a>: Iterator<Item = EdgeId<Self::Id>> + 'a
    where
        Self: 'a;

    fn adjacent_node_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::NodeIds<'a>;
    fn adjacent_edge_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::EdgeIds<'a>;
}

pub trait IterAdjacent: Base {
    type Nodes<'a>: Iterator<Item = NodeRef<'a, Self::Id, Self::NodeWeight>> + 'a
    where
        Self::NodeWeight: 'a,
        Self: 'a;
    type Edges<'a>: Iterator<Item = EdgeRef<'a, Self::Id, Self::EdgeWeight>> + 'a
    where
        Self::EdgeWeight: 'a,
        Self: 'a;

    /// This returns an iterator over all nodes adjacent to the specified node in the graph.
    fn iter_adjacent_nodes<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::Nodes<'a>;

    /// This returns an iterator over all edges adjacent to the specified node in the graph.
    fn iter_adjacent_edges<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::Edges<'a>;
}

pub trait IterAdjacentMut: Base {
    type NodesMut<'a>: Iterator<Item = NodeRefMut<'a, Self::Id, Self::NodeWeight>> + 'a
    where
        Self::NodeWeight: 'a,
        Self: 'a;
    type EdgesMut<'a>: Iterator<Item = EdgeRefMut<'a, Self::Id, Self::EdgeWeight>> + 'a
    where
        Self::EdgeWeight: 'a,
        Self: 'a;

    fn iter_adjacent_nodes_mut<'a>(&'a mut self, node_id: NodeId<Self::Id>) -> Self::NodesMut<'a>;
    fn iter_adjacent_edges_mut<'a>(&'a mut self, node_id: NodeId<Self::Id>) -> Self::EdgesMut<'a>;
}

pub trait Insert: Base {
    fn insert_node(&mut self, node: Self::NodeWeight) -> NodeId<Self::Id>;
    /// Is allowed to panic if from or to are not in the graph
    fn insert_edge(
        &mut self,
        from: NodeId<Self::Id>,
        to: NodeId<Self::Id>,
        weight: Self::EdgeWeight,
    ) -> EdgeId<Self::Id>;
}

pub trait Remove: Base {
    fn remove_node(
        &mut self,
        node_id: NodeId<Self::Id>,
    ) -> Option<Node<Self::Id, Self::NodeWeight>>;
    fn remove_edge(
        &mut self,
        edge_id: EdgeId<Self::Id>,
    ) -> Option<Edge<Self::Id, Self::EdgeWeight>>;
}

pub trait Reserve {
    fn reserve_nodes(&mut self, additional: usize);
    fn reserve_edges(&mut self, additional: usize);
}

pub trait Visitable: Base {
    type VisitMap: VisitMap<NodeId<Self::Id>>;

    fn visit_map(&self) -> Self::VisitMap;
}

pub trait Viewable: Base {
    type NodeMap<Attr: Clone + Debug + Default>: AttrMap<NodeId<Self::Id>, Attr>;
    type EdgeMap<Attr: Clone + Debug + Default>: AttrMap<EdgeId<Self::Id>, Attr>;

    fn node_map<Attr: Clone + Debug + Default>(&self) -> Self::NodeMap<Attr>;
    fn edge_map<Attr: Clone + Debug + Default>(&self) -> Self::EdgeMap<Attr>;

    // TODO not necessary any more ? since insert safe
    // fn update_node_map<Attr: Clone + Debug + Default>(&self, map: &mut Self::NodeMap<Attr>);
    // fn update_edge_map<Attr: Clone + Debug + Default>(&self, map: &mut Self::EdgeMap<Attr>);

    fn parents(&self) -> Parents<Self> {
        Parents::new(self.node_map())
    }

    fn distances<C>(&self) -> Distances<C, Self>
    where
        C: Clone + Debug,
        Self: Cost<C>,
    {
        let distances = self.node_map();
        let parents = self.parents();

        Distances::new(distances, parents)
    }

    fn union_find(&self) -> UnionFind<Self> {
        let parents = self.parents();
        let rank = self.node_map();

        UnionFind::new(parents, rank)
    }
}
