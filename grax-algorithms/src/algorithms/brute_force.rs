use std::iter::Sum;
use std::ops::Add;

use grax_core::collections::{GetEdge, NodeIter};
use grax_core::edge::*;
use grax_core::weight::{Maximum, Sortable};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use crate::util::Route;

pub fn brute_force<C, G>(graph: &G) -> Option<(Route<G>, C)>
where
    C: Default + Maximum + PartialOrd + Add<C, Output = C> + Copy + Send + Sync + Sum + Sortable,
    G: NodeIter + GetEdge + Send + Sync,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    let start = graph.node_ids().collect::<Vec<_>>();

    let best_cost = permute::permutations_of(&start)
        .par_bridge()
        .filter_map(|perm| {
            if let Some(mut edges) = perm
                .array_chunks::<2>()
                .map(|[from, to]| graph.find_edge_id(*from, *to))
                .collect::<Option<Vec<_>>>()
            {
                let from = edges.last().unwrap().to();
                let to = edges.first().unwrap().from();
                if let Some(edge_id) = graph.find_edge_id(from, to) {
                    edges.push(edge_id);

                    let total_cost: C = edges
                        .into_par_iter()
                        .map(|edge_id| *graph.edge(edge_id).unwrap().weight.cost())
                        .sum();

                    return Some(total_cost);
                }
            }
            None
        })
        .min_by(|a, b| a.sort(b))
        .unwrap();

    if best_cost == C::MAX {
        None
    } else {
        Some((Route::new(Vec::new()), best_cost))
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::brute_force;
    use crate::test::undigraph;
    use grax_impl::*;
    use more_asserts::*;
    use test::Bencher;

    #[bench]
    fn brute_force_k_10_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().1;
            assert_le!(total, 38.41);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_10e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().1;
            assert_le!(total, 27.26);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_12_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().1;
            assert_le!(total, 45.19);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_12e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().1;
            assert_le!(total, 36.13);
        })
    }

    // csr

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_10_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().1;
            assert_le!(total, 38.41);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_10e_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().1;
            assert_le!(total, 27.26);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_12_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().1;
            assert_le!(total, 45.19);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_12e_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().1;
            assert_le!(total, 36.13);
        })
    }

    // dense

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_10_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().1;
            assert_le!(total, 38.41);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_10e_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().1;
            assert_le!(total, 27.26);
        })
    }
    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_12_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().1;
            assert_le!(total, 45.19);
        })
    }
    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_12e_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().1;
            assert_le!(total, 36.13);
        })
    }
}
