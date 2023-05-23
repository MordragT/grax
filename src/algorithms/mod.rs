pub use bellman_ford::*;
pub use branch_bound::*;
pub use brute_force::*;
pub use dijkstra::*;
pub use double_tree::*;
pub use edmonds_karp::*;
pub use kruskal::*;
pub use nearest_neighbor::*;
pub use prim::*;
pub use search::*;

use crate::{
    graph::{Base, Get},
    prelude::{EdgeIndex, NodeIdentifier, NodeIndex},
};
use thiserror::Error;

mod bellman_ford;
mod branch_bound;
mod brute_force;
mod dijkstra;
mod double_tree;
mod edmonds_karp;
mod kruskal;
mod nearest_neighbor;
mod prim;
mod search;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[error("Negative Cycle detected")]
pub struct NegativeCycle;

pub struct ConnectedComponents<NodeId> {
    components: Vec<Vec<NodeId>>,
}

impl<NodeId> ConnectedComponents<NodeId> {
    pub fn new(components: Vec<Vec<NodeId>>) -> Self {
        Self { components }
    }

    pub fn count(&self) -> usize {
        self.components.len()
    }
}

#[derive(Debug)]
pub struct Tour<NodeId, Weight> {
    pub route: Vec<NodeId>,
    pub weight: Weight,
}

impl<NodeId, Weight> Tour<NodeId, Weight> {
    pub fn new(route: Vec<NodeId>, weight: Weight) -> Self {
        Self { route, weight }
    }

    pub fn edges(&self) -> impl Iterator<Item = (&NodeId, &NodeId)> {
        self.route.array_windows::<2>().map(|[from, to]| (from, to))
    }

    pub fn map<F, T>(self, mut f: F) -> Tour<NodeId, T>
    where
        F: FnMut(Weight) -> T,
    {
        let Tour { route, weight } = self;
        let weight = f(weight);
        Tour { route, weight }
    }
}

impl<NodeId: Copy, Weight> Tour<NodeId, Weight> {
    pub fn nodes<'a, N, G>(&'a self, graph: &'a G) -> impl Iterator<Item = &'a N> + 'a
    where
        G: Get<N, Weight> + Base<NodeId = NodeId>,
    {
        self.route.iter().map(|index| graph.node(*index).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Distances<NodeId: NodeIdentifier, Weight> {
    pub distances: Vec<Option<Weight>>,
    pub from: NodeId,
}

impl<NodeId: NodeIdentifier, Weight> Distances<NodeId, Weight> {
    pub fn new(from: NodeId, distances: Vec<Option<Weight>>) -> Self {
        Self { distances, from }
    }

    pub fn to(&self, to: NodeId) -> Option<&Weight> {
        self.distances[to.as_usize()].as_ref()
    }
}

#[derive(Debug)]
pub struct MinimumSpanningTree<G: Base> {
    pub tree: G,
    pub root: G::NodeId,
}

impl<G: Base> MinimumSpanningTree<G> {
    pub fn new(tree: G, root: G::NodeId) -> Self {
        Self { tree, root }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Flow<W> {
    pub current: W,
    pub max: W,
}

impl<W: Default> Flow<W> {
    pub fn new(max: W) -> Self {
        Self {
            max,
            current: W::default(),
        }
    }
}

pub struct ParentPath {
    pub(crate) parent: Vec<Option<NodeIndex>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct AugmentedPath {
    pub(crate) edges: Vec<EdgeIndex>,
}

impl AugmentedPath {
    pub(crate) fn new(edges: Vec<EdgeIndex>) -> Self {
        Self { edges }
    }
}

#[derive(Debug)]
pub struct UnionFind<NodeId> {
    parent: Vec<NodeId>,
    rank: Vec<usize>,
    path: Vec<NodeId>,
}

impl<NodeId: NodeIdentifier> UnionFind<NodeId> {
    pub fn root(&self) -> NodeId {
        self.parent[0]
    }

    pub fn rank(&self, index: NodeId) -> usize {
        self.rank[index.as_usize()]
    }

    pub fn find(&mut self, needle: NodeId) -> NodeId {
        let mut root = needle;

        self.path.clear();

        while self.parent[root.as_usize()] != root {
            self.path.push(root);
            root = self.parent[root.as_usize()];
        }

        // set root of every cached index in path to "root"
        // when union find is run for a longer time the
        // performance might degrade as find must traverse
        // more parents in the former loop
        // this allows to skip intermediate nodes and improves the performance
        for index in &self.path {
            self.parent[index.as_usize()] = root;
        }
        root
    }

    pub fn union(&mut self, x: NodeId, y: NodeId) {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x == root_y {
            return;
        }

        // keep depth of trees small by appending small tree to big tree
        // ensures find operation is not doing effectively a linked list search
        if self.rank[root_x.as_usize()] < self.rank[root_y.as_usize()] {
            std::mem::swap(&mut root_x, &mut root_y);
        }
        self.parent[root_y.as_usize()] = root_x;
        self.rank[root_x.as_usize()] += self.rank[root_y.as_usize()];
    }
}

// Set every parent of each tree to itself
// Meaning that every tree == 1 node
impl<NodeId, T: Iterator<Item = NodeId>> From<T> for UnionFind<NodeId> {
    fn from(nodes: T) -> Self {
        let parent: Vec<NodeId> = nodes.collect();
        //parent.sort();

        let rank = vec![1; parent.len()];

        Self {
            parent,
            rank,
            path: Vec::new(),
        }
    }
}
