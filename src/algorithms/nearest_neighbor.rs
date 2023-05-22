use super::{dijkstra_between, Tour};
use crate::{
    edge::EdgeRef,
    prelude::{GraphAdjacentTopology, GraphTopology, Maximum, NodeIndex, Sortable},
};
use std::ops::{Add, AddAssign};

pub fn nearest_neighbor_from_first<N, W, G>(graph: &G) -> Option<Tour<W>>
where
    W: Default + Copy + AddAssign + Add<W, Output = W> + Maximum + Sortable,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    match graph.indices().next() {
        Some(start) => nearest_neighbor(graph, start),
        None => None,
    }
}

pub fn nearest_neighbor<N, W, G>(graph: &G, start: NodeIndex) -> Option<Tour<W>>
where
    W: Default + Copy + AddAssign + Add<W, Output = W> + Maximum + Sortable,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
    enum Status {
        Visited,
        #[default]
        Unvisited,
        Diverged,
    }

    let mut states = vec![Status::default(); graph.node_count()];
    let mut path = vec![(start, W::default())];
    let mut prev = start;

    states[start.0] = Status::Visited;

    while let Some((node, _)) = path.last() && path.len() < graph.node_count() {

        let mut min_node = None;
        let mut min_weight = W::max();

        for EdgeRef { from:_, to, weight } in graph.adjacent_edges(*node) {
            if states[to.0] == Status::Unvisited && to != prev {
                if min_weight > *weight {
                    min_node = Some(to);
                    min_weight = *weight;
                }
            }
        }

        match min_node {
            Some(next) => {
                path.push((next, min_weight));
                states[next.0] = Status::Visited;
                prev = next;
            }
            None => {
                let open_end = path.iter().rposition(|(node, _)| {
                    graph.adjacent_indices(*node).any(|neigh| states[neigh.0] == Status::Unvisited)
                });

                if let Some(index) = open_end {
                    let branch_point = path[index].0;

                    if states[branch_point.0] == Status::Diverged {
                        return None;
                    } else {
                        states[branch_point.0] = Status::Diverged;
                    }
                    let splitted_off = path.split_off(index + 1);
                    for (node, _) in splitted_off.into_iter().rev() {
                        states[node.0] = Status::Unvisited;
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
    let weight = weight.into_iter().fold(W::default(), |mut accu, w| {
        accu += w;
        accu
    });

    Some(Tour::new(route, weight))
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
            let total = graph.nearest_neighbor_from_first().unwrap().weight;
            assert_le!(total, 38.41 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_10e_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().weight;
            assert_le!(total, 27.26 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_12_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/K_12.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().weight;
            assert_le!(total, 45.19 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_12e_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().weight;
            assert_le!(total, 36.13 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_10_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/K_10.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().weight;
            assert_le!(total, 38.41 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_10e_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().weight;
            assert_le!(total, 27.26 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_12_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/K_12.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().weight;
            assert_le!(total, 45.19 * 1.2);
        })
    }

    #[bench]
    fn nearest_neighbor_k_12e_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = graph.nearest_neighbor_from_first().unwrap().weight;
            assert_le!(total, 36.13 * 1.2);
        })
    }
}
