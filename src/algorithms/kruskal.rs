use crate::graph::{GraphTopology, Sortable};
use crate::{edge::EdgeRef, tree::UnionFind};
use std::ops::AddAssign;

pub fn kruskal<N, W, G>(graph: &G) -> W
where
    W: Default + Sortable + AddAssign + Copy,
    G: GraphTopology<N, W>,
{
    let mut total_weight = W::default();
    _kruskal(graph, |edge| total_weight += *edge.weight);
    total_weight
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
