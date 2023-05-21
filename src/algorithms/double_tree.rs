use std::ops::{Add, AddAssign};

use crate::prelude::{GraphAccess, GraphAdjacentTopology, GraphCompare, GraphTopology, Sortable};

use super::{dfs_tour, dijkstra_between, kruskal_mst, MinimumSpanningTree, Tour};

pub fn double_tree<N, W, G>(graph: &G) -> Option<Tour<W>>
where
    N: PartialEq,
    W: Default + Sortable + Copy + AddAssign + Add<W, Output = W>,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W> + GraphCompare<N, W>,
{
    let MinimumSpanningTree { tree: mst, root } = kruskal_mst(graph);

    let mut route = dfs_tour(&mst, root).route;
    route.push(root);

    if route.len() != graph.node_count() + 1 {
        return None;
    }

    let mut total_weight = W::default();
    for [from, to] in route.array_windows::<2>() {
        let weight = match mst.contains_edge(*from, *to) {
            Some(index) => *mst.weight(index),
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
    use crate::{adjacency_matrix::AdjacencyMatrix, prelude::*, test::undigraph};
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
