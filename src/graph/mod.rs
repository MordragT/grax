use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub use edge::*;
pub use traits::*;

use crate::{
    algorithms::{
        bellman_ford, bellman_ford_between, bfs_connected_components, bfs_tour, branch_bound,
        branch_bound_rec, brute_force, dfs_connected_components, dfs_tour, dijkstra,
        dijkstra_between, double_tree, edmonds_karp, kruskal_mst, kruskal_weight, nearest_neighbor,
        nearest_neighbor_from_first, prim, ConnectedComponents, Distances, MinimumSpanningTree,
        NegativeCycle, Tour,
    },
    prelude::NodeIndex,
};

mod edge;
mod traits;

pub trait Graph<N: Node, W: Weight>:
    Base
    + Capacity
    + Clear
    + Contains<N>
    + Count
    + Create<N>
    + Directed
    + Extend<N, W>
    + Get<N, W>
    + GetMut<N, W>
    + Index
    + IndexAdjacent
    + Insert<N, W>
    + IterEdges<W>
    + IterNodes<N>
    + IterNodesMut<N>
    + Remove<N, W>
    + Reserve
    + Sized
    + Clone
{
    fn bellman_ford_between(&self, from: NodeIndex, to: NodeIndex) -> Option<W> {
        bellman_ford_between(self, from, to)
    }

    fn bellman_ford(&self, start: NodeIndex) -> Result<Distances<W>, NegativeCycle> {
        bellman_ford(self, start)
    }

    fn dijkstra_between(&self, from: NodeIndex, to: NodeIndex) -> Option<W> {
        dijkstra_between(self, from, to)
    }

    fn dijkstra(&self, from: NodeIndex, to: NodeIndex) -> Distances<W> {
        dijkstra(self, from, to)
    }

    fn edmonds_karp(&self, from: NodeIndex, to: NodeIndex) -> W {
        edmonds_karp(self, from, to)
    }

    fn kruskal_weight(&self) -> W {
        kruskal_weight(self)
    }

    fn kruskal_mst(&self) -> MinimumSpanningTree<&N, W> {
        kruskal_mst(self)
    }

    fn prim(&self) -> W {
        prim(self)
    }

    fn dfs_connected_components(&self) -> ConnectedComponents {
        dfs_connected_components(self)
    }

    fn bfs_connected_components(&self) -> ConnectedComponents {
        bfs_connected_components(self)
    }

    fn dfs_tour(&self, from: NodeIndex) -> Tour<()> {
        dfs_tour(self, from)
    }

    fn bfs_tour(&self, from: NodeIndex) -> Tour<()> {
        bfs_tour(self, from)
    }

    fn nearest_neighbor(&self, start: NodeIndex) -> Option<Tour<W>> {
        nearest_neighbor(self, start)
    }

    fn nearest_neighbor_from_first(&self) -> Option<Tour<W>> {
        nearest_neighbor_from_first(self)
    }

    fn double_tree(&self) -> Option<Tour<W>> {
        double_tree(self)
    }

    fn branch_bound(&self) -> Option<Tour<W>> {
        branch_bound(self)
    }

    fn branch_bound_rec(&self) -> Option<Tour<W>> {
        branch_bound_rec(self)
    }

    fn brute_force(&self) -> Option<Tour<W>> {
        brute_force(self)
    }
}

pub trait WeightlessGraph<N>:
    Base
    + Capacity
    + Clear
    + Contains<N>
    + Count
    + Create<N>
    + Directed
    + Extend<N, ()>
    + Get<N, ()>
    + GetMut<N, ()>
    + Index
    + IndexAdjacent
    + Insert<N, ()>
    + IterEdges<()>
    + IterNodes<N>
    + IterNodesMut<N>
    + Remove<N, ()>
    + Reserve
    + Sized
    + Clone
{
    fn dfs_connected_components(&self) -> ConnectedComponents {
        dfs_connected_components(self)
    }

    fn bfs_connected_components(&self) -> ConnectedComponents {
        bfs_connected_components(self)
    }

    fn dfs_tour(&self, from: NodeIndex) -> Tour<()> {
        dfs_tour(self, from)
    }

    fn bfs_tour(&self, from: NodeIndex) -> Tour<()> {
        bfs_tour(self, from)
    }
}

pub trait Node: Default + PartialEq + Clone {}

impl<T: Default + PartialEq + Clone> Node for T {}
pub trait Weight:
    Sortable
    + Maximum
    + Default
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + AddAssign
    + SubAssign
    + Copy
    + Debug
{
}

impl<
        T: Sortable
            + Maximum
            + Default
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + AddAssign
            + SubAssign
            + Copy
            + Debug,
    > Weight for T
{
}

pub trait NodeId {
    fn as_usize(&self) -> usize;
}

pub trait EdgeId: Sized {
    type NodeId;

    /// Reveres the edge index
    fn rev(&self) -> Self;
    fn to(&self) -> Self::NodeId;
    fn from(&self) -> Self::NodeId;
    fn contains(&self, node_id: Self::NodeId) -> bool;
}

pub trait Sortable: PartialOrd {
    fn sort(&self, other: &Self) -> Ordering;
}

default impl<T: PartialOrd> Sortable for T {
    default fn sort(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Sortable for f64 {
    fn sort(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

impl Sortable for f32 {
    fn sort(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

pub trait Maximum {
    fn max() -> Self;
}

impl Maximum for f64 {
    fn max() -> Self {
        f64::INFINITY
    }
}

impl Maximum for f32 {
    fn max() -> Self {
        f32::INFINITY
    }
}

impl Maximum for u32 {
    fn max() -> Self {
        u32::MAX
    }
}
