use grax_core::edge::*;
use grax_core::prelude::*;
use grax_core::traits::*;
use grax_core::view::Distances;
use grax_core::weight::Sortable;

use priq::PriorityQueue;
use std::fmt::Debug;
use std::ops::Add;

pub fn dijkstra_between<C, G>(graph: &G, from: NodeId<G::Id>, to: NodeId<G::Id>) -> Option<C>
where
    C: Default + Sortable + Copy + Add<C, Output = C> + Debug,
    G: IterAdjacent + Viewable + Cost<C>,
{
    dijkstra(graph, from, to).and_then(|distances| distances.distance(to).cloned())
}

pub fn dijkstra<C, G>(graph: &G, from: NodeId<G::Id>, to: NodeId<G::Id>) -> Option<Distances<C, G>>
where
    C: Default + Sortable + Copy + Add<C, Output = C> + Debug,
    G: IterAdjacent + Viewable + Cost<C>,
{
    let mut priority_queue = PriorityQueue::new();
    let mut distances = graph.distances();

    distances.update_cost(from, C::default());
    priority_queue.put(C::default(), from);

    while let Some((dist, node)) = priority_queue.pop() {
        if node == to {
            return Some(distances);
        }

        if let Some(d) = distances.distance(node) && dist > *d {
            continue;
        }

        for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(node) {
            let to = edge_id.to();
            let next_dist = dist + *weight.cost();

            let visited_or_geq = match distances.distance(to) {
                Some(d) => next_dist >= *d,
                None => false,
            };

            if !visited_or_geq {
                distances.insert(from, to, next_dist);
                priority_queue.put(next_dist, to);
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::dijkstra_between;
    use crate::test::{digraph, id, undigraph};
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn dijkstra_g_1_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, id(0), id(1)).unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn dijkstra_g_1_2_undi_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, id(0), id(1)).unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn dijkstra_wege_1_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, id(2), id(0)).unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn dijkstra_wege_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, id(2), id(0)).unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    #[should_panic]
    fn dijkstra_wege_3_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege3.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, id(2), id(0)).unwrap();
            // cycle
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn dijkstra_g_1_2_di_sparse_mat(b: &mut Bencher) {
        let graph: SparseMatGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, id(0), id(1)).unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn dijkstra_g_1_2_undi_sparse_mat(b: &mut Bencher) {
        let graph: SparseMatGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, id(0), id(1)).unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn dijkstra_wege_1_di_sparse_mat(b: &mut Bencher) {
        let graph: SparseMatGraph<_, _, true> = digraph("../data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, id(2), id(0)).unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn dijkstra_wege_2_di_sparse_mat(b: &mut Bencher) {
        let graph: SparseMatGraph<_, _, true> = digraph("../data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, id(2), id(0)).unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    #[should_panic]
    fn dijkstra_wege_3_di_sparse_mat(b: &mut Bencher) {
        let graph: SparseMatGraph<_, _, true> = digraph("../data/Wege3.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_between(&graph, id(2), id(0)).unwrap();
            // cycle
            assert_eq!(total as f32, 2.0)
        })
    }
}
