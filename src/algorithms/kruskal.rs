use super::UnionFind;
use crate::graph::{Count, Get, Index, Iter, Sortable, WeightCost};
use crate::prelude::{Tree, TreeBuilder};
use std::ops::AddAssign;

pub fn kruskal<N, W, C, G>(graph: &G) -> (Tree<G>, C)
where
    C: Sortable + Default + AddAssign + Copy,
    W: WeightCost<Cost = C>,
    G: Iter<N, W> + Index + Count + Get<N, W>,
{
    let mut priority_queue = graph.iter_edges().collect::<Vec<_>>();
    priority_queue.sort_by(|this, other| this.weight.cost().sort(other.weight.cost()));

    let count = graph.node_count();
    let mut tree = TreeBuilder::with_count(count);
    let mut union_find = UnionFind::<G>::with_node_ids(graph.node_ids());

    let mut root = graph.node_ids().next().unwrap();
    let mut total_cost = C::default();

    for edge in priority_queue {
        let from = edge.from();
        let to = edge.to();

        if union_find.find(from) == union_find.find(to) {
            continue;
        }

        root = union_find.union(from, to);
        tree.insert(from, to);
        total_cost += *edge.weight.cost();
    }

    (tree.build(root, graph), total_cost)
}

#[cfg(test)]
mod test {
    extern crate test;
    use crate::{prelude::*, test::undigraph};
    use test::Bencher;

    #[bench]
    fn kruskal_graph_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn kruskal_graph_1_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_10_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_100_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    #[bench]
    fn kruskal_graph_1_2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn kruskal_graph_1_200_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_10_200_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_100_200_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 27550.51488);
        })
    }
}
