use super::{EdgeIdentifier, EdgeRef, EdgeRefMut, NodeIdentifier};

// TODO replace EdgeId NodeId with &EdgeId und &NodeId

/// A Base trait for graphs.
/// Must be implemented first to implement all the other Graph traits.
pub trait Base: Sized {
    /// A type used to identify an edge.
    type EdgeId: Copy + EdgeIdentifier<NodeId = Self::NodeId>;
    /// A type used to identify a node.
    type NodeId: Copy + NodeIdentifier;
}

pub trait Capacity {
    fn nodes_capacity(&self) -> usize;
    fn edges_capacity(&self) -> usize;
}

pub trait Clear {
    /// Clears the Graph completely
    fn clear(&mut self);
    fn clear_edges(&mut self);
}

pub trait Contains<Node>: Base {
    fn contains_node(&self, node: &Node) -> Option<Self::NodeId>;
    fn contains_edge(&self, from: Self::NodeId, to: Self::NodeId) -> Option<Self::EdgeId>;
}

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

/// Creatable Graph
pub trait Create<Node>: Sized {
    fn with_capacity(nodes: usize, edges: usize) -> Self;
    fn with_nodes(nodes: impl Iterator<Item = Node>) -> Self;
}

pub trait Directed {
    fn directed(&self) -> bool;
}

pub trait Extend<Node, Weight>: Base {
    fn extend_nodes(&mut self, nodes: impl Iterator<Item = Node>);
    fn extend_edges(&mut self, edges: impl Iterator<Item = (Self::EdgeId, Weight)>);
}

pub trait Get<Node, Weight>: Base {
    fn node(&self, node_id: Self::NodeId) -> Option<&Node>;
    fn weight(&self, edge_id: Self::EdgeId) -> Option<&Weight>;

    fn contains_node_id(&self, node_id: Self::NodeId) -> bool {
        self.node(node_id).is_some()
    }

    fn contains_edge_id(&self, edge_id: Self::EdgeId) -> bool {
        self.weight(edge_id).is_some()
    }
}

pub trait GetMut<Node, Weight>: Base {
    fn node_mut(&mut self, node_id: Self::NodeId) -> Option<&mut Node>;
    fn weight_mut(&mut self, edge_id: Self::EdgeId) -> Option<&mut Weight>;

    fn update_node(&mut self, node_id: Self::NodeId, node: Node) -> Option<Node> {
        match self.node_mut(node_id) {
            Some(dest) => Some(std::mem::replace(dest, node)),
            None => None,
        }
    }
    fn update_edge(&mut self, edge_id: Self::EdgeId, weight: Weight) -> Option<Weight> {
        match self.weight_mut(edge_id) {
            Some(dest) => Some(std::mem::replace(dest, weight)),
            None => None,
        }
    }
}

pub trait Index: Base {
    type EdgeIds<'a>: Iterator<Item = Self::EdgeId> + 'a
    where
        Self: 'a;
    type NodeIds<'a>: Iterator<Item = Self::NodeId> + 'a
    where
        Self: 'a;

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a>;
    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a>;
}

pub trait Iter<Node, Weight>: Base {
    type Nodes<'a>: Iterator<Item = &'a Node> + 'a
    where
        Node: 'a,
        Self: 'a;
    type Edges<'a>: Iterator<Item = EdgeRef<'a, Self::EdgeId, Weight>> + 'a
    where
        Weight: 'a,
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

pub trait IterMut<Node, Weight>: Base {
    type NodesMut<'a>: Iterator<Item = &'a mut Node> + 'a
    where
        Node: 'a,
        Self: 'a;
    type EdgesMut<'a>: Iterator<Item = EdgeRefMut<'a, Self::EdgeId, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    /// This returns an mutable iterator over all nodes in the graph.
    /// Due to constraints in the type system of rust this cannot be automatically implemented.
    /// But you can use the following to implement it for your Graph, provided you implement
    /// [Index](self::Index) and [Get](self::GetMut)
    /// ```rust
    /// self.node_ids()
    /// .map(|node_id| self.node_mut(node_id).unwrap())
    /// ```
    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a>;

    /// This returns an mutable iterator over all edges in the graph.
    /// Due to constraints in the type system of rust this cannot be automatically implemented.
    /// But you can use the following to implement it for your Graph, provided you implement
    /// [Index](self::Index) and [Get](self::GetMut)
    /// ```rust
    /// self.edge_ids()
    /// .map(|edge_id| EdgeRefMut::new(edge_id, self.weight_mut(edge_id).unwrap()))
    /// ```
    fn iter_edges_mut<'a>(&'a mut self) -> Self::EdgesMut<'a>;
}

pub trait IndexAdjacent: Base {
    type AdjacentNodeIds<'a>: Iterator<Item = Self::NodeId> + 'a
    where
        Self: 'a;
    type AdjacentEdgeIds<'a>: Iterator<Item = Self::EdgeId> + 'a
    where
        Self: 'a;

    fn adjacent_node_ids<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentNodeIds<'a>;
    fn adjacent_edge_ids<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentEdgeIds<'a>;
}

pub trait IterAdjacent<Node, Weight>: Base {
    type Nodes<'a>: Iterator<Item = &'a Node> + 'a
    where
        Node: 'a,
        Self: 'a;
    type Edges<'a>: Iterator<Item = EdgeRef<'a, Self::EdgeId, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    /// This returns an iterator over all nodes adjacent to the specified node in the graph.
    /// Due to constraints in the type system of rust this cannot be automatically implemented.
    /// But you can use the following to implement it for your Graph, provided you implement
    /// [Index](self::IndexAdjacent) and [Get](self::Get)
    /// ```rust
    /// self.adjacent_node_ids(node_id)
    /// .map(|node_id| self.node(node_id).unwrap())
    /// ```
    fn iter_adjacent_nodes<'a>(&'a self, node_id: Self::NodeId) -> Self::Nodes<'a>;

    /// This returns an iterator over all edges adjacent to the specified node in the graph.
    /// Due to constraints in the type system of rust this cannot be automatically implemented.
    /// But you can use the following to implement it for your Graph, provided you implement
    /// [Index](self::IndexAdjacent) and [Get](self::Get)
    /// ```rust
    /// self.adjacent_edge_ids(node_id)
    /// .map(|edge_id| EdgeRef::new(edge_id, self.weight(edge_id).unwrap()))
    /// ```
    fn iter_adjacent_edges<'a>(&'a self, node_id: Self::NodeId) -> Self::Edges<'a>;
}

pub trait IterAdjacentMut<Node, Weight>: Base {
    type NodesMut<'a>: Iterator<Item = &'a mut Node> + 'a
    where
        Node: 'a,
        Self: 'a;
    type EdgesMut<'a>: Iterator<Item = EdgeRefMut<'a, Self::EdgeId, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;
    /// This returns an mutable iterator over all nodes adjacent to the specified node in the graph.
    /// Due to constraints in the type system of rust this cannot be automatically implemented.
    /// But you can use the following to implement it for your Graph, provided you implement
    /// [Index](self::IndexAdjacent) and [Get](self::GetMut)
    /// ```rust
    /// todo!()
    /// ```
    fn iter_adjacent_nodes_mut<'a>(&'a mut self, node_id: Self::NodeId) -> Self::NodesMut<'a>;

    /// This returns an mutable iterator over all edges adjacent to the specified node in the graph.
    /// Due to constraints in the type system of rust this cannot be automatically implemented.
    /// But you can use the following to implement it for your Graph, provided you implement
    /// [Index](self::IndexAdjacent) and [Get](self::GetMut)
    /// ```rust
    /// todo!()
    /// ```
    fn iter_adjacent_edges_mut<'a>(&'a mut self, node_id: Self::NodeId) -> Self::EdgesMut<'a>;
}

pub trait Insert<Node, Weight>: Base {
    fn add_node(&mut self, node: Node) -> Self::NodeId;
    fn insert_edge(&mut self, edge_id: Self::EdgeId, weight: Weight) -> Option<Weight>;
}

pub trait Remove<Node, Weight>: Base {
    fn remove_node(&mut self, node_id: Self::NodeId) -> Node;
    fn remove_edge(&mut self, edge_id: Self::EdgeId) -> Option<Weight>;
}

pub trait Reserve {
    fn reserve_nodes(&mut self, additional: usize);
    fn reserve_edges(&mut self, additional: usize);
}
