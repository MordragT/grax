use std::ops::{Add, AddAssign};

use super::{dfs_tour, dijkstra_between, kruskal_mst, MinimumSpanningTree, Tour};
use crate::graph::{
    Base, Clear, Contains, Count, Create, Get, Index, IndexAdjacent, Insert, Iter, IterAdjacent,
    Sortable,
};

pub fn double_tree<N, W, G>(graph: &G) -> Option<Tour<G::NodeId, W>>
where
    N: PartialEq,
    W: Default + Sortable + Copy + AddAssign + Add<W, Output = W>,
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
    let MinimumSpanningTree::<G> { tree: mst, root } = kruskal_mst(graph);

    let mut route = dfs_tour(&mst, root).route;
    route.push(root);

    if route.len() != graph.node_count() + 1 {
        return None;
    }

    let mut total_weight = W::default();
    for [from, to] in route.array_windows::<2>() {
        let weight = match mst.contains_edge(*from, *to) {
            Some(index) => *mst.weight(index).unwrap(),
            None if let Some(weight) = dijkstra_between(graph, *from, *to) => weight,
            _ => return None,
        };
        total_weight += weight;
    }

    Some(Tour::new(route, total_weight))
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
