use std::{
    collections::VecDeque,
    ops::{AddAssign, Neg, Sub, SubAssign},
};

use crate::{
    graph::{
        Base, Count, Create, EdgeCapacity, EdgeCost, EdgeFlow, FlowWeight, Get, GetMut,
        IndexAdjacent, Insert, Iter,
    },
    prelude::{AdjacencyList, EdgeId, EdgeRef, NodeId},
};

// TODO fix edmonds karp to not use adj list but graph representation itself

pub fn edmonds_karp<N, W, C, G>(graph: &G, source: NodeId<G::Id>, sink: NodeId<G::Id>) -> C
where
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Neg<Output = C> + Sub<C, Output = C>,
    W: EdgeCost<Cost = C>,
    G: Iter<N, W> + Base<Id = usize> + Get<N, W>,
{
    let mut residual_graph = AdjacencyList::<_, _, true>::with_nodes(graph.iter_nodes());
    for EdgeRef { edge_id, weight } in graph.iter_edges() {
        let cost = *weight.cost();

        let capacity = FlowWeight::new(cost, cost, C::default());
        residual_graph.insert_edge(edge_id.from(), edge_id.to(), capacity);

        if !graph.contains_edge_id(edge_id.rev()) {
            residual_graph.insert_edge(
                edge_id.to(),
                edge_id.from(),
                FlowWeight::new(cost, -cost, cost),
            );
        }
    }

    _edmonds_karp(&mut residual_graph, source, sink)
}

pub(crate) fn _edmonds_karp<N, W, C, G>(
    graph: &mut G,
    source: NodeId<G::Id>,
    sink: NodeId<G::Id>,
) -> C
where
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C>,
    W: EdgeCapacity<Capacity = C> + EdgeFlow<Flow = C>,
    G: Count + IndexAdjacent + Get<N, W> + GetMut<N, W>,
{
    let mut total_flow = C::default();
    let mut parent = vec![None; graph.node_count()];

    // loop while bfs finds a path
    while bfs_augmenting_path(graph, source, sink, &mut parent) {
        let mut to = sink;
        let mut bottleneck = None;

        // compute the bottleneck
        while to != source {
            let from = parent[to.as_usize()].unwrap();
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
            let from = parent[to.as_usize()].unwrap();

            let weight = graph.weight_mut(EdgeId::new_unchecked(from, to)).unwrap();
            *weight.flow_mut() += bottleneck;

            let weight_rev = graph.weight_mut(EdgeId::new_unchecked(to, from)).unwrap();
            *weight_rev.flow_mut() -= bottleneck;

            to = from;
        }
    }

    total_flow
}

// TODO use bfs just put nodes with cap < 0 in visited vec
fn bfs_augmenting_path<N, W, C, G>(
    graph: &G,
    source: NodeId<G::Id>,
    sink: NodeId<G::Id>,
    parent: &mut Vec<Option<NodeId<G::Id>>>,
) -> bool
where
    C: Default + PartialOrd + Sub<C, Output = C> + Copy,
    W: EdgeCapacity<Capacity = C> + EdgeFlow<Flow = C>,
    G: Count + IndexAdjacent + Get<N, W>,
{
    let mut queue = VecDeque::new();
    let mut visited = vec![false; graph.node_count()];

    queue.push_front(source);
    visited[source.as_usize()] = true;

    while let Some(from) = queue.pop_front() {
        if from == sink {
            return true;
        }

        for to in graph.adjacent_node_ids(from) {
            let weight = graph.weight(EdgeId::new_unchecked(from, to)).unwrap();

            if !visited[to.as_usize()] && (*weight.capacity() - *weight.flow()) > C::default() {
                parent[to.as_usize()] = Some(from);
                queue.push_back(to);
                visited[to.as_usize()] = true;
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::edmonds_karp;
    use crate::{
        prelude::*,
        test::{digraph, id},
    };
    use test::Bencher;

    #[bench]
    fn edmonds_karp_g_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }

    #[bench]
    fn edmonds_karp_g_1_2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }
}
