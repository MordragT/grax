use crate::{
    prelude::{EdgeId, NodeId},
    traits::{Contains, Count, Index, IndexAdjacent, Remove, Viewable},
    view::ViewAdaptor,
};

use super::{AttrMap, View, ViewGraph};

pub struct FilterNodeView<G: Viewable> {
    nodes: G::NodeMap<bool>,
}

impl<G: Viewable> FilterNodeView<G> {
    pub fn new(nodes: G::NodeMap<bool>) -> Self {
        Self { nodes }
    }

    pub fn keep(&mut self, node_id: NodeId<G::Id>) {
        *self.nodes.get_mut(node_id) = true;
    }

    pub fn remove(&mut self, node_id: NodeId<G::Id>) {
        *self.nodes.get_mut(node_id) = false;
    }

    pub fn node_ids(&self) -> impl Iterator<Item = NodeId<G::Id>> + '_ {
        self.nodes.iter().map(|(key, _)| key)
    }
}

impl<G: Viewable> View for FilterNodeView<G> {}

impl<G: Viewable + Remove> ViewAdaptor<G> for FilterNodeView<G> {
    fn adapt(&self, graph: &mut G) {
        for node_id in self.node_ids() {
            graph.remove_node(node_id);
        }
    }
}

impl<'a, G> Contains for ViewGraph<'a, G, FilterNodeView<G>>
where
    G: Contains + Viewable,
{
    fn contains_edge(
        &self,
        from: crate::prelude::NodeId<Self::Id>,
        to: crate::prelude::NodeId<Self::Id>,
    ) -> Option<crate::prelude::EdgeId<Self::Id>> {
        if *self.view.nodes.get(from) && *self.view.nodes.get(to) {
            self.graph.contains_edge(from, to)
        } else {
            None
        }
    }

    fn contains_node(&self, node: &Self::NodeWeight) -> Option<crate::prelude::NodeId<Self::Id>> {
        if let Some(node_id) = self.graph.contains_node(node) && *self.view.nodes.get(node_id) {
            Some(node_id)
        } else {
            None
        }
    }
}

impl<'a, G> Count for ViewGraph<'a, G, FilterNodeView<G>>
where
    G: Count + Viewable,
{
    fn node_count(&self) -> usize {
        self.view.nodes.count()
    }

    fn edge_count(&self) -> usize {
        todo!()
    }
}

impl<'b, G> Index for ViewGraph<'b, G, FilterNodeView<G>>
where
    G: Index + Viewable,
{
    type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Id>> + 'a where Self: 'a;
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Id>> + 'a where Self: 'a;

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        self.view.nodes.iter().map(|(key, _)| key)
    }

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.graph.edge_ids().filter(|edge_id| {
            *self.view.nodes.get(edge_id.from()) && *self.view.nodes.get(edge_id.to())
        })
    }
}

impl<'b, G> IndexAdjacent for ViewGraph<'b, G, FilterNodeView<G>>
where
    G: IndexAdjacent + Viewable,
{
    type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Id>> + 'a where Self: 'a;
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Id>> + 'a where Self: 'a;

    fn adjacent_node_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::NodeIds<'a> {
        if *self.view.nodes.get(node_id) {
            Some(
                self.graph
                    .adjacent_node_ids(node_id)
                    .filter(|node_id| *self.view.nodes.get(*node_id)),
            )
            .into_iter()
            .flatten()
        } else {
            None.into_iter().flatten()
        }
    }

    fn adjacent_edge_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::EdgeIds<'a> {
        if *self.view.nodes.get(node_id) {
            Some(self.graph.adjacent_edge_ids(node_id).filter(|edge_id| {
                *self.view.nodes.get(edge_id.from()) && *self.view.nodes.get(edge_id.to())
            }))
            .into_iter()
            .flatten()
        } else {
            None.into_iter().flatten()
        }
    }
}
