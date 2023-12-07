use crate::{
    collections::{GetNode, GetNodeMut, NodeCount, NodeIter, RemoveNode},
    graph::{NodeAttribute, NodeIterAdjacent},
    prelude::{NodeId, NodeRef},
    view::{View, ViewAdaptor, ViewGraph},
};

pub struct FilterNodeView<G: NodeAttribute> {
    nodes: G::FixedNodeMap<bool>,
}

impl<G: NodeAttribute> FilterNodeView<G> {
    pub fn new(nodes: G::FixedNodeMap<bool>) -> Self {
        Self { nodes }
    }

    pub fn keep(&mut self, node_id: NodeId<G::Key>) {
        self.nodes.update_node(node_id, true);
    }

    pub fn remove(&mut self, node_id: NodeId<G::Key>) {
        self.nodes.update_node(node_id, false);
    }

    // pub fn node_ids(&self) -> impl Iterator<Item = NodeId<G::Key>> + '_ {
    //     self.nodes.iter().map(|(key, _)| key)
    // }
}

impl<G: NodeAttribute> View for FilterNodeView<G> {}

impl<G: NodeAttribute + RemoveNode> ViewAdaptor<G> for FilterNodeView<G> {
    fn adapt(&self, graph: &mut G) {
        for node_id in self.nodes.node_ids() {
            graph.remove_node(node_id);
        }
    }
}

impl<'a, G> GetNode for ViewGraph<'a, G, FilterNodeView<G>>
where
    G: GetNode + NodeAttribute,
{
    // fn contains_edge(
    //     &self,
    //     from: crate::prelude::NodeId<Self::Key>,
    //     to: crate::prelude::NodeId<Self::Key>,
    // ) -> Option<crate::prelude::EdgeId<Self::Key>> {
    //     if *self.view.nodes.get(from) && *self.view.nodes.get(to) {
    //         self.graph.contains_edge(from, to)
    //     } else {
    //         None
    //     }
    // }

    fn node(
        &self,
        node_id: NodeId<Self::Key>,
    ) -> Option<crate::prelude::NodeRef<Self::Key, Self::NodeWeight>> {
        if self.view.nodes.contains_node_id(node_id) {
            self.graph.node(node_id)
        } else {
            None
        }
    }

    // fn has_node_weight(
    //     &self,
    //     node: &Self::NodeWeight,
    // ) -> Option<crate::prelude::NodeId<Self::Key>> {
    //     if let Some(node_id) = self.graph.has_node_weight(node) && self.view.nodes.contains_node_id(node_id) {
    //         Some(node_id)
    //     } else {
    //         None
    //     }
    // }
}

impl<'a, G> NodeCount for ViewGraph<'a, G, FilterNodeView<G>>
where
    G: NodeCount + NodeAttribute,
{
    fn node_count(&self) -> usize {
        self.view.nodes.node_count()
    }
}

impl<'b, G> NodeIter for ViewGraph<'b, G, FilterNodeView<G>>
where
    G: NodeIter + NodeAttribute,
{
    type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Key>> + 'a where Self: 'a;
    type Nodes<'a> = impl Iterator<Item = NodeRef<'a, Self::Key, Self::NodeWeight>> + 'a where Self: 'a;

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        self.view.nodes.node_ids()
    }

    fn iter_nodes(&self) -> Self::Nodes<'_> {
        self.graph
            .iter_nodes()
            .filter(|node| self.view.nodes.contains_node_id(node.node_id))
    }

    // fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
    //     self.graph.edge_ids().filter(|edge_id| {
    //         *self.view.nodes.get(edge_id.from()) && *self.view.nodes.get(edge_id.to())
    //     })
    // }
}

impl<'b, G> NodeIterAdjacent for ViewGraph<'b, G, FilterNodeView<G>>
where
    G: NodeIterAdjacent + NodeAttribute,
{
    type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Key>> + 'a where Self: 'a;
    type Nodes<'a> = impl Iterator<Item = NodeRef<'a, Self::Key, Self::NodeWeight>> + 'a where Self: 'a;
    // type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Key>> + 'a where Self: 'a;

    fn adjacent_node_ids<'a>(&'a self, node_id: NodeId<Self::Key>) -> Self::NodeIds<'a> {
        if self.view.nodes.contains_node_id(node_id) {
            Some(
                self.graph
                    .adjacent_node_ids(node_id)
                    .filter(|node_id| self.view.nodes.contains_node_id(*node_id)),
            )
            .into_iter()
            .flatten()
        } else {
            None.into_iter().flatten()
        }
    }

    fn iter_adjacent_nodes(&self, node_id: NodeId<Self::Key>) -> Self::Nodes<'_> {
        if self.view.nodes.contains_node_id(node_id) {
            Some(
                self.graph
                    .iter_adjacent_nodes(node_id)
                    .filter(|node| self.view.nodes.contains_node_id(node.node_id)),
            )
            .into_iter()
            .flatten()
        } else {
            None.into_iter().flatten()
        }
    }

    // fn adjacent_edge_ids<'a>(&'a self, node_id: NodeId<Self::Key>) -> Self::EdgeIds<'a> {
    //     if *self.view.nodes.get(node_id) {
    //         Some(self.graph.adjacent_edge_ids(node_id).filter(|edge_id| {
    //             *self.view.nodes.get(edge_id.from()) && *self.view.nodes.get(edge_id.to())
    //         }))
    //         .into_iter()
    //         .flatten()
    //     } else {
    //         None.into_iter().flatten()
    //     }
    // }
}
