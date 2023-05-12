use std::{
    cmp::Ordering,
    ops::{Add, AddAssign},
};

pub use access::{GraphAccess, GraphCompare};
pub use mst::GraphMst;
pub use search::GraphSearch;
pub use topology::{GraphAdjacentTopology, GraphTopology};
pub use tsp::GraphTsp;

mod access;
mod mst;
mod search;
mod topology;
mod tsp;

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

pub trait Graph<N: Node, W: Weight>:
    GraphAccess<N, W>
    + GraphMst<N, W>
    + GraphSearch<N, W>
    + GraphTopology<N, W>
    + GraphAdjacentTopology<N, W>
    + GraphTsp<N, W>
{
}
pub trait Node: Default + PartialEq {}

impl<T: Default + PartialEq> Node for T {}
pub trait Weight:
    Sortable + Maximum + Default + Add<Self, Output = Self> + AddAssign + Clone
{
}

impl<T: Sortable + Maximum + Default + Add<T, Output = T> + AddAssign + Clone> Weight for T {}
