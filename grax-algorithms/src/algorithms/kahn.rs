use std::collections::VecDeque;

use grax_core::{
    collections::{NodeCount, NodeIter},
    graph::{EdgeIterAdjacent, NodeAttribute},
    prelude::*,
};

use crate::cycle::CycleDetected;

use super::TopologicalSort;

#[derive(Debug, Clone, Copy)]
pub struct Kahn;

impl<G> TopologicalSort<G> for Kahn
where
    G: NodeAttribute + EdgeIterAdjacent + NodeIter + NodeCount,
{
    fn sort(graph: &G) -> Result<Vec<NodeId<G::Key>>, CycleDetected> {
        kahn(graph)
    }
}

pub fn kahn<G>(graph: &G) -> Result<Vec<NodeId<G::Key>>, CycleDetected>
where
    G: NodeAttribute + EdgeIterAdjacent + NodeIter + NodeCount,
{
    let mut in_degree = graph.fixed_node_map(0);

    for from in graph.node_ids() {
        for edge_id in graph.adjacent_edge_ids(from) {
            let to = edge_id.to();
            in_degree[to] += 1;
        }
    }

    let mut queue = in_degree
        .iter_nodes()
        .filter_map(|NodeRef { node_id, weight }| if *weight == 0 { Some(node_id) } else { None })
        .collect::<VecDeque<_>>();

    let mut sorted = Vec::new();

    while let Some(from) = queue.pop_front() {
        sorted.push(from);

        for edge_id in graph.adjacent_edge_ids(from) {
            let to = edge_id.to();

            in_degree[to] -= 1;

            if in_degree[to] == 0 {
                queue.push_back(to);
            }
        }
    }

    if sorted.len() != graph.node_count() {
        Err(CycleDetected)
    } else {
        Ok(sorted)
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::kahn;
    use crate::{cycle::CycleDetected, test::id};
    use grax_core::graph::Create;
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn kahn_empty(b: &mut Bencher) {
        let graph = AdjGraph::<(), (), true>::new();

        b.iter(|| {
            let sorted = kahn(&graph).unwrap();
            assert_eq!(sorted, Vec::new());
        });
    }

    #[bench]
    fn kahn_linear(b: &mut Bencher) {
        // 0 --> 1 --> 2
        let graph = AdjGraph::<(), (), true>::with_edges([(0, 1, ()), (1, 2, ())], 3);

        b.iter(|| {
            let sorted = kahn(&graph).unwrap();
            assert_eq!(sorted, vec![id(0), id(1), id(2)]);
        });
    }

    #[bench]
    fn kahn_branching(b: &mut Bencher) {
        // 0 --> 1
        // 0 --> 2
        let graph = AdjGraph::<(), (), true>::with_edges([(0, 1, ()), (0, 2, ())], 3);

        b.iter(|| {
            let sorted = kahn(&graph).unwrap();
            assert_eq!(sorted, vec![id(0), id(1), id(2)]); // 0 2 1 also correct
        });
    }

    #[bench]
    fn kahn_cycle(b: &mut Bencher) {
        let graph = AdjGraph::<(), (), true>::with_edges([(0, 1, ()), (1, 2, ()), (2, 0, ())], 3);

        b.iter(|| {
            let result = kahn(&graph);
            assert_eq!(result, Err(CycleDetected));
        });
    }
}
