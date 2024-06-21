use crate::category::ShortestPath;
use crate::utility::Distances;
use grax_core::collections::*;
use grax_core::edge::*;
use grax_core::graph::*;
use grax_core::prelude::*;

use either::Either;

use std::fmt::Debug;
use std::ops::{Add, Sub};

pub struct BellmanFord;

impl<C, G> ShortestPath<C, G> for BellmanFord
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + NodeCount + NodeIter,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    fn shortest_path(graph: &G, from: NodeId<G::Key>) -> Distances<C, G> {
        bellman_ford(graph, from)
    }

    fn shortest_path_to(
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
    ) -> (Option<C>, Distances<C, G>) {
        bellman_ford_to(graph, from, to)
    }
}

pub fn bellman_ford_to<C, G>(
    graph: &G,
    from: NodeId<G::Key>,
    to: NodeId<G::Key>,
) -> (Option<C>, Distances<C, G>)
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + NodeCount + NodeIter,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    let distances = bellman_ford(graph, from);

    (distances.distance(to).copied(), distances)
}

pub fn bellman_ford<C, G>(graph: &G, start: NodeId<G::Key>) -> Distances<C, G>
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + NodeCount + NodeIter,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    let mut distances = Distances::new(graph);
    distances.update(start, C::default());

    for _ in 1..graph.node_count() {
        if !relax(graph, &mut distances) {
            break;
        }
    }

    distances
}

pub fn bellman_ford_cycle<C, G>(
    graph: &G,
    start: NodeId<G::Key>,
) -> Either<Distances<C, G>, G::FixedEdgeMap<bool>>
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + NodeCount + NodeIter,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    let mut distances = Distances::new(graph);
    distances.update(start, C::default());

    let mut updated = false;

    for _ in 1..graph.node_count() {
        updated = relax(graph, &mut distances);
        if !updated {
            break;
        }
    }

    if updated {
        let mut cycle = graph.visit_edge_map();

        for from in graph.node_ids() {
            if let Some(&dist) = distances.distance(from) {
                for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(from) {
                    let next = dist + *weight.cost();
                    let to = edge_id.to();

                    if let Some(&prev) = distances.distance(to)
                        && prev < next
                    {
                        continue;
                    } else {
                        cycle.visit(edge_id);
                    }
                }
            }
        }

        Either::Right(cycle)
    } else {
        Either::Left(distances)
    }
}

fn relax<C, G>(graph: &G, distances: &mut Distances<C, G>) -> bool
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + NodeIter,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    let mut updated = false;

    for from in graph.node_ids() {
        if let Some(&dist) = distances.distance(from) {
            for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(from) {
                let next = dist + *weight.cost();
                let to = edge_id.to();

                if let Some(&prev) = distances.distance(to)
                    && prev < next
                {
                    continue;
                } else {
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
            let total = bellman_ford_to(&graph, id(0), id(1)).0.unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_undi_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(0), id(1)).0.unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn bellman_ford_wege_1_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(2), id(0)).0.unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(2), id(0)).0.unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_3_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege3.txt").unwrap();

        b.iter(|| {
            let result = bellman_ford_cycle(&graph, id(2));
            assert!(result.is_right());
        })
    }

    // csr

    #[bench]
    fn bellman_ford_g_1_2_di_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(0), id(1)).0.unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_undi_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(0), id(1)).0.unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn bellman_ford_wege_1_di_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(2), id(0)).0.unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_2_di_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_to(&graph, id(2), id(0)).0.unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_3_di_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Wege3.txt").unwrap();

        b.iter(|| {
            let result = bellman_ford_cycle(&graph, id(2));
            assert!(result.is_right())
        })
    }
}
