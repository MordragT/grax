use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

use grax_core::{
    collections::{
        EdgeCollection, EdgeIter, GetEdge, GetNode, GetNodeMut, IndexEdge, IndexEdgeMut, IndexNode,
        IndexNodeMut, InsertEdge, NodeCollection, NodeCount, NodeIter, NodeIterMut,
    },
    edge::{weight::*, *},
    graph::{EdgeAttribute, EdgeIterAdjacent, NodeAttribute},
    node::{weight::*, NodeMut, NodeRef},
};
use more_asserts::assert_gt;

use crate::{algorithms::bellman_ford_to, problems::ShortestPath, weight::TotalOrd};

/// successive shortest path
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
        + IndexNode
        + IndexNodeMut
        + IndexEdge
        + IndexEdgeMut
        + GetEdge
        + EdgeIter
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

    let back_edges = graph
        .iter_edges()
        .filter_map(|EdgeRef { edge_id, weight }| {
            if !graph.contains_edge_id(edge_id.rev()) {
                Some((edge_id.to(), edge_id.from(), weight.clone().reverse()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    graph.extend_edges(back_edges);

    loop {
        let mut s_candidates = Vec::new();
        let mut t_candidates = Vec::new();

        for NodeRef { node_id, weight } in graph.iter_nodes() {
            let balance = *balances.node(node_id).unwrap().weight;
            let residual_balance = *weight.balance();

            if residual_balance < balance {
                s_candidates.push(node_id);
            } else if residual_balance > balance {
                t_candidates.push(node_id);
            }
        }

        if s_candidates.is_empty() && t_candidates.is_empty() {
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

            return Some(cost);
        }

        let filter = |EdgeRef { edge_id: _, weight }: EdgeRef<G::Key, G::EdgeWeight>| {
            (*weight.capacity() - *weight.flow()) > C::default()
        };

        let ShortestPath {
            distance: _,
            from: source,
            to: sink,
            distances: _,
            parents,
        } = s_candidates.into_iter().find_map(|from| {
            for &to in &t_candidates {
                if let Some(path) = bellman_ford_to(graph, from, to, filter) {
                    return Some(path);
                }
            }
            None
        })?;

        let s_delta = balances[source] - *graph[source].balance();
        let t_delta = *graph[sink].balance() - balances[sink];

        assert_gt!(s_delta, C::default());
        assert_gt!(t_delta, C::default());

        let gamma = parents
            .iter_edges_to(source, sink)
            .map(|edge_id| {
                let weight = &graph[edge_id];
                *weight.capacity() - *weight.flow()
            })
            .chain([s_delta, t_delta])
            .min_by(TotalOrd::total_ord)
            .unwrap();

        for edge_id in parents.iter_edges_to(source, sink) {
            *graph[edge_id].flow_mut() += gamma;
            *graph[edge_id.rev()].flow_mut() -= gamma;
        }

        *graph[source].balance_mut() += gamma;
        *graph[sink].balance_mut() -= gamma;
    }
}

#[cfg(test)]
mod test {
    use super::ssp;
    use crate::test::bgraph;
    use grax_impl::*;

    #[test]
    fn ssp_kostenminimal_1() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal1.txt").unwrap();
        let cost = ssp(&mut graph).unwrap();
        assert_eq!(cost, 3.0);
    }

    #[test]
    fn ssp_kostenminimal_2() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal2.txt").unwrap();
        let cost = ssp(&mut graph).unwrap();
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn ssp_kostenminimal_3() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal3.txt").unwrap();
        let cost = ssp(&mut graph);
        assert!(cost.is_none());
    }

    #[test]
    fn ssp_kostenminimal_4() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal4.txt").unwrap();
        let cost = ssp(&mut graph);
        assert!(cost.is_none());
    }

    #[test]
    fn ssp_kostenminimal_gross_1() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross1.txt").unwrap();
        let cost = ssp(&mut graph).unwrap();
        assert_eq!(cost, 1537.0);
    }

    #[test]
    fn ssp_kostenminimal_gross_2() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross2.txt").unwrap();
        let cost = ssp(&mut graph).unwrap();
        assert_eq!(cost, 1838.0);
    }

    #[test]
    fn ssp_kostenminimal_gross_3() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross3.txt").unwrap();
        let cost = ssp(&mut graph);
        assert!(cost.is_none());
    }
}
