pub use bellman_ford::*;
pub use bfs::*;
pub use branch_bound::*;
pub use brute_force::*;
pub use cycle_canceling::*;
pub use dfs::*;
pub use dijkstra::*;
pub use double_tree::*;
pub use edmonds_karp::*;
pub use ford_fulkerson::*;
pub use kruskal::*;
pub use nearest_neighbor::*;
pub use prim::*;
pub use ssp::*;
pub use union_find::*;

mod bellman_ford;
mod bfs;
mod branch_bound;
mod brute_force;
mod cdcl;
mod cycle_canceling;
mod dfs;
mod dijkstra;
mod double_tree;
mod edmonds_karp;
mod ford_fulkerson;
mod kruskal;
mod nearest_neighbor;
mod prim;
mod ssp;
mod union_find;

use crate::{
    cycle::TspCycle,
    flow::FlowBundle,
    path::{Path, ShortestPath},
    tree::{Mst, PathTree, ShortestPathTree},
    weight::TotalOrd,
};
use grax_core::{
    collections::{
        EdgeCollection, EdgeIter, EdgeIterMut, GetEdge, IndexEdge, InsertEdge, RemoveEdge,
    },
    edge::{
        weight::{Cost, Flow, ResidualCapacity, Reverse},
        Edge, EdgeRef,
    },
    graph::{AdaptEdges, NodeAttribute},
    index::{EdgeId, NodeId},
};
use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Mul, Neg, Sub},
};

pub trait McfSolver<C, G> {
    /// Returns the minimum cost flow if existent
    /// and mutates the graph's edges with the respective flow of each edge
    fn solve(graph: &mut G) -> Option<C>;
}

pub trait MstBuilder<C, G>: Sized + Copy
where
    G: NodeAttribute,
{
    /// Constructs a minimal spanning tree from a graph
    /// Returns none if such tree cannot be created
    fn mst(self, graph: &G) -> Option<Mst<C, G>>;
}

pub trait PathFinder<G>: Sized + Copy
where
    G: NodeAttribute + EdgeCollection,
{
    /// Returns the path between two nodes
    /// Returns none if no path could be found
    fn path_where<F>(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
        filter: F,
    ) -> Option<Path<G>>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool;

    /// Returns the path tree starting from the specified node
    fn path_tree_where<F>(self, graph: &G, from: NodeId<G::Key>, filter: F) -> PathTree<G>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool;

    /// Returns the path between two nodes
    /// Returns none if no path could be found
    fn path(self, graph: &G, from: NodeId<G::Key>, to: NodeId<G::Key>) -> Option<Path<G>> {
        self.path_where(graph, from, to, |_| true)
    }

    /// Returns the path tree starting from the specified node
    fn path_tree(self, graph: &G, from: NodeId<G::Key>) -> PathTree<G> {
        self.path_tree_where(graph, from, |_| true)
    }
}

pub trait ShortestPathFinder<C, G>: Sized + Copy
where
    C: Clone + Debug,
    G: NodeAttribute + EdgeCollection,
{
    /// Returns the shortest path between two nodes
    /// Returns none if no path could be found
    fn shortest_path_where<F>(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
        filter: F,
    ) -> Option<ShortestPath<C, G>>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool;

    /// Returns the shortest path tree starting from the specified node
    fn shortest_path_tree_where<F>(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        filter: F,
    ) -> ShortestPathTree<C, G>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool;

    /// Returns the shortest path between two nodes
    /// Returns none if no path could be found
    fn shortest_path(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
    ) -> Option<ShortestPath<C, G>> {
        self.shortest_path_where(graph, from, to, |_| true)
    }

    /// Returns the shortest path tree starting from the specified node
    fn shortest_path_tree(self, graph: &G, from: NodeId<G::Key>) -> ShortestPathTree<C, G> {
        self.shortest_path_tree_where(graph, from, |_| true)
    }
}

pub trait TspSolver<C, G>
where
    G: NodeAttribute,
{
    /// Returns depending on the implementation an exact or approximate shortest route
    /// Returns none if such cycle cannot be found
    fn solve(graph: &G) -> Option<TspCycle<C, G>>;
}

pub fn flow_adaptor<G1, G2, C>(graph: G1) -> G2
where
    C: Default + Copy + Neg<Output = C>,
    G1: EdgeCollection<EdgeWeight = C> + AdaptEdges<G2, FlowBundle<C>> + EdgeIter,
    G1::EdgeWeight: Cost<C>,
    G2: EdgeCollection<EdgeWeight = FlowBundle<C>>,
{
    graph.map_edges(|Edge { edge_id, weight }| {
        let cost = *weight.cost();

        let bundle = FlowBundle {
            capacity: cost,
            flow: C::default(),
            reverse: false,
        };

        Edge::new(edge_id, bundle)
    })
}

pub fn empty_flow<C, G>(graph: &mut G)
where
    C: Default,
    G: EdgeIterMut,
    G::EdgeWeight: Flow<C>,
{
    for edge in graph.iter_edges_mut() {
        *edge.weight.flow_mut() = C::default();
    }
}

fn sum_cost_flow<C, G>(graph: &G) -> C
where
    C: Mul<C, Output = C> + Sum + Copy,
    G: EdgeIter,
    G::EdgeWeight: Flow<C> + Cost<C> + Reverse,
{
    graph
        .iter_edges()
        .filter_map(|edge| {
            let weight = edge.weight;
            if !weight.is_reverse() {
                Some(*weight.flow() * *weight.cost())
            } else {
                None
            }
        })
        .sum()
}

fn min_residual_capacity<C, G>(
    graph: &G,
    edge_ids: impl IntoIterator<Item = EdgeId<G::Key>>,
) -> Option<C>
where
    C: TotalOrd + Copy + Sub<C, Output = C> + Default + PartialEq,
    G: IndexEdge,
    G::EdgeWeight: ResidualCapacity<C>,
{
    edge_ids
        .into_iter()
        .map(|edge_id| graph[edge_id].residual_capacity())
        .min_by(TotalOrd::total_ord)
}

fn insert_residual_edges<G>(graph: &mut G)
where
    G: InsertEdge + GetEdge + EdgeIter,
    G::EdgeWeight: Reverse,
{
    let edges = graph
        .iter_edges()
        .filter_map(|EdgeRef { edge_id, weight }| {
            if !graph.contains_edge_id(edge_id.reverse()) {
                Some((edge_id.to(), edge_id.from(), weight.reverse()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    graph.extend_edges(edges);
}

fn remove_residual_edges<G>(graph: &mut G)
where
    G: RemoveEdge,
    G::EdgeWeight: Reverse,
{
    graph.retain_edges(|edge| !edge.weight.is_reverse())
}
