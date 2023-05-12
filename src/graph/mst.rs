use super::{
    topology::{GraphAdjacentTopology, GraphTopology},
    Sortable,
};
use crate::{edge::EdgeRef, indices::NodeIndex, tree::UnionFind};
use priq::PriorityQueue;
use std::{
    cmp::Ordering,
    ops::{Add, AddAssign},
};

pub trait GraphMst<N, W: Sortable + Default + Add<W, Output = W> + AddAssign + Clone>:
    GraphTopology<N, W> + GraphAdjacentTopology<N, W> + Sized
{
    fn kruskal(&self) -> W {
        let mut total_weight = W::default();
        self._kruskal(|edge| total_weight += edge.weight.to_owned());
        total_weight
    }

    fn prim(&self) -> W {
        match self.indices().next() {
            Some(start) => self._prim(start),
            None => W::default(),
        }
    }

    fn djikstra(&self, from: NodeIndex, to: NodeIndex) -> Option<W> {
        let mut priority_queue = PriorityQueue::new();
        let mut distances = vec![None; self.node_count()];

        distances[from.0] = Some(W::default());
        priority_queue.put(W::default(), from);

        while let Some((dist, node)) = priority_queue.pop() {
            if node == to {
                return Some(dist);
            }

            for edge in self.adjacent_edges(node) {
                let next_dist = dist.clone() + edge.weight.to_owned();

                let visited_or_geq = match &distances[edge.to.0] {
                    Some(d) => next_dist >= d.to_owned(),
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
}

impl<
        N,
        W: Sortable + Default + Add<W, Output = W> + AddAssign + Clone,
        T: PrivateGraphMst<N, W>,
    > GraphMst<N, W> for T
{
}

pub(crate) trait PrivateGraphMst<N, W: Sortable + Default + AddAssign + Clone>:
    GraphTopology<N, W> + GraphAdjacentTopology<N, W>
{
    /// Returns the root node of union find
    fn _kruskal<F>(&self, mut f: F) -> UnionFind
    where
        F: FnMut(EdgeRef<W>),
    {
        let mut priority_queue = self.edges().collect::<Vec<_>>();
        priority_queue.sort_by(|this, other| this.weight.sort(other.weight));

        let mut union_find = UnionFind::from(self.indices());

        for edge in priority_queue {
            if union_find.find(edge.from) == union_find.find(edge.to) {
                continue;
            }
            union_find.union(edge.from, edge.to);
            f(edge);
        }

        union_find
    }

    fn _prim(&self, start: NodeIndex) -> W {
        let n = self.node_count();
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

            for edge in self.adjacent_edges(to) {
                if !visited[edge.to.0] {
                    if let Some(weight) = &mut weights[edge.to.0] {
                        if *weight > edge.weight {
                            *weight = edge.weight;
                            priority_queue.put(edge.weight.to_owned(), edge.to);
                        }
                    } else {
                        weights[edge.to.0] = Some(edge.weight);
                        priority_queue.put(edge.weight.to_owned(), edge.to);
                    }
                }
            }
        }

        total_weight
    }
}

impl<
        N,
        W: Sortable + Default + AddAssign + Clone,
        T: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
    > PrivateGraphMst<N, W> for T
{
}
