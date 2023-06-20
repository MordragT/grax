use crate::{
    graph::{Base, Count, Directed, Get, Index, IndexAdjacent, Iter, IterAdjacent},
    prelude::{EdgeId, EdgeRef, NodeId},
    structures::Parents,
};
use std::{collections::HashSet, num::NonZeroU32};

// TODO edges refactor to use simple usize like index so that hashset can be converted to vec for better performance

pub struct TreeBuilder<G: Base> {
    edges: HashSet<EdgeId<G::Id>>,
    nodes: Vec<bool>,
    rank: Vec<Option<NonZeroU32>>,
    parents: Parents<G>,
}

impl<G: Base> TreeBuilder<G> {
    pub fn with_count(count: usize) -> Self {
        Self {
            nodes: vec![false; count],
            edges: HashSet::new(),
            rank: vec![None; count],
            parents: Parents::with_count(count),
        }
    }

    pub fn insert(&mut self, from: NodeId<G::Id>, to: NodeId<G::Id>) -> &mut Self {
        self.nodes[from.as_usize()] = true;
        self.nodes[to.as_usize()] = true;

        self.parents.insert(to, from);
        self.parents.insert(from, to);

        let edge_id = EdgeId::new_unchecked(from, to);
        self.edges.insert(edge_id);
        self.edges.insert(edge_id.rev());
        self
    }

    pub fn rank(&mut self, node: NodeId<G::Id>, rank: u32) -> &mut Self {
        self.rank[node.as_usize()] = NonZeroU32::new(rank);
        self
    }
    // TODO return error
    pub fn build(self, root: NodeId<G::Id>, graph: &G) -> Tree<G> {
        let Self {
            nodes,
            edges,
            parents,
            rank,
        } = self;

        Tree {
            root,
            graph,
            nodes,
            edges,
            parents,
            rank,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<'a, G: Base> {
    graph: &'a G,
    root: NodeId<G::Id>,
    nodes: Vec<bool>,
    edges: HashSet<EdgeId<G::Id>>,
    parents: Parents<G>,
    rank: Vec<Option<NonZeroU32>>,
}

impl<'a, G: Base + Count> Tree<'a, G> {
    pub fn new(root: NodeId<G::Id>, graph: &'a G) -> Self {
        let count = graph.node_count();

        Self {
            root,
            graph,
            nodes: vec![false; count],
            edges: HashSet::new(),
            rank: vec![None; count],
            parents: Parents::with_count(count),
        }
    }

    pub fn root(&self) -> NodeId<G::Id> {
        self.root
    }
    pub fn insert(&mut self, from: NodeId<G::Id>, to: NodeId<G::Id>) -> EdgeId<G::Id> {
        self.nodes[from.as_usize()] = true;
        self.nodes[to.as_usize()] = true;

        self.parents.insert(to, from);
        self.parents.insert(from, to);

        let edge_id = EdgeId::new_unchecked(from, to);
        self.edges.insert(edge_id);
        self.edges.insert(edge_id.rev());
        edge_id
    }
}

impl<G: Base> Base for Tree<'_, G> {
    type Id = G::Id;
}

impl<G: Base> Directed for Tree<'_, G> {
    fn directed() -> bool {
        false
    }
}

impl<G: Base> Count for Tree<'_, G> {
    fn node_count(&self) -> usize {
        self.parents.count()
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

impl<'b, G: Base + Index> Index for Tree<'b, G> {
    type NodeIds<'a> = impl Iterator<Item = NodeId<Self::Id>> + 'a
    where Self: 'a;
    type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Id>> + 'a
    where Self: 'a;

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        self.graph
            .node_ids()
            .filter(|node_id| self.nodes[node_id.as_usize()])
    }

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.edges.iter().cloned()
    }
}

impl<G: IndexAdjacent> IndexAdjacent for Tree<'_, G> {
    type AdjacentNodeIds<'a> = impl Iterator<Item = NodeId<Self::Id>> + 'a
    where Self: 'a;
    type AdjacentEdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Id>> + 'a
    where Self: 'a;

    fn adjacent_node_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::AdjacentNodeIds<'a> {
        self.graph
            .adjacent_edge_ids(node_id)
            .filter(|edge_id| self.edges.contains(edge_id))
            .map(|edge_id| edge_id.to())
    }

    fn adjacent_edge_ids<'a>(&'a self, node_id: NodeId<Self::Id>) -> Self::AdjacentEdgeIds<'a> {
        self.graph
            .adjacent_edge_ids(node_id)
            .filter(|edge_id| self.edges.contains(edge_id))
    }
}

impl<'a, N, W, G: Get<N, W> + Index> Iter<N, W> for Tree<'a, G> {
    type Nodes<'b> = impl Iterator<Item = &'b N> + 'b
    where Self: 'b, N: 'b;

    type Edges<'b> = impl Iterator<Item = EdgeRef<'b, G::Id, W>> + 'b
    where Self: 'b, W: 'b;

    fn iter_nodes<'b>(&'b self) -> Self::Nodes<'b> {
        self.node_ids()
            .map(move |node_id| self.graph.node(node_id).unwrap())
    }

    fn iter_edges<'b>(&'b self) -> Self::Edges<'b> {
        self.edge_ids().map(move |edge_id| {
            let weight = self.graph.weight(edge_id).unwrap();
            EdgeRef::new(edge_id, weight)
        })
    }
}

impl<'a, N, W, G: Get<N, W> + IndexAdjacent> IterAdjacent<N, W> for Tree<'a, G> {
    type Nodes<'b> = impl Iterator<Item = &'b N> + 'b
    where Self: 'b, N: 'b;

    type Edges<'b> = impl Iterator<Item = EdgeRef<'b, G::Id, W>> + 'b
    where Self: 'b, W: 'b;

    fn iter_adjacent_nodes<'b>(&'b self, node_id: NodeId<Self::Id>) -> Self::Nodes<'b> {
        self.adjacent_node_ids(node_id)
            .map(|node_id| self.graph.node(node_id).unwrap())
    }

    fn iter_adjacent_edges<'b>(&'b self, node_id: NodeId<Self::Id>) -> Self::Edges<'b> {
        self.adjacent_edge_ids(node_id)
            .map(move |edge_id| EdgeRef::new(edge_id, self.graph.weight(edge_id).unwrap()))
    }
}
