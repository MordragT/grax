use crate::{
    graph::{Base, Count, EdgeCapacity, EdgeFlow, Get, GetMut, IndexAdjacent},
    prelude::{EdgeId, NodeId},
    structures::Parents,
};
use std::ops::{AddAssign, Sub, SubAssign};

pub(crate) fn _ford_fulkerson<N, W, C, G, F>(
    graph: &mut G,
    source: NodeId<G::Id>,
    sink: NodeId<G::Id>,
    mut sp: F,
) -> C
where
    F: FnMut(&G, NodeId<G::Id>, NodeId<G::Id>) -> Option<Parents<G>>,
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C>,
    W: EdgeCapacity<Capacity = C> + EdgeFlow<Flow = C>,
    G: Count + IndexAdjacent + Get + GetMut + Base<Node = N, Weight = W>,
{
    let mut total_flow = C::default();

    // loop while bfs finds a path
    while let Some(parents) = sp(graph, source, sink) {
        let mut to = sink;
        let mut bottleneck = None;

        // TODO compute route in parents instead of going over parents manually.

        // compute the bottleneck
        while to != source {
            let from = unsafe { parents.parent_unchecked(to) };
            let edge_id = EdgeId::new_unchecked(from, to);

            let weight = graph.weight(edge_id).unwrap();
            let residual_capacity = *weight.capacity() - *weight.flow();

            bottleneck = match bottleneck {
                Some(b) => {
                    if b > residual_capacity {
                        Some(residual_capacity)
                    } else {
                        Some(b)
                    }
                }
                None => Some(residual_capacity),
            };

            to = from;
        }

        let bottleneck = bottleneck.unwrap();
        total_flow += bottleneck;
        to = sink;

        // assign the bottleneck to every edge in the path
        while to != source {
            let from = unsafe { parents.parent_unchecked(to) };

            let weight = graph.weight_mut(EdgeId::new_unchecked(from, to)).unwrap();
            *weight.flow_mut() += bottleneck;

            let weight_rev = graph.weight_mut(EdgeId::new_unchecked(to, from)).unwrap();
            *weight_rev.flow_mut() -= bottleneck;

            to = from;
        }
    }

    total_flow
}
