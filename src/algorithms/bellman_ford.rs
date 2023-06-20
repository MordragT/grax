use crate::{
    graph::{Count, Index, IndexAdjacent, IterAdjacent, WeightCapacity, WeightCost},
    prelude::{EdgeRef, NodeId},
    structures::{Distances, Parents, Route},
};
use either::Either;

use std::ops::Add;

pub struct BellmanFord {}

pub fn bellman_ford_between<N, W, C, G>(
    graph: &G,
    from: NodeId<G::Id>,
    to: NodeId<G::Id>,
) -> Option<W::Cost>
where
    C: Default + Add<C, Output = C> + PartialOrd + Copy,
    W: WeightCost<Cost = C>,
    G: Index + Count + IndexAdjacent + IterAdjacent<N, W>,
{
    bellman_ford(graph, from).and_then(|d| d.distances[to.as_usize()])
}

pub fn bellman_ford<N, W, C, G>(
    graph: &G,
    start: NodeId<G::Id>,
) -> Option<Distances<G::Id, W::Cost>>
where
    C: Default + Add<C, Output = C> + PartialOrd + Copy,
    W: WeightCost<Cost = C>,
    G: Index + Count + IndexAdjacent + IterAdjacent<N, W>,
{
    let (cost_table, updated, _) = bellman_ford_init_relax(graph, start, |_| true);

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
    C: Default + Add<C, Output = C> + PartialOrd + Copy,
    W: WeightCost<Cost = C> + WeightCapacity<Capacity = C>,
    G: Index + Count + IndexAdjacent + IterAdjacent<N, W>,
{
    let (cost_table, updated, parents) =
        bellman_ford_init_relax(graph, start, |weight| weight.capacity() > &C::default());

    if updated {
        for from in graph.node_ids() {
            if let Some(cost) = cost_table[from.as_usize()] {
                for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(from) {
                    let to = edge_id.to();
                    let combined_cost = cost + *weight.cost();
                    let to_cost = cost_table[to.as_usize()];

                    let update = match to_cost {
                        Some(c) => c > combined_cost,
                        None => true,
                    };

                    if update && weight.capacity() > &C::default() {
                        return Either::Right(parents.find_cycle(to));
                    }
                }
            } else {
                continue;
            }
        }
        unreachable!();
    } else {
        Either::Left(Distances::new(start, cost_table))
    }
}

fn bellman_ford_init_relax<N, W, C, G, F>(
    graph: &G,
    start: NodeId<G::Id>,
    mut f: F,
) -> (Vec<Option<C>>, bool, Parents<G>)
where
    C: Default + Add<C, Output = C> + PartialOrd + Copy,
    W: WeightCost<Cost = C>,
    G: Index + Count + IndexAdjacent + IterAdjacent<N, W>,
    F: FnMut(&W) -> bool,
{
    let count = graph.node_count();
    let mut cost_table = vec![None; count];
    cost_table[start.as_usize()] = Some(W::Cost::default());

    let mut updated = false;
    let mut parents = Parents::with_count(count);

    for _ in 0..graph.node_count() {
        updated = false;

        for from in graph.node_ids() {
            if let Some(cost) = cost_table[from.as_usize()] {
                for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(from) {
                    let to = edge_id.to();
                    let combined_cost = cost + *weight.cost();
                    let to_cost = cost_table[to.as_usize()];

                    let update = match to_cost {
                        Some(c) => c > combined_cost,
                        None => true,
                    } && f(weight);

                    if update {
                        cost_table[to.as_usize()] = Some(combined_cost);
                        updated = true;
                        parents.insert(from, to);
                    }
                }
            } else {
                continue;
            }
        }

        if !updated {
            break;
        }
    }

    (cost_table, updated, parents)

    // if updated {
    //     let cycle = parents.find_cycle(node);
    //     Either::Right(cycle)
    // } else {
    //     Either::Left(Distances::new(start, cost_table))
    // }
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
