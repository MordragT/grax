use crate::{
    error::{GraphError, GraphResult},
    indices::NodeIndex,
};
use std::{collections::HashSet, ops::AddAssign};

use super::{
    access::{GraphAccess, GraphCompare},
    mst::{PrivateGraphMst, Sortable},
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

    fn double_tree(&self) -> GraphResult<W> {
        // TODO return Tree instead of just root.
        let mut adjacencies = vec![HashSet::new(); self.node_count()];

        let union_find = self._kruskal(|edge| {
            adjacencies[edge.from.0].insert(edge.to);
        });

        let root = union_find.into_root();

        // let mut visited = vec![false; self.node_count()];
        // let mut total_weight = W::default();

        // self.depth_search(root, &mut visited, true, |index| {
        //     let weight = self.weight(index);
        //     total_weight += weight.to_owned();
        // });

        // if visited.into_iter().all(|visit| visit == true) {
        //     Ok(total_weight)
        // } else {
        //     Err(GraphError::NoCycle)
        // }
        todo!()
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
