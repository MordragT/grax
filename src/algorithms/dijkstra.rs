use crate::graph::{GraphAdjacentTopology, GraphTopology, Sortable};
use crate::indices::NodeIndex;
use priq::PriorityQueue;
use std::ops::Add;

use super::Distances;

pub fn dijkstra_between<N, W, G>(graph: &G, from: NodeIndex, to: NodeIndex) -> Option<W>
where
    W: Default + Sortable + Copy + Add<W, Output = W>,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    dijkstra(graph, from, to).distances[to.0]
}

pub fn dijkstra<N, W, G>(graph: &G, from: NodeIndex, to: NodeIndex) -> Distances<W>
where
    W: Default + Sortable + Copy + Add<W, Output = W>,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut priority_queue = PriorityQueue::new();
    let mut distances = vec![None; graph.node_count()];

    distances[from.0] = Some(W::default());
    priority_queue.put(W::default(), from);

    while let Some((dist, node)) = priority_queue.pop() {
        if node == to {
            return Distances::new(from, distances);
        }

        if let Some(d) = distances[node.0] && dist > d {
            continue;
        }

        for edge in graph.adjacent_edges(node) {
            let next_dist = dist + *edge.weight;

            let visited_or_geq = match &distances[edge.to.0] {
                Some(d) => next_dist >= *d,
                None => false,
            };

            if !visited_or_geq {
                distances[edge.to.0] = Some(next_dist);
                priority_queue.put(next_dist, edge.to);
            }
        }
    }

    Distances::new(from, distances)
}

#[cfg(test)]
mod test {
    extern crate test;

    use crate::{
        algorithms::dijkstra_between,
        prelude::*,
        test::{digraph, undigraph},
    };
    use test::Bencher;

    #[bench]
    fn dijkstra_g_1_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, NodeIndex(0), NodeIndex(1)).unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn dijkstra_g_1_2_undi_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, NodeIndex(0), NodeIndex(1)).unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn dijkstra_wege_1_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, NodeIndex(2), NodeIndex(0)).unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn dijkstra_wege_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, NodeIndex(2), NodeIndex(0)).unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    #[should_panic]
    fn dijkstra_wege_3_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Wege3.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, NodeIndex(2), NodeIndex(0)).unwrap();
            // cycle
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn dijkstra_g_1_2_di_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, NodeIndex(0), NodeIndex(1)).unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn dijkstra_g_1_2_undi_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, NodeIndex(0), NodeIndex(1)).unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn dijkstra_wege_1_di_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, NodeIndex(2), NodeIndex(0)).unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn dijkstra_wege_2_di_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, NodeIndex(2), NodeIndex(0)).unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    #[should_panic]
    fn dijkstra_wege_3_di_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Wege3.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, NodeIndex(2), NodeIndex(0)).unwrap();
            // cycle
            assert_eq!(total as f32, 2.0)
        })
    }
}
