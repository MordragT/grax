use std::fmt::Debug;
use std::ops::{Add, AddAssign};

use crate::{dfs_iter_edges, kruskal, prim};

use grax_core::algorithms::Mst;
use grax_core::collections::{EdgeCollection, EdgeIter, GetEdge, NodeCount, NodeIter};
use grax_core::edge::*;
use grax_core::graph::{Cost, EdgeAttribute, EdgeIterAdjacent, NodeAttribute};
use grax_core::view::{FilterEdgeView, ViewGraph};
use grax_core::weight::{Maximum, Sortable};

pub fn double_tree<C, G>(graph: &G) -> Option<(FilterEdgeView<G>, C)>
where
    C: Default + Sortable + Copy + AddAssign + Add<C, Output = C> + Debug + Maximum,
    G: NodeAttribute
        + EdgeAttribute
        + Cost<C>
        + NodeIter
        + EdgeIter
        + EdgeIterAdjacent
        + GetEdge
        + EdgeCollection<EdgeWeight: Send + Sync>
        + NodeCount,
{
    let Mst {
        root,
        filter,
        total_cost: _,
    } = prim(graph)?;

    // TODO differentiate between owned graph and ref graph
    let tree = ViewGraph::new(graph, filter);
    let mut view = FilterEdgeView::new(graph);

    let mut cost = C::default();

    for edge_id in dfs_iter_edges(&tree, root) {
        cost += *graph.edge(edge_id).unwrap().weight.cost();
        view.keep(edge_id);
    }

    Some((view, cost))
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::double_tree;
    use crate::test::undigraph;
    use grax_impl::*;
    use more_asserts::*;
    use test::Bencher;

    #[bench]
    fn double_tree_k_10_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&graph).unwrap().1;
            assert_le!(total, 38.41 * 1.5);
            assert_ge!(total, 38.41 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_10e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&graph).unwrap().1;
            assert_le!(total, 27.26 * 2.0);
            assert_ge!(total, 27.26 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_12_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&graph).unwrap().1;
            assert_le!(total, 45.19 * 1.5);
            assert_ge!(total, 45.19 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_12e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&graph).unwrap().1;
            assert_le!(total, 36.13 * 2.0);
            assert_ge!(total, 36.13 * 0.5);
        })
    }

    // csr

    #[bench]
    fn double_tree_k_10_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&graph).unwrap().1;
            assert_le!(total, 38.41 * 1.5);
            assert_ge!(total, 38.41 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_10e_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&graph).unwrap().1;
            assert_le!(total, 27.26 * 2.0);
            assert_ge!(total, 27.26 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_12_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&graph).unwrap().1;
            assert_le!(total, 45.19 * 1.5);
            assert_ge!(total, 45.19 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_12e_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&graph).unwrap().1;
            assert_le!(total, 36.13 * 2.0);
            assert_ge!(total, 36.13 * 0.5);
        })
    }

    // dense

    #[bench]
    fn double_tree_k_10_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&graph).unwrap().1;
            assert_le!(total, 38.41 * 1.5);
            assert_ge!(total, 38.41 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_10e_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&graph).unwrap().1;
            assert_le!(total, 27.26 * 2.0);
            assert_ge!(total, 27.26 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_12_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&graph).unwrap().1;
            assert_le!(total, 45.19 * 1.5);
            assert_ge!(total, 45.19 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_12e_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&graph).unwrap().1;
            assert_le!(total, 36.13 * 2.0);
            assert_ge!(total, 36.13 * 0.5);
        })
    }
}
