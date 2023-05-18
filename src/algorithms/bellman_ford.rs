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
