use crate::util::Route;

use super::{dijkstra_to, nearest_neighbor};
use grax_core::collections::NodeCount;
use grax_core::collections::NodeIter;
use grax_core::collections::VisitNodeMap;
use grax_core::edge::*;
use grax_core::graph::EdgeIterAdjacent;
use grax_core::graph::NodeAttribute;
use grax_core::graph::NodeIterAdjacent;
use grax_core::prelude::*;
use grax_core::weight::Maximum;
use grax_core::weight::Sortable;

use std::fmt::Debug;
use std::ops::{Add, AddAssign};

pub fn branch_bound<C, G>(graph: &G) -> Option<(Route<G>, C)>
where
    C: Default + Copy + AddAssign + Add<C, Output = C> + Maximum + Sortable + Debug,
    G: NodeIterAdjacent + EdgeIterAdjacent + NodeAttribute + NodeIter + NodeCount,
    G::EdgeWeight: EdgeCost<Cost = C>,
    G::FixedNodeMap<bool>: Clone,
{
    match graph.node_ids().next() {
        Some(start) => Some(_branch_bound(graph, start)),
        None => None,
    }
}

pub fn branch_bound_rec<C, G>(graph: &G) -> Option<(Route<G>, C)>
where
    C: Default + Copy + Add<C, Output = C> + AddAssign + PartialOrd + Sortable + Maximum + Debug,
    G: NodeIterAdjacent + EdgeIterAdjacent + NodeAttribute + NodeIter + NodeCount,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    match graph.node_ids().next() {
        Some(start) => {
            let mut baseline = nearest_neighbor(graph, start)
                .map(|tour| tour.1)
                .unwrap_or(Maximum::MAX);
            let mut path = vec![start];
            let mut visited = graph.visit_node_map();
            let cost = C::default();

            _branch_bound_rec(
                start,
                graph,
                start,
                &mut path,
                &mut visited,
                cost,
                &mut baseline,
            );

            Some((Route::new(path), baseline))
        }
        None => None,
    }
}

pub(crate) fn _branch_bound<C, G>(graph: &G, start: NodeId<G::Key>) -> (Route<G>, C)
where
    C: Default + Copy + AddAssign + Add<C, Output = C> + Maximum + Sortable + Debug,
    G: NodeIterAdjacent + EdgeIterAdjacent + NodeAttribute + NodeIter + NodeCount,
    G::EdgeWeight: EdgeCost<Cost = C>,
    G::FixedNodeMap<bool>: Clone,
{
    let mut stack = Vec::new();
    let mut total_cost = nearest_neighbor(graph, start)
        .map(|tour| tour.1)
        .unwrap_or(Maximum::MAX);
    let mut route = Vec::new();

    let mut visited = graph.visit_node_map();
    visited.visit(start);

    stack.push((C::default(), vec![start], visited));

    while let Some((cost, path, visited)) = stack.pop() {
        let node = path
            .last()
            .expect("INTERNAL: Path always expected to have atleast one element");

        for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(*node) {
            let to = edge_id.to();
            let cost = cost + *weight.cost();

            if !visited.is_visited(to) && cost < total_cost {
                let mut visited = visited.clone();
                visited.visit(to);

                let mut path = path.clone();
                path.push(to);

                if visited.all_visited() {
                    if let Some(cost_to_start) =
                        dijkstra_to(graph, path[path.len() - 1], start).distance
                    {
                        let cost = cost + cost_to_start;

                        if cost < total_cost {
                            total_cost = cost;
                            std::mem::swap(&mut path, &mut route);
                        }
                    }
                } else {
                    stack.push((cost, path, visited));
                }
            }
        }
    }

    (Route::new(route), total_cost)
}

pub(crate) fn _branch_bound_rec<C, G>(
    start: NodeId<G::Key>,
    graph: &G,
    node: NodeId<G::Key>,
    path: &mut Vec<NodeId<G::Key>>,
    visited: &mut G::FixedNodeMap<bool>,
    cost: C,
    baseline: &mut C,
) where
    C: Default + Copy + Add<C, Output = C> + AddAssign + PartialOrd + Sortable + Debug,
    G: EdgeIterAdjacent + NodeAttribute + NodeCount,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    if visited.all_visited()
        && let Some(cost_to_start) = dijkstra_to(graph, node, start).distance
    {
        let total_cost = cost + cost_to_start;
        if total_cost < *baseline {
            *baseline = total_cost;
        }
    }

    for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(node) {
        let to = edge_id.to();
        let cost = cost + *weight.cost();

        if !visited.is_visited(to) && cost < *baseline {
            visited.visit(to);
            path.push(to);

            _branch_bound_rec(start, graph, to, path, visited, cost, baseline);

            visited.unvisit(to);
            path.pop();
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::{branch_bound, branch_bound_rec};
    use crate::test::undigraph;
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn branch_bound_k_10_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = branch_bound(&graph).unwrap().1 as f32;
            assert_eq!(total, 38.41);
        })
    }

    #[bench]
    fn branch_bound_k_10e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = branch_bound(&graph).unwrap().1 as f32;
            assert_eq!(total, 27.26);
        })
    }

    #[bench]
    fn branch_bound_k_12_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = branch_bound(&graph).unwrap().1 as f32;
            assert_eq!(total, 45.19);
        })
    }

    #[bench]
    fn branch_bound_k_12e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = branch_bound(&graph).unwrap().1 as f32;
            assert_eq!(total, 36.13);
        })
    }

    #[bench]
    fn branch_bound_rec_k_10_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = branch_bound_rec(&graph).unwrap().1 as f32;
            assert_eq!(total, 38.41);
        })
    }

    #[bench]
    fn branch_bound_rec_k_10e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = branch_bound_rec(&graph).unwrap().1 as f32;
            assert_eq!(total, 27.26);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn branch_bound_rec_k_12_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = branch_bound_rec(&graph).unwrap().1 as f32;
            assert_eq!(total, 45.19);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn branch_bound_rec_k_12e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = branch_bound_rec(&graph).unwrap().1 as f32;
            assert_eq!(total, 36.13);
        })
    }
}
