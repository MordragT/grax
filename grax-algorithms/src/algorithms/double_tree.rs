use std::fmt::Debug;
use std::iter::Sum;
use std::ops::AddAssign;

use crate::{dfs_where, kruskal};

use crate::util::{Mst, Path};
use grax_core::collections::{EdgeCollection, EdgeIter, GetEdge, NodeCount, NodeIter};
use grax_core::edge::*;
use grax_core::graph::{EdgeAttribute, EdgeIterAdjacent, NodeAttribute};
use grax_core::weight::{Maximum, Sortable};

pub fn double_tree<C, G>(graph: &G) -> Option<(C, Path<G>)>
where
    C: Default + Sortable + Copy + AddAssign + Debug + Maximum + Sum<C>,
    G: NodeAttribute
        + EdgeAttribute
        + NodeIter
        + EdgeIter
        + EdgeIterAdjacent
        + GetEdge
        + EdgeCollection<EdgeWeight: Send + Sync>
        + NodeCount,
    G::EdgeWeight: EdgeCost<Cost = C>,
    G::FixedEdgeMap<bool>: 'static,
    G::FixedNodeMap<bool>: 'static,
{
    let Mst {
        root,
        edges,
        total_cost: _,
    } = kruskal(graph)?;

    let path = dfs_where(graph, root, |edge| edges.contains_edge_id(edge.edge_id));
    let cost = path
        .parents
        .edge_ids()
        .map(|edge_id| *graph.edge(edge_id).unwrap().weight.cost())
        .sum();

    Some((cost, path))
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
        let mut graph: AdjGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&mut graph).unwrap().0;
            println!("{total}");

            assert_le!(total, 38.41 * 1.5);
            assert_ge!(total, 38.41 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_10e_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&mut graph).unwrap().0;
            println!("{total}");

            assert_le!(total, 27.26 * 2.0);
            assert_ge!(total, 27.26 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_12_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&mut graph).unwrap().0;
            println!("{total}");

            assert_le!(total, 45.19 * 1.5);
            assert_ge!(total, 45.19 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_12e_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&mut graph).unwrap().0;
            println!("{total}");

            assert_le!(total, 36.13 * 2.0);
            assert_ge!(total, 36.13 * 0.5);
        })
    }

    // csr

    #[bench]
    fn double_tree_k_10_csr_graph(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&mut graph).unwrap().0;
            println!("{total}");

            assert_le!(total, 38.41 * 1.5);
            assert_ge!(total, 38.41 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_10e_csr_graph(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&mut graph).unwrap().0;
            println!("{total}");

            assert_le!(total, 27.26 * 2.0);
            assert_ge!(total, 27.26 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_12_csr_graph(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&mut graph).unwrap().0;
            println!("{total}");

            assert_le!(total, 45.19 * 1.5);
            assert_ge!(total, 45.19 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_12e_csr_graph(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&mut graph).unwrap().0;
            println!("{total}");

            assert_le!(total, 36.13 * 2.0);
            assert_ge!(total, 36.13 * 0.5);
        })
    }

    // dense

    #[bench]
    fn double_tree_k_10_dense_mat(b: &mut Bencher) {
        let mut graph: MatGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&mut graph).unwrap().0;
            println!("{total}");

            assert_le!(total, 38.41 * 1.5);
            assert_ge!(total, 38.41 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_10e_dense_mat(b: &mut Bencher) {
        let mut graph: MatGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&mut graph).unwrap().0;
            println!("{total}");

            assert_le!(total, 27.26 * 2.0);
            assert_ge!(total, 27.26 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_12_dense_mat(b: &mut Bencher) {
        let mut graph: MatGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&mut graph).unwrap().0;
            println!("{total}");

            assert_le!(total, 45.19 * 1.5);
            assert_ge!(total, 45.19 * 0.5);
        })
    }

    #[bench]
    fn double_tree_k_12e_dense_mat(b: &mut Bencher) {
        let mut graph: MatGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = double_tree(&mut graph).unwrap().0;
            println!("{total}");

            assert_le!(total, 36.13 * 2.0);
            assert_ge!(total, 36.13 * 0.5);
        })
    }
}
