use crate::category::{MinimumSpanningTree, Mst};
use grax_core::collections::FixedNodeMap;
use grax_core::collections::NodeCount;
use grax_core::collections::NodeIter;
use grax_core::collections::RemoveEdge;
use grax_core::collections::RemoveNode;
use grax_core::collections::VisitNodeMap;
use grax_core::edge::*;
use grax_core::graph::EdgeAttribute;
use grax_core::graph::EdgeIterAdjacent;
use grax_core::graph::NodeAttribute;
use grax_core::node::NodeRef;
use grax_core::weight::Maximum;
use grax_core::weight::Sortable;
use orx_priority_queue::DaryHeap;
use orx_priority_queue::PriorityQueue;

use std::fmt::Debug;
use std::ops::AddAssign;

pub struct Prim;

impl<C, G> MinimumSpanningTree<C, G> for Prim
where
    C: Default + Sortable + AddAssign + Copy + Debug + Maximum,
    G: NodeCount
        + NodeIter
        + EdgeIterAdjacent
        + NodeAttribute
        + EdgeAttribute
        + RemoveEdge
        + RemoveNode,
    G::EdgeWeight: EdgeCost<Cost = C>,
    G::FixedNodeMap<bool>: 'static,
{
    fn minimum_spanning_tree(graph: &G) -> Option<Mst<C, G>> {
        prim(graph)
    }
}

pub fn prim<C, G>(graph: &G) -> Option<Mst<C, G>>
where
    C: Default + Sortable + AddAssign + Copy + Debug + Maximum,
    G: NodeCount
        + NodeIter
        + EdgeIterAdjacent
        + NodeAttribute
        + EdgeAttribute
        + RemoveEdge
        + RemoveNode,
    G::EdgeWeight: EdgeCost<Cost = C>,
    G::FixedNodeMap<bool>: 'static,
{
    let root = graph.node_ids().next()?;

    let mut visit = graph.visit_node_map();
    let mut priority_queue = DaryHeap::<_, _, 4>::with_capacity(graph.node_count() / 2);
    priority_queue.push(root, C::default());

    // einfach mit W::max init
    let mut costs = graph.fixed_node_map(C::MAX);
    let mut total_cost = C::default();

    while let Some((from, cost)) = priority_queue.pop() {
        if visit.is_visited(from) {
            continue;
        }
        visit.visit(from);
        total_cost += cost;

        for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(from) {
            let to = edge_id.to();
            if !visit.is_visited(to) {
                let edge_cost = *weight.cost();
                let cost = costs.get_mut(to);
                if *cost > edge_cost {
                    *cost = edge_cost;
                    priority_queue.push(to, edge_cost);
                }
            }
        }
    }

    Some(Mst {
        root,
        filter: Box::new(move |graph| {
            for NodeRef { node_id, weight } in visit.iter_nodes() {
                if !weight {
                    graph.remove_node(node_id);
                }
            }
        }),
        total_cost,
    })
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::prim;
    use crate::test::undigraph;
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn prim_graph_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn prim_graph_1_20_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn prim_graph_1_200_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn prim_graph_10_20_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[bench]
    fn prim_graph_10_200_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[bench]
    fn prim_graph_100_200_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    // dense

    #[bench]
    fn prim_graph_1_2_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn prim_graph_1_20_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn prim_graph_1_200_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn prim_graph_10_20_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn prim_graph_10_200_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn prim_graph_100_200_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    // csr

    #[bench]
    fn prim_graph_1_2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn prim_graph_1_20_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn prim_graph_1_200_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn prim_graph_10_20_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[bench]
    fn prim_graph_10_200_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[bench]
    fn prim_graph_100_200_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 27550.51488);
        })
    }
}