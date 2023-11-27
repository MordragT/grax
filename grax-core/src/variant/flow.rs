// No more edge traits (edge cost, edge capacity etc.) but more
// graph traits which are then implemented by the adaptors:
// for example a Flow trait where the implementor needs to provide functions:
// flow and flow_mut which returns the respective flow

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    edge::{EdgeCost, EdgeFlow},
    prelude::{EdgeId, NodeId},
    traits::{
        Base, Capacity, Clear, Contains, Cost, Count, Extend, Flow, Get, GetMut, Index,
        IndexAdjacent, Insert, Iter, IterAdjacent, Remove, Reserve, Viewable, Visitable,
    },
    view::AttrMap,
};

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct FlowBundle<C> {
    pub cost: C,
    pub capacity: C,
    pub flow: C,
    pub reverse: bool,
}

impl<C: Clone + Debug> EdgeCost for FlowBundle<C> {
    type Cost = C;

    fn cost(&self) -> &Self::Cost {
        &self.cost
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        &mut self.cost
    }
}

impl<C: Clone + Debug> EdgeFlow for FlowBundle<C> {
    type Flow = C;

    fn flow(&self) -> &Self::Flow {
        &self.flow
    }

    fn flow_mut(&mut self) -> &mut Self::Flow {
        &mut self.flow
    }

    fn capacity(&self) -> &Self::Flow {
        &self.capacity
    }

    fn capacity_mut(&mut self) -> &mut Self::Flow {
        &mut self.capacity
    }

    fn is_reverse(&self) -> bool {
        self.reverse
    }

    fn reverse(&mut self) {
        self.reverse = true;
    }
}

#[derive(Clone, Debug)]
pub struct FlowGraph<G: Viewable, C: Clone + Debug> {
    graph: G,
    flow_map: G::EdgeMap<Option<FlowBundle<C>>>,
}

impl<G, C> FlowGraph<G, C>
where
    G: Viewable,
    C: Clone + Debug,
{
    pub fn from_unchecked(graph: G, flow_map: G::EdgeMap<Option<FlowBundle<C>>>) -> Self {
        Self { graph, flow_map }
    }
}

impl<G, C> Base for FlowGraph<G, C>
where
    G: Viewable,
    C: Clone + Debug,
{
    type Id = G::Id;
    type Node = G::Node;
    type Weight = G::Weight;
}

impl<G, C> Flow<C> for FlowGraph<G, C>
where
    G: Viewable,
    C: Clone + Debug,
{
    type EdgeFlow = FlowBundle<C>;

    fn flow(&self, edge_id: EdgeId<Self::Id>) -> Option<&Self::EdgeFlow> {
        self.flow_map.get(edge_id).as_ref()
    }

    fn flow_mut(&mut self, edge_id: EdgeId<Self::Id>) -> Option<&mut Self::EdgeFlow> {
        self.flow_map.get_mut(edge_id).as_mut()
    }
}

impl<G, C> Cost<C> for FlowGraph<G, C>
where
    G: Viewable,
    C: Clone + Debug,
{
    type EdgeCost = FlowBundle<C>;

    fn cost(&self, edge_id: EdgeId<Self::Id>) -> Option<&Self::EdgeCost> {
        self.flow_map.get(edge_id).as_ref()
    }

    fn cost_mut(&mut self, edge_id: EdgeId<Self::Id>) -> Option<&mut Self::EdgeCost> {
        self.flow_map.get_mut(edge_id).as_mut()
    }
}

impl<C, G> Clear for FlowGraph<G, C>
where
    G: Viewable + Base + Clear,
    C: Clone + Debug,
{
    fn clear(&mut self) {
        self.graph.clear();
        self.flow_map.clear();
    }

    fn clear_edges(&mut self) {
        self.graph.clear_edges();
        self.flow_map.clear();
    }
}

impl<C, G> Get for FlowGraph<G, C>
where
    G: Viewable + Base + Get,
    C: Clone + Debug,
{
    fn node(&self, node_id: NodeId<Self::Id>) -> Option<&Self::Node> {
        self.graph.node(node_id)
    }

    fn weight(&self, edge_id: EdgeId<Self::Id>) -> Option<&Self::Weight> {
        self.graph.weight(edge_id)
    }
}

impl<C, G> GetMut for FlowGraph<G, C>
where
    G: Viewable + Base + GetMut,
    C: Clone + Debug,
{
    fn node_mut(&mut self, node_id: NodeId<Self::Id>) -> Option<&mut Self::Node> {
        self.graph.node_mut(node_id)
    }

    fn weight_mut(&mut self, edge_id: EdgeId<Self::Id>) -> Option<&mut Self::Weight> {
        self.graph.weight_mut(edge_id)
    }
}

impl<C, G> Extend for FlowGraph<G, C>
where
    G: Viewable + Base + Extend,
    C: Clone + Debug,
{
    fn extend_edges(
        &mut self,
        edges: impl IntoIterator<Item = (NodeId<Self::Id>, NodeId<Self::Id>, Self::Weight)>,
    ) {
        self.graph.extend_edges(edges)
    }

    fn extend_nodes(&mut self, nodes: impl IntoIterator<Item = Self::Node>) {
        self.graph.extend_nodes(nodes)
    }
}

impl<C, G> Insert for FlowGraph<G, C>
where
    G: Viewable + Base + Insert,
    C: Clone + Debug,
{
    fn insert_edge(
        &mut self,
        from: NodeId<Self::Id>,
        to: NodeId<Self::Id>,
        weight: Self::Weight,
    ) -> EdgeId<Self::Id> {
        self.graph.insert_edge(from, to, weight)
    }
    fn insert_node(&mut self, node: Self::Node) -> NodeId<Self::Id> {
        self.graph.insert_node(node)
    }
}

impl<C, G> Remove for FlowGraph<G, C>
where
    G: Viewable + Base + Remove,
    C: Clone + Debug,
{
    fn remove_edge(&mut self, edge_id: EdgeId<Self::Id>) -> Option<Self::Weight> {
        self.graph.remove_edge(edge_id)
    }
    fn remove_node(&mut self, node_id: NodeId<Self::Id>) -> Option<Self::Node> {
        self.graph.remove_node(node_id)
    }
}

impl<C, G> Reserve for FlowGraph<G, C>
where
    G: Viewable + Base + Reserve,
    C: Clone + Debug,
{
    fn reserve_edges(&mut self, additional: usize) {
        self.graph.reserve_edges(additional)
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.graph.reserve_nodes(additional)
    }
}

impl<C, G> Capacity for FlowGraph<G, C>
where
    G: Viewable + Base + Capacity,
    C: Clone + Debug,
{
    fn edges_capacity(&self) -> usize {
        self.graph.edges_capacity()
    }

    fn nodes_capacity(&self) -> usize {
        self.graph.nodes_capacity()
    }
}

impl<C, G> Contains for FlowGraph<G, C>
where
    G: Viewable + Base + Contains,
    C: Clone + Debug,
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

impl<C, G> Count for FlowGraph<G, C>
where
    G: Viewable + Base + Count,
    C: Clone + Debug,
{
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    fn node_count(&self) -> usize {
        self.graph.node_count()
    }
}

//  IIterMut,  IterAdjacentMut, Balance, Flow, Cost, Create, Directed

impl<C, G> Viewable for FlowGraph<G, C>
where
    G: Viewable + Base,
    C: Clone + Debug,
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

impl<C, G> Visitable for FlowGraph<G, C>
where
    G: Viewable + Base + Visitable,
    C: Clone + Debug,
{
    type VisitMap = G::VisitMap;

    fn visit_map(&self) -> Self::VisitMap {
        self.graph.visit_map()
    }
}

impl<C, G> Index for FlowGraph<G, C>
where
    G: Viewable + Base + Index,
    C: Clone + Debug,
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

impl<C, G> IndexAdjacent for FlowGraph<G, C>
where
    G: Viewable + Base + IndexAdjacent,
    C: Clone + Debug,
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

impl<C, G> Iter for FlowGraph<G, C>
where
    G: Viewable + Base + Iter,
    C: Clone + Debug,
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

impl<C, G> IterAdjacent for FlowGraph<G, C>
where
    G: Viewable + Base + IterAdjacent,
    C: Clone + Debug,
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
