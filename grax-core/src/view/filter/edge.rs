use crate::{
    collections::{EdgeCount, EdgeIter, GetEdge, RemoveEdge, VisitEdgeMap},
    graph::{EdgeAttribute, EdgeIterAdjacent},
    prelude::{EdgeId, EdgeRef, NodeId},
    view::{View, ViewAdaptor, ViewGraph},
};

// TODO replace bool with marker and add filter function on marker

// TODO do not assume that the edgemap already filtered out edge_ids etc.

#[derive(Debug)]
pub struct FilterEdgeView<G: EdgeAttribute> {
    edges: G::VisitEdgeMap,
}

impl<G: EdgeAttribute> FilterEdgeView<G> {
    pub fn new(graph: &G) -> Self {
        let edges = graph.visit_edge_map();
        Self { edges }
    }

    pub fn keep(&mut self, edge_id: EdgeId<G::Key>) {
        self.edges.visit(edge_id);
    }

    pub fn remove(&mut self, edge_id: EdgeId<G::Key>) {
        self.edges.unvisit(edge_id);
    }

    pub fn is_kept(&self, edge_id: EdgeId<G::Key>) -> bool {
        self.edges.is_visited(edge_id)
    }

    // pub fn edge_ids(&self) -> impl Iterator<Item = EdgeId<G::Key>> + '_ {
    //     self.edges.iter().map(|(key, _)| key)
    // }
}

impl<G: EdgeAttribute> View for FilterEdgeView<G> {}

impl<G: EdgeAttribute + RemoveEdge> ViewAdaptor<G> for FilterEdgeView<G> {
    fn adapt(&self, graph: &mut G) {
        for edge_id in self.edges.iter_unvisited() {
            graph.remove_edge(edge_id);
        }
    }
}

impl<'a, G> GetEdge for ViewGraph<'a, G, FilterEdgeView<G>>
where
    G: GetEdge + EdgeAttribute,
{
    fn edge(
        &self,
        edge_id: EdgeId<Self::Key>,
    ) -> Option<crate::prelude::EdgeRef<Self::Key, Self::EdgeWeight>> {
        if let Some(edge) = self.graph.edge(edge_id) && self.view.is_kept(edge_id) {
            Some(edge)
        } else {
            None
        }
    }
    // fn find_edge(
    //     &self,
    //     from: crate::prelude::NodeId<Self::Key>,
    //     to: crate::prelude::NodeId<Self::Key>,
    // ) -> Option<crate::prelude::EdgeId<Self::Key>> {
    //     if let Some(edge_id) = self.graph.has_edge(from, to) && self.view.is_kept((edge_id) {
    //         Some(edge_id)
    //     } else {
    //         None
    //     }
    // }
}

impl<'a, G> EdgeCount for ViewGraph<'a, G, FilterEdgeView<G>>
where
    G: EdgeCount + EdgeAttribute,
{
    fn edge_count(&self) -> usize {
        self.view.edges.edge_count()
    }
}

impl<'b, G> EdgeIter for ViewGraph<'b, G, FilterEdgeView<G>>
where
    G: EdgeIter + EdgeAttribute,
{
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, Self::Key, Self::EdgeWeight>> + 'a where Self: 'a;
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Key>> + 'a where Self: 'a;

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.view.edges.iter_visited()
    }

    fn iter_edges(&self) -> Self::Edges<'_> {
        self.graph
            .iter_edges()
            .filter(|edge| self.view.is_kept(edge.edge_id))
    }
}

impl<'b, G> EdgeIterAdjacent for ViewGraph<'b, G, FilterEdgeView<G>>
where
    G: EdgeIterAdjacent + EdgeAttribute,
{
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, Self::Key, Self::EdgeWeight>> + 'a where Self: 'a;
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Key>> + 'a where Self: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: NodeId<Self::Key>) -> Self::EdgeIds<'a> {
        self.graph
            .adjacent_edge_ids(node_id)
            .filter(|edge_id| self.view.is_kept(*edge_id))
    }

    fn iter_adjacent_edges(&self, node_id: NodeId<Self::Key>) -> Self::Edges<'_> {
        self.graph
            .iter_adjacent_edges(node_id)
            .filter(|edge| self.view.is_kept(edge.edge_id))
    }

    // fn adjacent_node_ids<'a>(
    //     &'a self,
    //     node_id: crate::prelude::NodeId<Self::Key>,
    // ) -> Self::NodeIds<'a> {
    //     self.graph
    //         .adjacent_node_ids(node_id)
    //         .filter(move |to| *self.view.edges.get(EdgeId::new_unchecked(node_id, *to)))
    // }
}
