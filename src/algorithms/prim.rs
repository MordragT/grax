use crate::graph::{GraphAdjacentTopology, GraphTopology, Sortable};
use crate::indices::NodeIndex;
use priq::PriorityQueue;
use std::ops::AddAssign;

pub fn prim<N, W, G>(graph: &G) -> W
where
    W: Default + Sortable + AddAssign + Copy,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    match graph.indices().next() {
        Some(start) => _prim(graph, start),
        None => W::default(),
    }
}

pub(crate) fn _prim<N, W, G>(graph: &G, start: NodeIndex) -> W
where
    W: Default + Sortable + AddAssign + Copy,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let n = graph.node_count();
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

        for edge in graph.adjacent_edges(to) {
            if !visited[edge.to.0] {
                if let Some(weight) = &mut weights[edge.to.0] {
                    if *weight > edge.weight {
                        *weight = edge.weight;
                        priority_queue.put(*edge.weight, edge.to);
                    }
                } else {
                    weights[edge.to.0] = Some(edge.weight);
                    priority_queue.put(*edge.weight, edge.to);
                }
            }
        }
    }

    total_weight
}

#[cfg(test)]
mod test {
    extern crate test;
    use crate::{adjacency_matrix::AdjacencyMatrix, prelude::*, test::undigraph};
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

    #[bench]
    fn prim_graph_10_200_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = graph.prim() as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[bench]
    fn prim_graph_100_200_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = graph.prim() as f32;
            assert_eq!(count, 27550.51488);
        })
    }
}
