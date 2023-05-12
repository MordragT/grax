use super::{_nearest_neighbor, dijkstra};
use crate::{
    edge::EdgeRef,
    error::GraphResult,
    prelude::{GraphAdjacentTopology, GraphTopology, Maximum, NodeIndex, Sortable},
};
use std::ops::{Add, AddAssign};

pub fn branch_bound<N, W, G>(graph: &G) -> GraphResult<W>
where
    W: Default + Copy + AddAssign + Add<W, Output = W> + Maximum + Sortable,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    match graph.indices().next() {
        Some(start) => _branch_bound(graph, start, true),
        None => Ok(W::default()),
    }
}

pub(crate) fn _branch_bound<N, W, G>(graph: &G, start: NodeIndex, compare: bool) -> GraphResult<W>
where
    W: Default + Copy + AddAssign + Add<W, Output = W> + Maximum + Sortable,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut stack = Vec::new();
    let mut total_cost = _nearest_neighbor(graph, start).unwrap();

    let mut visited = vec![false; graph.node_count()];
    visited[start.0] = true;

    stack.push((W::default(), vec![start], visited));

    while let Some((cost, path, visited)) = stack.pop() {
        let node = path
            .last()
            .expect("INTERNAL: Path always expected to have atleast one element");

        for EdgeRef {
            from: _,
            to,
            weight,
        } in graph.adjacent_edges(*node)
        {
            let cost = cost.clone() + weight.clone();

            if !visited[to.0] && (cost < total_cost || !compare) {
                let mut visited = visited.clone();
                visited[to.0] = true;

                let mut path = path.clone();
                path.push(to);

                if visited.iter().all(|v| *v == true) {
                    if let Some(cost_to_start) = dijkstra(graph, path[path.len() - 1], start) {
                        let cost = cost + cost_to_start;

                        if cost < total_cost {
                            total_cost = cost;
                        }
                    }
                } else {
                    stack.push((cost, path, visited));
                }
            }
        }
    }

    Ok(total_cost)
}
