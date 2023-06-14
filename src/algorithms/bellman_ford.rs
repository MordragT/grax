use crate::{
    graph::{Count, EdgeCost, Index, IndexAdjacent, IterAdjacent},
    prelude::{EdgeIdentifier, EdgeRef, NodeIdentifier},
};
use either::Either;

use super::{Distances, NegativeCycle};
use std::ops::Add;

pub fn bellman_ford_between<N, W, C, G>(
    graph: &G,
    from: G::NodeId,
    to: G::NodeId,
) -> Option<W::Cost>
where
    C: Default + Add<C, Output = C> + PartialOrd + Copy,
    W: EdgeCost<Cost = C>,
    G: Index + Count + IndexAdjacent + IterAdjacent<N, W>,
{
    bellman_ford(graph, from)
        .left()
        .and_then(|d| d.distances[to.as_usize()])
}

pub fn bellman_ford<N, W, C, G>(
    graph: &G,
    start: G::NodeId,
) -> Either<Distances<G::NodeId, W::Cost>, NegativeCycle<G::NodeId>>
where
    C: Default + Add<C, Output = C> + PartialOrd + Copy,
    W: EdgeCost<Cost = C>,
    G: Index + Count + IndexAdjacent + IterAdjacent<N, W>,
{
    _bellman_ford(graph, start, |_| true)
}

pub(crate) fn _bellman_ford<N, W, C, G, F>(
    graph: &G,
    start: G::NodeId,
    mut f: F,
) -> Either<Distances<G::NodeId, W::Cost>, NegativeCycle<G::NodeId>>
where
    C: Default + Add<C, Output = C> + PartialOrd + Copy,
    W: EdgeCost<Cost = C>,
    G: Index + Count + IndexAdjacent + IterAdjacent<N, W>,
    F: FnMut(&W) -> bool,
{
    let mut cost_table = vec![None; graph.node_count()];
    cost_table[start.as_usize()] = Some(W::Cost::default());

    let mut updated = false;
    let mut parents = vec![None; graph.node_count()];

    for _ in 0..graph.node_count() {
        updated = false;

        for index in graph.node_ids() {
            if let Some(cost) = cost_table[index.as_usize()] {
                for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(index) {
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
                        parents[to.as_usize()] = Some(index);
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

    if updated {
        Either::Right(NegativeCycle::new(start, parents))
    } else {
        Either::Left(Distances::new(start, cost_table))
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use crate::{
        prelude::*,
        test::{digraph, undigraph},
    };
    use test::Bencher;

    #[bench]
    fn bellman_ford_g_1_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph
                .bellman_ford_between(NodeIndex(0), NodeIndex(1))
                .unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_undi_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph
                .bellman_ford_between(NodeIndex(0), NodeIndex(1))
                .unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn bellman_ford_wege_1_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = graph
                .bellman_ford_between(NodeIndex(2), NodeIndex(0))
                .unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = graph
                .bellman_ford_between(NodeIndex(2), NodeIndex(0))
                .unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_3_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Wege3.txt").unwrap();

        b.iter(|| {
            let result = graph.bellman_ford(NodeIndex(2));
            assert!(result.is_right())
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_di_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph
                .bellman_ford_between(NodeIndex(0), NodeIndex(1))
                .unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_undi_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph
                .bellman_ford_between(NodeIndex(0), NodeIndex(1))
                .unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn bellman_ford_wege_1_di_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = graph
                .bellman_ford_between(NodeIndex(2), NodeIndex(0))
                .unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_2_di_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = graph
                .bellman_ford_between(NodeIndex(2), NodeIndex(0))
                .unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_3_di_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Wege3.txt").unwrap();

        b.iter(|| {
            let result = graph.bellman_ford(NodeIndex(2));
            assert!(result.is_right())
        })
    }
}
