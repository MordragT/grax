use crate::{
    graph::{Count, EdgeCost, IndexAdjacent, IterAdjacent, Sortable},
    prelude::{EdgeIdentifier, NodeIdentifier},
};
use priq::PriorityQueue;
use std::ops::Add;

use super::Distances;

pub fn dijkstra_between<N, W, C, G>(graph: &G, from: G::NodeId, to: G::NodeId) -> Option<C>
where
    C: Default + Sortable + Copy + Add<C, Output = C>,
    W: EdgeCost<Cost = C>,
    G: IndexAdjacent + Count + IterAdjacent<N, W>,
{
    dijkstra(graph, from, to).distances[to.as_usize()]
}

pub fn dijkstra<N, W, C, G>(graph: &G, from: G::NodeId, to: G::NodeId) -> Distances<G::NodeId, C>
where
    C: Default + Sortable + Copy + Add<C, Output = C>,
    W: EdgeCost<Cost = C>,
    G: IndexAdjacent + Count + IterAdjacent<N, W>,
{
    let mut priority_queue = PriorityQueue::new();
    let mut distances = vec![None; graph.node_count()];

    distances[from.as_usize()] = Some(C::default());
    priority_queue.put(C::default(), from);

    while let Some((dist, node)) = priority_queue.pop() {
        if node == to {
            return Distances::new(from, distances);
        }

        if let Some(d) = distances[node.as_usize()] && dist > d {
            continue;
        }

        for edge in graph.iter_adjacent_edges(node) {
            let to = edge.edge_id.to();
            let next_dist = dist + *edge.weight.cost();

            let visited_or_geq = match &distances[to.as_usize()] {
                Some(d) => next_dist >= *d,
                None => false,
            };

            if !visited_or_geq {
                distances[to.as_usize()] = Some(next_dist);
                priority_queue.put(next_dist, to);
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
