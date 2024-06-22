use crate::util::Parents;
use crate::util::Path;

use grax_core::collections::GetEdge;
use grax_core::collections::NodeIter;
use grax_core::collections::VisitNodeMap;
use grax_core::edge::*;
use grax_core::graph::EdgeIterAdjacent;
use grax_core::graph::NodeAttribute;
use grax_core::prelude::*;
use grax_core::weight::Maximum;
use grax_core::weight::Sortable;

use std::fmt::Debug;
use std::ops::{Add, AddAssign};

pub fn nearest_neighbor_from_first<C, G>(graph: &G) -> Option<(C, Path<G>)>
where
    C: Default + Copy + AddAssign + Add<C, Output = C> + Maximum + Sortable + Debug,
    G: NodeIter + EdgeIterAdjacent + NodeAttribute + GetEdge,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    match graph.node_ids().next() {
        Some(start) => nearest_neighbor(graph, start),
        None => None,
    }
}

pub fn nearest_neighbor<C, G>(graph: &G, start: NodeId<G::Key>) -> Option<(C, Path<G>)>
where
    C: Default + Copy + AddAssign + Add<C, Output = C> + Maximum + Sortable + Debug,
    G: EdgeIterAdjacent + NodeAttribute + GetEdge,
    G::EdgeWeight: EdgeCost<Cost = C>,
{
    let mut cost = C::default();
    let mut parents = Parents::new(graph);
    let mut visited = graph.visit_node_map();
    let mut stack = Vec::new();
    stack.push(start);

    while let Some(from) = stack.pop() {
        visited.visit(from);

        if let Some(EdgeRef { edge_id, weight }) = graph
            .iter_adjacent_edges(from)
            .filter(|edge| !visited.is_visited(edge.edge_id.to()))
            .min_by(|a, b| a.weight.cost().sort(b.weight.cost()))
        {
            let to = edge_id.to();
            parents.insert(from, to);
            stack.push(to);
            cost += *weight.cost();
        } else {
            // no more unvisited neighbors
            // compute cost from last node to start node

            parents.insert(from, start);
            let edge_id = EdgeId::new_unchecked(from, start);
            cost += *graph.edge(edge_id).unwrap().weight.cost();
            break;
        }
    }

    assert!(visited.all_visited());

    Some((cost, Path { parents }))
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::nearest_neighbor_from_first;
    use crate::test::undigraph;
    use grax_impl::*;
    use more_asserts::*;
    use test::Bencher;

    #[bench]
    fn nearest_neighbor_k_10_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10.txt").unwrap();

        b.iter(|| {
            let total = nearest_neighbor_from_first(&graph).unwrap().0;
            assert_le!(total, 38.41 * 1.2);
            assert_ge!(total, 38.41)
        })
    }

    #[bench]
    fn nearest_neighbor_k_10e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_10e.txt").unwrap();

        b.iter(|| {
            let total = nearest_neighbor_from_first(&graph).unwrap().0;
            assert_le!(total, 27.26 * 1.2);
            assert_ge!(total, 27.26)
        })
    }

    #[bench]
    fn nearest_neighbor_k_12_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12.txt").unwrap();

        b.iter(|| {
            let total = nearest_neighbor_from_first(&graph).unwrap().0;
            assert_le!(total, 45.19 * 1.2);
            assert_ge!(total, 45.19)
        })
    }

    #[bench]
    fn nearest_neighbor_k_12e_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/K_12e.txt").unwrap();

        b.iter(|| {
            let total = nearest_neighbor_from_first(&graph).unwrap().0;
            assert_le!(total, 36.13 * 1.2);
            assert_ge!(total, 36.13)
        })
    }
}
