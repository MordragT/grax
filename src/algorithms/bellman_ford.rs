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

pub fn bellman_ford<N, W, C, G>(
    graph: &G,
    start: NodeId<G::Id>,
) -> Option<Distances<G::Id, W::Cost>>
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
        cost_table,
        parents: _,
        updated,
        graph: _,
    } = bf;

    if !updated {
        Some(Distances::new(start, cost_table))
    } else {
        None
    }
}

pub fn bellman_ford_cycle<N, W, C, G>(
    graph: &G,
    start: NodeId<G::Id>,
) -> Either<Distances<G::Id, W::Cost>, Route<G>>
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
        let route = bf.parents.find_cycle(to);
        Either::Right(route)
    } else {
        Either::Left(Distances::new(start, bf.cost_table))
    }
}

struct BellmanFord<'a, C, G: Base> {
    cost_table: Vec<Option<C>>,
    parents: Parents<G>,
    updated: bool,
    graph: &'a G,
}

impl<'a, C: Default + Clone, G: Base + Count> BellmanFord<'a, C, G> {
    fn init(graph: &'a G, start: NodeId<G::Id>) -> Self {
        let count = graph.node_count();
        let mut cost_table = vec![None; count];
        cost_table[start.as_usize()] = Some(C::default());

        Self {
            cost_table,
            parents: Parents::with_count(count),
            updated: false,
            graph,
        }
    }
}

impl<'a, C, G: Base> BellmanFord<'a, C, G>
where
    C: Default + Add<C, Output = C> + PartialOrd + Copy + Sub<C, Output = C>,
{
    fn relax<N, W>(&mut self)
    where
        W: EdgeCost<Cost = C> + EdgeCapacity<Capacity = C> + EdgeFlow<Flow = C>,
        G: Index + Count + IndexAdjacent + IterAdjacent + Base<Node = N, Weight = W>,
    {
        for from in self.graph.node_ids() {
            if let Some(cost) = self.cost_table[from.as_usize()] {
                for EdgeRef { edge_id, weight } in self.graph.iter_adjacent_edges(from) {
                    let to = edge_id.to();
                    let combined_cost = cost + *weight.cost();
                    let to_cost = self.cost_table[to.as_usize()];

                    let update = match to_cost {
                        Some(c) => c > combined_cost,
                        None => true,
                    } && (*weight.capacity() - *weight.flow()) > C::default();

                    if update {
                        self.cost_table[to.as_usize()] = Some(combined_cost);
                        self.updated = true;
                        self.parents.insert(from, to);
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
            if let Some(cost) = self.cost_table[from.as_usize()] {
                for EdgeRef { edge_id, weight } in self.graph.iter_adjacent_edges(from) {
                    let to = edge_id.to();
                    let combined_cost = cost + *weight.cost();
                    let to_cost = self.cost_table[to.as_usize()];

                    let update = match to_cost {
                        Some(c) => c > combined_cost,
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
