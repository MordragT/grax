use std::fmt::Debug;

pub use edge::*;
pub use index::*;
pub use node::*;
pub use traits::*;
pub use weight::*;

use crate::{
    algorithms::{
        bellman_ford, bellman_ford_between, bfs, bfs_scc, branch_bound, branch_bound_rec,
        brute_force, dfs, dfs_scc, dijkstra, dijkstra_between, double_tree, edmonds_karp, kruskal,
        nearest_neighbor, nearest_neighbor_from_first, prim,
    },
    prelude::{Tree},
    structures::{Distances,  Route},
};

mod edge;
mod index;
mod node;
#[cfg(test)]
pub mod test;
mod traits;
mod weight;

pub trait Graph<N: Node, W: Weight>:
    Base<Node = N, Weight = W>
    + Capacity
    + Clear
    + Contains
    + Count
    + Create
    + Directed
    + Extend
    + Get
    + GetMut
    + Index
    + IndexAdjacent
    + Insert
    + Iter
    + IterMut
    + IterAdjacent
    + IterAdjacentMut
    + Remove
    + Reserve
    // + for<'a> Ref<'a, N, W>
    + Sized
    + Clone
    + Debug
{
    fn bellman_ford_between(&self, from: NodeId<Self::Id>, to: NodeId<Self::Id>) -> Option<W::Cost>
    where W: EdgeCapacity<Capacity = W::Cost> + EdgeFlow<Flow = W::Cost> {
        bellman_ford_between(self, from, to)
    }

    fn bellman_ford(
        &self,
        start: NodeId<Self::Id>,
    ) -> Option<Distances<W::Cost, Self>> 
    where W: EdgeCapacity<Capacity = W::Cost> + EdgeFlow<Flow = W::Cost> {
        bellman_ford(self, start)
    }

    fn dijkstra(&self, from: NodeId<Self::Id>, to: NodeId<Self::Id>) -> Option<Distances<W::Cost, Self>> {
        dijkstra(self, from, to)
    }

    // fn edmonds_karp(&self, from: NodeId<Self::Id>, to: NodeId<Self::Id>) -> W::Cost 
    // where Self::Id = usize{
    //     edmonds_karp(self, from, to)
    // }

    fn kruskal_weight(&self) -> W::Cost {
        kruskal(self).1
    }

    fn kruskal(&self) -> (Tree<Self>, W::Cost) {
        kruskal(self)
    }

    fn prim(&self) -> W::Cost {
        prim(self)
    }

    fn dfs_scc(&self) -> Vec<Tree<Self>> {
        dfs_scc(self)
    }

    fn bfs_scc(&self) -> Vec<Tree<Self>> {
        bfs_scc(self)
    }

    fn dfs(&self, from: NodeId<Self::Id>) -> Tree<Self> {
        dfs(self, from)
    }

    fn bfs(&self, from: NodeId<Self::Id>) -> Tree<Self> {
        bfs(self, from)
    }

    fn nearest_neighbor(&self, start: NodeId<Self::Id>) -> Option<(Route<Self>, W::Cost)> {
        nearest_neighbor(self, start)
    }

    fn nearest_neighbor_from_first(&self) -> Option<(Route<Self>, W::Cost)> {
        nearest_neighbor_from_first(self)
    }

    fn double_tree(&self) -> Option<(Route<Self>, W::Cost)>
    {
        double_tree(self)
    }

    fn branch_bound(&self) -> Option<(Route<Self>, W::Cost)> {
        branch_bound(self)
    }

    fn branch_bound_rec(&self) -> Option<(Route<Self>, W::Cost)> {
        branch_bound_rec(self)
    }

    fn brute_force(&self) -> Option<(Route<Self>, W::Cost)> {
        brute_force(self)
    }
}

pub trait WeightlessGraph<N>:
    Base
    + Capacity
    + Clear
    + Contains
    + Count
    + Create
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
    fn dfs_scc(&self) -> Vec<Tree<Self>> {
        dfs_scc(self)
    }

    fn bfs_scc(&self) -> Vec<Tree<Self>> {
        bfs_scc(self)
    }

    fn dfs(&self, from: NodeId<Self::Id>) -> Tree<Self> {
        dfs(self, from)
    }

    fn bfs(&self, from: NodeId<Self::Id>) -> Tree<Self> {
        bfs(self, from)
    }
}
