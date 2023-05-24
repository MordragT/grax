use std::{
    collections::{HashMap, VecDeque},
    ops::{AddAssign, SubAssign},
};

use crate::{
    graph::{Contains, Count, Create, Get, IndexAdjacent, Insert, Iter},
    prelude::{Edge, EdgeIdentifier, NodeIdentifier},
};

pub fn edmonds_karp<N, W, G>(graph: &G, source: G::NodeId, sink: G::NodeId) -> W
where
    W: Default + PartialOrd + Copy + AddAssign + SubAssign,
    G: Create<N>
        + Contains<N>
        + Clone
        + Iter<N, W>
        + Get<N, W>
        + Insert<N, W>
        + Count
        + IndexAdjacent,
{
    let mut graph = graph.clone();
    let mut capacity = HashMap::new();

    // construct new residual graph and add capacities to edges
    let edges: Vec<Edge<G::EdgeId, W>> = graph
        .iter_edges()
        .map(|edge| edge.to_owned())
        .collect::<Vec<_>>();
    for Edge { edge_id, weight } in edges {
        if !graph.contains_edge_id(edge_id.rev()) {
            graph.insert_edge(edge_id.rev(), weight);
            capacity.insert(edge_id.rev(), W::default());
        }

        capacity.insert(edge_id, weight);
    }

    let mut total_flow = W::default();
    let mut parent = vec![None; graph.node_count()];

    // loop while bfs finds a path
    while bfs_augmenting_path(&graph, source, sink, &mut parent, &capacity) {
        let mut to = sink;
        let mut bottleneck = None;

        // compute the bottleneck
        while to != source {
            let from = parent[to.as_usize()].unwrap();
            let edge_id = G::EdgeId::between(from, to);
            let residual_capacity = capacity.get(&edge_id).unwrap();

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

            let cap = capacity.get_mut(&G::EdgeId::between(from, to)).unwrap();
            *cap -= bottleneck;

            assert!(*cap >= W::default());

            let cap_rev = capacity.get_mut(&G::EdgeId::between(to, from)).unwrap();
            *cap_rev += bottleneck;

            assert!(*cap_rev >= W::default());

            to = from;
        }
    }

    total_flow
}

fn bfs_augmenting_path<W, G>(
    graph: &G,
    source: G::NodeId,
    sink: G::NodeId,
    parent: &mut Vec<Option<G::NodeId>>,
    capacity: &HashMap<G::EdgeId, W>,
) -> bool
where
    W: Default + PartialOrd,
    G: Count + IndexAdjacent,
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
            let cap = capacity.get(&G::EdgeId::between(from, to)).unwrap();

            if !visited[to.as_usize()] && cap > &W::default() {
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

    use crate::{prelude::*, test::digraph};
    use test::Bencher;

    #[bench]
    fn edmonds_karp_g_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 5.0)
        })
    }

    #[bench]
    fn edmonds_karp_g_1_2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 5.0)
        })
    }
}
