pub struct IdGraph<G: Base> {}

impl<'a, G: Viewable> Base for IdGraph<'a, G> {
    type Id = G::Id;
    type Node = NodeId<G::Id>;
    type Weight = EdgeId<G::Id>;
}

impl<G: Create> Create for IdGraph<G> {
    fn new() -> Self {
        Self {
            graph: G::new(),
            nodes: HashSet::new(),
        }
    }

    fn with_capacity(nodes: usize, edges: usize) -> Self {
        Self {
            graph: G::with_capacity(0, edges),
            nodes: HashSet::with_capacity(nodes),
        }
    }

    fn with_nodes(nodes: impl IntoIterator<Item = Self::Node>) -> Self {
        let nodes = nodes.into_iter().collect();

        Self {
            graph: G::new(),
            nodes,
        }
    }
}

impl<G: Clear + Base> Clear for IdGraph<G> {
    fn clear(&mut self) {
        self.graph.clear();
        self.nodes.clear();
    }

    fn clear_edges(&mut self) {
        self.graph.clear_edges();
    }
}

impl<G: Capacity + Base> Capacity for IdGraph<G> {
    fn edges_capacity(&self) -> usize {
        self.graph.edges_capacity()
    }

    fn nodes_capacity(&self) -> usize {
        self.nodes.capacity()
    }
}

impl<G: Contains> Contains for IdGraph<G> {
    fn contains_edge(
        &self,
        from: NodeId<Self::Id>,
        to: NodeId<Self::Id>,
    ) -> Option<crate::prelude::EdgeId<Self::Id>> {
        self.graph.contains_edge(from, to)
    }

    fn contains_node(&self, node: &Self::Node) -> Option<NodeId<Self::Id>> {
        if self.nodes.contains(node) {
            Some(*node)
        } else {
            None
        }
    }
}

impl<G: Count + Base> Count for IdGraph<G> {
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl<G: Get> Get for IdGraph<G> {
    fn node(&self, node_id: NodeId<Self::Id>) -> Option<&Self::Node> {
        self.nodes.get(&node_id)
    }

    fn weight(&self, edge_id: crate::prelude::EdgeId<Self::Id>) -> Option<&Self::Weight> {
        self.graph.weight(edge_id)
    }
}

impl<G: Index> Index for IdGraph<G> {
    type NodeIds<'a> = G::NodeIds<'a> where G: 'a;
    type EdgeIds<'a> = G::EdgeIds<'a> where G: 'a;

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.graph.edge_ids()
    }

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        self.graph.node_ids()
    }
}

impl<G: Insert> Insert for IdGraph<G> {
    fn insert_node(&mut self, node: Self::Node) -> NodeId<Self::Id> {
        self.nodes.insert(node);
        node
    }

    fn insert_edge(
        &mut self,
        from: NodeId<Self::Id>,
        to: NodeId<Self::Id>,
        weight: Self::Weight,
    ) -> crate::prelude::EdgeId<Self::Id> {
        self.graph.insert_edge(from, to, weight)
    }
}

impl<G: Iter> Iter for IdGraph<G> {
    type Edges<'a> = G::Edges<'a> where G: 'a;
    type Nodes<'a> = impl Iterator<Item = &'a Self::Node> where Self: 'a;

    fn iter_edges<'a>(&'a self) -> Self::Edges<'a> {
        self.graph.iter_edges()
    }

    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a> {
        self.nodes.iter()
    }
}

impl<G: IndexAdjacent> IndexAdjacent for IdGraph<G> {
    type AdjacentEdgeIds<'a> = G::AdjacentEdgeIds<'a> where G: 'a;
    type AdjacentNodeIds<'a> = G::AdjacentNodeIds<'a> where G: 'a;

    fn adjacent_edge_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::AdjacentEdgeIds<'a> {
        self.graph.adjacent_edge_ids(node_id)
    }

    fn adjacent_node_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::AdjacentNodeIds<'a> {
        self.graph.adjacent_node_ids(node_id)
    }
}

// impl<G: IterAdjacent + IndexAdjacent> IterAdjacent for IdGraph<G> {
//     type Edges<'a> = G::Edges<'a> where G: 'a;
//     type Nodes<'a> = impl Iterator<Item = &'a NodeId<G::Id>> where G: 'a;

//     fn iter_adjacent_edges<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::Edges<'a> {
//         self.graph.iter_adjacent_edges(node_id)
//     }

//     fn iter_adjacent_nodes<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::Nodes<'a> {
//         self.graph.adjacent_node_ids(node_id)
//     }
// }

// impl<G: IterAdjacentMut> IterAdjacentMut for IdGraph<G> {
//     type EdgesMut<'a> = G::EdgesMut<'a> where G: 'a;
//     type NodesMut<'a> = G::NodesMut<'a> where G: 'a;

//     fn iter_adjacent_edges_mut<'a>(&'a mut self, node_id: NodeId<Self::Id>) -> Self::EdgesMut<'a> {
//         self.graph.iter_adjacent_edges_mut(node_id)
//     }

//     fn iter_adjacent_nodes_mut<'a>(&'a mut self, node_id: NodeId<Self::Id>) -> Self::NodesMut<'a> {
//         self.graph.iter_adjacent_nodes_mut(node_id)
//     }
// }

impl<G: Reserve + Base> Reserve for IdGraph<G> {
    fn reserve_edges(&mut self, additional: usize) {
        self.graph.reserve_edges(additional)
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.graph.reserve_nodes(additional)
    }
}

impl<G: Visitable> Visitable for IdGraph<G> {
    type VisitMap = G::VisitMap;

    fn visit_map(&self) -> Self::VisitMap {
        self.graph.visit_map()
    }
}

impl<G: Viewable> Viewable for IdGraph<G> {
    type EdgeMap<Attr: Clone + std::fmt::Debug + Default> = G::EdgeMap<Attr>;
    type NodeMap<Attr: Clone + std::fmt::Debug + Default> = G::NodeMap<Attr>;

    fn edge_map<Attr: Clone + std::fmt::Debug + Default>(&self) -> Self::EdgeMap<Attr> {
        self.graph.edge_map()
    }

    fn node_map<Attr: Clone + std::fmt::Debug + Default>(&self) -> Self::NodeMap<Attr> {
        self.graph.node_map()
    }
}
