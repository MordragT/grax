use std::{
    cmp::Ordering,
    ops::{Add, AddAssign},
};

pub use access::{GraphAccess, GraphCompare};
pub use topology::{GraphAdjacentTopology, GraphTopology};

use crate::{error::GraphResult, prelude::NodeIndex};

use self::{
    mst::{dijkstra, kruskal, prim},
    search::{breadth_search_connected_components, depth_search_connected_components},
    tsp::{branch_bound, brute_force, double_tree, nearest_neighbor},
};

mod access;
pub mod mst;
pub mod search;
mod topology;
pub mod tsp;

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
    fn dijkstra(&self, from: NodeIndex, to: NodeIndex) -> Option<W> {
        dijkstra(self, from, to)
    }

    fn kruskal(&self) -> W {
        kruskal(self)
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

    fn nearest_neighbor(&self) -> Option<W> {
        nearest_neighbor(self)
    }

    fn double_tree(&self) -> GraphResult<W> {
        double_tree(self)
    }

    fn branch_bound(&self) -> GraphResult<W> {
        branch_bound(self)
    }

    fn brute_force(&self) -> Option<W> {
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
