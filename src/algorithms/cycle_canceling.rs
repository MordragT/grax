use super::{bellman_ford_cycle, Mcf};
use crate::graph::{
    Base, Count, EdgeCapacity, EdgeCost, EdgeDirection, EdgeFlow, Get, GetMut, Index,
    IndexAdjacent, Insert, Iter, IterAdjacent, Maximum, NodeBalance, Remove,
};
use either::Either;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

pub fn cycle_canceling<N, W, C, G>(graph: &G) -> C
where
    N: Default + NodeBalance<Balance = C>,
    W: Default
        + EdgeCapacity<Capacity = C>
        + EdgeCost<Cost = C>
        + EdgeDirection
        + EdgeFlow<Flow = C>,
    C: Maximum
        + Default
        + PartialOrd
        + Copy
        + Neg<Output = C>
        + AddAssign
        + SubAssign
        + Add<C, Output = C>
        + Mul<C, Output = C>
        + Sub<C, Output = C>
        + Debug,
    G: Index
        + Get
        + GetMut
        + Insert
        + Remove
        + Count
        + IndexAdjacent
        + Index
        + Iter
        + IterAdjacent
        + Base<Node = N, Weight = W>
        + Clone
        + Debug,
{
    let mut mcf = Mcf::init(graph);
    assert!(mcf.solvable());
    let Mcf {
        mut residual_graph,
        source: _,
        sink: _,
    } = mcf;

    let mut total_flow = C::default();

    for start in graph.node_ids() {
        while let Either::Right(cycle) = bellman_ford_cycle(&residual_graph, start) {
            let mut bottleneck = C::MAX;

            for edge_id in cycle.edge_id_cycle() {
                let weight = residual_graph.weight(edge_id).unwrap();
                let residual_capacity = *weight.capacity() - *weight.flow();

                if residual_capacity < bottleneck {
                    bottleneck = residual_capacity;
                }
            }

            assert!(bottleneck >= C::default());
            if bottleneck == C::default() {
                break;
            }

            total_flow += bottleneck;

            for edge_id in cycle.edge_id_cycle() {
                let weight = residual_graph.weight_mut(edge_id).unwrap();
                *weight.flow_mut() += bottleneck;
                // *weight.capacity_mut() -= bottleneck;
                assert!(weight.flow() >= &C::default());

                let weight_rev = residual_graph.weight_mut(edge_id.rev()).unwrap();
                *weight_rev.flow_mut() -= bottleneck;
                // *weight_rev.capacity_mut() += bottleneck;
                assert!(weight_rev.flow() >= &C::default());
            }
        }
    }

    let cost = residual_graph
        .iter_edges()
        .fold(C::default(), |mut akku, edge| {
            let weight = edge.weight;
            if !weight.is_reverse() {
                akku += *weight.flow() * *weight.cost();
            }

            akku
        });

    cost
}

#[cfg(test)]
mod test {
    use super::cycle_canceling;
    use crate::{prelude::AdjacencyList, test::bgraph};

    #[test]
    fn cycle_canceling_kostenminimal_1() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal1.txt").unwrap();
        let cost = cycle_canceling(&graph);
        assert_eq!(cost, 3.0);
    }

    #[test]
    fn cycle_canceling_kostenminimal_2() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal2.txt").unwrap();
        let cost = cycle_canceling(&graph);
        assert_eq!(cost, 0.0);
    }

    #[test]
    #[should_panic]
    fn cycle_canceling_kostenminimal_3() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal3.txt").unwrap();
        let _cost = cycle_canceling(&graph);
    }

    #[test]
    #[should_panic]
    fn cycle_canceling_kostenminimal_4() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal4.txt").unwrap();
        let _cost = cycle_canceling(&graph);
    }

    #[test]
    fn cycle_canceling_kostenminimal_gross_1() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal_gross1.txt").unwrap();
        let cost = cycle_canceling(&graph);
        assert_eq!(cost, 1537.0);
    }

    #[test]
    fn cycle_canceling_kostenminimal_gross_2() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal_gross2.txt").unwrap();
        let cost = cycle_canceling(&graph);
        assert_eq!(cost, 1838.0);
    }

    #[test]
    #[should_panic]
    fn cycle_canceling_kostenminimal_gross_3() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal_gross3.txt").unwrap();
        let _cost = cycle_canceling(&graph);
    }
}
