use super::{nearest_neighbor, TspSolver};
use crate::{
    cycle::{Cycle, TspCycle},
    parents::Parents,
    weight::{Bounded, TotalOrd},
};

use grax_core::collections::*;
use grax_core::edge::{weight::*, *};
use grax_core::graph::*;
use grax_core::prelude::*;
use std::fmt::Debug;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy)]
pub struct BranchBound;

impl<C, G> TspSolver<C, G> for BranchBound
where
    C: Debug + Copy + Default + PartialOrd + AddAssign<C> + Add<C, Output = C> + TotalOrd + Bounded,
    G: NodeIterAdjacent + EdgeIterAdjacent + NodeAttribute + NodeIter + NodeCount + IndexEdge,
    G::EdgeWeight: Cost<C>,
{
    fn solve(graph: &G) -> Option<TspCycle<C, G>> {
        branch_bound_rec(graph)
    }
}

// test algorithms::branch_bound::test::branch_bound_k_10e_adj_list         ... bench:   4,498,493.70 ns/iter (+/- 390,846.68)

pub fn branch_bound<C, G>(graph: &G) -> Option<TspCycle<C, G>>
where
    C: Debug + Copy + Default + PartialOrd + AddAssign<C> + Add<C, Output = C> + TotalOrd + Bounded,
    G: NodeIterAdjacent + EdgeIterAdjacent + NodeAttribute + NodeIter + NodeCount + IndexEdge,
    G::EdgeWeight: Cost<C>,
    G::FixedNodeMap<bool>: Clone,
{
    let start = graph.node_ids().next()?;

    let TspCycle {
        cost: mut best_cost,
        cycle: mut best_cycle,
    } = nearest_neighbor(graph).unwrap_or(TspCycle {
        cost: Bounded::MAX,
        cycle: Cycle {
            parents: Parents::new(graph),
            member: start,
        },
    });

    let mut visited = graph.visit_node_map();
    visited.visit(start);

    let mut stack = Vec::new();
    stack.push((C::default(), Parents::new(graph), start, visited));

    while let Some((cost, parents, from, visited)) = stack.pop() {
        for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(from) {
            let to = edge_id.to();
            let mut cost = cost + *weight.cost();

            if !visited.is_visited(to) && cost < best_cost {
                let mut visited = visited.clone();
                visited.visit(to);

                let mut parents = parents.clone();
                parents.insert(from, to);

                if visited.all_visited() {
                    cost += *graph[EdgeId::new_unchecked(to, start)].cost();

                    if cost < best_cost {
                        best_cost = cost;
                        best_cycle = Cycle {
                            parents,
                            member: to,
                        };
                    }
                } else {
                    stack.push((cost, parents, to, visited));
                }
            }
        }
    }

    Some(TspCycle {
        cost: best_cost,
        cycle: best_cycle,
    })
}

// test algorithms::branch_bound::test::branch_bound_rec_k_10e_adj_list     ... bench:   2,234,155.80 ns/iter (+/- 24,961.04)

pub fn branch_bound_rec<C, G>(graph: &G) -> Option<TspCycle<C, G>>
where
    C: Debug + Copy + Default + PartialOrd + AddAssign<C> + Add<C, Output = C> + TotalOrd + Bounded,
    G: NodeIterAdjacent + EdgeIterAdjacent + NodeAttribute + NodeIter + NodeCount + IndexEdge,
    G::EdgeWeight: Cost<C>,
{
    let mut best_cost = nearest_neighbor(graph)
        .map(|result| result.cost)
        .unwrap_or(Bounded::MAX);

    let start = graph.node_ids().next()?;

    let mut parents = Parents::new(graph);
    let mut visited = graph.visit_node_map();
    visited.visit(start);

    _branch_bound_rec(
        graph,
        start,
        &mut best_cost,
        C::default(),
        &mut parents,
        start,
        &mut visited,
    );

    Some(TspCycle {
        cost: best_cost,
        cycle: Cycle {
            member: start,
            parents,
        },
    })
}

pub(crate) fn _branch_bound_rec<C, G>(
    graph: &G,
    start: NodeId<G::Key>,
    best_cost: &mut C,
    mut cost: C,
    parents: &mut Parents<G>,
    from: NodeId<G::Key>,
    visited: &mut G::FixedNodeMap<bool>,
) where
    C: Debug + Copy + Default + PartialOrd + AddAssign<C> + Add<C, Output = C> + TotalOrd + Bounded,
    G: EdgeIterAdjacent + NodeAttribute + NodeCount + IndexEdge,
    G::EdgeWeight: Cost<C>,
{
    if visited.all_visited() {
        cost += *graph[EdgeId::new_unchecked(from, start)].cost();

        if cost < *best_cost {
            *best_cost = cost;
        }
    }

    for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(from) {
        let to = edge_id.to();
        let cost = cost + *weight.cost();

        if !visited.is_visited(to) && cost < *best_cost {
            visited.visit(to);
            parents.insert(from, to);

            _branch_bound_rec(graph, start, best_cost, cost, parents, to, visited);

            visited.unvisit(to);
            parents.remove_parent(to);
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::{branch_bound, branch_bound_rec};
    use crate::test::undigraph;
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn branch_bound_k_10_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = branch_bound(&graph).unwrap().cost as f32;
            assert_eq!(total, 38.41);
        })
    }

    #[bench]
    fn branch_bound_k_10e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = branch_bound(&graph).unwrap().cost as f32;
            assert_eq!(total, 27.26);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn branch_bound_k_12_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = branch_bound(&graph).unwrap().cost as f32;
            assert_eq!(total, 45.19);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn branch_bound_k_12e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = branch_bound(&graph).unwrap().cost as f32;
            assert_eq!(total, 36.13);
        })
    }

    #[bench]
    fn branch_bound_rec_k_10_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = branch_bound_rec(&graph).unwrap().cost as f32;
            assert_eq!(total, 38.41);
        })
    }

    #[bench]
    fn branch_bound_rec_k_10e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = branch_bound_rec(&graph).unwrap().cost as f32;
            assert_eq!(total, 27.26);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn branch_bound_rec_k_12_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = branch_bound_rec(&graph).unwrap().cost as f32;
            assert_eq!(total, 45.19);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn branch_bound_rec_k_12e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = branch_bound_rec(&graph).unwrap().cost as f32;
            assert_eq!(total, 36.13);
        })
    }
}
