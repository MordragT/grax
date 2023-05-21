use crate::prelude::{GraphTopology, NodeIndex};

pub fn edmonds_karp<N, W, G>(graph: &G, from: NodeIndex, to: NodeIndex) -> W
where
    W: Default + Clone,
    G: GraphTopology<N, W>,
{
    let mut flow = vec![W::default(); graph.node_count()];

    todo!()
}

#[cfg(test)]
mod test {
    extern crate test;

    use crate::{adjacency_matrix::AdjacencyMatrix, prelude::*, test::digraph};
    use test::Bencher;

    #[bench]
    fn edmonds_karp_g_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 5.0)
        })
    }

    #[bench]
    fn edmonds_karp_g_1_2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 5.0)
        })
    }
}
