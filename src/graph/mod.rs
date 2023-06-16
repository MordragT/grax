pub use edge::*;
use either::Either;
pub use node::*;
pub use traits::*;
pub use weight::*;

use crate::{
    algorithms::{
        bellman_ford, bellman_ford_between, bfs_connected_components, bfs_tour, branch_bound,
        branch_bound_rec, brute_force, dfs_connected_components, dfs_tour, dijkstra,
        dijkstra_between, double_tree, edmonds_karp, kruskal_mst, kruskal_weight, nearest_neighbor,
        nearest_neighbor_from_first, prim, ConnectedComponents, Tour,
    },
    structures::{Distances, MinimumSpanningTree, Parents},
};

mod edge;
mod node;
#[cfg(test)]
pub mod test;
mod traits;
mod weight;

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
    ) -> Either<Distances<Self::NodeId, W::Cost>, Parents<Self::NodeId>> {
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
