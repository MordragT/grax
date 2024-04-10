use crate::category::{MinimumSpanningTree, Mst};
use crate::UnionFind;
use grax_core::collections::{
    EdgeCollection, EdgeIter, NodeIter, RemoveEdge, RemoveNode, VisitEdgeMap,
};
use grax_core::edge::*;
use grax_core::graph::{EdgeAttribute, NodeAttribute};
use grax_core::weight::Sortable;
use rayon::slice::ParallelSliceMut;

use std::fmt::Debug;
use std::ops::AddAssign;

pub struct Kruskal;

impl<C, G> MinimumSpanningTree<C, G> for Kruskal
where
    C: Sortable + Default + AddAssign + Copy + Debug,
    G: NodeIter
        + EdgeIter
        + EdgeAttribute
        + NodeAttribute
        + EdgeCollection<EdgeWeight: Send + Sync>
        + RemoveNode
        + RemoveEdge,
    G::EdgeWeight: EdgeCost<Cost = C>,
    G::FixedEdgeMap<bool>: 'static,
{
    fn minimum_spanning_tree(graph: &G) -> Option<Mst<C, G>> {
        kruskal(graph)
    }
}

pub fn kruskal<C, G>(graph: &G) -> Option<Mst<C, G>>
where
    C: Sortable + Default + AddAssign + Copy + Debug,
    G: NodeIter
        + EdgeIter
        + EdgeAttribute
        + NodeAttribute
        + EdgeCollection<EdgeWeight: Send + Sync>
        + RemoveNode
        + RemoveEdge,
    G::EdgeWeight: EdgeCost<Cost = C>,
    G::FixedEdgeMap<bool>: 'static,
{
    let mut root = graph.node_ids().next()?;

    // cloning edges seems to be faster than borrowed
    let mut priority_queue = graph
        .iter_edges()
        .map(|edge| edge.to_owned())
        .collect::<Vec<_>>();
    priority_queue.par_sort_unstable_by(|a, b| a.weight.cost().sort(&b.weight.cost()));

    let mut union_find = UnionFind::new(graph);

    let mut edges = graph.visit_edge_map();
    let mut total_cost = C::default();

    for Edge { edge_id, weight } in priority_queue {
        let from = edge_id.from();
        let to = edge_id.to();

        if union_find.find(from) == union_find.find(to) {
            edges.visit(edge_id);
            continue;
        }

        root = union_find.union(from, to);
        edges.visit(edge_id);
        total_cost += *weight.cost();
    }

    Some(Mst {
        root,
        filter: Box::new(move |graph| {
            for EdgeRef { edge_id, weight } in edges.iter_edges() {
                if !weight {
                    graph.remove_edge(edge_id);
                }
            }
        }),
        total_cost,
    })
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::kruskal;
    use crate::test::undigraph;
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn kruskal_graph_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn kruskal_graph_1_200_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[bench]
    fn kruskal_graph_10_200_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_100_200_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    // dense

    #[bench]
    fn kruskal_graph_1_2_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn kruskal_graph_1_200_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_10_200_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_100_200_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    // csr

    #[bench]
    fn kruskal_graph_1_2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn kruskal_graph_1_200_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[bench]
    fn kruskal_graph_10_200_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_100_200_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    // hash

    #[bench]
    fn kruskal_graph_1_2_hash_graph(b: &mut Bencher) {
        let graph: HashGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20_hash_graph(b: &mut Bencher) {
        let graph: HashGraph<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn kruskal_graph_1_200_hash_graph(b: &mut Bencher) {
        let graph: HashGraph<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20_hash_graph(b: &mut Bencher) {
        let graph: HashGraph<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[bench]
    fn kruskal_graph_10_200_hash_graph(b: &mut Bencher) {
        let graph: HashGraph<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_100_200_hash_graph(b: &mut Bencher) {
        let graph: HashGraph<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).unwrap().total_cost as f32;
            assert_eq!(count, 27550.51488);
        })
    }
}
