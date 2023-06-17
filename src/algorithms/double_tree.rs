use std::fmt::Debug;
use std::ops::{Add, AddAssign};

use super::{dfs_iter, dijkstra_between, kruskal};
use crate::graph::{
    Base, Clear, Contains, Count, Create, Get, Index, IndexAdjacent, Insert, Iter, IterAdjacent,
    Sortable, WeightCost,
};
use crate::structures::Tour;

pub fn double_tree<N, W, C, G>(graph: &G) -> Option<Tour<G::NodeId, C>>
where
    N: PartialEq,
    C: Default + Sortable + Copy + AddAssign + Add<C, Output = C> + Debug,
    W: WeightCost<Cost = C> + Copy,
    G: Base
        + Count
        + IndexAdjacent
        + IterAdjacent<N, W>
        + Iter<N, W>
        + Index
        + Get<N, W>
        + Create<N>
        + Insert<N, W>
        + Clear
        + Contains<N>
        + Clone,
{
    let tree = kruskal(graph).0;
    let route = dfs_iter(&tree, tree.root()).collect::<Vec<_>>();
    let mut tour = Tour::new(route, C::default());
    let mut total_cost = C::default();

    for (from, to) in tour.edges() {
        let weight = match graph.contains_edge(*from, *to) {
            Some(edge_id) => *graph.weight(edge_id).unwrap().cost(),
            None if let Some(weight) = dijkstra_between(graph, *from, *to) => weight,
            _ => return None,
        };
        total_cost += weight;
    }

    tour.weight = total_cost;

    Some(tour)
}

#[cfg(test)]
mod test {
    extern crate test;
    use crate::{prelude::*, test::undigraph};
    use more_asserts::*;
    use test::Bencher;

    #[bench]
    fn double_tree_k_10_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/K_10.txt").unwrap();

        b.iter(|| {
            let total = graph.double_tree().unwrap().weight;
            assert_le!(total, 38.41 * 1.2);
        })
    }

    #[bench]
    fn double_tree_k_10e_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = graph.double_tree().unwrap().weight;
            assert_le!(total, 27.26 * 1.3);
        })
    }

    #[bench]
    fn double_tree_k_12_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/K_12.txt").unwrap();

        b.iter(|| {
            let total = graph.double_tree().unwrap().weight;
            assert_le!(total, 45.19 * 1.2);
        })
    }

    #[bench]
    fn double_tree_k_12e_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = graph.double_tree().unwrap().weight;
            assert_le!(total, 36.13 * 1.2);
        })
    }

    #[bench]
    fn double_tree_k_10_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/K_10.txt").unwrap();

        b.iter(|| {
            let total = graph.double_tree().unwrap().weight;
            assert_le!(total, 38.41 * 1.2);
        })
    }

    #[bench]
    fn double_tree_k_10e_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = graph.double_tree().unwrap().weight;
            assert_le!(total, 27.26 * 1.3);
        })
    }

    #[bench]
    fn double_tree_k_12_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/K_12.txt").unwrap();

        b.iter(|| {
            let total = graph.double_tree().unwrap().weight;
            assert_le!(total, 45.19 * 1.2);
        })
    }

    #[bench]
    fn double_tree_k_12e_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = graph.double_tree().unwrap().weight;
            assert_le!(total, 36.13 * 1.2);
        })
    }
}
