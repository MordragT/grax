use crate::{
    adjacency_list::AdjacencyOptions,
    edge::EdgeRef,
    error::{GraphError, GraphResult},
    indices::NodeIndex,
    prelude::{mst::dijkstra, AdjacencyList},
};
use std::ops::{Add, AddAssign};

use super::{
    access::{GraphAccess, GraphCompare},
    mst::_kruskal,
    search::_depth_search,
    topology::{GraphAdjacentTopology, GraphTopology},
    Maximum, Sortable,
};

pub fn nearest_neighbor<N, W, G>(graph: &G) -> Option<W>
where
    W: Default + Copy + AddAssign + Add<W, Output = W> + Maximum + Sortable,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    match graph.indices().next() {
        Some(start) => _nearest_neighbor(graph, start),
        None => None,
    }
}

pub fn double_tree<N, W, G>(graph: &G) -> GraphResult<W>
where
    N: PartialEq,
    W: Default + Sortable + Copy + AddAssign + Add<W, Output = W>,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W> + GraphCompare<N, W>,
{
    let mut mst = AdjacencyList::with(AdjacencyOptions {
        directed: graph.directed(),
        nodes: Some(graph.nodes().collect()),
    });

    let union_find = _kruskal(graph, |edge| {
        mst.add_edge(edge.from, edge.to, edge.weight.clone())
            .unwrap();
        mst.add_edge(edge.to, edge.from, edge.weight.clone())
            .unwrap();
    });
    let root = union_find.root();

    let mut euler_tour = vec![];
    let mut visited = vec![false; graph.node_count()];

    _depth_search(&mst, root, &mut visited, true, |index| {
        euler_tour.push(index);
    });

    euler_tour.push(root);

    let mut total_weight = W::default();
    for [from, to] in euler_tour.array_windows::<2>() {
        let weight = match mst.contains_edge(*from, *to) {
            Some(index) => mst.weight(index).clone(),
            None => dijkstra(graph, *from, *to).ok_or(GraphError::NoCycle)?,
        };
        total_weight += weight;
    }

    if visited.into_iter().all(|visit| visit == true) {
        Ok(total_weight)
    } else {
        Err(GraphError::NoCycle)
    }
}

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

pub fn brute_force<N, W, G>(graph: &G) -> Option<W>
where
    N: PartialEq,
    W: Default + Maximum + PartialOrd + AddAssign + Copy,
    G: GraphTopology<N, W> + GraphCompare<N, W> + GraphAccess<N, W>,
{
    let mut best_path = Vec::new();
    let mut best_weight = W::max();

    let start = (0..graph.node_count()).map(NodeIndex).collect::<Vec<_>>();

    for perm in permute::permutations_of(&start) {
        let mut perm = perm.map(ToOwned::to_owned).collect::<Vec<_>>();
        perm.push(perm[0]);

        let edges = perm
            .array_windows::<2>()
            .map(|w| graph.contains_edge(w[0], w[1]))
            .collect::<Option<Vec<_>>>();

        if let Some(edges) = edges {
            let total_weight = edges.into_iter().map(|edge| *graph.weight(edge)).fold(
                W::default(),
                |mut accu, w| {
                    accu += w;
                    accu
                },
            );

            if total_weight < best_weight {
                best_path = perm.clone();
                best_weight = total_weight;
            }
        }
    }

    if best_weight == W::max() {
        None
    } else {
        Some(best_weight)
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

pub(crate) fn _nearest_neighbor<N, W, G>(graph: &G, start: NodeIndex) -> Option<W>
where
    W: Default + Copy + AddAssign + Add<W, Output = W> + Maximum + Sortable,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
    enum Status {
        Visited,
        #[default]
        Unvisited,
        Diverged,
    }

    let mut states = vec![Status::default(); graph.node_count()];
    let mut path = vec![(start, W::default())];
    let mut prev = start;

    states[start.0] = Status::Visited;

    while let Some((node, _)) = path.last() && path.len() < graph.node_count() {

        let mut min_node = None;
        let mut min_weight = W::max();

        for EdgeRef { from:_, to, weight } in graph.adjacent_edges(*node) {
            if states[to.0] == Status::Unvisited && to != prev {
                if min_weight > *weight {
                    min_node = Some(to);
                    min_weight = *weight;
                }
            }
        }

        match min_node {
            Some(next) => {
                path.push((next, min_weight));
                states[next.0] = Status::Visited;
                prev = next;
            }
            None => {
                let open_end = path.iter().rposition(|(node, _)| {
                    graph.adjacent_indices(*node).any(|neigh| states[neigh.0] == Status::Unvisited)
                });

                if let Some(index) = open_end {
                    let branch_point = path[index].0;

                    if states[branch_point.0] == Status::Diverged {
                        return None;
                    } else {
                        states[branch_point.0] = Status::Diverged;
                    }
                    let splitted_off = path.split_off(index + 1);
                    for (node, _) in splitted_off.into_iter().rev() {
                        states[node.0] = Status::Unvisited;
                        prev = node;
                    }
                } else {
                    return None;
                }
            }
        }
    }

    assert!(states
        .into_iter()
        .all(|visit| visit == Status::Visited || visit == Status::Diverged));

    match dijkstra(graph, prev, start) {
        Some(weight) => path.push((start, weight)),
        None => return None,
    }

    let total_weight = path.into_iter().fold(W::default(), |mut accu, (_, w)| {
        accu += w;
        accu
    });

    Some(total_weight)
}
