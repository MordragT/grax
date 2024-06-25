use super::{bellman_ford_cycle, edmonds_karp};
use crate::{problems::McfSolver, weight::TotalOrd};

use either::Either;
use grax_core::{
    collections::*,
    edge::{weight::*, *},
    graph::{EdgeAttribute, EdgeIterAdjacent, NodeAttribute},
    node::{weight::*, *},
};
use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct CycleCanceling;

impl<C, G> McfSolver<C, G> for CycleCanceling {
    fn solve(_: &G) -> Option<C> {
        todo!()
    }
}

pub fn cycle_canceling<C, G>(graph: &mut G) -> Option<C>
where
    C: Default
        + Add<C, Output = C>
        + AddAssign<C>
        + Sum
        + Sub<C, Output = C>
        + SubAssign<C>
        + Mul<C, Output = C>
        + Neg<Output = C>
        + Copy
        + Clone
        + PartialOrd
        + Debug
        + TotalOrd,
    G: EdgeCollection<EdgeWeight = FlowCostBundle<C>>
        + NodeCollection<NodeWeight = C>
        + EdgeAttribute
        + NodeAttribute
        + EdgeIterAdjacent
        + InsertEdge
        + InsertNode
        + EdgeIter
        + NodeIter
        + GetEdge
        + IndexEdge
        + IndexEdgeMut
        + NodeCount,
{
    dbg!(&graph);
    let mut to_insert = Vec::new();

    for EdgeRef { edge_id, weight } in graph.iter_edges() {
        if !graph.contains_edge_id(edge_id.rev()) {
            to_insert.push((edge_id.to(), edge_id.from(), weight.clone().reverse()));
        }
    }

    let source = graph.insert_node(C::default());
    let sink = graph.insert_node(C::default());

    for NodeRef { node_id, weight } in graph.iter_nodes() {
        let balance = *weight.balance();

        if balance > C::default() {
            // supply
            let weight = FlowCostBundle {
                capacity: balance,
                ..Default::default()
            };

            to_insert.push((source, node_id, weight.clone()));
            to_insert.push((node_id, source, weight.reverse()));
        } else if balance < C::default() {
            // demand
            let weight = FlowCostBundle {
                capacity: -balance,
                ..Default::default()
            };

            to_insert.push((node_id, sink, weight.clone()));
            to_insert.push((sink, node_id, weight.reverse()));
        }
    }

    graph.extend_edges(to_insert);

    if edmonds_karp(graph, source, sink)
        != graph
            .iter_nodes()
            .filter_map(|node| {
                let balance = *node.weight.balance();
                if balance > C::default() {
                    Some(balance)
                } else {
                    None
                }
            })
            .sum()
    {
        return None;
    }

    let node_ids = graph.node_ids().collect::<Vec<_>>();
    let start = graph.insert_node(C::default());

    for node_id in node_ids {
        graph.insert_edge(start, node_id, FlowCostBundle::default());
    }

    let filter = |EdgeRef { edge_id, weight }: EdgeRef<G::Key, G::EdgeWeight>| {
        edge_id.contains(start) || (*weight.capacity() - *weight.flow()) > C::default()
    };

    while let Either::Right(cycle) = bellman_ford_cycle(graph, start, filter) {
        let bottleneck = cycle
            .iter_edges()
            .map(|edge_id| {
                let weight = &graph[edge_id];
                *weight.capacity() - *weight.flow()
            })
            .min_by(TotalOrd::total_ord)
            .unwrap();

        for edge_id in cycle.iter_edges() {
            *graph[edge_id].flow_mut() += bottleneck;
            *graph[edge_id.rev()].flow_mut() -= bottleneck;
        }
    }

    let cost = graph
        .iter_edges()
        .filter_map(|edge| {
            let weight = edge.weight;
            if !weight.is_reverse() {
                Some(*weight.flow() * *weight.cost())
            } else {
                None
            }
        })
        .sum();

    Some(cost)
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::cycle_canceling;
    use crate::test::bgraph;
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn cycle_canceling_kostenminimal_1_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal1.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone()).unwrap();
            assert_eq!(cost, 3.0);
        });
    }

    #[bench]
    fn cycle_canceling_kostenminimal_2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal2.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone()).unwrap();
            assert_eq!(cost, 0.0);
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_3_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal3.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone());
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_4_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal4.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone());
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_gross_1_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross1.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone()).unwrap();
            assert_eq!(cost, 1537.0);
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_gross_2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross2.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone()).unwrap();
            assert_eq!(cost, 1838.0);
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_gross_3_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross3.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone());
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_1_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal1.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone()).unwrap();
            assert_eq!(cost, 3.0);
        });
    }

    #[bench]
    fn cycle_canceling_kostenminimal_2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal2.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone()).unwrap();
            assert_eq!(cost, 0.0);
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_3_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal3.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone());
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_4_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal4.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone());
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_gross_1_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal_gross1.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone()).unwrap();
            assert_eq!(cost, 1537.0);
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_gross_2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal_gross2.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone()).unwrap();
            assert_eq!(cost, 1838.0);
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_gross_3_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal_gross3.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph.clone());
            assert!(cost.is_none());
        })
    }
}
