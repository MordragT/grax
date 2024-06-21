use grax_core::{
    collections::{
        EdgeCollection, EdgeIter, GetEdge, InsertEdge, InsertNode, NodeCollection, NodeIter,
    },
    edge::EdgeRef,
    node::NodeRef,
};
use grax_flow::{BalancedNode, EdgeFlow, FlowBundle, NodeBalance};
use std::{
    fmt::Debug,
    ops::{Neg, Sub},
};

pub fn cycle_canceling<G, N, W, C>(graph: &mut G) -> C
where
    N: Default,
    W: Default + Copy + Debug,
    C: Default + Sub<C, Output = C> + Neg<Output = C> + Copy + PartialOrd + Debug,
    G: EdgeCollection<EdgeWeight = FlowBundle<W, C>>
        + NodeCollection<NodeWeight = BalancedNode<N, C>>
        + InsertEdge
        + InsertNode
        + EdgeIter
        + NodeIter
        + GetEdge,
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

    todo!()
}

// pub fn cycle_canceling<C, G>(graph: &G) -> C
// where
//     C: Maximum
//         + Default
//         + PartialOrd
//         + Copy
//         + Neg<Output = C>
//         + AddAssign
//         + SubAssign
//         + Add<C, Output = C>
//         + Mul<C, Output = C>
//         + Sub<C, Output = C>
//         + Debug,
//     G: Index
//         + Get
//         + GetMut
//         + Insert
//         + Remove
//         + Count
//         + IndexAdjacent
//         + Index
//         + Iter
//         + IterAdjacent
//         + Viewable
//         + Visitable
//         + Flow<C>
//         + Cost<C>
//         + Balance<C>
//         + Clone
//         + Debug,
// {
//     let mut mcf = Mcf::init(graph);
//     assert!(mcf.solvable());
//     let Mcf {
//         mut residual_graph,
//         source: _,
//         sink: _,
//     } = mcf;

//     let mut total_flow = C::default();

//     for start in graph.node_ids() {
//         while let Either::Right(cycle) = bellman_ford_cycle(&residual_graph, start) {
//             let mut bottleneck = C::MAX;

//             for edge_id in cycle.edge_id_cycle() {
//                 let weight = residual_graph.weight(edge_id).unwrap();
//                 let residual_capacity = *weight.capacity() - *weight.flow();

//                 if residual_capacity < bottleneck {
//                     bottleneck = residual_capacity;
//                 }
//             }

//             assert!(bottleneck >= C::default());
//             if bottleneck == C::default() {
//                 break;
//             }

//             total_flow += bottleneck;

//             for edge_id in cycle.edge_id_cycle() {
//                 let weight = residual_graph.weight_mut(edge_id).unwrap();
//                 *weight.flow_mut() += bottleneck;
//                 // *weight.capacity_mut() -= bottleneck;
//                 assert!(weight.flow() >= &C::default());

//                 let weight_rev = residual_graph.weight_mut(edge_id.rev()).unwrap();
//                 *weight_rev.flow_mut() -= bottleneck;
//                 // *weight_rev.capacity_mut() += bottleneck;
//                 assert!(weight_rev.flow() >= &C::default());
//             }
//         }
//     }

//     let cost = residual_graph
//         .iter_edges()
//         .fold(C::default(), |mut akku, edge| {
//             let weight = edge.weight;
//             if !weight.is_reverse() {
//                 akku += *weight.flow() * *weight.cost();
//             }

//             akku
//         });

//     cost
// }

#[cfg(test)]
mod test {
    use super::cycle_canceling;
    use crate::test::bgraph;
    use grax_impl::*;

    #[test]
    fn cycle_canceling_kostenminimal_1() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal1.txt").unwrap();
        let cost = cycle_canceling(&mut graph);
        assert_eq!(cost, 3.0);
    }

    #[test]
    fn cycle_canceling_kostenminimal_2() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal2.txt").unwrap();
        let cost = cycle_canceling(&mut graph);
        assert_eq!(cost, 0.0);
    }

    #[test]
    #[should_panic]
    fn cycle_canceling_kostenminimal_3() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal3.txt").unwrap();
        let _cost = cycle_canceling(&mut graph);
    }

    #[test]
    #[should_panic]
    fn cycle_canceling_kostenminimal_4() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal4.txt").unwrap();
        let _cost = cycle_canceling(&mut graph);
    }

    #[test]
    fn cycle_canceling_kostenminimal_gross_1() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross1.txt").unwrap();
        let cost = cycle_canceling(&mut graph);
        assert_eq!(cost, 1537.0);
    }

    #[test]
    fn cycle_canceling_kostenminimal_gross_2() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross2.txt").unwrap();
        let cost = cycle_canceling(&mut graph);
        assert_eq!(cost, 1838.0);
    }

    #[test]
    #[should_panic]
    fn cycle_canceling_kostenminimal_gross_3() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross3.txt").unwrap();
        let _cost = cycle_canceling(&mut graph);
    }
}
