use grax_core::collections::GetEdgeMut;
use grax_core::edge::*;
use grax_core::graph::Flow;
use grax_core::prelude::*;
use grax_core::view::Parents;
use grax_core::{collections::GetEdge, graph::NodeAttribute};

use std::{
    fmt::Debug,
    ops::{AddAssign, Sub, SubAssign},
};

pub(crate) fn _ford_fulkerson<C, G, F>(
    graph: &mut G,
    source: NodeId<G::Key>,
    sink: NodeId<G::Key>,
    mut sp: F,
) -> C
where
    F: FnMut(&G, NodeId<G::Key>, NodeId<G::Key>) -> Option<Parents<G>>,
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C> + Debug,
    G: GetEdge + GetEdgeMut + Debug + Flow<C> + NodeAttribute,
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

            let weight = graph.edge(edge_id).unwrap().weight;
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

            let weight = graph
                .edge_mut(EdgeId::new_unchecked(from, to))
                .unwrap()
                .weight;
            *weight.flow_mut() += bottleneck;

            let weight_rev = graph
                .edge_mut(EdgeId::new_unchecked(to, from))
                .unwrap()
                .weight;
            *weight_rev.flow_mut() -= bottleneck;

            to = from;
        }
    }

    total_flow
}
