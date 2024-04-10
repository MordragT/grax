use crate::utility::Route;

use super::dijkstra_between;
use grax_core::collections::FixedNodeMap;
use grax_core::collections::GetNodeMut;
use grax_core::collections::NodeCount;
use grax_core::collections::NodeIter;
use grax_core::edge::*;
use grax_core::graph::EdgeIterAdjacent;
use grax_core::graph::NodeAttribute;
use grax_core::graph::NodeIterAdjacent;
use grax_core::prelude::*;
use grax_core::weight::Maximum;
use grax_core::weight::Sortable;

use std::fmt::Debug;
use std::ops::{Add, AddAssign};

pub fn nearest_neighbor_from_first<C, G>(graph: &G) -> Option<(Route<G>, C)>
where
    C: Default + Copy + AddAssign + Add<C, Output = C> + Maximum + Sortable + Debug,
    G: NodeIter + NodeCount + EdgeIterAdjacent + NodeIterAdjacent + NodeAttribute,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    match graph.node_ids().next() {
        Some(start) => nearest_neighbor(graph, start),
        None => None,
    }
}

pub fn nearest_neighbor<C, G>(graph: &G, start: NodeId<G::Key>) -> Option<(Route<G>, C)>
where
    C: Default + Copy + AddAssign + Add<C, Output = C> + Maximum + Sortable + Debug,
    G: NodeCount + EdgeIterAdjacent + NodeIterAdjacent + NodeAttribute,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
    enum Status {
        Visited,
        #[default]
        Unvisited,
        Diverged,
    }

    let mut states = graph.fixed_node_map(Status::default());

    // let mut states = vec![Status::default(); graph.node_count()];
    let mut path = vec![(start, C::default())];
    let mut prev = start;

    states.update_node(start, Status::Visited);
    // states[start.raw()] = Status::Visited;

    while let Some((node, _)) = path.last()
        && path.len() < graph.node_count()
    {
        let mut min_node = None;
        let mut min_cost = C::MAX;

        for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(*node) {
            let cost = *weight.cost();
            let to = edge_id.to();
            if *states.get(to) == Status::Unvisited && to != prev {
                if min_cost > cost {
                    min_node = Some(to);
                    min_cost = cost;
                }
            }
        }

        match min_node {
            Some(next) => {
                path.push((next, min_cost));
                *states.get_mut(next) = Status::Visited;
                prev = next;
            }
            None => {
                let open_end = path.iter().rposition(|(node, _)| {
                    graph
                        .adjacent_node_ids(*node)
                        .any(|neigh| *states.get(neigh) == Status::Unvisited)
                });

                if let Some(index) = open_end {
                    let branch_point = path[index].0;

                    if *states.get(branch_point) == Status::Diverged {
                        return None;
                    } else {
                        *states.get_mut(branch_point) = Status::Diverged;
                    }
                    let splitted_off = path.split_off(index + 1);
                    for (node, _) in splitted_off.into_iter().rev() {
                        *states.get_mut(node) = Status::Unvisited;
                        prev = node;
                    }
                } else {
                    return None;
                }
            }
        }
    }

    assert!(states.iter_nodes().all(
        |NodeRef { node_id, weight }| *weight == Status::Visited || *weight == Status::Diverged
    ));

    match dijkstra_between(graph, prev, start) {
        Some(weight) => path.push((start, weight)),
        None => return None,
    }

    let (route, weight): (_, Vec<_>) = path.into_iter().unzip();
    let weight = weight.into_iter().fold(C::default(), |mut accu, w| {
        accu += w;
        accu
    });

    Some((Route::new(route), weight))
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::nearest_neighbor_from_first;
    use crate::test::undigraph;
    use grax_impl::*;
    use more_asserts::*;
    use test::Bencher;

    #[bench]
    fn nearest_neighbor_k_10_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = nearest_neighbor_from_first(&graph).unwrap().1;
            assert_le!(total, 38.41 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_10e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = nearest_neighbor_from_first(&graph).unwrap().1;
            assert_le!(total, 27.26 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_12_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = nearest_neighbor_from_first(&graph).unwrap().1;
            assert_le!(total, 45.19 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_12e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = nearest_neighbor_from_first(&graph).unwrap().1;
            assert_le!(total, 36.13 * 1.2);
        })
    }
}
