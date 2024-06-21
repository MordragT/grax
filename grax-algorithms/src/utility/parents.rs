use grax_core::{
    collections::{GetNode, GetNodeMut, NodeCount, NodeIter},
    graph::NodeAttribute,
    prelude::{EdgeId, NodeId},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parents<G: NodeAttribute>(G::FixedNodeMap<Option<NodeId<G::Key>>>);

impl<G: NodeAttribute> Parents<G> {
    pub fn new(graph: &G) -> Self {
        let parents = graph.fixed_node_map(None);
        Self(parents)
    }

    pub fn count(&self) -> usize {
        self.0.node_count()
    }

    pub fn is_empty(&self) -> bool {
        self.0.nodes_empty()
    }

    pub fn insert(&mut self, from: NodeId<G::Key>, to: NodeId<G::Key>) -> Option<NodeId<G::Key>> {
        self.0.update_node(to, Some(from)).flatten()
    }

    pub fn parent(&self, child: NodeId<G::Key>) -> Option<NodeId<G::Key>> {
        self.0.node(child).and_then(|parent| *parent.weight)
    }

    pub fn edge_ids(&self) -> impl Iterator<Item = EdgeId<G::Key>> + '_ {
        self.0.iter_nodes().filter_map(|node| {
            if let Some(parent) = node.weight {
                let child = node.node_id;
                Some(EdgeId::new_unchecked(*parent, child))
            } else {
                None
            }
        })
    }

    /// Panics if there is no connection between source and sink
    pub fn iter_parents(
        &self,
        source: NodeId<G::Key>,
        sink: NodeId<G::Key>,
    ) -> impl Iterator<Item = NodeId<G::Key>> + '_
    where
        G::Key: 'static,
    {
        let mut to = sink;

        std::iter::from_fn(move || {
            while to != source {
                let from = self.parent(to).unwrap();
                to = from;
                return Some(from);
            }
            None
        })
    }

    /// Panics if there is no connection between source and sink
    pub fn iter_parent_edges(
        &self,
        source: NodeId<G::Key>,
        sink: NodeId<G::Key>,
    ) -> impl Iterator<Item = EdgeId<G::Key>> + '_
    where
        G::Key: 'static,
    {
        let mut to = sink;

        std::iter::from_fn(move || {
            while to != source {
                let from = self.parent(to).unwrap();
                to = from;
                return Some(EdgeId::new_unchecked(from, to));
            }
            None
        })
    }
}
