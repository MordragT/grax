use crate::{
    error::{GraphError, GraphResult},
    indices::NodeIndex,
    prelude::EdgeIndex,
};
use std::{collections::HashSet, ops::AddAssign};

use super::{
    access::{GraphAccess, GraphCompare},
    mst::{PrivateGraphMst, Sortable},
    search::PrivateGraphSearch,
    topology::{GraphAdjacentTopology, GraphTopology},
};

// Sortable + PartialEq

pub trait GraphTsp<N: PartialEq, W: Sortable + PartialOrd + Default + AddAssign + Clone>:
    GraphTopology<N, W> + GraphAdjacentTopology<N, W> + GraphAccess<N, W> + GraphCompare<N, W> + Sized
{
    fn nearest_neighbor(&self) -> GraphResult<W> {
        match self.indices().next() {
            Some(start) => self._nearest_neighbor(start),
            None => Ok(W::default()),
        }
    }

    fn double_tree(&mut self) -> GraphResult<W> {
        let mut edges = HashSet::<EdgeIndex>::new();

        let union_find = self._kruskal(|edge| {
            edges.insert(edge.clone().into());
            edges.insert(edge.rev().into());
        });
        let root = union_find.root();

        // müssen edges wirklich entfernt werden ?
        self.retain_edges(edges.into_iter());

        let mut euler_tour = vec![];
        let mut visited = vec![false; self.node_count()];

        let mut branch = Vec::new();
        let mut parent = root;
        self.depth_search(root, &mut visited, true, |index| {
            // if rank is higher than rank of parent
            // or if index points to leaf (which is the same in the next step ?):
            // add nodes to euler
            let index_rank = union_find.rank(index);
            let parent_rank = union_find.rank(parent);

            if index_rank > parent_rank {
                branch.reverse();
                for idx in branch.drain(..) {
                    euler_tour.push(idx);
                }
            }
            branch.push(index);
            parent = index;
        });

        branch.reverse();
        for idx in branch.drain(..) {
            euler_tour.push(idx);
        }
        // euler_tour.push(root);

        dbg!(&euler_tour);

        let mut total_weight = W::default();
        for [from, to] in euler_tour.array_windows::<2>() {
            dbg!(from, to);
            let weight = match self.contains_edge(*from, *to) {
                Some(index) => self.weight(index).clone(),
                None => unreachable!(),
            };
            total_weight += weight;
        }

        if visited.into_iter().all(|visit| visit == true) {
            Ok(total_weight)
        } else {
            Err(GraphError::NoCycle)
        }
    }

    fn branch_bound(&self) -> W {
        todo!()
    }
}

impl<
        N: PartialEq,
        W: PartialOrd + Default + AddAssign + Clone + Sortable,
        T: GraphTopology<N, W> + GraphAdjacentTopology<N, W> + GraphAccess<N, W> + GraphCompare<N, W>,
    > GraphTsp<N, W> for T
{
}

trait PrivateGraphTsp<N: PartialEq, W: PartialOrd + Default + AddAssign + Clone>:
    GraphTopology<N, W> + GraphAdjacentTopology<N, W> + GraphAccess<N, W> + GraphCompare<N, W>
{
    fn _nearest_neighbor(&self, start: NodeIndex) -> GraphResult<W> {
        let mut visited = vec![false; self.node_count()];
        let mut total_weight = W::default();

        let mut target = (start, W::default());

        loop {
            visited[target.0 .0] = true;
            total_weight += target.1;

            let mut next = None;

            for edge in self.adjacent_edges(target.0) {
                if !visited[edge.to.0] {
                    if let Some((_, weight)) = next {
                        if weight > edge.weight {
                            next = Some((edge.to, edge.weight));
                        }
                    } else {
                        next = Some((edge.to, edge.weight));
                    }
                }
            }

            target = match next {
                Some((to, weight)) => (to, weight.to_owned()),
                None => break,
            };
        }

        if visited.into_iter().all(|visit| visit == true) {
            if let Some(edge_index) = self.contains_edge(target.0, start) {
                total_weight += self.weight(edge_index).to_owned();
                return Ok(total_weight);
            }
        }
        Err(GraphError::NNAbort)
    }
}

impl<
        N: PartialEq,
        W: PartialOrd + Default + AddAssign + Clone,
        T: GraphTopology<N, W> + GraphAdjacentTopology<N, W> + GraphAccess<N, W> + GraphCompare<N, W>,
    > PrivateGraphTsp<N, W> for T
{
}
