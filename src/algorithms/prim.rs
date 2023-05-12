use crate::graph::{GraphAdjacentTopology, GraphTopology, Sortable};
use crate::indices::NodeIndex;
use priq::PriorityQueue;
use std::ops::AddAssign;

pub fn prim<N, W, G>(graph: &G) -> W
where
    W: Default + Sortable + AddAssign + Copy,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    match graph.indices().next() {
        Some(start) => _prim(graph, start),
        None => W::default(),
    }
}

pub(crate) fn _prim<N, W, G>(graph: &G, start: NodeIndex) -> W
where
    W: Default + Sortable + AddAssign + Copy,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let n = graph.node_count();
    let mut visited = vec![false; n];
    let mut priority_queue = PriorityQueue::with_capacity(n);
    // einfach mit W::max init
    let mut weights = vec![None; n];
    let mut total_weight = W::default();

    priority_queue.put(W::default(), start);

    while let Some((weight, to)) = priority_queue.pop() {
        if visited[to.0] {
            continue;
        }
        visited[to.0] = true;
        total_weight += weight;

        for edge in graph.adjacent_edges(to) {
            if !visited[edge.to.0] {
                if let Some(weight) = &mut weights[edge.to.0] {
                    if *weight > edge.weight {
                        *weight = edge.weight;
                        priority_queue.put(*edge.weight, edge.to);
                    }
                } else {
                    weights[edge.to.0] = Some(edge.weight);
                    priority_queue.put(*edge.weight, edge.to);
                }
            }
        }
    }

    total_weight
}
