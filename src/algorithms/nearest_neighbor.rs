use super::dijkstra_between;
use crate::{
    graph::{Count, EdgeCost, Index, IndexAdjacent, IterAdjacent, Maximum, Sortable},
    prelude::{EdgeRef, NodeId},
    structures::Route,
};
use std::ops::{Add, AddAssign};

pub fn nearest_neighbor_from_first<N, W, C, G>(graph: &G) -> Option<(Route<G>, C)>
where
    C: Default + Copy + AddAssign + Add<C, Output = C> + Maximum + Sortable,
    W: EdgeCost<Cost = C>,
    G: Count + Index + IndexAdjacent + IterAdjacent<N, W>,
{
    match graph.node_ids().next() {
        Some(start) => nearest_neighbor(graph, start),
        None => None,
    }
}

pub fn nearest_neighbor<N, W, C, G>(graph: &G, start: NodeId<G::Id>) -> Option<(Route<G>, C)>
where
    C: Default + Copy + AddAssign + Add<C, Output = C> + Maximum + Sortable,
    W: EdgeCost<Cost = C>,
    G: Count + IndexAdjacent + IterAdjacent<N, W>,
{
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
    enum Status {
        Visited,
        #[default]
        Unvisited,
        Diverged,
    }

    let mut states = vec![Status::default(); graph.node_count()];
    let mut path = vec![(start, C::default())];
    let mut prev = start;

    states[start.as_usize()] = Status::Visited;

    while let Some((node, _)) = path.last() && path.len() < graph.node_count() {

        let mut min_node = None;
        let mut min_cost = C::max();

        for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(*node) {
            let to = edge_id.to();
            if states[to.as_usize()] == Status::Unvisited && to != prev {
                if min_cost > *weight.cost() {
                    min_node = Some(to);
                    min_cost = *weight.cost();
                }
            }
        }

        match min_node {
            Some(next) => {
                path.push((next, min_cost));
                states[next.as_usize()] = Status::Visited;
                prev = next;
            }
            None => {
                let open_end = path.iter().rposition(|(node, _)| {
                    graph.adjacent_node_ids(*node).any(|neigh| states[neigh.as_usize()] == Status::Unvisited)
                });

                if let Some(index) = open_end {
                    let branch_point = path[index].0;

                    if states[branch_point.as_usize()] == Status::Diverged {
                        return None;
                    } else {
                        states[branch_point.as_usize()] = Status::Diverged;
                    }
                    let splitted_off = path.split_off(index + 1);
                    for (node, _) in splitted_off.into_iter().rev() {
                        states[node.as_usize()] = Status::Unvisited;
                        prev = node;
                    }
                } else {
                    return None;
                }
            }
        }
    }

    assert!(states
        .into_iter()
        .all(|visit| visit == Status::Visited || visit == Status::Diverged));

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
    use crate::{prelude::*, test::undigraph};
    use more_asserts::*;
    use test::Bencher;

    #[bench]
    fn nearest_neighbor_k_10_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/K_10.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().1;
            assert_le!(total, 38.41 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_10e_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().1;
            assert_le!(total, 27.26 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_12_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/K_12.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().1;
            assert_le!(total, 45.19 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_12e_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().1;
            assert_le!(total, 36.13 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_10_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/K_10.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().1;
            assert_le!(total, 38.41 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_10e_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().1;
            assert_le!(total, 27.26 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_12_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/K_12.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().1;
            assert_le!(total, 45.19 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_12e_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().1;
            assert_le!(total, 36.13 * 1.2);
        })
    }
}
