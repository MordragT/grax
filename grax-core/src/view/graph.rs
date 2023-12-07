use std::fmt::Debug;

use crate::{
    collections::{EdgeCollection, EdgeCount, GetEdge, GetNode, Keyed, NodeCollection, NodeCount},
    graph::{Directed, EdgeAttribute, NodeAttribute},
    prelude::{EdgeId, EdgeRef, NodeId, NodeRef},
};

use super::View;

#[derive(Debug, Clone)]
pub struct ViewGraph<'a, G, V> {
    pub(crate) graph: &'a G,
    pub(crate) view: V,
}

impl<'a, G, V> ViewGraph<'a, G, V>
where
    G: Keyed,
    V: View,
{
    pub fn new(graph: &'a G, view: V) -> Self {
        Self { graph, view }
    }
}

impl<'a, G, V> Keyed for ViewGraph<'a, G, V>
where
    G: Keyed,
    V: View,
{
    type Key = G::Key;
}

impl<'a, G, V> EdgeCollection for ViewGraph<'a, G, V>
where
    G: EdgeCollection,
    V: View,
{
    type EdgeWeight = G::EdgeWeight;

    fn edges_capacity(&self) -> usize {
        self.graph.edges_capacity()
    }
}

impl<'a, G, V> NodeCollection for ViewGraph<'a, G, V>
where
    G: NodeCollection,
    V: View,
{
    type NodeWeight = G::NodeWeight;

    fn nodes_capacity(&self) -> usize {
        self.graph.nodes_capacity()
    }
}

impl<'a, G, V> GetEdge for ViewGraph<'a, G, V>
where
    G: GetEdge,
    V: View,
{
    default fn edge(
        &self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<EdgeRef<Self::Key, Self::EdgeWeight>> {
        self.graph.edge(edge_id)
    }

    // default fn has_edge(
    //     &self,
    //     from: NodeId<Self::Key>,
    //     to: NodeId<Self::Key>,
    // ) -> Option<EdgeId<Self::Key>> {
    //     self.graph.has_edge(from, to)
    // }
}

impl<'a, G, V> GetNode for ViewGraph<'a, G, V>
where
    G: GetNode,
    V: View,
{
    default fn node(
        &self,
        node_id: NodeId<Self::Key>,
    ) -> Option<NodeRef<Self::Key, Self::NodeWeight>> {
        self.graph.node(node_id)
    }

    // default fn has_node_weight(&self, node: &Self::NodeWeight) -> Option<NodeId<Self::Key>> {
    //     self.graph.has_node_weight(node)
    // }
}

impl<'a, G, V> NodeCount for ViewGraph<'a, G, V>
where
    G: NodeCount,
    V: View,
{
    default fn node_count(&self) -> usize {
        self.graph.node_count()
    }
}

impl<'a, G, V> EdgeCount for ViewGraph<'a, G, V>
where
    G: EdgeCount,
    V: View,
{
    default fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

//  IIterMut,  IterAdjacentMut, Balance, Flow, Cost, Create, Directed

impl<'a, G, V> Directed for ViewGraph<'a, G, V>
where
    G: Directed,
    V: View,
{
    fn directed() -> bool {
        G::directed()
    }
}

impl<'a, G, V> NodeAttribute for ViewGraph<'a, G, V>
where
    G: NodeAttribute,
    V: View,
{
    type FixedNodeMap<T: Debug + Clone> = G::FixedNodeMap<T>;
    type NodeMap<T: Debug + Clone> = G::NodeMap<T>;
    type VisitNodeMap = G::VisitNodeMap;

    fn fixed_node_map<T: Debug + Clone>(&self, fill: T) -> Self::FixedNodeMap<T> {
        self.graph.fixed_node_map(fill)
    }

    fn node_map<T: Debug + Clone>(&self) -> Self::NodeMap<T> {
        self.graph.node_map()
    }

    fn visit_node_map(&self) -> Self::VisitNodeMap {
        self.graph.visit_node_map()
    }
}

impl<'a, G, V> EdgeAttribute for ViewGraph<'a, G, V>
where
    G: EdgeAttribute,
    V: View,
{
    type FixedEdgeMap<T: Debug + Clone> = G::FixedEdgeMap<T>;
    type EdgeMap<T: Debug + Clone> = G::EdgeMap<T>;
    type VisitEdgeMap = G::VisitEdgeMap;

    fn fixed_edge_map<T: Debug + Clone>(&self, fill: T) -> Self::FixedEdgeMap<T> {
        self.graph.fixed_edge_map(fill)
    }

    fn edge_map<T: Debug + Clone>(&self) -> Self::EdgeMap<T> {
        self.graph.edge_map()
    }

    fn visit_edge_map(&self) -> Self::VisitEdgeMap {
        self.graph.visit_edge_map()
    }
}
