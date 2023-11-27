use grax_core::adaptor::partial::PartialGraphAdaptor;
use grax_core::edge::*;
use grax_core::traits::*;
use grax_core::variant::connected::ConnectedGraph;

use std::ops::AddAssign;

pub fn kruskal<C, G>(graph: &G) -> (ConnectedGraph<PartialGraphAdaptor<G>>, C)
where
    C: Sortable + Default + AddAssign + Copy,
    G: Index + Cost<C> + Viewable + Create,
{
    let mut priority_queue = graph
        .edge_ids()
        .map(|edge_id| {
            let cost = *graph.cost(edge_id).unwrap().cost();
            (edge_id, cost)
        })
        .collect::<Vec<_>>();

    priority_queue.sort_by(|this, other| this.1.sort(&other.1));

    let mut union_find = graph.union_find();
    let mut tree = PartialGraphAdaptor::new(graph);
    let mut root = graph.node_ids().next().unwrap();
    let mut total_cost = C::default();

    for (edge_id, cost) in priority_queue {
        let from = edge_id.from();
        let to = edge_id.to();

        if union_find.find(from) == union_find.find(to) {
            continue;
        }

        root = union_find.union(from, to);
        tree.keep_edge_id(edge_id);
        total_cost += cost;
    }

    let tree = ConnectedGraph::from_unchecked(tree, root);

    (tree, total_cost)
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
        let graph: AdjacencyList<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).1 as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).1 as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn kruskal_graph_1_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).1 as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).1 as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_10_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).1 as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_100_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).1 as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    #[bench]
    fn kruskal_graph_1_2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).1 as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).1 as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_1_200_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).1 as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).1 as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_10_200_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).1 as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn kruskal_graph_100_200_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = undigraph("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            let count = kruskal(&graph).1 as f32;
            assert_eq!(count, 27550.51488);
        })
    }
}
