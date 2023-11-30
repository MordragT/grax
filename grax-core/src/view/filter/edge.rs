use crate::{
    prelude::{EdgeId, NodeId},
    traits::{Contains, Count, Index, IndexAdjacent, Remove, Viewable},
    view::ViewAdaptor,
};

use super::{AttrMap, View, ViewGraph};

pub struct FilterEdgeView<G: Viewable> {
    edges: G::EdgeMap<bool>,
}

impl<G: Viewable> FilterEdgeView<G> {
    pub fn new(edges: G::EdgeMap<bool>) -> Self {
        Self { edges }
    }

    pub fn keep(&mut self, edge_id: EdgeId<G::Id>) {
        *self.edges.get_mut(edge_id) = true;
    }

    pub fn remove(&mut self, edge_id: EdgeId<G::Id>) {
        *self.edges.get_mut(edge_id) = false;
    }

    pub fn edge_ids(&self) -> impl Iterator<Item = EdgeId<G::Id>> + '_ {
        self.edges.iter().map(|(key, _)| key)
    }
}

impl<G: Viewable> View for FilterEdgeView<G> {}

impl<G: Viewable + Remove> ViewAdaptor<G> for FilterEdgeView<G> {
    fn adapt(&self, graph: &mut G) {
        for edge_id in self.edge_ids() {
            graph.remove_edge(edge_id);
        }
    }
}

impl<'a, G> Contains for ViewGraph<'a, G, FilterEdgeView<G>>
where
    G: Contains + Viewable,
{
    fn contains_edge(
        &self,
        from: crate::prelude::NodeId<Self::Id>,
        to: crate::prelude::NodeId<Self::Id>,
    ) -> Option<crate::prelude::EdgeId<Self::Id>> {
        if let Some(edge_id) = self.graph.contains_edge(from, to) && *self.view.edges.get(edge_id) {
            Some(edge_id)
        } else {
            None
        }
    }

    fn contains_node(&self, node: &Self::NodeWeight) -> Option<crate::prelude::NodeId<Self::Id>> {
        self.graph.contains_node(node)
    }
}

impl<'a, G> Count for ViewGraph<'a, G, FilterEdgeView<G>>
where
    G: Count + Viewable,
{
    fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    fn edge_count(&self) -> usize {
        self.view
            .edges
            .iter()
            .filter(|(_, exists)| **exists)
            .count()
    }
}

impl<'b, G> Index for ViewGraph<'b, G, FilterEdgeView<G>>
where
    G: Index + Viewable,
{
    type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Id>> + 'a where Self: 'a;
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Id>> + 'a where Self: 'a;

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.view.edge_ids()
    }

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        self.graph.node_ids()
    }
}

impl<'b, G> IndexAdjacent for ViewGraph<'b, G, FilterEdgeView<G>>
where
    G: IndexAdjacent + Viewable,
{
    type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Id>> + 'a where Self: 'a;
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Id>> + 'a where Self: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::EdgeIds<'a> {
        self.graph
            .adjacent_edge_ids(node_id)
            .filter(|edge_id| *self.view.edges.get(*edge_id))
    }

    fn adjacent_node_ids<'a>(
        &'a self,
        node_id: crate::prelude::NodeId<Self::Id>,
    ) -> Self::NodeIds<'a> {
        self.graph
            .adjacent_node_ids(node_id)
            .filter(move |to| *self.view.edges.get(EdgeId::new_unchecked(node_id, *to)))
    }
}
