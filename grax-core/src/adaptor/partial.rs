use crate::{
    edge::Edge,
    prelude::{EdgeId, NodeId},
    traits::{
        Base, Capacity, Clear, Contains, Count, Create, Get, GetMut, Index, IndexAdjacent, Insert,
        Iter, IterAdjacent, IterAdjacentMut, IterMut, Reserve, Viewable, Visitable,
    },
    view::AttrMap,
};

pub struct PartialGraphAdaptor<'a, G: Viewable> {
    graph: &'a G,
    edges: G::EdgeMap<bool>,
    nodes: G::NodeMap<bool>,
}

impl<'a, G: Viewable> PartialGraphAdaptor<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        Self {
            graph,
            edges: graph.edge_map(),
            nodes: graph.node_map(),
        }
    }

    pub fn keep_edge_id(&mut self, edge_id: EdgeId<G::Id>) {
        *self.nodes.get_mut(edge_id.from()) = true;
        *self.nodes.get_mut(edge_id.to()) = true;
        *self.edges.get_mut(edge_id) = true;
    }

    pub fn keep_node_id(&mut self, node_id: NodeId<G::Id>) {
        *self.nodes.get_mut(node_id) = true;
    }
}

impl<'a, G: Viewable + Contains> PartialGraphAdaptor<'a, G> {
    pub fn keep_edge(&mut self, from: NodeId<G::Id>, to: NodeId<G::Id>) -> Option<EdgeId<G::Id>> {
        match self.graph.contains_edge(from, to) {
            Some(edge_id) => {
                self.keep_edge_id(edge_id);
                Some(edge_id)
            }
            None => None,
        }
    }

    pub fn keep_node(&mut self, node: &G::Node) -> Option<NodeId<G::Id>> {
        match self.graph.contains_node(node) {
            Some(node_id) => {
                *self.nodes.get_mut(node_id) = true;
                Some(node_id)
            }
            None => None,
        }
    }
}

impl<'a, G: Viewable> Base for PartialGraphAdaptor<'a, G> {
    type Id = G::Id;
    type Node = G::Node;
    type Weight = G::Weight;
}

impl<'a, G: Viewable + Contains> Contains for PartialGraphAdaptor<'a, G> {
    fn contains_edge(
        &self,
        from: NodeId<Self::Id>,
        to: NodeId<Self::Id>,
    ) -> Option<crate::prelude::EdgeId<Self::Id>> {
        match self.graph.contains_edge(from, to) {
            Some(edge_id) => {
                if *self.edges.get(edge_id) {
                    Some(edge_id)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn contains_node(&self, node: &Self::Node) -> Option<NodeId<Self::Id>> {
        match self.graph.contains_node(node) {
            Some(node_id) => {
                if *self.nodes.get(node_id) {
                    Some(node_id)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl<'a, G: Viewable + Count> Count for PartialGraphAdaptor<'a, G> {
    fn node_count(&self) -> usize {
        self.nodes.iter().filter(|(_, keep)| **keep).count()
    }

    fn edge_count(&self) -> usize {
        self.edges.iter().filter(|(_, keep)| **keep).count()
    }
}

impl<'a, G: Viewable + Get> Get for PartialGraphAdaptor<'a, G> {
    fn node(&self, node_id: NodeId<Self::Id>) -> Option<&Self::Node> {
        if *self.nodes.get(node_id) {
            self.graph.node(node_id)
        } else {
            None
        }
    }

    fn weight(&self, edge_id: crate::prelude::EdgeId<Self::Id>) -> Option<&Self::Weight> {
        if *self.edges.get(edge_id) {
            self.graph.weight(edge_id)
        } else {
            None
        }
    }
}

// impl<'a, G: Viewable + GetMut> GetMut for PartialGraphAdaptor<'a, G> {
//     fn node_mut(&mut self, node_id: NodeId<Self::Id>) -> Option<&mut Self::Node> {
//         if *self.nodes.get(node_id) {
//             self.graph.node_mut(node_id)
//         } else {
//             None
//         }
//     }

//     fn weight_mut(
//         &mut self,
//         edge_id: crate::prelude::EdgeId<Self::Id>,
//     ) -> Option<&mut Self::Weight> {
//         if *self.edges.get(edge_id) {
//             self.graph.weight_mut(edge_id)
//         } else {
//             None
//         }
//     }
// }

impl<'a, G: Viewable + Index> Index for PartialGraphAdaptor<'a, G> {
    type EdgeIds<'b> = impl Iterator<Item = EdgeId<G::Id>> + 'b where Self: 'b ;
    type NodeIds<'b> = impl Iterator<Item = NodeId<G::Id>> + 'b where Self: 'b;

    fn node_ids<'b>(&'b self) -> Self::NodeIds<'b> {
        self.graph
            .node_ids()
            .filter(|node_id| *self.nodes.get(*node_id))
    }

    fn edge_ids<'b>(&'b self) -> Self::EdgeIds<'b> {
        self.graph
            .edge_ids()
            .filter(|edge_id| *self.edges.get(*edge_id))
    }
}

impl<'a, G: Viewable + IndexAdjacent> IndexAdjacent for PartialGraphAdaptor<'a, G> {
    type AdjacentEdgeIds<'b> = impl Iterator<Item = EdgeId<G::Id>> + 'b where Self: 'b ;
    type AdjacentNodeIds<'b> = impl Iterator<Item = NodeId<G::Id>> + 'b where Self: 'b;

    fn adjacent_edge_ids<'b>(&'b self, node_id: NodeId<Self::Id>) -> Self::AdjacentEdgeIds<'b> {
        if *self.nodes.get(node_id) {
            Some(
                self.graph
                    .adjacent_edge_ids(node_id)
                    .filter(|edge_id| *self.edges.get(*edge_id)),
            )
            .into_iter()
            .flatten()
        } else {
            None.into_iter().flatten()
        }
    }

    fn adjacent_node_ids<'b>(&'b self, node_id: NodeId<Self::Id>) -> Self::AdjacentNodeIds<'b> {
        if *self.nodes.get(node_id) {
            Some(
                self.graph
                    .adjacent_node_ids(node_id)
                    .filter(|node_id| *self.nodes.get(*node_id)),
            )
            .into_iter()
            .flatten()
        } else {
            None.into_iter().flatten()
        }
    }
}

impl<'a, G: Viewable> Viewable for PartialGraphAdaptor<'a, G> {
    type EdgeMap<Attr: Clone + std::fmt::Debug + Default> = G::EdgeMap<Attr>;
    type NodeMap<Attr: Clone + std::fmt::Debug + Default> = G::NodeMap<Attr>;

    fn edge_map<Attr: Clone + std::fmt::Debug + Default>(&self) -> Self::EdgeMap<Attr> {
        self.graph.edge_map()
    }

    fn node_map<Attr: Clone + std::fmt::Debug + Default>(&self) -> Self::NodeMap<Attr> {
        self.graph.node_map()
    }

    // fn update_edge_map<Attr: Clone + std::fmt::Debug + Default>(
    //     &self,
    //     map: &mut Self::EdgeMap<Attr>,
    // ) {
    //     self.graph.update_edge_map(map)
    // }

    // fn update_node_map<Attr: Clone + std::fmt::Debug + Default>(
    //     &self,
    //     map: &mut Self::NodeMap<Attr>,
    // ) {
    //     self.graph.update_node_map(map)
    // }
}

impl<'a, G: Viewable + Visitable> Visitable for PartialGraphAdaptor<'a, G> {
    type VisitMap = G::VisitMap;

    fn visit_map(&self) -> Self::VisitMap {
        self.graph.visit_map()
    }
}
