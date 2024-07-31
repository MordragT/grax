use super::{bellman_ford_to, insert_residual_edges, sum_cost_flow};
use crate::{
    algorithms::remove_residual_edges, flow::FlowCostBundle, path::ShortestPath,
    prelude::empty_flow, weight::TotalOrd,
};

use grax_core::{
    collections::{
        EdgeCollection, EdgeIter, EdgeIterMut, GetEdge, GetNodeMut, IndexEdge, IndexEdgeMut,
        IndexNode, IndexNodeMut, InsertEdge, NodeCollection, NodeCount, NodeIter, NodeIterMut,
        RemoveEdge,
    },
    edge::{weight::*, *},
    graph::{EdgeAttribute, EdgeIterAdjacent, NodeAttribute},
    node::{weight::*, NodeMut, NodeRef},
};
use more_asserts::assert_gt;
use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

pub fn ssp<C, G>(graph: &mut G) -> Option<C>
where
    C: PartialOrd
        + Default
        + Copy
        + Debug
        + Neg<Output = C>
        + Add<C, Output = C>
        + Sub<C, Output = C>
        + Mul<C, Output = C>
        + AddAssign<C>
        + SubAssign<C>
        + Sum
        + TotalOrd,
    G: EdgeCollection<EdgeWeight = FlowCostBundle<C>>
        + NodeCollection<NodeWeight = C>
        + InsertEdge
        + RemoveEdge
        + IndexNode
        + IndexNodeMut
        + IndexEdge
        + IndexEdgeMut
        + GetEdge
        + EdgeIter
        + EdgeIterMut
        + NodeIter
        + NodeIterMut
        + NodeAttribute
        + EdgeIterAdjacent
        + EdgeAttribute
        + NodeCount,
{
    let mut balances = graph.fixed_node_map(C::default());

    for NodeMut { node_id, weight } in graph.iter_nodes_mut() {
        balances.update_node(node_id, **weight.balance());
        *weight.balance_mut() = C::default()
    }

    let to_augment = graph
        .iter_edges()
        .filter_map(|EdgeRef { edge_id, weight }| {
            if *weight.cost() < C::default() {
                let flow = *weight.capacity();
                Some((edge_id, flow))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for (edge_id, flow) in to_augment {
        *graph[edge_id].flow_mut() += flow;
        *graph[edge_id.from()].balance_mut() += flow;
        *graph[edge_id.to()].balance_mut() -= flow;
    }

    insert_residual_edges(graph);

    loop {
        let mut s_candidates = Vec::new();
        let mut t_candidates = Vec::new();

        for NodeRef { node_id, weight } in graph.iter_nodes() {
            let balance = balances[node_id];
            let residual_balance = *weight.balance();

            if residual_balance < balance {
                s_candidates.push(node_id);
            } else if residual_balance > balance {
                t_candidates.push(node_id);
            }
        }

        if s_candidates.is_empty() && t_candidates.is_empty() {
            let cost = sum_cost_flow(graph);

            remove_residual_edges(graph);

            return Some(cost);
        }

        let filter =
            |edge: EdgeRef<_, G::EdgeWeight>| edge.weight.residual_capacity() > C::default();

        let Some(ShortestPath {
            distance: _,
            from: source,
            to: sink,
            distances: _,
            parents,
        }) = s_candidates.into_iter().find_map(|from| {
            for &to in &t_candidates {
                if let Some(path) = bellman_ford_to(graph, from, to, filter) {
                    return Some(path);
                }
            }
            None
        })
        else {
            remove_residual_edges(graph);
            empty_flow(graph);

            for NodeRef { node_id, weight } in balances.iter_nodes() {
                graph[node_id] = *weight;
            }

            return None;
        };

        let s_delta = balances[source] - *graph[source].balance();
        let t_delta = *graph[sink].balance() - balances[sink];

        assert_gt!(s_delta, C::default());
        assert_gt!(t_delta, C::default());

        let gamma = parents
            .iter_edges_to(source, sink)
            .map(|edge_id| graph[edge_id].residual_capacity())
            .chain([s_delta, t_delta])
            .min_by(TotalOrd::total_ord)
            .unwrap();

        for edge_id in parents.iter_edges_to(source, sink) {
            *graph[edge_id].flow_mut() += gamma;
            *graph[edge_id.reverse()].flow_mut() -= gamma;
        }

        *graph[source].balance_mut() += gamma;
        *graph[sink].balance_mut() -= gamma;
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::ssp;
    use crate::algorithms::empty_flow;
    use crate::test::bgraph;
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn ssp_kostenminimal_1_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal1.txt").unwrap();
        let orig = graph.clone();
        b.iter(|| {
            let cost = ssp(&mut graph).unwrap();
            assert_eq!(cost, 3.0);
            empty_flow(&mut graph);
            assert_eq!(graph, orig);
        });
    }

    #[bench]
    fn ssp_kostenminimal_2_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal2.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph).unwrap();
            assert_eq!(cost, 0.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn ssp_kostenminimal_3_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal3.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph);
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn ssp_kostenminimal_4_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal4.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph);
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn ssp_kostenminimal_gross_1_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross1.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph).unwrap();
            assert_eq!(cost, 1537.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn ssp_kostenminimal_gross_2_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross2.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph).unwrap();
            assert_eq!(cost, 1838.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn ssp_kostenminimal_gross_3_adj_list(b: &mut Bencher) {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross3.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph);
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn ssp_kostenminimal_1_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal1.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph).unwrap();
            assert_eq!(cost, 3.0);
            empty_flow(&mut graph);
        });
    }

    #[bench]
    fn ssp_kostenminimal_2_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal2.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph).unwrap();
            assert_eq!(cost, 0.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn ssp_kostenminimal_3_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal3.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph);
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn ssp_kostenminimal_4_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal4.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph);
            assert!(cost.is_none());
        })
    }

    #[bench]
    fn ssp_kostenminimal_gross_1_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal_gross1.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph).unwrap();
            assert_eq!(cost, 1537.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn ssp_kostenminimal_gross_2_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal_gross2.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph).unwrap();
            assert_eq!(cost, 1838.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn ssp_kostenminimal_gross_3_csr_mat(b: &mut Bencher) {
        let mut graph: CsrGraph<_, _, true> = bgraph("../data/Kostenminimal_gross3.txt").unwrap();
        b.iter(|| {
            let cost = ssp(&mut graph);
            assert!(cost.is_none());
        })
    }
}
