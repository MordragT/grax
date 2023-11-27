use grax_core::edge::*;
use grax_core::prelude::*;
use grax_core::traits::*;
use grax_core::view::AttrMap;
use grax_core::view::VisitMap;

use priq::PriorityQueue;
use std::fmt::Debug;
use std::ops::AddAssign;

pub fn prim<C, G>(graph: &G) -> C
where
    C: Default + Sortable + AddAssign + Copy + Debug,
    G: Index + IndexAdjacent + Cost<C> + Visitable + Count + Viewable,
{
    match graph.node_ids().next() {
        Some(start) => _prim(graph, start),
        None => C::default(),
    }
}

pub(crate) fn _prim<C, G>(graph: &G, start: NodeId<G::Id>) -> C
where
    C: Default + Sortable + AddAssign + Copy + Debug,
    G: IndexAdjacent + Cost<C> + Visitable + Count + Viewable,
{
    let mut visit = graph.visit_map();
    let mut priority_queue = PriorityQueue::with_capacity(graph.node_count());
    // einfach mit W::max init
    let mut costs = graph.node_map();
    let mut total_cost = C::default();

    priority_queue.put(C::default(), start);

    while let Some((cost, to)) = priority_queue.pop() {
        if visit.is_visited(to) {
            continue;
        }
        visit.visit(to);
        total_cost += cost;

        for edge_id in graph.adjacent_edge_ids(to) {
            let to = edge_id.to();
            if !visit.is_visited(to) {
                let edge_cost = *graph.cost(edge_id).unwrap().cost();

                if let Some(cost) = &mut costs.get_mut(to) {
                    if *cost > edge_cost {
                        *cost = edge_cost;
                        priority_queue.put(edge_cost, to);
                    }
                } else {
                    *costs.get_mut(to) = Some(edge_cost);
                    priority_queue.put(edge_cost, to);
                }
            }
        }
    }

    total_cost
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
        let graph: AdjacencyList<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph) as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn prim_graph_1_20_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph) as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn prim_graph_1_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph) as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn prim_graph_10_20_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph) as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn prim_graph_10_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph) as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn prim_graph_100_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph) as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    #[bench]
    fn prim_graph_1_2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph) as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn prim_graph_1_20_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph) as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn prim_graph_1_200_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph) as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn prim_graph_10_20_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph) as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn prim_graph_10_200_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph) as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn prim_graph_100_200_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = prim(&graph) as f32;
            assert_eq!(count, 27550.51488);
        })
    }
}
