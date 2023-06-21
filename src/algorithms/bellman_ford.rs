use crate::{
    graph::{Base, Count, EdgeCapacity, EdgeCost, EdgeFlow, Index, IndexAdjacent, IterAdjacent},
    prelude::{EdgeRef, NodeId},
    structures::{Distances, Parents, Route},
};
use either::Either;

use std::ops::{Add, Sub};

pub fn bellman_ford_between<N, W, C, G>(
    graph: &G,
    from: NodeId<G::Id>,
    to: NodeId<G::Id>,
) -> Option<W::Cost>
where
    C: Default + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    W: EdgeCost<Cost = C> + EdgeCapacity<Capacity = C> + EdgeFlow<Flow = C>,
    G: Index + Count + IndexAdjacent + IterAdjacent + Base<Node = N, Weight = W>,
{
    bellman_ford(graph, from).and_then(|d| d.distances[to.as_usize()])
}

pub fn bellman_ford<N, W, C, G>(graph: &G, start: NodeId<G::Id>) -> Option<Distances<W::Cost, G>>
where
    C: Default + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    W: EdgeCost<Cost = C> + EdgeCapacity<Capacity = C> + EdgeFlow<Flow = C>,
    G: Index + Count + IndexAdjacent + IterAdjacent + Base<Node = N, Weight = W>,
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
    start: NodeId<G::Id>,
) -> Either<Distances<W::Cost, G>, Route<G>>
where
    C: Default + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    W: EdgeCost<Cost = C> + EdgeCapacity<Capacity = C> + EdgeFlow<Flow = C>,
    G: Index + Count + IndexAdjacent + IterAdjacent + Base<Node = N, Weight = W>,
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
        let route = bf.distances.parents.find_cycle(to);
        Either::Right(route)
    } else {
        Either::Left(bf.distances)
    }
}

struct BellmanFord<'a, C, G: Base<Weight: EdgeCost<Cost = C>>> {
    distances: Distances<C, G>,
    updated: bool,
    graph: &'a G,
}

impl<'a, C: Default + Clone, G: Base<Weight: EdgeCost<Cost = C>> + Count> BellmanFord<'a, C, G> {
    fn init(graph: &'a G, start: NodeId<G::Id>) -> Self {
        let count = graph.node_count();
        let mut distances = Distances::with_count(count);

        distances.add_cost(start, C::default());

        Self {
            distances,
            updated: false,
            graph,
        }
    }
}

impl<'a, C, G> BellmanFord<'a, C, G>
where
    C: Default + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
    G: Base<Weight: EdgeCost<Cost = C>>,
{
    fn relax<N, W>(&mut self)
    where
        W: EdgeCost<Cost = C> + EdgeCapacity<Capacity = C> + EdgeFlow<Flow = C>,
        G: Index + Count + IndexAdjacent + IterAdjacent + Base<Node = N, Weight = W>,
    {
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

    fn relax_cycle<N, W>(&mut self) -> NodeId<G::Id>
    where
        W: EdgeCost<Cost = C> + EdgeCapacity<Capacity = C> + EdgeFlow<Flow = C>,
        G: Index + Count + IndexAdjacent + IterAdjacent + Base<Node = N, Weight = W>,
    {
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

    use crate::{
        prelude::*,
        test::{digraph, id, undigraph},
    };
    use test::Bencher;

    #[bench]
    fn bellman_ford_g_1_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph.bellman_ford_between(id(0), id(1)).unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_undi_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph.bellman_ford_between(id(0), id(1)).unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn bellman_ford_wege_1_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = graph.bellman_ford_between(id(2), id(0)).unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = graph.bellman_ford_between(id(2), id(0)).unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_3_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Wege3.txt").unwrap();

        b.iter(|| {
            let result = graph.bellman_ford(id(2));
            assert!(result.is_none());
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_di_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph.bellman_ford_between(id(0), id(1)).unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_undi_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph.bellman_ford_between(id(0), id(1)).unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn bellman_ford_wege_1_di_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = graph.bellman_ford_between(id(2), id(0)).unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_2_di_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = graph.bellman_ford_between(id(2), id(0)).unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_3_di_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Wege3.txt").unwrap();

        b.iter(|| {
            let result = graph.bellman_ford(id(2));
            assert!(result.is_none())
        })
    }
}
