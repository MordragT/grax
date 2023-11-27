use std::fmt::Debug;

use crate::{
    edge::{EdgeCost, EdgeFlow},
    node::NodeBalance,
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
    type Node;
    type Weight;
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
    fn contains_node(&self, node: &Self::Node) -> Option<NodeId<Self::Id>>;
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
    fn with_nodes(nodes: impl IntoIterator<Item = Self::Node>) -> Self;
}

pub trait Directed {
    fn directed() -> bool;
}

pub trait Extend: Base {
    fn extend_nodes(&mut self, nodes: impl IntoIterator<Item = Self::Node>);
    /// Is allowed to panic if the specified nodes are not within the graph
    fn extend_edges(
        &mut self,
        edges: impl IntoIterator<Item = (NodeId<Self::Id>, NodeId<Self::Id>, Self::Weight)>,
    );
}

pub trait Get: Base {
    fn node(&self, node_id: NodeId<Self::Id>) -> Option<&Self::Node>;
    fn weight(&self, edge_id: EdgeId<Self::Id>) -> Option<&Self::Weight>;

    fn contains_node_id(&self, node_id: NodeId<Self::Id>) -> bool {
        self.node(node_id).is_some()
    }

    fn contains_edge_id(&self, edge_id: EdgeId<Self::Id>) -> bool {
        self.weight(edge_id).is_some()
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
    fn node_mut(&mut self, node_id: NodeId<Self::Id>) -> Option<&mut Self::Node>;
    fn weight_mut(&mut self, edge_id: EdgeId<Self::Id>) -> Option<&mut Self::Weight>;

    fn update_node(&mut self, node_id: NodeId<Self::Id>, node: Self::Node) -> Option<Self::Node> {
        match self.node_mut(node_id) {
            Some(dest) => Some(std::mem::replace(dest, node)),
            None => None,
        }
    }
    fn update_edge(
        &mut self,
        edge_id: EdgeId<Self::Id>,
        weight: Self::Weight,
    ) -> Option<Self::Weight> {
        match self.weight_mut(edge_id) {
            Some(dest) => Some(std::mem::replace(dest, weight)),
            None => None,
        }
    }
}

pub trait Cost<C>: Base {
    type EdgeCost: EdgeCost<Cost = C>;

    fn cost(&self, edge_id: EdgeId<Self::Id>) -> Option<&Self::EdgeCost>;
    fn cost_mut(&mut self, edge_id: EdgeId<Self::Id>) -> Option<&mut Self::EdgeCost>;
}

// default impl<T: Base<Weight: EdgeCost> + Get + GetMut> Cost<<T::Weight as EdgeCost>::Cost> for T {
//     type EdgeCost = T::Weight;

//     fn cost(&self, edge_id: EdgeId<Self::Id>) -> Option<&Self::EdgeCost> {
//         self.weight(edge_id)
//     }

//     fn cost_mut(&mut self, edge_id: EdgeId<Self::Id>) -> Option<&mut Self::EdgeCost> {
//         self.weight_mut(edge_id)
//     }
// }

pub trait Flow<F>: Base {
    type EdgeFlow: EdgeFlow<Flow = F>;

    fn flow(&self, edge_id: EdgeId<Self::Id>) -> Option<&Self::EdgeFlow>;
    fn flow_mut(&mut self, edge_id: EdgeId<Self::Id>) -> Option<&mut Self::EdgeFlow>;

    // fn flow_weight(&self, edge_id: EdgeId<Self::Id>) -> Option<(&Self::Weight, &Self::EdgeFlow)> {
    //     self.weight(edge_id)
    //         .map(|weight| self.flow(edge_id).map(|flow| (weight, flow)))
    //         .flatten()
    // }
}

// default impl<T: Base<Weight: EdgeFlow> + Get + GetMut> Flow<<T::Weight as EdgeFlow>::Flow> for T {
//     type EdgeFlow = T::Weight;

//     fn flow(&self, edge_id: EdgeId<Self::Id>) -> Option<&Self::EdgeFlow> {
//         self.weight(edge_id)
//     }

//     fn flow_mut(&mut self, edge_id: EdgeId<Self::Id>) -> Option<&mut Self::EdgeFlow> {
//         self.weight_mut(edge_id)
//     }
// }

pub trait Balance<B>: Base {
    type NodeBalance: NodeBalance<Balance = B>;

    fn balance(&self, node_id: NodeId<Self::Id>) -> Option<&Self::NodeBalance>;
    fn balance_mut(&mut self, node_id: NodeId<Self::Id>) -> Option<&mut Self::NodeBalance>;

    // fn balance_node(&self, node_id: NodeId<Self::Id>) -> Option<(&Self::Node, &Self::NodeBalance)> {
    //     self.node(node_id)
    //         .map(|node| self.balance(node_id).map(|balance| (node, balance)))
    //         .flatten()
    // }
}

// default impl<T: Base<Node: NodeBalance> + Get + GetMut> Balance<<T::Node as NodeBalance>::Balance>
//     for T
// {
//     type NodeBalance = T::Node;

//     fn balance(&self, node_id: NodeId<Self::Id>) -> Option<&Self::NodeBalance> {
//         self.node(node_id)
//     }

//     fn balance_mut(&mut self, node_id: NodeId<Self::Id>) -> Option<&mut Self::NodeBalance> {
//         self.node_mut(node_id)
//     }
// }

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
    type Nodes<'a>: Iterator<Item = &'a Self::Node> + 'a
    where
        Self::Node: 'a,
        Self: 'a;
    type Edges<'a>: Iterator<Item = EdgeRef<'a, Self::Id, Self::Weight>> + 'a
    where
        Self::Weight: 'a,
        Self: 'a;

    /// This returns an iterator over all nodes in the graph.
    /// Due to constraints in the type system of rust this cannot be automatically implemented.
    /// But you can use the following to implement it for your Graph, provided you implement
    /// [Index](self::Index) and [Get](self::Get)
    /// ```rust
    /// self.node_ids().map(|node_id| self.node(node_id).unwrap())
    /// ```
    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a>;

    /// This returns an iterator over all edges in the graph.
    /// Due to constraints in the type system of rust this cannot be automatically implemented.
    /// But you can use the following to implement it for your Graph, provided you implement
    /// [Index](self::Index) and [Get](self::Get)
    /// ```rust
    /// self.edge_ids()
    /// .map(|edge_id| EdgeRef::new(edge_id, self.weight(edge_id).unwrap()))
    /// ```
    fn iter_edges<'a>(&'a self) -> Self::Edges<'a>;
}

pub trait IterMut: Base {
    type NodesMut<'a>: Iterator<Item = &'a mut Self::Node> + 'a
    where
        Self::Node: 'a,
        Self: 'a;
    type EdgesMut<'a>: Iterator<Item = EdgeRefMut<'a, Self::Id, Self::Weight>> + 'a
    where
        Self::Weight: 'a,
        Self: 'a;

    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a>;
    fn iter_edges_mut<'a>(&'a mut self) -> Self::EdgesMut<'a>;
}

pub trait IndexAdjacent: Base {
    type AdjacentNodeIds<'a>: Iterator<Item = NodeId<Self::Id>> + 'a
    where
        Self: 'a;
    type AdjacentEdgeIds<'a>: Iterator<Item = EdgeId<Self::Id>> + 'a
    where
        Self: 'a;

    fn adjacent_node_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::AdjacentNodeIds<'a>;
    fn adjacent_edge_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::AdjacentEdgeIds<'a>;
}

pub trait IterAdjacent: Base {
    type Nodes<'a>: Iterator<Item = &'a Self::Node> + 'a
    where
        Self::Node: 'a,
        Self: 'a;
    type Edges<'a>: Iterator<Item = EdgeRef<'a, Self::Id, Self::Weight>> + 'a
    where
        Self::Weight: 'a,
        Self: 'a;

    /// This returns an iterator over all nodes adjacent to the specified node in the graph.
    /// Due to constraints in the type system of rust this cannot be automatically implemented.
    /// But you can use the following to implement it for your Graph, provided you implement
    /// [Index](self::IndexAdjacent) and [Get](self::Get)
    /// ```rust
    /// self.adjacent_node_ids(node_id)
    /// .map(|node_id| self.node(node_id).unwrap())
    /// ```
    fn iter_adjacent_nodes<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::Nodes<'a>;

    /// This returns an iterator over all edges adjacent to the specified node in the graph.
    /// Due to constraints in the type system of rust this cannot be automatically implemented.
    /// But you can use the following to implement it for your Graph, provided you implement
    /// [Index](self::IndexAdjacent) and [Get](self::Get)
    /// ```rust
    /// self.adjacent_edge_ids(node_id)
    /// .map(|edge_id| EdgeRef::new(edge_id, self.weight(edge_id).unwrap()))
    /// ```
    fn iter_adjacent_edges<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::Edges<'a>;
}

pub trait IterAdjacentMut: Base {
    type NodesMut<'a>: Iterator<Item = &'a mut Self::Node> + 'a
    where
        Self::Node: 'a,
        Self: 'a;
    type EdgesMut<'a>: Iterator<Item = EdgeRefMut<'a, Self::Id, Self::Weight>> + 'a
    where
        Self::Weight: 'a,
        Self: 'a;

    fn iter_adjacent_nodes_mut<'a>(&'a mut self, node_id: NodeId<Self::Id>) -> Self::NodesMut<'a>;
    fn iter_adjacent_edges_mut<'a>(&'a mut self, node_id: NodeId<Self::Id>) -> Self::EdgesMut<'a>;
}

pub trait Insert: Base {
    fn insert_node(&mut self, node: Self::Node) -> NodeId<Self::Id>;
    /// Is allowed to panic if from or to are not in the graph
    fn insert_edge(
        &mut self,
        from: NodeId<Self::Id>,
        to: NodeId<Self::Id>,
        weight: Self::Weight,
    ) -> EdgeId<Self::Id>;
}

pub trait Remove: Base {
    fn remove_node(&mut self, node_id: NodeId<Self::Id>) -> Option<Self::Node>;
    fn remove_edge(&mut self, edge_id: EdgeId<Self::Id>) -> Option<Self::Weight>;
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
