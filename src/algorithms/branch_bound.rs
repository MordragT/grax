use super::{dijkstra_between, nearest_neighbor, Tour};
use crate::{
    edge::EdgeRef,
    prelude::{GraphAdjacentTopology, GraphTopology, Maximum, NodeIndex, Sortable},
};
use std::ops::{Add, AddAssign};

pub fn branch_bound<N, W, G>(graph: &G) -> Option<Tour<W>>
where
    W: Default + Copy + AddAssign + Add<W, Output = W> + Maximum + Sortable,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    match graph.indices().next() {
        Some(start) => Some(_branch_bound(graph, start)),
        None => None,
    }
}

pub fn branch_bound_rec<N, W, G>(graph: &G) -> Option<Tour<W>>
where
    W: Default + Copy + Add<W, Output = W> + AddAssign + PartialOrd + Sortable + Maximum,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    match graph.indices().next() {
        Some(start) => {
            let mut baseline = nearest_neighbor(graph, start)
                .map(|tour| tour.weight)
                .unwrap_or(Maximum::max());
            let mut path = vec![start];
            let mut visited = vec![false; graph.node_count()];
            let cost = W::default();

            _branch_bound_rec(
                start,
                graph,
                start,
                &mut path,
                &mut visited,
                cost,
                &mut baseline,
            );

            Some(Tour::new(path, baseline))
        }
        None => None,
    }
}

pub(crate) fn _branch_bound<N, W, G>(graph: &G, start: NodeIndex) -> Tour<W>
where
    W: Default + Copy + AddAssign + Add<W, Output = W> + Maximum + Sortable,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut stack = Vec::new();
    let mut total_cost = nearest_neighbor(graph, start)
        .map(|tour| tour.weight)
        .unwrap_or(Maximum::max());
    let mut route = Vec::new();

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
            let cost = cost + *weight;

            if !visited[to.0] && cost < total_cost {
                let mut visited = visited.clone();
                visited[to.0] = true;

                let mut path = path.clone();
                path.push(to);

                if visited.iter().all(|v| *v == true) {
                    if let Some(cost_to_start) =
                        dijkstra_between(graph, path[path.len() - 1], start)
                    {
                        let cost = cost + cost_to_start;

                        if cost < total_cost {
                            total_cost = cost;
                            std::mem::swap(&mut path, &mut route);
                        }
                    }
                } else {
                    stack.push((cost, path, visited));
                }
            }
        }
    }

    Tour::new(route, total_cost)
}

pub(crate) fn _branch_bound_rec<N, W, G>(
    start: NodeIndex,
    graph: &G,
    node: NodeIndex,
    path: &mut Vec<NodeIndex>,
    visited: &mut Vec<bool>,
    cost: W,
    baseline: &mut W,
) where
    W: Default + Copy + Add<W, Output = W> + AddAssign + PartialOrd + Sortable,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    if visited.iter().all(|v| *v) && let Some(cost_to_start) = dijkstra_between(graph, node, start) {
        let total_cost = cost + cost_to_start;
        if total_cost < *baseline {
            *baseline = total_cost;
        }
    }

    for EdgeRef {
        from: _,
        to,
        weight,
    } in graph.adjacent_edges(node)
    {
        let cost = cost + *weight;

        if !visited[to.0] && cost < *baseline {
            visited[to.0] = true;
            path.push(to);

            _branch_bound_rec(start, graph, to, path, visited, cost, baseline);

            visited[to.0] = false;
            path.pop();
        }
    }
}
