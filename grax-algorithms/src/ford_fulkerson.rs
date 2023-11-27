use grax_core::edge::*;
use grax_core::prelude::*;
use grax_core::traits::*;
use grax_core::view::Parents;

use std::{
    fmt::Debug,
    ops::{AddAssign, Sub, SubAssign},
};

pub(crate) fn _ford_fulkerson<C, G, F>(
    graph: &mut G,
    source: NodeId<G::Id>,
    sink: NodeId<G::Id>,
    mut sp: F,
) -> C
where
    F: FnMut(&G, NodeId<G::Id>, NodeId<G::Id>) -> Option<Parents<G>>,
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C> + Debug,
    G: Count + IndexAdjacent + Get + GetMut + Base + Debug + Flow<C> + Viewable,
{
    let mut total_flow = C::default();

    // loop while bfs finds a path
    while let Some(parents) = sp(graph, source, sink) {
        if parents.is_empty() {
            break;
        }

        let mut to = sink;
        let mut bottleneck = None;

        // TODO compute route in parents instead of going over parents manually.

        // compute the bottleneck
        while to != source {
            let from = parents.parent(to).unwrap();
            let edge_id = EdgeId::new_unchecked(from, to);

            let weight = graph.flow(edge_id).unwrap();
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
            let from = parents.parent(to).unwrap();

            let weight = graph.flow_mut(EdgeId::new_unchecked(from, to)).unwrap();
            *weight.flow_mut() += bottleneck;

            let weight_rev = graph.flow_mut(EdgeId::new_unchecked(to, from)).unwrap();
            *weight_rev.flow_mut() -= bottleneck;

            to = from;
        }
    }

    total_flow
}
