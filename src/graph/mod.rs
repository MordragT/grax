use std::{
    cmp::Ordering,
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub use edge::*;
pub use traits::*;

use crate::algorithms::{
    bellman_ford, bellman_ford_between, bfs_connected_components, bfs_tour, branch_bound,
    branch_bound_rec, brute_force, dfs_connected_components, dfs_tour, dijkstra, dijkstra_between,
    double_tree, edmonds_karp, kruskal_mst, kruskal_weight, nearest_neighbor,
    nearest_neighbor_from_first, prim, ConnectedComponents, Distances, MinimumSpanningTree,
    NegativeCycle, Tour,
};

mod edge;
#[cfg(test)]
pub mod test;
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
    + Iter<N, W>
    + IterMut<N, W>
    + IterAdjacent<N, W>
    + IterAdjacentMut<N, W>
    + Remove<N, W>
    + Reserve
    // + for<'a> Ref<'a, N, W>
    + Sized
    + Clone
{
    fn bellman_ford_between(&self, from: Self::NodeId, to: Self::NodeId) -> Option<W::Cost> {
        bellman_ford_between(self, from, to)
    }

    fn bellman_ford(
        &self,
        start: Self::NodeId,
    ) -> Result<Distances<Self::NodeId, W::Cost>, NegativeCycle> {
        bellman_ford(self, start)
    }

    fn dijkstra_between(&self, from: Self::NodeId, to: Self::NodeId) -> Option<W::Cost> {
        dijkstra_between(self, from, to)
    }

    fn dijkstra(&self, from: Self::NodeId, to: Self::NodeId) -> Distances<Self::NodeId, W::Cost> {
        dijkstra(self, from, to)
    }

    // fn edmonds_karp(&self, from: Self::NodeId, to: Self::NodeId) -> W {
    //     edmonds_karp(self, from, to)
    // }

    fn kruskal_weight(&self) -> W::Cost {
        kruskal_weight(self)
    }

    fn kruskal_mst(&self) -> MinimumSpanningTree<Self>
    {
        kruskal_mst(self)
    }

    fn prim(&self) -> W::Cost {
        prim(self)
    }

    fn dfs_connected_components(&self) -> ConnectedComponents<Self::NodeId> {
        dfs_connected_components(self)
    }

    fn bfs_connected_components(&self) -> ConnectedComponents<Self::NodeId> {
        bfs_connected_components(self)
    }

    fn dfs_tour(&self, from: Self::NodeId) -> Tour<Self::NodeId, ()> {
        dfs_tour(self, from)
    }

    fn bfs_tour(&self, from: Self::NodeId) -> Tour<Self::NodeId, ()> {
        bfs_tour(self, from)
    }

    fn nearest_neighbor(&self, start: Self::NodeId) -> Option<Tour<Self::NodeId, W::Cost>> {
        nearest_neighbor(self, start)
    }

    fn nearest_neighbor_from_first(&self) -> Option<Tour<Self::NodeId, W::Cost>> {
        nearest_neighbor_from_first(self)
    }

    fn double_tree(&self) -> Option<Tour<Self::NodeId, W::Cost>>
    {
        double_tree(self)
    }

    fn branch_bound(&self) -> Option<Tour<Self::NodeId, W::Cost>> {
        branch_bound(self)
    }

    fn branch_bound_rec(&self) -> Option<Tour<Self::NodeId, W::Cost>> {
        branch_bound_rec(self)
    }

    fn brute_force(&self) -> Option<Tour<Self::NodeId, W::Cost>> {
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
    // + Extend<N, ()>
    // + Get<N, ()>
    // + GetMut<N, ()>
    + Index
    + IndexAdjacent
    // + Insert<N, ()>
    // + Iter<N, ()>
    // + IterMut<N, ()>
    // + IterAdjacent<N, ()>
    // + IterAdjacentMut<N, ()>
    // + Remove<N, ()>
    + Reserve
    + Sized
    + Clone
{
    fn dfs_connected_components(&self) -> ConnectedComponents<Self::NodeId> {
        dfs_connected_components(self)
    }

    fn bfs_connected_components(&self) -> ConnectedComponents<Self::NodeId> {
        bfs_connected_components(self)
    }

    fn dfs_tour(&self, from: Self::NodeId) -> Tour<Self::NodeId, ()> {
        dfs_tour(self, from)
    }

    fn bfs_tour(&self, from: Self::NodeId) -> Tour<Self::NodeId, ()> {
        bfs_tour(self, from)
    }
}

// pub trait Ref<N: Node, W: Weight> {
//     type GraphRef: for<'a> Graph<&'a N, W>;
// }

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BalancedNode<N, W> {
    pub node: N,
    pub balance: W,
}

impl<N, W> BalancedNode<N, W> {
    pub fn new(node: N, balance: W) -> Self {
        Self { node, balance }
    }
}

pub trait Node: Default + PartialEq + Clone {}

impl<T: Default + PartialEq + Clone> Node for T {}

pub trait EdgeCapacity {
    type Capacity;

    fn capacity(&self) -> &Self::Capacity;
    fn capacity_mut(&mut self) -> &mut Self::Capacity;
}

pub trait EdgeCost {
    type Cost;

    fn cost(&self) -> &Self::Cost;
    fn cost_mut(&mut self) -> &mut Self::Cost;
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct CapacityWeight<W> {
    pub capacity: W,
    pub cost: W,
}

impl<W> CapacityWeight<W> {
    pub fn new(capacity: W, weight: W) -> Self {
        Self {
            capacity,
            cost: weight,
        }
    }
}

impl<W> EdgeCapacity for CapacityWeight<W> {
    type Capacity = W;

    fn capacity(&self) -> &Self::Capacity {
        &self.capacity
    }

    fn capacity_mut(&mut self) -> &mut Self::Capacity {
        &mut self.capacity
    }
}

impl<W> EdgeCost for CapacityWeight<W> {
    type Cost = W;

    fn cost(&self) -> &Self::Cost {
        &self.cost
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        &mut self.cost
    }
}

impl EdgeCost for f32 {
    type Cost = f32;

    fn cost(&self) -> &Self::Cost {
        &self
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        self
    }
}

impl EdgeCost for f64 {
    type Cost = f64;

    fn cost(&self) -> &Self::Cost {
        &self
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        self
    }
}

pub trait Weight: EdgeCost<Cost: Cost> + Copy {}

impl<T: EdgeCost<Cost: Cost> + Copy> Weight for T {}

pub trait Cost:
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
    > Cost for T
{
}

pub trait NodeIdentifier: Hash + Eq + Copy + Debug {
    fn as_usize(&self) -> usize;
}

pub trait EdgeIdentifier: Hash + Eq + Copy + Debug {
    type NodeId: NodeIdentifier;

    fn between(from: Self::NodeId, to: Self::NodeId) -> Self;

    /// Reveres the edge index
    fn rev(&self) -> Self;
    fn to(&self) -> Self::NodeId;
    fn from(&self) -> Self::NodeId;
    fn contains(&self, node_id: Self::NodeId) -> bool;
    fn as_usize(&self) -> (usize, usize);
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
