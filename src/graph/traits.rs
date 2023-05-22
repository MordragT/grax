use super::EdgeRef;

// TODO replace EdgeId NodeId with &EdgeId und &NodeId

/// Graph base trait
pub trait Base {
    /// Should not be able to be constructed outside the Graph
    type EdgeId: Copy;
    /// Should not be able to be constructed outside the Graph
    type NodeId: Copy;
}

pub trait Capacity {
    fn nodes_capacity(&self) -> usize;
    fn edges_capacity(&self) -> usize;
}

pub trait Clear {
    /// Clears the Graph completely
    fn clear(&mut self);
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

    fn contains_node(&self, node_id: Self::NodeId) -> bool {
        self.node(node_id).is_some()
    }

    fn contains_edge(&self, edge_id: Self::EdgeId) -> bool {
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
    type NodeIndices<'a>: Iterator<Item = Self::NodeId> + 'a
    where
        Self: 'a;
    type EdgeIndices<'a>: Iterator<Item = Self::EdgeId> + 'a
    where
        Self: 'a;

    fn node_indices<'a>(&'a self) -> Self::NodeIndices<'a>;
    fn edge_indices<'a>(&'a self) -> Self::EdgeIndices<'a>;
}

pub trait IndexAdjacent: Base {
    type AdjacentNodeIndices<'a>: Iterator<Item = Self::NodeId> + 'a
    where
        Self: 'a;
    type AdjacentEdgeIndices<'a>: Iterator<Item = Self::EdgeId> + 'a
    where
        Self: 'a;

    fn adjacent_node_indices<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentNodeIndices<'a>;
    fn adjacent_edge_indices<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentEdgeIndices<'a>;
}

pub trait Insert<Node, Weight>: Base {
    fn add_node(&mut self, node: Node) -> Self::NodeId;
    fn insert_edge(&mut self, edge_id: Self::EdgeId, weight: Weight) -> Option<Weight>;
}

pub trait IterEdges<Weight>: Base {
    type Edges<'a>: Iterator<Item = EdgeRef<'a, Self::EdgeId, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_edges<'a>(&'a self) -> Self::Edges<'a>;
}

pub trait IterNodes<Node>: Base {
    type Nodes<'a>: Iterator<Item = &'a Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a>;
}

pub trait IterNodesMut<Node>: Base {
    type NodesMut<'a>: Iterator<Item = &'a mut Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a>;
}

pub trait Remove<Node, Weight>: Base {
    fn remove_node(&mut self, node_id: Self::NodeId) -> Option<Node>;
    fn remove_edge(&mut self, edge_id: Self::EdgeId) -> Option<Weight>;
}

pub trait Reserve {
    fn reserve_nodes(&mut self, additional: usize);
    fn reserve_edges(&mut self, additional: usize);
}
