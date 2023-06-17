use crate::{
    graph::{
        Base, Cost, Count, Directed, EdgeIdentifier, Get, Index, IndexAdjacent, Iter, IterAdjacent,
        WeightCost,
    },
    prelude::{EdgeRef, NodeIdentifier},
    structures::Parents,
};
use std::{
    collections::HashMap,
    marker::PhantomData,
    num::NonZeroU32,
    ops::{Deref, DerefMut},
};

pub struct TreeBuilder<G: Base> {
    adjacencies: HashMap<G::NodeId, Vec<G::NodeId>>,
    rank: Vec<Option<NonZeroU32>>,
    parents: Parents<G>,
    edge_count: usize,
}

impl<G: Base> TreeBuilder<G> {
    pub fn with_count(count: usize) -> Self {
        Self {
            adjacencies: HashMap::new(),
            rank: vec![None; count],
            parents: Parents::with_count(count),
            edge_count: 0,
        }
    }

    // pub fn root(&mut self, root: G::NodeId) -> &mut Self {
    //     self.root = Some(root);
    //     self
    // }

    pub fn insert(&mut self, from: G::NodeId, to: G::NodeId) -> &mut Self {
        if let Some(adj) = self.adjacencies.get_mut(&from) {
            adj.push(to);
        } else {
            self.adjacencies.insert(from, vec![to]);
        }

        if let Some(adj) = self.adjacencies.get_mut(&to) {
            adj.push(from);
        } else {
            self.adjacencies.insert(to, vec![from]);
        }

        self.parents.insert(to, from);
        self.parents.insert(from, to);
        self.edge_count += 1;
        self
    }

    pub fn rank(&mut self, node: G::NodeId, rank: u32) -> &mut Self {
        self.rank[node.as_usize()] = NonZeroU32::new(rank);
        self
    }

    // TODO return error
    pub fn build(self, root: G::NodeId) -> Tree<G> {
        let Self {
            adjacencies,
            parents,
            rank,
            edge_count,
        } = self;

        Tree {
            root,
            adjacencies,
            parents,
            rank,
            edge_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<G: Base> {
    root: G::NodeId,
    adjacencies: HashMap<G::NodeId, Vec<G::NodeId>>,
    parents: Parents<G>,
    rank: Vec<Option<NonZeroU32>>,
    edge_count: usize,
}

impl<G: Base> Tree<G> {
    pub fn new(root: G::NodeId, count: usize) -> Self {
        Self {
            root,
            adjacencies: HashMap::new(),
            rank: vec![None; count],
            parents: Parents::with_count(count),
            edge_count: 0,
        }
    }

    pub fn root(&self) -> G::NodeId {
        self.root
    }
    pub fn insert(&mut self, from: G::NodeId, to: G::NodeId) -> G::EdgeId {
        if let Some(adj) = self.adjacencies.get_mut(&from) {
            adj.push(to);
        } else {
            self.adjacencies.insert(from, vec![to]);
        }

        if let Some(adj) = self.adjacencies.get_mut(&to) {
            adj.push(from);
        } else {
            self.adjacencies.insert(to, vec![from]);
        }

        self.parents.insert(to, from);
        self.parents.insert(from, to);
        self.edge_count += 1;
        G::EdgeId::between(from, to)
    }
}

impl<G: Base> Base for Tree<G> {
    type EdgeId = G::EdgeId;
    type NodeId = G::NodeId;
}

impl<G: Base> Directed for Tree<G> {
    fn directed() -> bool {
        false
    }
}

impl<G: Base> Count for Tree<G> {
    fn node_count(&self) -> usize {
        self.parents.count()
    }

    fn edge_count(&self) -> usize {
        // self.adjacencies.values().fold(0, |mut akku, vec| {
        //     akku += vec.len();
        //     akku
        // })
        self.edge_count
    }
}

impl<G: Base> Index for Tree<G> {
    type NodeIds<'a> = impl Iterator<Item = Self::NodeId>
    where Self: 'a;
    type EdgeIds<'a> = impl Iterator<Item = Self::EdgeId> + 'a
    where Self: 'a;

    fn node_ids<'a>(&'a self) -> Self::NodeIds<'a> {
        (0..self.adjacencies.len()).map(G::NodeId::from)
    }

    fn edge_ids<'a>(&'a self) -> Self::EdgeIds<'a> {
        self.parents.edge_ids()
    }
}

impl<G: Base> IndexAdjacent for Tree<G> {
    type AdjacentNodeIds<'a> = impl Iterator<Item = Self::NodeId> + 'a
    where Self: 'a;
    type AdjacentEdgeIds<'a> = impl Iterator<Item = Self::EdgeId> + 'a
    where Self: 'a;

    fn adjacent_node_ids<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentNodeIds<'a> {
        self.adjacencies
            .get(&node_id)
            .into_iter()
            .flatten()
            .cloned()
    }

    fn adjacent_edge_ids<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentEdgeIds<'a> {
        self.adjacencies
            .get(&node_id)
            .into_iter()
            .flatten()
            .map(move |&to| G::EdgeId::between(node_id, to))
    }
}

pub struct TreeVisitor<'a, N, W, G: Get<N, W>> {
    tree: Tree<G>,
    graph: &'a G,
    node: PhantomData<N>,
    weight: PhantomData<W>,
}

impl<'a, N, W, G: Get<N, W>> Deref for TreeVisitor<'a, N, W, G> {
    type Target = Tree<G>;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl<'a, N, W, G: Get<N, W>> DerefMut for TreeVisitor<'a, N, W, G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tree
    }
}

impl<'a, N, W, G: Get<N, W>> Base for TreeVisitor<'a, N, W, G> {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;
}

impl<'a, N, W, G: Get<N, W>> Iter<N, W> for TreeVisitor<'a, N, W, G> {
    type Nodes<'b> = impl Iterator<Item = &'b N> + 'b
    where Self: 'b;

    type Edges<'b> = impl Iterator<Item = EdgeRef<'b, G::EdgeId, W>> + 'b
    where Self: 'b;

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

impl<'a, N, W, G: Get<N, W>> IterAdjacent<N, W> for TreeVisitor<'a, N, W, G> {
    type Nodes<'b> = impl Iterator<Item = &'b N> + 'b
    where Self: 'b;

    type Edges<'b> = impl Iterator<Item = EdgeRef<'b, G::EdgeId, W>> + 'b
    where Self: 'b;

    fn iter_adjacent_nodes<'b>(&'b self, node_id: Self::NodeId) -> Self::Nodes<'b> {
        self.adjacent_node_ids(node_id)
            .map(|node_id| self.graph.node(node_id).unwrap())
    }

    fn iter_adjacent_edges<'b>(&'b self, node_id: Self::NodeId) -> Self::Edges<'b> {
        self.adjacent_node_ids(node_id).map(move |to| {
            let edge_id = G::EdgeId::between(node_id, to);
            EdgeRef::new(edge_id, self.graph.weight(edge_id).unwrap())
        })
    }
}

impl<'a, N, W, G: Get<N, W>> TreeVisitor<'a, N, W, G> {
    pub fn new(tree: Tree<G>, graph: &'a G) -> Self {
        Self {
            tree,
            graph,
            node: PhantomData,
            weight: PhantomData,
        }
    }
}

impl<'a, N, W: WeightCost<Cost: Cost>, G: Get<N, W>> TreeVisitor<'a, N, W, G> {
    pub fn total_cost(&self) -> W::Cost {
        self.tree
            .edge_ids()
            .map(|edge_id| self.graph.weight(edge_id).unwrap().cost())
            .cloned()
            .fold(W::Cost::default(), |mut akku, cost| {
                akku += cost;
                akku
            })
    }
}
