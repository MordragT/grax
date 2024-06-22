use std::fmt::Debug;

use crate::{
    edge::{Edge, EdgeMut, EdgeRef},
    index::{EdgeId, NodeId},
    node::{Node, NodeMut, NodeRef},
    prelude::Identifier,
};

pub trait Keyed: Debug {
    type Key: Identifier;
}

pub trait NodeCollection {
    type NodeWeight;

    fn nodes_capacity(&self) -> usize;
}

pub trait EdgeCollection {
    type EdgeWeight;

    fn edges_capacity(&self) -> usize;
}

pub trait NodeCount {
    fn node_count(&self) -> usize;

    fn nodes_empty(&self) -> bool {
        self.node_count() == 0
    }
}

pub trait EdgeCount {
    fn edge_count(&self) -> usize;

    fn edges_empty(&self) -> bool {
        self.edge_count() == 0
    }
}

pub trait GetNode: NodeCollection + Keyed {
    fn node(&self, node_id: NodeId<Self::Key>) -> Option<NodeRef<Self::Key, Self::NodeWeight>>;
    // fn has_node_weight(&self, weight: &Self::NodeWeight) -> Option<NodeId<Self::Key>>;

    fn contains_node_id(&self, node_id: NodeId<Self::Key>) -> bool {
        self.node(node_id).is_some()
    }

    fn find_node_id(&self, key: Self::Key) -> Option<NodeId<Self::Key>> {
        let node_id = NodeId::new_unchecked(key);
        if self.contains_node_id(node_id) {
            Some(node_id)
        } else {
            None
        }
    }

    // fn contains_node_weight(&self, weight: &Self::NodeWeight) -> bool {
    //     self.has_node_weight(weight).is_some()
    // }
}

pub trait GetNodeMut: NodeCollection + Keyed {
    fn node_mut(
        &mut self,
        node_id: NodeId<Self::Key>,
    ) -> Option<NodeMut<Self::Key, Self::NodeWeight>>;

    /// Updates node only if it is existent
    fn update_node(
        &mut self,
        node_id: NodeId<Self::Key>,
        weight: Self::NodeWeight,
    ) -> Option<Self::NodeWeight> {
        if let Some(node) = self.node_mut(node_id) {
            Some(std::mem::replace(node.weight, weight))
        } else {
            None
        }
    }
}

pub trait GetEdge: EdgeCollection + Keyed {
    fn edge(&self, edge_id: EdgeId<Self::Key>) -> Option<EdgeRef<Self::Key, Self::EdgeWeight>>;

    // TODO maybe replace with const T check parameter in Id and then provide function which goes from EdgeId<Key, Unchecked> -> EdgeId<Key, Checked>
    // fn has_edge(&self, from: NodeId<Self::Key>, to: NodeId<Self::Key>)
    //     -> Option<EdgeId<Self::Key>>;

    fn contains_edge_id(&self, edge_id: EdgeId<Self::Key>) -> bool {
        self.edge(edge_id).is_some()
    }

    fn find_edge_id(
        &self,
        from: NodeId<Self::Key>,
        to: NodeId<Self::Key>,
    ) -> Option<EdgeId<Self::Key>> {
        let edge_id = EdgeId::new_unchecked(from, to);
        if self.contains_edge_id(edge_id) {
            Some(edge_id)
        } else {
            None
        }
    }
}

pub trait GetEdgeMut: EdgeCollection + Keyed {
    fn edge_mut(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<EdgeMut<Self::Key, Self::EdgeWeight>>;

    /// Updates edge only if it is existent
    fn update_edge(
        &mut self,
        edge_id: EdgeId<Self::Key>,
        weight: Self::EdgeWeight,
    ) -> Option<Self::EdgeWeight> {
        if let Some(edge) = self.edge_mut(edge_id) {
            Some(std::mem::replace(edge.weight, weight))
        } else {
            None
        }
    }
}

pub trait InsertNode: NodeCollection + Keyed {
    fn insert_node(&mut self, weight: Self::NodeWeight) -> NodeId<Self::Key>;
    fn reserve_nodes(&mut self, additional: usize);

    fn extend_nodes(&mut self, nodes: impl IntoIterator<Item = Self::NodeWeight>) {
        for node in nodes {
            self.insert_node(node);
        }
    }
}

pub trait InsertEdge: EdgeCollection + Keyed {
    /// Is allowed to panic if from or to are not in the graph
    fn insert_edge(
        &mut self,
        from: NodeId<Self::Key>,
        to: NodeId<Self::Key>,
        weight: Self::EdgeWeight,
    ) -> EdgeId<Self::Key>;
    fn reserve_edges(&mut self, additional: usize);

    /// Is allowed to panic if the specified nodes are not within the graph
    fn extend_edges(
        &mut self,
        edges: impl IntoIterator<Item = (NodeId<Self::Key>, NodeId<Self::Key>, Self::EdgeWeight)>,
    ) {
        for (from, to, weight) in edges {
            self.insert_edge(from, to, weight);
        }
    }
}

pub trait RemoveNode: NodeCollection + Keyed {
    fn remove_node(
        &mut self,
        node_id: NodeId<Self::Key>,
    ) -> Option<Node<Self::Key, Self::NodeWeight>>;
}

pub trait RemoveEdge: EdgeCollection + Keyed {
    fn remove_edge(
        &mut self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<Edge<Self::Key, Self::EdgeWeight>>;
}

// pub trait RetainNodes: NodeCollection + Keyed {
//     fn retain_nodes<F>(&mut self, visit: F)
//     where
//         F: FnMut(NodeRef<'_, Self::Key, Self::NodeWeight>) -> bool;
// }

// pub trait RetainEdges: EdgeCollection + Keyed {
//     fn retain_edges<F>(&mut self, visit: F)
//     where
//         F: FnMut(EdgeRef<'_, Self::Key, Self::EdgeWeight>) -> bool;
// }

pub trait NodeIter: NodeCollection + Keyed {
    type NodeIds<'a>: Iterator<Item = NodeId<Self::Key>> + 'a
    where
        Self: 'a;

    type Nodes<'a>: Iterator<Item = NodeRef<'a, Self::Key, Self::NodeWeight>> + 'a
    where
        Self::NodeWeight: 'a,
        Self: 'a;

    fn node_ids(&self) -> Self::NodeIds<'_>;
    /// This returns an iterator over all nodes in the graph.
    fn iter_nodes(&self) -> Self::Nodes<'_>;
}

pub trait NodeIterMut: NodeCollection + Keyed {
    type NodesMut<'a>: Iterator<Item = NodeMut<'a, Self::Key, Self::NodeWeight>> + 'a
    where
        Self::NodeWeight: 'a,
        Self: 'a;

    fn iter_nodes_mut(&mut self) -> Self::NodesMut<'_>;
}

pub trait EdgeIter: EdgeCollection + Keyed {
    type EdgeIds<'a>: Iterator<Item = EdgeId<Self::Key>> + 'a
    where
        Self: 'a;
    type Edges<'a>: Iterator<Item = EdgeRef<'a, Self::Key, Self::EdgeWeight>> + 'a
    where
        Self::EdgeWeight: 'a,
        Self: 'a;

    fn edge_ids(&self) -> Self::EdgeIds<'_>;
    /// This returns an iterator over all edges in the graph.
    fn iter_edges(&self) -> Self::Edges<'_>;
}

pub trait EdgeIterMut: EdgeCollection + Keyed {
    type EdgesMut<'a>: Iterator<Item = EdgeMut<'a, Self::Key, Self::EdgeWeight>> + 'a
    where
        Self::EdgeWeight: 'a,
        Self: 'a;

    fn iter_edges_mut(&mut self) -> Self::EdgesMut<'_>;
}

pub trait FixedNodeMap<K: Identifier, V>:
    NodeCollection<NodeWeight = V>
    + Keyed<Key = K>
    + NodeCount
    + GetNode
    + GetNodeMut
    + NodeIter
    + NodeIterMut
    + Debug
    + Clone
{
    fn get(&self, node_id: NodeId<K>) -> &V {
        self.node(node_id).map(|node| node.weight).unwrap()
    }

    fn get_mut(&mut self, node_id: NodeId<K>) -> &mut V {
        self.node_mut(node_id).map(|node| node.weight).unwrap()
    }
}

pub trait VisitNodeMap<K: Identifier>: Keyed<Key = K> {
    type IterVisited<'a>: Iterator<Item = NodeId<Self::Key>> + 'a
    where
        Self: 'a;
    type IterUnvisited<'a>: Iterator<Item = NodeId<Self::Key>> + 'a
    where
        Self: 'a;

    fn visit(&mut self, node_id: NodeId<Self::Key>);
    fn unvisit(&mut self, node_id: NodeId<Self::Key>);
    fn is_visited(&self, node_id: NodeId<Self::Key>) -> bool;
    fn all_visited(&self) -> bool;

    fn iter_visited(&self) -> Self::IterVisited<'_>;
    fn iter_unvisited(&self) -> Self::IterUnvisited<'_>;
}

impl<K: Identifier, T: FixedNodeMap<K, bool>> VisitNodeMap<K> for T {
    type IterUnvisited<'a> = impl Iterator<Item = NodeId<K>> + 'a where T: 'a;
    type IterVisited<'a> = impl Iterator<Item = NodeId<K>> + 'a where T: 'a;

    fn visit(&mut self, node_id: NodeId<K>) {
        self.update_node(node_id, true);
    }
    fn unvisit(&mut self, node_id: NodeId<K>) {
        self.update_node(node_id, false);
    }
    fn is_visited(&self, node_id: NodeId<K>) -> bool {
        *self.get(node_id)
    }
    fn all_visited(&self) -> bool {
        self.iter_nodes().all(|node| *node.weight)
    }

    fn iter_unvisited(&self) -> Self::IterUnvisited<'_> {
        self.iter_nodes().filter_map(|node| {
            if !*node.weight {
                Some(node.node_id)
            } else {
                None
            }
        })
    }

    fn iter_visited(&self) -> Self::IterVisited<'_> {
        self.iter_nodes().filter_map(|node| {
            if *node.weight {
                Some(node.node_id)
            } else {
                None
            }
        })
    }
}

pub trait NodeMap<K: Identifier, V>: FixedNodeMap<K, V> + InsertNode + RemoveNode {
    // get or insert
}

pub trait FixedEdgeMap<K: Identifier, V>:
    EdgeCollection<EdgeWeight = V>
    + Keyed<Key = K>
    + EdgeCount
    + GetEdge
    + GetEdgeMut
    + EdgeIter
    + EdgeIterMut
    + Clone
    + Debug
{
    fn get(&self, edge_id: EdgeId<K>) -> &V {
        self.edge(edge_id).map(|edge| edge.weight).unwrap()
    }

    fn get_mut(&mut self, edge_id: EdgeId<K>) -> &mut V {
        self.edge_mut(edge_id).map(|edge| edge.weight).unwrap()
    }
}

pub trait VisitEdgeMap<K: Identifier>: Keyed<Key = K> {
    type IterVisited<'a>: Iterator<Item = EdgeId<Self::Key>> + 'a
    where
        Self: 'a;
    type IterUnvisited<'a>: Iterator<Item = EdgeId<Self::Key>> + 'a
    where
        Self: 'a;

    fn visit(&mut self, edge_id: EdgeId<Self::Key>);
    fn unvisit(&mut self, edge_id: EdgeId<Self::Key>);
    fn is_visited(&self, edge_id: EdgeId<Self::Key>) -> bool;
    fn all_visited(&self) -> bool;
    fn iter_visited(&self) -> Self::IterVisited<'_>;
    fn iter_unvisited(&self) -> Self::IterUnvisited<'_>;
}

impl<K: Identifier, T: FixedEdgeMap<K, bool>> VisitEdgeMap<K> for T {
    type IterUnvisited<'a> = impl Iterator<Item = EdgeId<K>> + 'a where T: 'a;
    type IterVisited<'a> = impl Iterator<Item = EdgeId<K>> + 'a where T: 'a;

    fn visit(&mut self, edge_id: EdgeId<K>) {
        self.update_edge(edge_id, true);
    }
    fn unvisit(&mut self, edge_id: EdgeId<K>) {
        self.update_edge(edge_id, false);
    }
    fn is_visited(&self, edge_id: EdgeId<K>) -> bool {
        *self.get(edge_id)
    }
    fn all_visited(&self) -> bool {
        self.iter_edges().all(|edge| *edge.weight)
    }

    fn iter_unvisited(&self) -> Self::IterUnvisited<'_> {
        self.iter_edges().filter_map(|edge| {
            if !*edge.weight {
                Some(edge.edge_id)
            } else {
                None
            }
        })
    }

    fn iter_visited(&self) -> Self::IterVisited<'_> {
        self.iter_edges().filter_map(|edge| {
            if *edge.weight {
                Some(edge.edge_id)
            } else {
                None
            }
        })
    }
}

pub trait EdgeMap<K: Identifier, V>: FixedEdgeMap<K, V> + InsertEdge + RemoveEdge {
    // get or insert
}
