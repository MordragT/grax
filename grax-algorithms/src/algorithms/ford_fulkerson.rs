use crate::problems::PathFinder;
use crate::weight::TotalOrd;

use grax_core::collections::GetEdgeMut;
use grax_core::edge::{weight::*, *};
use grax_core::prelude::*;
use grax_core::{collections::GetEdge, graph::NodeAttribute};

use std::{
    fmt::Debug,
    ops::{AddAssign, Sub, SubAssign},
};

pub fn ford_fulkerson<C, G>(
    graph: &mut G,
    source: NodeId<G::Key>,
    sink: NodeId<G::Key>,
    path_finder: impl PathFinder<G>,
) -> C
where
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C> + Debug + TotalOrd,
    G: GetEdge + GetEdgeMut + Debug + NodeAttribute,
    G::EdgeWeight: Flow<C> + Capacity<C>,
{
    let mut total_flow = C::default();

    let filter = |EdgeRef { edge_id: _, weight }: EdgeRef<G::Key, G::EdgeWeight>| {
        (*weight.capacity() - *weight.flow()) > C::default()
    };

    // loop while path_finder finds a path
    while let Some(path) = path_finder.path_where(graph, source, sink, filter) {
        let parents = path.parents;

        if parents.is_empty() {
            break;
        }

        let bottleneck = parents
            .iter_edges_to(source, sink)
            .map(|edge_id| {
                let weight = graph.edge(edge_id).unwrap().weight;
                *weight.capacity() - *weight.flow()
            })
            .min_by(TotalOrd::total_ord)
            .unwrap();

        total_flow += bottleneck;

        for edge_id in parents.iter_edges_to(source, sink) {
            let weight = graph.edge_mut(edge_id).unwrap().weight;
            *weight.flow_mut() += bottleneck;

            let weight_rev = graph.edge_mut(edge_id.rev()).unwrap().weight;
            *weight_rev.flow_mut() -= bottleneck;
        }
    }

    total_flow
}
