use grax_core::collections::{EdgeIter, NodeIter};
use grax_core::edge::*;
use grax_core::graph::{Cost, EdgeAttribute, NodeAttribute};
use grax_core::index::NodeId;
use grax_core::view::{FilterEdgeView, UnionFind};
use grax_core::weight::Sortable;

use std::ops::AddAssign;

pub struct KruskalResult<G: EdgeAttribute, C> {
    pub root: NodeId<G::Key>,
    pub view: FilterEdgeView<G>,
    pub cost: C,
}

pub fn kruskal<C, G>(graph: &G) -> KruskalResult<G, C>
where
    C: Sortable + Default + AddAssign + Copy,
    G: NodeIter + EdgeIter + Cost<C> + EdgeAttribute + NodeAttribute,
{
    let mut priority_queue = graph.iter_edges().collect::<Vec<_>>();
    priority_queue.sort_unstable_by(|this, other| this.weight.cost().sort(&other.weight.cost()));

    let mut union_find = UnionFind::new(graph);

    let mut view = FilterEdgeView::new(graph);
    let mut root = graph.node_ids().next().unwrap();
    let mut total_cost = C::default();

    for EdgeRef { edge_id, weight } in priority_queue {
        let from = edge_id.from();
        let to = edge_id.to();

        if union_find.find(from) == union_find.find(to) {
            continue;
        }

        root = union_find.union(from, to);
        view.keep(edge_id);
        total_cost += *weight.cost();
    }

    KruskalResult {
        root,
        view,
        cost: total_cost,
    }
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
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn kruskal_graph_1_200_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[bench]
    fn kruskal_graph_10_200_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_100_200_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    // dense

    #[bench]
    fn kruskal_graph_1_2_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn kruskal_graph_1_200_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_10_200_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_100_200_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    // csr

    #[bench]
    fn kruskal_graph_1_2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn kruskal_graph_1_200_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[bench]
    fn kruskal_graph_10_200_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_100_200_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    // hash

    #[bench]
    fn kruskal_graph_1_2_hash_graph(b: &mut Bencher) {
        let graph: HashGraph<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20_hash_graph(b: &mut Bencher) {
        let graph: HashGraph<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn kruskal_graph_1_200_hash_graph(b: &mut Bencher) {
        let graph: HashGraph<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20_hash_graph(b: &mut Bencher) {
        let graph: HashGraph<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[bench]
    fn kruskal_graph_10_200_hash_graph(b: &mut Bencher) {
        let graph: HashGraph<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_100_200_hash_graph(b: &mut Bencher) {
        let graph: HashGraph<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).cost as f32;
            assert_eq!(count, 27550.51488);
        })
    }
}
