use super::topology::{GraphAdjacentTopology, GraphTopology};
use crate::{edge::EdgeRef, indices::NodeIndex, tree::UnionFind};
use priq::PriorityQueue;
use std::{cmp::Ordering, ops::AddAssign};

pub trait Sortable: PartialOrd {
    fn sort(&self, other: &Self) -> Ordering;
}

default impl<T: PartialOrd> Sortable for T {
    default fn sort(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Sortable for f64 {
    fn sort(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

impl Sortable for f32 {
    fn sort(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

pub trait GraphMst<N, W: Sortable + Default + AddAssign + Clone>:
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
}

impl<N, W: Sortable + Default + AddAssign + Clone, T: PrivateGraphMst<N, W>> GraphMst<N, W> for T {}

pub(crate) trait PrivateGraphMst<N, W: Sortable + Default + AddAssign + Clone>:
    GraphTopology<N, W> + GraphAdjacentTopology<N, W>
{
    /// Returns the root node of union find
    fn _kruskal<F>(&self, mut f: F) -> UnionFind
    where
        F: FnMut(EdgeRef<W>),
    {
        let mut priority_queue = self.edges().collect::<Vec<_>>();
        priority_queue.sort_unstable_by(|this, other| this.weight.sort(other.weight));

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
