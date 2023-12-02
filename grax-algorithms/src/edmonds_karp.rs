use std::{
    collections::HashSet,
    fmt::Debug,
    ops::{AddAssign, Neg, Sub, SubAssign},
};

use grax_core::{
    adaptor::flow::FlowBundle, edge::*, prelude::*, traits::*, view::Parents, weight::Maximum,
};

use super::{_ford_fulkerson, bfs_sp};

pub fn edmonds_karp_adaptor<G1, G2, W1, C>(graph: G1) -> G2
where
    C: Default + Copy + Neg<Output = C>,
    W1: Clone + Maximum + Default,
    G1: Base<EdgeWeight = W1> + AdaptEdge<G2, FlowBundle<W1, C>> + Index + Cost<C>,
    G2: Base<EdgeWeight = FlowBundle<W1, C>>,
{
    let edge_ids = graph.edge_ids().collect::<HashSet<_>>();

    graph.split_map_edge(|edge| {
        let Edge { edge_id, weight } = edge;

        let cost = *weight.cost();

        let bundle = FlowBundle {
            cost: cost,
            weight: weight.clone(),
            capacity: cost,
            flow: C::default(),
            reverse: false,
        };

        let mut edges = vec![Edge::new(edge_id, bundle)];

        if !edge_ids.contains(&edge_id.rev()) {
            let bundle = FlowBundle {
                weight: weight.clone(),
                capacity: cost,
                cost: -cost,
                flow: cost,
                reverse: true,
            };

            edges.push(Edge::new(edge_id.rev(), bundle));
        }

        edges
    })
}

pub fn edmonds_karp<C, G>(graph: &mut G, source: NodeId<G::Id>, sink: NodeId<G::Id>) -> C
where
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C> + Debug,
    G: Count
        + IndexAdjacent
        + IterAdjacent
        + Get
        + GetMut
        + Viewable
        + Debug
        + Flow<C>
        + Cost<C>
        + Visitable,
{
    fn shortest_path<C, G>(
        graph: &G,
        source: NodeId<G::Id>,
        sink: NodeId<G::Id>,
    ) -> Option<Parents<G>>
    where
        C: Default + Sub<C, Output = C> + PartialOrd + Copy + Debug,
        G: IterAdjacent + IndexAdjacent + Count + Get + Viewable + Flow<C> + Cost<C> + Visitable,
    {
        bfs_sp(graph, source, sink, |weight| {
            (*weight.capacity() - *weight.flow()) > C::default()
        })
    }

    _ford_fulkerson(graph, source, sink, shortest_path)
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::{edmonds_karp, edmonds_karp_adaptor};
    use crate::test::{digraph, id};
    use grax_impl::*;
    use test::Bencher;

    // adj

    #[bench]
    fn edmonds_karp_g_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let mut graph = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();

        b.iter(|| {
            let mut graph = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();

        b.iter(|| {
            let mut graph = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }

    // sparse

    #[bench]
    fn edmonds_karp_g_1_2_sparse_mat(b: &mut Bencher) {
        let graph: SparseMatGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let mut graph: SparseMatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_sparse_mat(b: &mut Bencher) {
        let graph: SparseMatGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();

        b.iter(|| {
            let mut graph: SparseMatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_sparse_mat(b: &mut Bencher) {
        let graph: SparseMatGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();

        b.iter(|| {
            let mut graph: SparseMatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }

    // dense

    #[bench]
    fn edmonds_karp_g_1_2_dense_mat(b: &mut Bencher) {
        let graph: DenseMatGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let mut graph: DenseMatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_dense_mat(b: &mut Bencher) {
        let graph: DenseMatGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();

        b.iter(|| {
            let mut graph: DenseMatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_dense_mat(b: &mut Bencher) {
        let graph: DenseMatGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();

        b.iter(|| {
            let mut graph: DenseMatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }

    // csr

    #[bench]
    fn edmonds_karp_g_1_2_csr_mat(b: &mut Bencher) {
        let graph: CsrMatGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let mut graph: CsrMatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_csr_mat(b: &mut Bencher) {
        let graph: CsrMatGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();

        b.iter(|| {
            let mut graph: CsrMatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_csr_mat(b: &mut Bencher) {
        let graph: CsrMatGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();

        b.iter(|| {
            let mut graph: CsrMatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }

    // sparse to dense

    #[bench]
    fn edmonds_karp_g_1_2_sparse_to_dense_mat(b: &mut Bencher) {
        let graph: SparseMatGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let mut graph: DenseMatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    // dense to sparse

    #[bench]
    fn edmonds_karp_g_1_2_dense_to_sparse_mat(b: &mut Bencher) {
        let graph: DenseMatGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let mut graph: SparseMatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }
}
