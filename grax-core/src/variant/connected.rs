use crate::{
    prelude::NodeId,
    traits::{
        Base, Capacity, Clear, Contains, Count, Create, Get, GetMut, Index, IndexAdjacent, Insert,
        Iter, IterAdjacent, IterAdjacentMut, IterMut, Reserve, Root, Viewable, Visitable,
    },
};

#[derive(Debug, Clone)]
pub struct ConnectedGraph<G: Base> {
    graph: G,
    root: NodeId<G::Id>,
}

impl<G: Base> ConnectedGraph<G> {
    pub fn from_unchecked(graph: G, root: NodeId<G::Id>) -> Self {
        Self { graph, root }
    }
}

impl<G: Base> Root for ConnectedGraph<G> {
    fn root(&self) -> NodeId<G::Id> {
        self.root
    }
}

impl<G: Clear + Base> ConnectedGraph<G> {
    pub fn clear(&mut self) {
        self.graph.clear()
    }
}

impl<G: Create + Insert> ConnectedGraph<G> {
    pub fn new(root: G::NodeWeight) -> Self {
        let mut graph = G::new();
        let root = graph.insert_node(root);

        Self { graph, root }
    }
}

impl<G: Insert> ConnectedGraph<G> {
    pub fn insert(
        &mut self,
        parent: NodeId<G::Id>,
        child: G::NodeWeight,
        weight: G::EdgeWeight,
    ) -> NodeId<G::Id> {
        let child_id = self.graph.insert_node(child);
        self.graph.insert_edge(parent, child_id, weight);
        child_id
    }
}

impl<G: Base> Base for ConnectedGraph<G> {
    type Id = G::Id;
    type NodeWeight = G::NodeWeight;
    type EdgeWeight = G::EdgeWeight;
}

impl<G: Capacity + Base> Capacity for ConnectedGraph<G> {
    fn edges_capacity(&self) -> usize {
        self.graph.edges_capacity()
    }

    fn nodes_capacity(&self) -> usize {
        self.graph.nodes_capacity()
    }
}

impl<G: Contains> Contains for ConnectedGraph<G> {
    fn contains_edge(
        &self,
        from: NodeId<Self::Id>,
        to: NodeId<Self::Id>,
    ) -> Option<crate::prelude::EdgeId<Self::Id>> {
        self.graph.contains_edge(from, to)
    }

    fn contains_node(&self, node: &Self::NodeWeight) -> Option<NodeId<Self::Id>> {
        self.graph.contains_node(node)
    }
}

impl<G: Count + Base> Count for ConnectedGraph<G> {
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    fn node_count(&self) -> usize {
        self.graph.node_count()
    }
}

impl<G: Get> Get for ConnectedGraph<G> {
    fn node(
        &self,
        node_id: NodeId<Self::Id>,
    ) -> Option<crate::node::NodeRef<Self::Id, Self::NodeWeight>> {
        self.graph.node(node_id)
    }

    fn edge(
        &self,
        edge_id: crate::prelude::EdgeId<Self::Id>,
    ) -> Option<crate::edge::EdgeRef<Self::Id, Self::EdgeWeight>> {
        self.graph.edge(edge_id)
    }
}

impl<G: GetMut> GetMut for ConnectedGraph<G> {
    fn node_mut(
        &mut self,
        node_id: NodeId<Self::Id>,
    ) -> Option<crate::node::NodeRefMut<Self::Id, Self::NodeWeight>> {
        self.graph.node_mut(node_id)
    }

    fn edge_mut(
        &mut self,
        edge_id: crate::prelude::EdgeId<Self::Id>,
    ) -> Option<crate::edge::EdgeRefMut<Self::Id, Self::EdgeWeight>> {
        self.graph.edge_mut(edge_id)
    }
}

impl<G: Index> Index for ConnectedGraph<G> {
    type NodeIds<'a> = G::NodeIds<'a> where G: 'a;
    type EdgeIds<'a> = G::EdgeIds<'a> where G: 'a;

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.graph.edge_ids()
    }

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        self.graph.node_ids()
    }
}

impl<G: Iter> Iter for ConnectedGraph<G> {
    type Edges<'a> = G::Edges<'a> where G: 'a;
    type Nodes<'a> = G::Nodes<'a> where G: 'a;

    fn iter_edges<'a>(&'a self) -> Self::Edges<'a> {
        self.graph.iter_edges()
    }

    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a> {
        self.graph.iter_nodes()
    }
}

impl<G: IterMut> IterMut for ConnectedGraph<G> {
    type EdgesMut<'a> = G::EdgesMut<'a> where G: 'a;
    type NodesMut<'a> = G::NodesMut<'a> where G: 'a;

    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a> {
        self.graph.iter_nodes_mut()
    }

    fn iter_edges_mut<'a>(&'a mut self) -> Self::EdgesMut<'a> {
        self.graph.iter_edges_mut()
    }
}

impl<G: IndexAdjacent> IndexAdjacent for ConnectedGraph<G> {
    type EdgeIds<'a> = G::EdgeIds<'a> where G: 'a;
    type NodeIds<'a> = G::NodeIds<'a> where G: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::EdgeIds<'a> {
        self.graph.adjacent_edge_ids(node_id)
    }

    fn adjacent_node_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::NodeIds<'a> {
        self.graph.adjacent_node_ids(node_id)
    }
}

impl<G: IterAdjacent> IterAdjacent for ConnectedGraph<G> {
    type Edges<'a> = G::Edges<'a> where G: 'a;
    type Nodes<'a> = G::Nodes<'a> where G: 'a;

    fn iter_adjacent_edges<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::Edges<'a> {
        self.graph.iter_adjacent_edges(node_id)
    }

    fn iter_adjacent_nodes<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::Nodes<'a> {
        self.graph.iter_adjacent_nodes(node_id)
    }
}

impl<G: IterAdjacentMut> IterAdjacentMut for ConnectedGraph<G> {
    type EdgesMut<'a> = G::EdgesMut<'a> where G: 'a;
    type NodesMut<'a> = G::NodesMut<'a> where G: 'a;

    fn iter_adjacent_edges_mut<'a>(&'a mut self, node_id: NodeId<Self::Id>) -> Self::EdgesMut<'a> {
        self.graph.iter_adjacent_edges_mut(node_id)
    }

    fn iter_adjacent_nodes_mut<'a>(&'a mut self, node_id: NodeId<Self::Id>) -> Self::NodesMut<'a> {
        self.graph.iter_adjacent_nodes_mut(node_id)
    }
}

impl<G: Reserve + Base> Reserve for ConnectedGraph<G> {
    fn reserve_edges(&mut self, additional: usize) {
        self.graph.reserve_edges(additional)
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.graph.reserve_nodes(additional)
    }
}

impl<G: Visitable> Visitable for ConnectedGraph<G> {
    type VisitMap = G::VisitMap;

    fn visit_map(&self) -> Self::VisitMap {
        self.graph.visit_map()
    }
}

impl<G: Viewable> Viewable for ConnectedGraph<G> {
    type EdgeMap<Attr: Clone + std::fmt::Debug + Default> = G::EdgeMap<Attr>;
    type NodeMap<Attr: Clone + std::fmt::Debug + Default> = G::NodeMap<Attr>;

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

    fn edge_map<Attr: Clone + std::fmt::Debug + Default>(&self) -> Self::EdgeMap<Attr> {
        self.graph.edge_map()
    }

    fn node_map<Attr: Clone + std::fmt::Debug + Default>(&self) -> Self::NodeMap<Attr> {
        self.graph.node_map()
    }
}
