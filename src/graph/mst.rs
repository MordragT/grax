use super::{
    topology::{GraphAdjacentTopology, GraphTopology},
    Sortable,
};
use crate::{edge::EdgeRef, indices::NodeIndex, tree::UnionFind};
use priq::PriorityQueue;
use std::ops::{Add, AddAssign};

pub fn kruskal<N, W, G>(graph: &G) -> W
where
    W: Default + Sortable + AddAssign + Copy,
    G: GraphTopology<N, W>,
{
    let mut total_weight = W::default();
    _kruskal(graph, |edge| total_weight += *edge.weight);
    total_weight
}

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

pub fn dijkstra<N, W, G>(graph: &G, from: NodeIndex, to: NodeIndex) -> Option<W>
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
            return Some(dist);
        }

        for edge in graph.adjacent_edges(node) {
            let next_dist = dist.clone() + *edge.weight;

            let visited_or_geq = match &distances[edge.to.0] {
                Some(d) => next_dist >= *d,
                None => false,
            };

            if !visited_or_geq {
                distances[edge.to.0] = Some(next_dist.clone());
                priority_queue.put(next_dist, edge.to);
            }
        }
    }

    None
}

pub(crate) fn _kruskal<N, W, G, F>(graph: &G, mut f: F) -> UnionFind
where
    W: Sortable,
    G: GraphTopology<N, W>,
    F: FnMut(EdgeRef<W>),
{
    let mut priority_queue = graph.edges().collect::<Vec<_>>();
    priority_queue.sort_by(|this, other| this.weight.sort(other.weight));

    let mut union_find = UnionFind::from(graph.indices());

    for edge in priority_queue {
        if union_find.find(edge.from) == union_find.find(edge.to) {
            continue;
        }
        union_find.union(edge.from, edge.to);
        f(edge);
    }

    union_find
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
