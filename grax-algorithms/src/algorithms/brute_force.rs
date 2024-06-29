use super::TspSolver;
use crate::{
    cycle::{Cycle, TspCycle},
    parents::Parents,
    weight::{Bounded, TotalOrd},
};

use grax_core::collections::{GetEdge, NodeIter};
use grax_core::edge::weight::*;
use grax_core::graph::NodeAttribute;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct BruteForce;

impl<C, G> TspSolver<C, G> for BruteForce
where
    C: Default + Bounded + PartialOrd + Add<C, Output = C> + Copy + Send + Sync + Sum + TotalOrd,
    G: NodeIter + GetEdge + NodeAttribute + Send + Sync,
    G::EdgeWeight: Cost<C>,
{
    fn solve(graph: &G) -> Option<TspCycle<C, G>> {
        brute_force(graph)
    }
}

pub fn brute_force<C, G>(graph: &G) -> Option<TspCycle<C, G>>
where
    C: Default + Bounded + PartialOrd + Add<C, Output = C> + Copy + Send + Sync + Sum + TotalOrd,
    G: NodeIter + GetEdge + NodeAttribute + Send + Sync,
    G::EdgeWeight: Cost<C>,
{
    let start = graph.node_ids().collect::<Vec<_>>();

    let (best_cost, edges) = permute::permutations_of(&start)
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
                        .iter()
                        .copied()
                        .map(|edge_id| *graph.edge(edge_id).unwrap().weight.cost())
                        .sum();

                    return Some((total_cost, edges));
                }
            }
            None
        })
        .min_by(|a, b| a.0.total_ord(&b.0))?;

    if best_cost == C::MAX {
        None
    } else {
        let mut parents = Parents::new(graph);
        parents.extend(
            edges
                .into_iter()
                .map(|edge_id| (edge_id.from(), edge_id.to())),
        );

        let cycle = Cycle {
            member: parents.first().unwrap(),
            parents,
        };

        Some(TspCycle {
            cost: best_cost,
            cycle,
        })
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
            let total = brute_force(&graph).unwrap().cost;
            assert_le!(total, 38.41);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_10e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().cost;
            assert_le!(total, 27.26);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_12_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().cost;
            assert_le!(total, 45.19);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_12e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().cost;
            assert_le!(total, 36.13);
        })
    }

    // csr

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_10_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().cost;
            assert_le!(total, 38.41);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_10e_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().cost;
            assert_le!(total, 27.26);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_12_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().cost;
            assert_le!(total, 45.19);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_12e_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().cost;
            assert_le!(total, 36.13);
        })
    }

    // dense

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_10_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().cost;
            assert_le!(total, 38.41);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_10e_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().cost;
            assert_le!(total, 27.26);
        })
    }
    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_12_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().cost;
            assert_le!(total, 45.19);
        })
    }
    #[cfg(feature = "extensive")]
    #[bench]
    fn brute_force_k_12e_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = brute_force(&graph).unwrap().cost;
            assert_le!(total, 36.13);
        })
    }
}
