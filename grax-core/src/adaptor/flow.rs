use std::fmt::Debug;

use crate::{
    edge::{EdgeRef, Maximum},
    prelude::NodeId,
    traits::{
        Base, Capacity, Contains, Cost, Count, Flow, Index, IndexAdjacent, Iter, IterAdjacent,
        Viewable, Visitable,
    },
    variant::flow::FlowBundle,
    view::AttrMap,
};

pub struct FlowGraphAdaptor<'a, G: Viewable + Base<Weight: Clone + Debug>> {
    graph: &'a G,
    flow_map: G::EdgeMap<Option<FlowBundle<G::Weight>>>,
}

impl<'a, G> FlowGraphAdaptor<'a, G>
where
    G: Viewable + Base<Weight: Copy + Debug + Default + Maximum> + Iter,
{
    pub fn new(graph: &'a G) -> Self {
        let mut flow_map = graph.edge_map();

        for EdgeRef { edge_id, weight } in graph.iter_edges() {
            let bundle = FlowBundle {
                cost: *weight,
                capacity: Maximum::MAX,
                flow: Default::default(),
                reverse: false,
            };

            *flow_map.get_mut(edge_id) = Some(bundle);
        }

        Self { graph, flow_map }
    }
}

impl<'a, G> Base for FlowGraphAdaptor<'a, G>
where
    G: Viewable + Base<Weight: Clone + Debug>,
{
    type Id = G::Id;
    type Node = G::Node;
    type Weight = G::Weight;
}

impl<'a, G> Flow<G::Weight> for FlowGraphAdaptor<'a, G>
where
    G: Viewable + Base<Weight: Clone + Debug>,
{
    type EdgeFlow = FlowBundle<G::Weight>;

    fn flow(&self, edge_id: crate::prelude::EdgeId<Self::Id>) -> Option<&Self::EdgeFlow> {
        self.flow_map.get(edge_id).as_ref()
    }

    fn flow_mut(
        &mut self,
        edge_id: crate::prelude::EdgeId<Self::Id>,
    ) -> Option<&mut Self::EdgeFlow> {
        self.flow_map.get_mut(edge_id).as_mut()
    }
}

impl<'a, G> Cost<G::Weight> for FlowGraphAdaptor<'a, G>
where
    G: Viewable + Base<Weight: Clone + Debug>,
{
    type EdgeCost = FlowBundle<G::Weight>;

    fn cost(&self, edge_id: crate::prelude::EdgeId<Self::Id>) -> Option<&Self::EdgeCost> {
        self.flow_map.get(edge_id).as_ref()
    }

    fn cost_mut(
        &mut self,
        edge_id: crate::prelude::EdgeId<Self::Id>,
    ) -> Option<&mut Self::EdgeCost> {
        self.flow_map.get_mut(edge_id).as_mut()
    }
}

// impl<'a, G> Deref for FlowGraphAdaptor<'a, G>
// where
//     G: Viewable + Base<Weight: Clone + Debug>,
// {
//     type Target = G;

//     fn deref(&self) -> &Self::Target {
//         &self.graph
//     }
// }

// delegate: Clear, Extend, GetMut, Insert, Remove, Reserve

impl<'a, G> Capacity for FlowGraphAdaptor<'a, G>
where
    G: Viewable + Base<Weight: Clone + Debug> + Capacity,
{
    fn edges_capacity(&self) -> usize {
        self.graph.edges_capacity()
    }

    fn nodes_capacity(&self) -> usize {
        self.graph.nodes_capacity()
    }
}

impl<'a, G> Contains for FlowGraphAdaptor<'a, G>
where
    G: Viewable + Base<Weight: Clone + Debug> + Contains,
{
    fn contains_edge(
        &self,
        from: NodeId<Self::Id>,
        to: NodeId<Self::Id>,
    ) -> Option<crate::prelude::EdgeId<Self::Id>> {
        self.graph.contains_edge(from, to)
    }

    fn contains_node(&self, node: &Self::Node) -> Option<NodeId<Self::Id>> {
        self.graph.contains_node(node)
    }
}

impl<'a, G> Count for FlowGraphAdaptor<'a, G>
where
    G: Viewable + Base<Weight: Clone + Debug> + Count,
{
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    fn node_count(&self) -> usize {
        self.graph.node_count()
    }
}

//  IIterMut,  IterAdjacentMut, Balance, Flow, Cost, Create, Directed

impl<'a, G> Viewable for FlowGraphAdaptor<'a, G>
where
    G: Viewable + Base<Weight: Clone + Debug>,
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

impl<'a, G> Visitable for FlowGraphAdaptor<'a, G>
where
    G: Viewable + Base<Weight: Clone + Debug> + Visitable,
{
    type VisitMap = G::VisitMap;

    fn visit_map(&self) -> Self::VisitMap {
        self.graph.visit_map()
    }
}

impl<'b, G> Index for FlowGraphAdaptor<'b, G>
where
    G: Viewable + Base<Weight: Clone + Debug> + Index,
{
    type EdgeIds<'a> = G::EdgeIds<'a> where G: 'a, Self: 'a;
    type NodeIds<'a> = G::NodeIds<'a> where G: 'a, Self: 'a;

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.graph.edge_ids()
    }

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        self.graph.node_ids()
    }
}

impl<'b, G> IndexAdjacent for FlowGraphAdaptor<'b, G>
where
    G: Viewable + Base<Weight: Clone + Debug> + IndexAdjacent,
{
    type AdjacentEdgeIds<'a> = G::AdjacentEdgeIds<'a> where G: 'a, Self: 'a;
    type AdjacentNodeIds<'a> = G::AdjacentNodeIds<'a> where G: 'a, Self: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::AdjacentEdgeIds<'a> {
        self.graph.adjacent_edge_ids(node_id)
    }

    fn adjacent_node_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::AdjacentNodeIds<'a> {
        self.graph.adjacent_node_ids(node_id)
    }
}

impl<'b, G> Iter for FlowGraphAdaptor<'b, G>
where
    G: Viewable + Base<Weight: Clone + Debug> + Iter,
{
    type Edges<'a> = G::Edges<'a> where G: 'a, Self: 'a;
    type Nodes<'a> = G::Nodes<'a> where G: 'a, Self: 'a;

    fn iter_edges<'a>(&'a self) -> Self::Edges<'a> {
        self.graph.iter_edges()
    }

    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a> {
        self.graph.iter_nodes()
    }
}

impl<'b, G> IterAdjacent for FlowGraphAdaptor<'b, G>
where
    G: Viewable + Base<Weight: Clone + Debug> + IterAdjacent,
{
    type Edges<'a> = G::Edges<'a> where G: 'a, Self: 'a;
    type Nodes<'a> = G::Nodes<'a> where G: 'a, Self: 'a;

    fn iter_adjacent_edges<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::Edges<'a> {
        self.graph.iter_adjacent_edges(node_id)
    }

    fn iter_adjacent_nodes<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::Nodes<'a> {
        self.graph.iter_adjacent_nodes(node_id)
    }
}
