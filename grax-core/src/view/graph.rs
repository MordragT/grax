use std::fmt::Debug;

use crate::{
    prelude::{EdgeId, NodeId},
    traits::{
        Base, Capacity, Contains, Count, Directed, Get, Index, IndexAdjacent, Iter, IterAdjacent,
        Viewable, Visitable,
    },
};

use super::View;

#[derive(Debug, Clone)]
pub struct ViewGraph<'a, G: Base, V: View> {
    pub(crate) graph: &'a G,
    pub(crate) view: V,
}

impl<'a, G, V> ViewGraph<'a, G, V>
where
    G: Base,
    V: View,
{
    pub fn new(graph: &'a G, view: V) -> Self {
        Self { graph, view }
    }
}

impl<'a, G, V> Base for ViewGraph<'a, G, V>
where
    G: Base,
    V: View,
{
    type Id = G::Id;
    type NodeWeight = G::NodeWeight;
    type EdgeWeight = G::EdgeWeight;
}

// delegate: Clear, Extend, GetMut, Insert, Remove, Reserve

impl<'a, G, V> Capacity for ViewGraph<'a, G, V>
where
    G: Base + Capacity,
    V: View,
{
    default fn edges_capacity(&self) -> usize {
        self.graph.edges_capacity()
    }

    default fn nodes_capacity(&self) -> usize {
        self.graph.nodes_capacity()
    }
}

impl<'a, G, V> Contains for ViewGraph<'a, G, V>
where
    G: Contains,
    V: View,
{
    default fn contains_edge(
        &self,
        from: NodeId<Self::Id>,
        to: NodeId<Self::Id>,
    ) -> Option<crate::prelude::EdgeId<Self::Id>> {
        self.graph.contains_edge(from, to)
    }

    default fn contains_node(&self, node: &Self::NodeWeight) -> Option<NodeId<Self::Id>> {
        self.graph.contains_node(node)
    }
}

impl<'a, G, V> Get for ViewGraph<'a, G, V>
where
    G: Get,
    V: View,
{
    fn node(
        &self,
        node_id: NodeId<Self::Id>,
    ) -> Option<crate::node::NodeRef<Self::Id, Self::NodeWeight>> {
        self.graph.node(node_id)
    }

    fn edge(
        &self,
        edge_id: EdgeId<Self::Id>,
    ) -> Option<crate::edge::EdgeRef<Self::Id, Self::EdgeWeight>> {
        self.graph.edge(edge_id)
    }
}
impl<'a, G, V> Count for ViewGraph<'a, G, V>
where
    G: Base + Count,
    V: View,
{
    default fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    default fn node_count(&self) -> usize {
        self.graph.node_count()
    }
}

//  IIterMut,  IterAdjacentMut, Balance, Flow, Cost, Create, Directed

impl<'a, G, V> Directed for ViewGraph<'a, G, V>
where
    G: Base + Directed,
    V: View,
{
    fn directed() -> bool {
        G::directed()
    }
}

impl<'a, G, V> Viewable for ViewGraph<'a, G, V>
where
    G: Viewable,
    V: View,
{
    type EdgeMap<Attr: Clone + Debug + Default> = G::EdgeMap<Attr>;
    type NodeMap<Attr: Clone + Debug + Default> = G::NodeMap<Attr>;

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

    fn edge_map<Attr: Clone + Debug + Default>(&self) -> Self::EdgeMap<Attr> {
        self.graph.edge_map()
    }

    fn node_map<Attr: Clone + Debug + Default>(&self) -> Self::NodeMap<Attr> {
        self.graph.node_map()
    }
}

impl<'a, G, V> Visitable for ViewGraph<'a, G, V>
where
    G: Visitable,
    V: View,
{
    type VisitMap = G::VisitMap;

    fn visit_map(&self) -> Self::VisitMap {
        self.graph.visit_map()
    }
}

// impl<'b, G, V> Index for ViewGraph<'b, G, V>
// where
//     G: Index,
//     V: View,
// {
//     type EdgeIds<'a> = G::EdgeIds<'a> where G: 'a, Self: 'a;
//     type NodeIds<'a> = G::NodeIds<'a> where G: 'a, Self: 'a;
//     // default type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Id>> + 'a where Self: 'a;
//     // default type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Id>> + 'a where Self: 'a;

//     fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
//         self.graph.edge_ids()
//     }

//     fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
//         self.graph.node_ids()
//     }
// }

// impl<'b, G, V> IndexAdjacent for ViewGraph<'b, G, V>
// where
//     G: IndexAdjacent,
//     V: View,
// {
//     type AdjacentEdgeIds<'a> = G::AdjacentEdgeIds<'a> where G: 'a, Self: 'a;
//     type AdjacentNodeIds<'a> = G::AdjacentNodeIds<'a> where G: 'a, Self: 'a;

//     fn adjacent_edge_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::AdjacentEdgeIds<'a> {
//         self.graph.adjacent_edge_ids(node_id)
//     }

//     fn adjacent_node_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::AdjacentNodeIds<'a> {
//         self.graph.adjacent_node_ids(node_id)
//     }
// }

// impl<'b, G, V> Iter for ViewGraph<'b, G, V>
// where
//     G: Iter,
//     V: View,
// {
//     type Edges<'a> = G::Edges<'a> where G: 'a, Self: 'a;
//     type Nodes<'a> = G::Nodes<'a> where G: 'a, Self: 'a;

//     fn iter_edges<'a>(&'a self) -> Self::Edges<'a> {
//         self.graph.iter_edges()
//     }

//     fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a> {
//         self.graph.iter_nodes()
//     }
// }

// impl<'b, G, V> IterAdjacent for ViewGraph<'b, G, V>
// where
//     G: IterAdjacent,
//     V: View,
// {
//     type Edges<'a> = G::Edges<'a> where G: 'a, Self: 'a;
//     type Nodes<'a> = G::Nodes<'a> where G: 'a, Self: 'a;

//     fn iter_adjacent_edges<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::Edges<'a> {
//         self.graph.iter_adjacent_edges(node_id)
//     }

//     fn iter_adjacent_nodes<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::Nodes<'a> {
//         self.graph.iter_adjacent_nodes(node_id)
//     }
// }
