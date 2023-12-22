// use crate::view::{Distances, Route};
use grax_core::collections::*;
use grax_core::edge::*;
use grax_core::graph::*;
use grax_core::prelude::*;
use grax_core::view::{parents_cycle, Distances, Route};

use either::Either;

use std::fmt::Debug;
use std::ops::{Add, Sub};

pub fn bellman_ford_between<C, G>(graph: &G, from: NodeId<G::Key>, to: NodeId<G::Key>) -> Option<C>
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + Cost<C> + Flow<C> + NodeCount + NodeIter,
{
    bellman_ford(graph, from).and_then(|d| d.distances.get(to).to_owned())
}

pub fn bellman_ford<C, G>(graph: &G, start: NodeId<G::Key>) -> Option<Distances<C, G>>
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + Cost<C> + Flow<C> + NodeCount + NodeIter,
{
    let mut bf = BellmanFord::init(graph, start);

    for _ in 1..graph.node_count() {
        bf.updated = false;
        bf.relax();
        if !bf.updated {
            break;
        }
    }

    let BellmanFord {
        distances,
        updated,
        graph: _,
    } = bf;

    if !updated {
        Some(distances)
    } else {
        None
    }
}

pub fn bellman_ford_cycle<N, W, C, G>(
    graph: &G,
    start: NodeId<G::Key>,
) -> Either<Distances<C, G>, Route<G>>
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + Cost<C> + Flow<C> + NodeCount + NodeIter,
{
    let mut bf = BellmanFord::init(graph, start);

    for _ in 1..graph.node_count() {
        bf.updated = false;
        bf.relax();
        if !bf.updated {
            break;
        }
    }

    if bf.updated {
        let to = bf.relax_cycle();
        let route = parents_cycle(graph, &bf.distances.parents, to);
        Either::Right(route)
    } else {
        Either::Left(bf.distances)
    }
}

struct BellmanFord<'a, C: Clone + Debug, G: Cost<C> + NodeAttribute> {
    distances: Distances<C, G>,
    updated: bool,
    graph: &'a G,
}

impl<'a, C, G> BellmanFord<'a, C, G>
where
    C: Default + Debug + Clone,
    G: Cost<C> + NodeAttribute,
{
    fn init(graph: &'a G, start: NodeId<G::Key>) -> Self {
        let mut distances = Distances::new(graph);
        distances.update(start, C::default());

        Self {
            distances,
            updated: false,
            graph,
        }
    }
}

impl<'a, C, G> BellmanFord<'a, C, G>
where
    C: Default + Debug + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    G: Cost<C> + Flow<C> + NodeAttribute + EdgeIterAdjacent + NodeIter,
{
    fn relax(&mut self) {
        for from in self.graph.node_ids() {
            if let Some(&cost) = self.distances.distance(from) {
                for EdgeRef { edge_id, weight } in self.graph.iter_adjacent_edges(from) {
                    let to = edge_id.to();
                    let combined_cost = cost + *weight.cost();
                    let to_cost = self.distances.distance(to);

                    let update = match to_cost {
                        Some(&c) => c > combined_cost,
                        None => true,
                    } && (*weight.capacity() - *weight.flow()) > C::default();

                    if update {
                        self.distances.insert(from, to, combined_cost);
                        self.updated = true;
                    }
                }
            } else {
                continue;
            }
        }
    }

    fn relax_cycle(&mut self) -> NodeId<G::Key> {
        for from in self.graph.node_ids() {
            if let Some(&cost) = self.distances.distance(from) {
                for EdgeRef { edge_id, weight } in self.graph.iter_adjacent_edges(from) {
                    let to = edge_id.to();
                    let combined_cost = cost + *weight.cost();
                    let to_cost = self.distances.distance(to);

                    let update = match to_cost {
                        Some(&c) => c > combined_cost,
                        None => true,
                    } && (*weight.capacity() - *weight.flow()) > C::default();

                    if update {
                        return to;
                    }
                }
            } else {
                continue;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::{bellman_ford, bellman_ford_between};
    use crate::test::{digraph, id, undigraph};
    use grax_core::adaptor::flow::flow_adaptor;
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn bellman_ford_g_1_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();
        let graph: AdjGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = bellman_ford_between(&graph, id(0), id(1)).unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_undi_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();
        let graph: AdjGraph<_, _> = flow_adaptor(graph);

        b.iter(|| {
            let total = bellman_ford_between(&graph, id(0), id(1)).unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn bellman_ford_wege_1_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege1.txt").unwrap();
        let graph: AdjGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = bellman_ford_between(&graph, id(2), id(0)).unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege2.txt").unwrap();
        let graph: AdjGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = bellman_ford_between(&graph, id(2), id(0)).unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_3_di_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Wege3.txt").unwrap();
        let graph: AdjGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let result = bellman_ford(&graph, id(2));
            assert!(result.is_none());
        })
    }

    // csr

    #[bench]
    fn bellman_ford_g_1_2_di_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();
        let graph: CsrGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = bellman_ford_between(&graph, id(0), id(1)).unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_undi_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();
        let graph: CsrGraph<_, _> = flow_adaptor(graph);

        b.iter(|| {
            let total = bellman_ford_between(&graph, id(0), id(1)).unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn bellman_ford_wege_1_di_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Wege1.txt").unwrap();
        let graph: CsrGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = bellman_ford_between(&graph, id(2), id(0)).unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_2_di_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Wege2.txt").unwrap();
        let graph: CsrGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = bellman_ford_between(&graph, id(2), id(0)).unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_3_di_csr_graph(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Wege3.txt").unwrap();
        let graph: CsrGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let result = bellman_ford(&graph, id(2));
            assert!(result.is_none())
        })
    }
}
