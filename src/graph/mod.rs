use std::{
    cmp::Ordering,
    ops::{Add, AddAssign},
};

pub use access::{GraphAccess, GraphCompare};
pub use topology::{GraphAdjacentTopology, GraphTopology};

use crate::{
    algorithms::{
        bellman_ford, bellman_ford_between, branch_bound, branch_bound_rec,
        breadth_search_connected_components, brute_force, depth_search_connected_components,
        dijkstra, dijkstra_between, double_tree, kruskal_mst, kruskal_weight, nearest_neighbor,
        nearest_neighbor_from_first, prim, Distances, MinimumSpanningTree, NegativeCycle, Tour,
    },
    prelude::NodeIndex,
};

mod access;
mod topology;

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

pub trait WeightlessGraph<N>: GraphTopology<N, ()> + GraphAdjacentTopology<N, ()> + Sized {
    fn depth_search_connected_components(&self) -> u32 {
        depth_search_connected_components(self)
    }

    fn breadth_search_connected_components(&self) -> u32 {
        breadth_search_connected_components(self)
    }
}

pub trait Graph<N: Node, W: Weight>:
    GraphAccess<N, W> + GraphTopology<N, W> + GraphAdjacentTopology<N, W> + GraphCompare<N, W> + Sized
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

    fn kruskal_weight(&self) -> W {
        kruskal_weight(self)
    }

    fn kruskal_mst(&self) -> MinimumSpanningTree<&N, W> {
        kruskal_mst(self)
    }

    fn prim(&self) -> W {
        prim(self)
    }

    fn depth_search_connected_components(&self) -> u32 {
        depth_search_connected_components(self)
    }

    fn breadth_search_connected_components(&self) -> u32 {
        breadth_search_connected_components(self)
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
pub trait Node: Default + PartialEq {}

impl<T: Default + PartialEq> Node for T {}
pub trait Weight:
    Sortable + Maximum + Default + Add<Self, Output = Self> + AddAssign + Copy
{
}

impl<T: Sortable + Maximum + Default + Add<T, Output = T> + AddAssign + Copy> Weight for T {}
