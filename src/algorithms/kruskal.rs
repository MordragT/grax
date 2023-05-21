use crate::adjacency_list::AdjacencyOptions;
use crate::graph::{GraphAccess, GraphTopology, Sortable};
use crate::prelude::AdjacencyList;
use crate::{edge::EdgeRef, tree::UnionFind};
use std::ops::AddAssign;

use super::MinimumSpanningTree;

pub fn kruskal_weight<N, W, G>(graph: &G) -> W
where
    W: Default + Sortable + AddAssign + Copy,
    G: GraphTopology<N, W>,
{
    let mut total_weight = W::default();
    _kruskal(graph, |edge| total_weight += *edge.weight);
    total_weight
}

pub fn kruskal_mst<N, W, G>(graph: &G) -> MinimumSpanningTree<&N, W>
where
    W: Default + Sortable + AddAssign + Copy,
    G: GraphTopology<N, W>,
{
    let mut mst = AdjacencyList::with(AdjacencyOptions {
        nodes: Some(graph.nodes().collect()),
    });

    let union_find = _kruskal(graph, |edge| {
        mst.add_edge(edge.from, edge.to, *edge.weight).unwrap();
        mst.add_edge(edge.to, edge.from, *edge.weight).unwrap();
    });
    let root = union_find.root();

    MinimumSpanningTree::new(mst, root)
}

pub(crate) fn _kruskal<N, W, G, F>(graph: &G, mut f: F) -> UnionFind
where
    W: Sortable,
    G: GraphTopology<N, W>,
    F: FnMut(EdgeRef<W>),
{
    let mut priority_queue = graph.edges().collect::<Vec<_>>();
    priority_queue.sort_by(|this, other| this.weight.sort(other.weight));

    let mut union_find = UnionFind::from(graph.indices());

    for edge in priority_queue {
        if union_find.find(edge.from) == union_find.find(edge.to) {
            continue;
        }
        union_find.union(edge.from, edge.to);
        f(edge);
    }

    union_find
}

#[cfg(test)]
mod test {
    extern crate test;
    use crate::{adjacency_matrix::AdjacencyMatrix, prelude::*, test::undigraph};
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

    #[bench]
    fn kruskal_graph_10_200_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = undigraph("data/G_10_200.txt").unwrap();

        b.iter(|| {
            let count = graph.kruskal_weight() as f32;
            assert_eq!(count, 372.14417);
        })
    }

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
}
