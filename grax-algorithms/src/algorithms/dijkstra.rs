use crate::{
    distances::Distances, parents::Parents, path::ShortestPath, tree::ShortestPathTree,
    weight::TotalOrd,
};

use grax_core::collections::NodeCount;
use grax_core::edge::{weight::*, *};
use grax_core::graph::{EdgeIterAdjacent, NodeAttribute};
use grax_core::prelude::*;
use orx_priority_queue::{DaryHeap, PriorityQueue};
use std::fmt::Debug;
use std::ops::Add;

// #[derive(Clone, Copy)]
// pub struct Dijkstra;

// impl<C, G> ShortestPathFinder<C, G> for Dijkstra
// where
//     C: Default + Sortable + Copy + Add<C, Output = C> + Debug,
//     G: EdgeIterAdjacent + NodeAttribute + NodeCount,
//     G::EdgeWeight: Cost<C>,
// {
//     fn shortest_path(self, graph: &G, from: NodeId<G::Key>) -> ShortestPath<C, G> {
//         dijkstra(graph, from)
//     }

//     fn shortest_path_to(
//         self,
//         graph: &G,
//         from: NodeId<G::Key>,
//         to: NodeId<G::Key>,
//     ) -> ShortestPathTo<C, G> {
//         dijkstra_to(graph, from, to)
//     }
// }

pub fn dijkstra_to<C, G>(
    graph: &G,
    from: NodeId<G::Key>,
    to: NodeId<G::Key>,
) -> Option<ShortestPath<C, G>>
where
    C: Default + TotalOrd + Copy + Add<C, Output = C> + Debug + PartialOrd,
    G: EdgeIterAdjacent + NodeAttribute + NodeCount,
    G::EdgeWeight: Cost<C>,
{
    let mut priority_queue = DaryHeap::<_, _, 4>::with_capacity(graph.node_count());
    let mut distances = Distances::new(graph);
    let mut parents = Parents::new(graph);

    distances.update(from, C::default());
    priority_queue.push(from, C::default());

    while let Some((node, dist)) = priority_queue.pop() {
        if node == to {
            let distance = distances.distance(node).cloned().unwrap();
            return Some(ShortestPath {
                distance,
                from,
                to,
                distances,
                parents,
            });
        }

        if let Some(&prev) = distances.distance(node)
            && prev < dist
        {
            continue;
        }

        for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(node) {
            let next = dist + *weight.cost();
            let to = edge_id.to();

            if let Some(&prev) = distances.distance(to)
                && prev < next
            {
                continue;
            } else {
                parents.insert(node, to);
                distances.update(to, next);
                priority_queue.push(to, next);
            }
        }
    }

    None
}

pub fn dijkstra<C, G>(graph: &G, from: NodeId<G::Key>) -> ShortestPathTree<C, G>
where
    C: Default + TotalOrd + Copy + Add<C, Output = C> + Debug + PartialOrd,
    G: EdgeIterAdjacent + NodeAttribute + NodeCount,
    G::EdgeWeight: Cost<C>,
{
    let mut priority_queue = DaryHeap::<_, _, 4>::with_capacity(graph.node_count());
    let mut distances = Distances::new(graph);
    let mut parents = Parents::new(graph);

    distances.update(from, C::default());
    priority_queue.push(from, C::default());

    while let Some((node, dist)) = priority_queue.pop() {
        if let Some(&prev) = distances.distance(node)
            && prev < dist
        {
            continue;
        }

        for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(node) {
            let next = dist + *weight.cost();
            let to = edge_id.to();

            if let Some(&prev) = distances.distance(to)
                && prev < next
            {
                continue;
            } else {
                parents.insert(node, to);
                distances.update(to, next);
                priority_queue.push(to, next);
            }
        }
    }

    ShortestPathTree {
        from,
        distances,
        parents,
    }
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
            let total = dijkstra_to(&graph, id(0), id(1)).unwrap().distance;
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn dijkstra_g_1_2_undi_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_to(&graph, id(0), id(1)).unwrap().distance;
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn dijkstra_wege_1_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_to(&graph, id(2), id(0)).unwrap().distance;
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn dijkstra_wege_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_to(&graph, id(2), id(0)).unwrap().distance;
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    #[should_panic]
    fn dijkstra_wege_3_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege3.txt").unwrap();

        b.iter(|| {
            let total = dijkstra_to(&graph, id(2), id(0)).unwrap().distance;
            // cycle
            assert_eq!(total as f32, 2.0)
        })
    }
}
