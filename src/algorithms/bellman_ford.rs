use crate::{
    graph::{Count, Index, IndexAdjacent, IterAdjacent},
    prelude::{EdgeIdentifier, EdgeRef, NodeIdentifier},
};

use super::{Distances, NegativeCycle};
use std::ops::Add;

pub fn bellman_ford_between<N, W, G>(graph: &G, from: G::NodeId, to: G::NodeId) -> Option<W>
where
    W: Default + Add<W, Output = W> + PartialOrd + Copy,
    G: Index + Count + IndexAdjacent + IterAdjacent<N, W>,
{
    bellman_ford(graph, from)
        .ok()
        .and_then(|d| d.distances[to.as_usize()])
}

pub fn bellman_ford<N, W, G>(
    graph: &G,
    start: G::NodeId,
) -> Result<Distances<G::NodeId, W>, NegativeCycle>
where
    W: Default + Add<W, Output = W> + PartialOrd + Copy,
    G: Index + Count + IndexAdjacent + IterAdjacent<N, W>,
{
    let mut cost_table = vec![None; graph.node_count()];
    cost_table[start.as_usize()] = Some(W::default());

    let mut updated = false;

    for _ in 0..graph.node_count() {
        updated = false;

        for index in graph.node_ids() {
            if let Some(cost) = cost_table[index.as_usize()] {
                for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(index) {
                    let to = edge_id.to();
                    let combined_cost = cost + *weight;
                    let to_cost = cost_table[to.as_usize()];

                    match to_cost {
                        Some(c) if c > combined_cost => {
                            cost_table[to.as_usize()] = Some(combined_cost);
                            updated = true;
                        }
                        None => {
                            cost_table[to.as_usize()] = Some(combined_cost);
                            updated = true;
                        }
                        _ => (),
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
        Err(NegativeCycle)
    } else {
        Ok(Distances::new(start, cost_table))
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use crate::{
        algorithms::NegativeCycle,
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
            assert_eq!(result, Err(NegativeCycle))
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
            assert_eq!(result, Err(NegativeCycle))
        })
    }
}
