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

    pub fn has_parent(&self, child: NodeId<G::Key>) -> bool {
        self.parent(child).is_some()
    }

    pub fn contains_edge_id(&self, edge_id: EdgeId<G::Key>) -> bool {
        self.parent(edge_id.to())
            .is_some_and(|parent| parent == edge_id.from())
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

    // pub fn first(&self) -> Option<NodeId<G::Key>> {
    //     self.0
    //         .iter_nodes()
    //         .filter_map(|node| node.weight.as_ref())
    //         .next()
    //         .copied()
    // }

    pub fn node_ids(&self) -> impl Iterator<Item = NodeId<G::Key>> + '_ {
        self.0.node_ids()
    }

    pub fn iter(&self, mut from: NodeId<G::Key>) -> impl Iterator<Item = NodeId<G::Key>> + '_ {
        std::iter::from_fn(move || {
            if let Some(parent) = self.parent(from) {
                from = parent;
                Some(parent)
            } else {
                None
            }
        })
    }

    pub fn iter_edges(
        &self,
        mut from: NodeId<G::Key>,
    ) -> impl Iterator<Item = EdgeId<G::Key>> + '_ {
        std::iter::from_fn(move || {
            if let Some(parent) = self.parent(from) {
                let edge_id = EdgeId::new_unchecked(parent, from);
                from = parent;
                Some(edge_id)
            } else {
                None
            }
        })
    }

    /// Panics if there is no connection between source and sink
    pub fn iter_to(
        &self,
        source: NodeId<G::Key>,
        sink: NodeId<G::Key>,
    ) -> impl Iterator<Item = NodeId<G::Key>> + '_ {
        let mut to = sink;

        std::iter::from_fn(move || {
            if to != source {
                to = self.parent(to).unwrap();
                Some(to)
            } else {
                None
            }
        })
    }

    /// Panics if there is no connection between source and sink
    pub fn iter_edges_to(
        &self,
        source: NodeId<G::Key>,
        sink: NodeId<G::Key>,
    ) -> impl Iterator<Item = EdgeId<G::Key>> + '_ {
        let mut to = sink;

        std::iter::from_fn(move || {
            if to != source {
                let from = self.parent(to).unwrap();
                let edge_id = EdgeId::new_unchecked(from, to);
                to = from;
                Some(edge_id)
            } else {
                None
            }
        })
    }
}
