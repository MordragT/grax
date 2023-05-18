use super::Distances;
use crate::edge::EdgeRef;
use crate::graph::{GraphAdjacentTopology, GraphTopology};
use crate::indices::NodeIndex;
use std::ops::Add;

pub fn bellman_ford_between<N, W, G>(graph: &G, from: NodeIndex, to: NodeIndex) -> Option<W>
where
    W: Default + Add<W, Output = W> + PartialOrd + Copy,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let distances = bellman_ford(graph, from);

    distances.distances[to.0]
}

pub fn bellman_ford<N, W, G>(graph: &G, start: NodeIndex) -> Distances<W>
where
    W: Default + Add<W, Output = W> + PartialOrd + Copy,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut cost_table = vec![None; graph.node_count()];
    cost_table[start.0] = Some(W::default());

    for _ in 1..graph.node_count() {
        let mut updated = false;

        for index in graph.indices() {
            if let Some(cost) = cost_table[index.0] {
                for EdgeRef {
                    from: _,
                    to,
                    weight,
                } in graph.adjacent_edges(index)
                {
                    let combined_cost = cost + *weight;
                    let to_cost = cost_table[to.0];

                    match to_cost {
                        Some(c) if c > combined_cost => {
                            cost_table[to.0] = Some(combined_cost);
                            updated = true;
                        }
                        None => {
                            cost_table[to.0] = Some(combined_cost);
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
    Distances::new(start, cost_table)
}

#[cfg(test)]
mod test {
    extern crate test;

    use crate::{
        algorithms::bellman_ford_between,
        prelude::*,
        test::{digraph, undigraph},
    };
    use test::Bencher;

    #[bench]
    fn bellman_ford_g_1_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_between(&graph, NodeIndex(0), NodeIndex(1)).unwrap();
            assert_eq!(total as f32, 5.56283)
        })
    }

    #[bench]
    fn bellman_ford_g_1_2_undi_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_between(&graph, NodeIndex(0), NodeIndex(1)).unwrap();
            assert_eq!(total as f32, 2.36802)
        })
    }

    #[bench]
    fn bellman_ford_wege_1_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Wege1.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_between(&graph, NodeIndex(2), NodeIndex(0)).unwrap();
            assert_eq!(total as f32, 6.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_2_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Wege2.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_between(&graph, NodeIndex(2), NodeIndex(0)).unwrap();
            assert_eq!(total as f32, 2.0)
        })
    }

    #[bench]
    fn bellman_ford_wege_3_di_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Wege3.txt").unwrap();

        b.iter(|| {
            let total = bellman_ford_between(&graph, NodeIndex(2), NodeIndex(0)).unwrap();
            // cycle
            assert_eq!(total as f32, 2.0)
        })
    }
}
