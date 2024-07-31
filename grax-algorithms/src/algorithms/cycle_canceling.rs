use super::{
    bellman_ford_cycle, min_residual_capacity, sum_cost_flow, Bfs, McfSolver, _ford_fulkerson,
    empty_flow,
};
use crate::{flow::FlowCostBundle, weight::TotalOrd};

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

impl<C, G> McfSolver<C, G> for CycleCanceling
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
        + RemoveNode
        + RemoveEdge
        + EdgeIter
        + EdgeIterMut
        + NodeIter
        + GetEdge
        + IndexEdge
        + IndexEdgeMut
        + NodeCount,
{
    fn solve(graph: &mut G) -> Option<C> {
        cycle_canceling(graph)
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
        + RemoveNode
        + RemoveEdge
        + EdgeIter
        + EdgeIterMut
        + NodeIter
        + GetEdge
        + IndexEdge
        + IndexEdgeMut
        + NodeCount,
{
    let mut to_insert = Vec::new();

    for EdgeRef { edge_id, weight } in graph.iter_edges() {
        if !graph.contains_edge_id(edge_id.reverse()) {
            to_insert.push((edge_id.to(), edge_id.from(), weight.reverse()));
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

    if _ford_fulkerson(graph, source, sink, Bfs)
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
        graph.remove_node(sink);
        graph.remove_node(source);

        graph.retain_edges(|EdgeRef { edge_id, weight }| {
            !weight.is_reverse() && !edge_id.contains(sink) && !edge_id.contains(source)
        });
        empty_flow(graph);
        return None;
    }

    let node_ids = graph.node_ids().collect::<Vec<_>>();
    let start = graph.insert_node(C::default());

    for node_id in node_ids {
        graph.insert_edge(start, node_id, FlowCostBundle::default());
    }

    let filter = |edge: EdgeRef<_, FlowCostBundle<C>>| {
        edge.edge_id.contains(start) || edge.weight.residual_capacity() > C::default()
    };

    while let Either::Right(cycle) = bellman_ford_cycle(graph, start, filter) {
        let gamma = min_residual_capacity(graph, cycle.iter_edges()).unwrap();

        for edge_id in cycle.iter_edges() {
            *graph[edge_id].flow_mut() += gamma;
            *graph[edge_id.reverse()].flow_mut() -= gamma;
        }
    }

    let cost = sum_cost_flow(graph);

    graph.retain_edges(|EdgeRef { edge_id, weight }| {
        !weight.is_reverse()
            && !edge_id.contains(start)
            && !edge_id.contains(sink)
            && !edge_id.contains(source)
    });

    // order is important as underlying graph may or may not have stable node indices
    // so that removal of nodes might shift all nodes
    graph.remove_node(start);
    graph.remove_node(sink);
    graph.remove_node(source);

    Some(cost)
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::cycle_canceling;
    use crate::algorithms::empty_flow;
    use crate::test::bgraph;
    use grax_impl::*;
    use test::Bencher;

    // #[test]
    // fn cycle_canceling_cleanup() {
    //     use crate::prelude::ssp;

    //     let graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal1.txt").unwrap();

    //     let mut cc_graph = graph.clone();
    //     let cost = cycle_canceling(&mut cc_graph).unwrap();
    //     assert_eq!(cost, 3.0);

    //     let mut ssp_graph = graph.clone();
    //     let cost = ssp(&mut ssp_graph).unwrap();
    //     assert_eq!(cost, 3.0);

    //     assert_eq!(cc_graph, ssp_graph);
    // }

    #[bench]
    fn cycle_canceling_kostenminimal_1_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal1.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph).unwrap();
            assert_eq!(cost, 3.0);
            empty_flow(&mut graph);
        });
    }

    #[bench]
    fn cycle_canceling_kostenminimal_2_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal2.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph).unwrap();
            assert_eq!(cost, 0.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_3_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal3.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph);
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_4_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal4.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph);
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_gross_1_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross1.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph).unwrap();
            assert_eq!(cost, 1537.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_gross_2_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross2.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph).unwrap();
            assert_eq!(cost, 1838.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_gross_3_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross3.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph);
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_1_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal1.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph).unwrap();
            assert_eq!(cost, 3.0);
            empty_flow(&mut graph);
        });
    }

    #[bench]
    fn cycle_canceling_kostenminimal_2_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal2.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph).unwrap();
            assert_eq!(cost, 0.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_3_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal3.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph);
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_4_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal4.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph);
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_gross_1_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal_gross1.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph).unwrap();
            assert_eq!(cost, 1537.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_gross_2_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal_gross2.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph).unwrap();
            assert_eq!(cost, 1838.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn cycle_canceling_kostenminimal_gross_3_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal_gross3.txt").unwrap();
        b.iter(|| {
            let cost = cycle_canceling(&mut graph);
            assert!(cost.is_none());
        })
    }
}
