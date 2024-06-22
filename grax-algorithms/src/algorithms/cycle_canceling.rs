use super::{bellman_ford_cycle, edmonds_karp};
use crate::problems::McfSolver;

use either::Either;
use grax_core::{
    collections::*,
    edge::{EdgeCost, EdgeRef},
    graph::{EdgeAttribute, EdgeIterAdjacent, NodeAttribute},
    node::NodeRef,
    weight::Sortable,
};
use grax_flow::{BalancedNode, EdgeFlow, FlowBundle, NodeBalance};
use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct CycleCanceling;

impl<C, G> McfSolver<C, G> for CycleCanceling {
    fn solve(graph: &G) -> Option<C> {
        todo!()
    }
}

pub fn cycle_canceling<G, N, W, C>(graph: &mut G) -> Option<C>
where
    N: Default,
    W: Default + Copy + Debug,
    C: Default
        + Add<C, Output = C>
        + AddAssign<C>
        + Sum
        + Sub<C, Output = C>
        + SubAssign<C>
        + Mul<C, Output = C>
        + Neg<Output = C>
        + Copy
        + PartialOrd
        + Debug
        + Sortable,

    G: EdgeCollection<EdgeWeight = FlowBundle<W, C>>
        + NodeCollection<NodeWeight = BalancedNode<N, C>>
        + EdgeAttribute
        + NodeAttribute
        + EdgeIterAdjacent
        + InsertEdge
        + InsertNode
        + EdgeIter
        + NodeIter
        + GetEdge
        + GetEdgeMut
        + NodeCount,
{
    let mut to_insert = Vec::new();

    for EdgeRef { edge_id, weight } in graph.iter_edges() {
        if !graph.contains_edge_id(edge_id.rev()) {
            to_insert.push((edge_id.to(), edge_id.from(), weight.clone().rev()));
        }
    }

    let source = graph.insert_node(BalancedNode::default());
    let sink = graph.insert_node(BalancedNode::default());

    for NodeRef { node_id, weight } in graph.iter_nodes() {
        let balance = *weight.balance();

        if balance > C::default() {
            // supply
            let weight = FlowBundle {
                capacity: balance,
                ..Default::default()
            };

            to_insert.push((source, node_id, weight.clone()));
            to_insert.push((node_id, source, weight.rev()));
        } else if balance < C::default() {
            // demand
            let weight = FlowBundle {
                capacity: -balance,
                ..Default::default()
            };

            to_insert.push((node_id, sink, weight.clone()));
            to_insert.push((sink, node_id, weight.rev()));
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
    let start = graph.insert_node(BalancedNode::default());

    for node_id in node_ids {
        graph.insert_edge(start, node_id, FlowBundle::default());
    }

    let filter = |EdgeRef { edge_id, weight }: EdgeRef<G::Key, G::EdgeWeight>| {
        edge_id.contains(start) || (*weight.capacity() - *weight.flow()) > C::default()
    };

    while let Either::Right(cycle) = bellman_ford_cycle(graph, start, filter) {
        let bottleneck = cycle
            .iter_edges()
            .map(|edge_id| {
                let weight = graph.edge(edge_id).unwrap().weight;
                *weight.capacity() - *weight.flow()
            })
            .min_by(|a, b| a.sort(b))
            .unwrap();

        for edge_id in cycle.iter_edges() {
            let weight = graph.edge_mut(edge_id).unwrap().weight;
            *weight.flow_mut() += bottleneck;

            let weight_rev = graph.edge_mut(edge_id.rev()).unwrap().weight;
            *weight_rev.flow_mut() -= bottleneck;
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
    use super::cycle_canceling;
    use crate::test::bgraph;
    use grax_impl::*;

    #[test]
    fn cycle_canceling_kostenminimal_1() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal1.txt").unwrap();
        let cost = cycle_canceling(&mut graph).unwrap();
        assert_eq!(cost, 3.0);
    }

    #[test]
    fn cycle_canceling_kostenminimal_2() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal2.txt").unwrap();
        let cost = cycle_canceling(&mut graph).unwrap();
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn cycle_canceling_kostenminimal_3() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal3.txt").unwrap();
        let cost = cycle_canceling(&mut graph);
        assert!(cost.is_none());
    }

    #[test]
    fn cycle_canceling_kostenminimal_4() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal4.txt").unwrap();
        let cost = cycle_canceling(&mut graph);
        assert!(cost.is_none());
    }

    #[test]
    fn cycle_canceling_kostenminimal_gross_1() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross1.txt").unwrap();
        let cost = cycle_canceling(&mut graph).unwrap();
        assert_eq!(cost, 1537.0);
    }

    #[test]
    fn cycle_canceling_kostenminimal_gross_2() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross2.txt").unwrap();
        let cost = cycle_canceling(&mut graph).unwrap();
        assert_eq!(cost, 1838.0);
    }

    #[test]
    fn cycle_canceling_kostenminimal_gross_3() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross3.txt").unwrap();
        let cost = cycle_canceling(&mut graph);
        assert!(cost.is_none());
    }
}
