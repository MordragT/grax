use std::{
    collections::VecDeque,
    ops::{AddAssign, Neg, SubAssign},
};

use crate::{
    graph::{
        Base, Count, Create, FlowWeight, Get, GetMut, IndexAdjacent, Insert, Iter, WeightCapacity,
        WeightCost,
    },
    prelude::{AdjacencyList, EdgeId, EdgeRef, NodeId},
};

pub fn edmonds_karp<N, W, C, G>(graph: &G, source: NodeId<G::Id>, sink: NodeId<G::Id>) -> C
where
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Neg<Output = C>,
    W: WeightCost<Cost = C>,
    G: Iter<N, W> + Base<Id = usize> + Get<N, W>,
{
    let mut residual_graph = AdjacencyList::<_, _, true>::with_nodes(graph.iter_nodes());
    for EdgeRef { edge_id, weight } in graph.iter_edges() {
        let capacity = FlowWeight::new(*weight.cost(), *weight.cost());
        residual_graph.insert_edge(edge_id.from(), edge_id.to(), capacity);

        if !graph.contains_edge_id(edge_id.rev()) {
            residual_graph.insert_edge(
                edge_id.to(),
                edge_id.from(),
                FlowWeight::new(C::default(), -*weight.cost()),
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
    C: Default + PartialOrd + Copy + AddAssign + SubAssign,
    W: WeightCapacity<Capacity = C>,
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
            let residual_capacity = graph.weight(edge_id).unwrap().capacity();

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

        let bottleneck = *bottleneck.unwrap();
        total_flow += bottleneck;
        to = sink;

        // assign the bottleneck to every edge in the path
        while to != source {
            let from = parent[to.as_usize()].unwrap();

            let weight = graph.weight_mut(EdgeId::new_unchecked(from, to)).unwrap();
            *weight.capacity_mut() -= bottleneck;

            let weight_rev = graph.weight_mut(EdgeId::new_unchecked(to, from)).unwrap();
            *weight_rev.capacity_mut() += bottleneck;

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
    C: Default + PartialOrd,
    W: WeightCapacity<Capacity = C>,
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
            let cap = graph
                .weight(EdgeId::new_unchecked(from, to))
                .unwrap()
                .capacity();

            if !visited[to.as_usize()] && cap > &C::default() {
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
