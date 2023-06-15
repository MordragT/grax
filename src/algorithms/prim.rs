use crate::{
    graph::{Count, Index, IndexAdjacent, IterAdjacent, Sortable, WeightCost},
    prelude::NodeIdentifier,
};
use priq::PriorityQueue;
use std::ops::AddAssign;

pub fn prim<N, W, C, G>(graph: &G) -> C
where
    C: Default + Sortable + AddAssign + Copy,
    W: WeightCost<Cost = C>,
    G: Index + Count + IndexAdjacent + IterAdjacent<N, W>,
{
    match graph.node_ids().next() {
        Some(start) => _prim(graph, start),
        None => C::default(),
    }
}

pub(crate) fn _prim<N, W, C, G>(graph: &G, start: G::NodeId) -> C
where
    C: Default + Sortable + AddAssign + Copy,
    W: WeightCost<Cost = C>,
    G: IndexAdjacent + Count + IterAdjacent<N, W>,
{
    let n = graph.node_count();
    let mut visited = vec![false; n];
    let mut priority_queue = PriorityQueue::with_capacity(n);
    // einfach mit W::max init
    let mut costs = vec![None; n];
    let mut total_cost = C::default();

    priority_queue.put(C::default(), start);

    while let Some((cost, to)) = priority_queue.pop() {
        if visited[to.as_usize()] {
            continue;
        }
        visited[to.as_usize()] = true;
        total_cost += cost;

        for edge in graph.iter_adjacent_edges(to) {
            let to = edge.to();
            if !visited[to.as_usize()] {
                if let Some(cost) = &mut costs[to.as_usize()] {
                    if *cost > edge.weight.cost() {
                        *cost = edge.weight.cost();
                        priority_queue.put(*edge.weight.cost(), to);
                    }
                } else {
                    costs[to.as_usize()] = Some(edge.weight.cost());
                    priority_queue.put(*edge.weight.cost(), to);
                }
            }
        }
    }

    total_cost
}

#[cfg(test)]
mod test {
    extern crate test;
    use crate::{prelude::*, test::undigraph};
    use test::Bencher;

    #[bench]
    fn prim_graph_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = graph.prim() as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn prim_graph_1_20_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = graph.prim() as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn prim_graph_1_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = graph.prim() as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn prim_graph_10_20_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = graph.prim() as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[bench]
    fn prim_graph_10_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = graph.prim() as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[bench]
    fn prim_graph_100_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = graph.prim() as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    #[bench]
    fn prim_graph_1_2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = graph.prim() as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn prim_graph_1_20_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = graph.prim() as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn prim_graph_1_200_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = graph.prim() as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn prim_graph_10_20_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = graph.prim() as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    // #[bench]
    // fn prim_graph_10_200_adj_mat(b: &mut Bencher) {
    //     let graph: AdjacencyMatrix<_, _> = undigraph("data/G_10_200.txt").unwrap();

    //     b.iter(|| {
    //         let count = graph.prim() as f32;
    //         assert_eq!(count, 372.14417);
    //     })
    // }

    // #[bench]
    // fn prim_graph_100_200_adj_mat(b: &mut Bencher) {
    //     let graph: AdjacencyMatrix<_, _> = undigraph("data/G_100_200.txt").unwrap();

    //     b.iter(|| {
    //         let count = graph.prim() as f32;
    //         assert_eq!(count, 27550.51488);
    //     })
    // }
}
