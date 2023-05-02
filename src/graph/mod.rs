use std::ops::{Add, AddAssign};

pub use access::{GraphAccess, GraphCompare};
pub use mst::{GraphMst, Sortable};
pub use search::GraphSearch;
pub use topology::{GraphAdjacentTopology, GraphTopology};
pub use tsp::GraphTsp;

mod access;
mod mst;
mod search;
mod topology;
mod tsp;

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
pub trait Weight: Sortable + Default + Add<Self, Output = Self> + AddAssign + Clone {}

impl<T: Sortable + Default + Add<T, Output = T> + AddAssign + Clone> Weight for T {}
