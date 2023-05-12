use crate::{
    adjacency_list::AdjacencyOptions,
    edge::{Edge, EdgeRef},
    error::{GraphError, GraphResult},
    indices::NodeIndex,
    prelude::AdjacencyList,
};
use std::{
    collections::VecDeque,
    ops::{Add, AddAssign},
};

use super::{
    access::{GraphAccess, GraphCompare},
    mst::PrivateGraphMst,
    search::PrivateGraphSearch,
    topology::{GraphAdjacentTopology, GraphTopology},
    GraphMst, Maximum, Sortable,
};

// Sortable + PartialEq

pub trait GraphTsp<
    N: PartialEq,
    W: Sortable + Maximum + PartialOrd + Default + Add<W, Output = W> + AddAssign + Clone,
>:
    GraphTopology<N, W>
    + GraphAdjacentTopology<N, W>
    + GraphAccess<N, W>
    + GraphCompare<N, W>
    + GraphMst<N, W>
    + Sized
    + Clone
{
    fn nearest_neighbor(&self) -> Option<W> {
        match self.indices().next() {
            Some(start) => self._nearest_neighbor(start),
            None => None,
        }
    }

    fn double_tree(&mut self) -> GraphResult<W> {
        let mut mst = AdjacencyList::with(AdjacencyOptions {
            directed: self.directed(),
            nodes: Some(self.nodes().collect()),
        });

        let union_find = self._kruskal(|edge| {
            mst.add_edge(edge.from, edge.to, edge.weight.clone())
                .unwrap();
            mst.add_edge(edge.to, edge.from, edge.weight.clone())
                .unwrap();
        });
        let root = union_find.root();

        let mut euler_tour = vec![];
        let mut visited = vec![false; self.node_count()];

        mst.depth_search(root, &mut visited, true, |index| {
            euler_tour.push(index);
        });

        euler_tour.push(root);

        let mut total_weight = W::default();
        for [from, to] in euler_tour.array_windows::<2>() {
            let weight = match mst.contains_edge(*from, *to) {
                Some(index) => mst.weight(index).clone(),
                None => self.djikstra(*from, *to).ok_or(GraphError::NoCycle)?,
            };
            total_weight += weight;
        }

        if visited.into_iter().all(|visit| visit == true) {
            Ok(total_weight)
        } else {
            Err(GraphError::NoCycle)
        }
    }

    fn branch_bound(&self) -> GraphResult<W> {
        match self.indices().next() {
            Some(start) => self._branch_bound(start),
            None => Ok(W::default()),
        }
    }

    fn brute_force(&self) -> Option<W> {
        let mut best_path = Vec::new();
        let mut best_weight = W::max();

        let start = (0..self.node_count()).map(NodeIndex).collect::<Vec<_>>();

        for perm in permute::permutations_of(&start) {
            let mut perm = perm.map(ToOwned::to_owned).collect::<Vec<_>>();
            perm.push(perm[0]);

            let edges = perm
                .array_windows::<2>()
                .map(|w| self.contains_edge(w[0], w[1]))
                .collect::<Option<Vec<_>>>();

            if let Some(edges) = edges {
                let total_weight = edges
                    .into_iter()
                    .map(|edge| self.weight(edge).to_owned())
                    .fold(W::default(), |mut accu, w| {
                        accu += w;
                        accu
                    });

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
}

impl<
        N: PartialEq,
        W: PartialOrd + Default + Add<W, Output = W> + AddAssign + Clone + Sortable + Maximum,
        T: GraphTopology<N, W>
            + GraphAdjacentTopology<N, W>
            + GraphAccess<N, W>
            + GraphCompare<N, W>
            + Clone,
    > GraphTsp<N, W> for T
{
}

trait PrivateGraphTsp<
    N: PartialEq,
    W: PartialOrd + Default + Add<W, Output = W> + AddAssign + Clone + Sortable + Maximum,
>:
    GraphTopology<N, W>
    + GraphAdjacentTopology<N, W>
    + GraphAccess<N, W>
    + GraphCompare<N, W>
    + PrivateGraphMst<N, W>
    + Sized
{
    fn _branch_bound(&self, start: NodeIndex) -> GraphResult<W> {
        let mut stack = Vec::new();
        let mut total_cost = self._nearest_neighbor(start).unwrap();

        stack.push((W::default(), vec![start], vec![false; self.node_count()]));

        while let Some((cost, path, visited)) = stack.pop() {
            let node = path
                .last()
                .expect("INTERNAL: Path always expected to have atleast one element");

            for EdgeRef {
                from: _,
                to,
                weight,
            } in self.adjacent_edges(*node)
            {
                let cost = cost.clone() + weight.clone();

                if !visited[to.0] && cost < total_cost {
                    let mut visited = visited.clone();
                    visited[to.0] = true;

                    let mut path = path.clone();
                    path.push(to);

                    if visited.iter().all(|v| *v == true) {
                        total_cost = cost;
                    } else {
                        stack.push((cost, path, visited));
                    }
                }
            }
        }

        Ok(total_cost)
    }

    fn _nearest_neighbor(&self, start: NodeIndex) -> Option<W> {
        #[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
        enum Status {
            Visited,
            #[default]
            Unvisited,
            Diverged,
        }

        let mut states = vec![Status::default(); self.node_count()];
        let mut path = vec![(start, W::default())];
        let mut prev = start;

        states[start.0] = Status::Visited;

        while let Some((node, _)) = path.last() && path.len() < self.node_count() {

            let mut min_node = None;
            let mut min_weight = W::max();

            for EdgeRef { from:_, to, weight } in self.adjacent_edges(*node) {
                if states[to.0] == Status::Unvisited && to != prev {
                    let weight = weight.to_owned();
                    if min_weight > weight {
                        min_node = Some(to);
                        min_weight = weight;
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
                        self.adjacent_indices(*node).any(|neigh| states[neigh.0] == Status::Unvisited)
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

        match self.djikstra(prev, start) {
            Some(weight) => path.push((start, weight)),
            None => return None,
        }

        let total_weight = path.into_iter().fold(W::default(), |mut accu, (_, w)| {
            accu += w;
            accu
        });

        Some(total_weight)
    }
}

impl<
        N: PartialEq,
        W: PartialOrd + Default + Add<W, Output = W> + AddAssign + Clone + Sortable + Maximum,
        T: GraphTopology<N, W>
            + GraphAdjacentTopology<N, W>
            + GraphAccess<N, W>
            + GraphCompare<N, W>
            + PrivateGraphMst<N, W>,
    > PrivateGraphTsp<N, W> for T
{
}
