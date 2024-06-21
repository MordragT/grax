use crate::util::{Cycle, Distances, Parents, ShortestPathFinder};
use crate::util::{ShortestPath, ShortestPathTo};
use grax_core::collections::*;
use grax_core::edge::*;
use grax_core::graph::*;
use grax_core::prelude::*;

use either::Either;

use std::fmt::Debug;
use std::ops::{Add, Sub};

#[derive(Clone, Copy)]
pub struct BellmanFord;

impl<C, G> ShortestPathFinder<C, G> for BellmanFord
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + NodeCount + NodeIter,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    fn shortest_path_were<F>(
        self,
        graph: &G,
        from: NodeId<<G as Keyed>::Key>,
        filter: F,
    ) -> ShortestPath<C, G>
    where
        F: Fn(EdgeRef<<G as Keyed>::Key, <G as EdgeCollection>::EdgeWeight>) -> bool,
    {
        bellman_ford(graph, from, filter)
    }

    fn shortest_path_to_where<F>(
        self,
        graph: &G,
        from: NodeId<<G as Keyed>::Key>,
        to: NodeId<<G as Keyed>::Key>,
        filter: F,
    ) -> ShortestPathTo<C, G>
    where
        F: Fn(EdgeRef<<G as Keyed>::Key, <G as EdgeCollection>::EdgeWeight>) -> bool,
    {
        bellman_ford_to(graph, from, to, filter)
    }
}

pub fn bellman_ford_to<C, F, G>(
    graph: &G,
    from: NodeId<G::Key>,
    to: NodeId<G::Key>,
    filter: F,
) -> ShortestPathTo<C, G>
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    F: Fn(EdgeRef<<G as Keyed>::Key, <G as EdgeCollection>::EdgeWeight>) -> bool,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + NodeCount + NodeIter,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    let ShortestPath { distances, parents } = bellman_ford(graph, from, filter);
    let distance = distances.distance(to).copied();

    ShortestPathTo {
        distance,
        distances,
        parents,
    }
}

pub fn bellman_ford<C, F, G>(graph: &G, start: NodeId<G::Key>, filter: F) -> ShortestPath<C, G>
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    F: Fn(EdgeRef<<G as Keyed>::Key, <G as EdgeCollection>::EdgeWeight>) -> bool,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + NodeCount + NodeIter,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    let mut parents = Parents::new(graph);

    let mut distances = Distances::new(graph);
    distances.update(start, C::default());

    for _ in 1..graph.node_count() {
        if !relax(graph, &mut distances, &mut parents, &filter) {
            break;
        }
    }

    ShortestPath { distances, parents }
}

pub fn bellman_ford_cycle<C, F, G>(
    graph: &G,
    start: NodeId<G::Key>,
    filter: F,
) -> Either<ShortestPath<C, G>, Cycle<G>>
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    F: Fn(EdgeRef<<G as Keyed>::Key, <G as EdgeCollection>::EdgeWeight>) -> bool,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + NodeCount + NodeIter,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    let mut parents = Parents::new(graph);

    let mut distances = Distances::new(graph);
    distances.update(start, C::default());

    let mut updated = false;

    for _ in 0..graph.node_count() {
        updated = relax(graph, &mut distances, &mut parents, &filter);
        if !updated {
            break;
        }
    }

    if updated {
        let member = detect_cycle_member(graph, &distances, &filter);

        Either::Right(Cycle { parents, member })
    } else {
        Either::Left(ShortestPath { distances, parents })
    }
}

fn detect_cycle_member<C, F, G>(
    graph: &G,
    distances: &Distances<C, G>,
    filter: &F,
) -> NodeId<G::Key>
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    F: Fn(EdgeRef<<G as Keyed>::Key, <G as EdgeCollection>::EdgeWeight>) -> bool,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + NodeIter,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    for from in graph.node_ids() {
        if let Some(&dist) = distances.distance(from) {
            for edge @ EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(from) {
                if !filter(edge) {
                    continue;
                }

                let next = dist + *weight.cost();
                let to = edge_id.to();

                if let Some(&prev) = distances.distance(to)
                    && prev <= next
                {
                    continue;
                } else {
                    return to;
                }
            }
        }
    }

    unreachable!()
}

fn relax<C, F, G>(
    graph: &G,
    distances: &mut Distances<C, G>,
    parents: &mut Parents<G>,
    filter: &F,
) -> bool
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    F: Fn(EdgeRef<<G as Keyed>::Key, <G as EdgeCollection>::EdgeWeight>) -> bool,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + NodeIter,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    let mut updated = false;

    for from in graph.node_ids() {
        if let Some(&dist) = distances.distance(from) {
            for edge @ EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(from) {
                if !filter(edge) {
                    continue;
                }

                let next = dist + *weight.cost();
                let to = edge_id.to();

                if let Some(&prev) = distances.distance(to)
                    && prev <= next
                {
                    continue;
                } else {
                    parents.insert(from, to);
                    distances.update(to, next);
                    updated = true;
                }
            }
        }
    }

    updated
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::{bellman_ford_cycle, bellman_ford_to};
    use crate::test::{digraph, id, undigraph};
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn bellman_ford_g_1_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(0), id(1), |_| true)
                .distance
                .unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_undi_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(0), id(1), |_| true)
                .distance
                .unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn bellman_ford_wege_1_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(2), id(0), |_| true)
                .distance
                .unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(2), id(0), |_| true)
                .distance
                .unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_3_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege3.txt").unwrap();

        b.iter(|| {
            let result = bellman_ford_cycle(&graph, id(2), |_| true);
            assert!(result.is_right());
        })
    }

    // csr

    #[bench]
    fn bellman_ford_g_1_2_di_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(0), id(1), |_| true)
                .distance
                .unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_undi_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(0), id(1), |_| true)
                .distance
                .unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn bellman_ford_wege_1_di_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(2), id(0), |_| true)
                .distance
                .unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_2_di_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(2), id(0), |_| true)
                .distance
                .unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_3_di_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Wege3.txt").unwrap();

        b.iter(|| {
            let result = bellman_ford_cycle(&graph, id(2), |_| true);
            assert!(result.is_right())
        })
    }
}
