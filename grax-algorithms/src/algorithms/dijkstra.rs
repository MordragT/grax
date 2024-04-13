use crate::category::ShortestPath;
use crate::utility::Distances;
use grax_core::collections::NodeCount;
use grax_core::edge::*;
use grax_core::graph::{EdgeIterAdjacent, NodeAttribute};
use grax_core::prelude::*;
use grax_core::weight::Sortable;

use orx_priority_queue::{DaryHeap, PriorityQueue};

use std::fmt::Debug;
use std::ops::Add;

pub struct Dijkstra;

impl<C, G> ShortestPath<C, G> for Dijkstra
where
    C: Default + Sortable + Copy + Add<C, Output = C> + Debug,
    G: EdgeIterAdjacent + NodeAttribute + NodeCount,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    fn shortest_path(graph: &G, from: NodeId<G::Key>) -> Distances<C, G> {
        dijkstra(graph, from)
    }

    fn shortest_path_to(
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
    ) -> (Option<C>, Distances<C, G>) {
        dijkstra_to(graph, from, to)
    }
}

pub fn dijkstra_to<C, G>(
    graph: &G,
    from: NodeId<G::Key>,
    to: NodeId<G::Key>,
) -> (Option<C>, Distances<C, G>)
where
    C: Default + Sortable + Copy + Add<C, Output = C> + Debug,
    G: EdgeIterAdjacent + NodeAttribute + NodeCount,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    let mut priority_queue = DaryHeap::<_, _, 4>::with_capacity(graph.node_count());
    let mut distances = Distances::new(graph);

    distances.update(from, C::default());
    priority_queue.push(from, C::default());

    while let Some((node, cost)) = priority_queue.pop() {
        if node == to {
            return (distances.distance(node).cloned(), distances);
        }

        if let Some(d) = distances.distance(node)
            && cost > *d
        {
            continue;
        }

        for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(node) {
            let to = edge_id.to();

            let cost = cost + *weight.cost();

            if distances.replace_min(from, to, cost) {
                priority_queue.push(to, cost);
            }
        }
    }

    (distances.distance(to).cloned(), distances)
}

pub fn dijkstra<C, G>(graph: &G, from: NodeId<G::Key>) -> Distances<C, G>
where
    C: Default + Sortable + Copy + Add<C, Output = C> + Debug,
    G: EdgeIterAdjacent + NodeAttribute + NodeCount,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    let mut priority_queue = DaryHeap::<_, _, 4>::with_capacity(graph.node_count());
    let mut distances = Distances::new(graph);

    distances.update(from, C::default());
    priority_queue.push(from, C::default());

    while let Some((node, cost)) = priority_queue.pop() {
        if let Some(d) = distances.distance(node)
            && cost > *d
        {
            continue;
        }

        for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(node) {
            let to = edge_id.to();

            let cost = cost + *weight.cost();

            if distances.replace_min(from, to, cost) {
                priority_queue.push(to, cost);
            }
        }
    }

    distances
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::dijkstra_to;
    use crate::test::{digraph, id, undigraph};
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn dijkstra_g_1_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_to(&graph, id(0), id(1)).0.unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn dijkstra_g_1_2_undi_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_to(&graph, id(0), id(1)).0.unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn dijkstra_wege_1_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_to(&graph, id(2), id(0)).0.unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn dijkstra_wege_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_to(&graph, id(2), id(0)).0.unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    #[should_panic]
    fn dijkstra_wege_3_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege3.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_to(&graph, id(2), id(0)).0.unwrap();
            // cycle
            assert_eq!(total as f32, 2.0)
        })
    }
}
