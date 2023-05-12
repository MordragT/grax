use crate::graph::{GraphAdjacentTopology, GraphTopology, Sortable};
use crate::indices::NodeIndex;
use priq::PriorityQueue;
use std::ops::Add;

use super::Distances;

pub fn dijkstra<N, W, G>(graph: &G, from: NodeIndex, to: NodeIndex) -> Option<W>
where
    W: Default + Sortable + Copy + Add<W, Output = W>,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    dijkstra_distances(graph, from, to).distances[to.0]
}

pub fn dijkstra_distances<N, W, G>(graph: &G, from: NodeIndex, to: NodeIndex) -> Distances<W>
where
    W: Default + Sortable + Copy + Add<W, Output = W>,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut priority_queue = PriorityQueue::new();
    let mut distances = vec![None; graph.node_count()];

    distances[from.0] = Some(W::default());
    priority_queue.put(W::default(), from);

    while let Some((dist, node)) = priority_queue.pop() {
        if node == to {
            return Distances::new(from, distances);
        }

        for edge in graph.adjacent_edges(node) {
            let next_dist = dist + *edge.weight;

            let visited_or_geq = match &distances[edge.to.0] {
                Some(d) => next_dist >= *d,
                None => false,
            };

            if !visited_or_geq {
                distances[edge.to.0] = Some(next_dist);
                priority_queue.put(next_dist, edge.to);
            }
        }
    }

    Distances::new(from, distances)
}
