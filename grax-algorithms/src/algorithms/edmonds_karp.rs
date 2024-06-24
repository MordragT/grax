use crate::weight::TotalOrd;

use super::{ford_fulkerson, Bfs};

use grax_core::{
    collections::{EdgeCollection, EdgeIter, IndexEdge, IndexEdgeMut},
    edge::{weight::*, *},
    graph::{AdaptEdges, EdgeAttribute, EdgeIterAdjacent, NodeAttribute},
    prelude::*,
};
use std::{
    collections::HashSet,
    fmt::Debug,
    ops::{AddAssign, Neg, Sub, SubAssign},
};

pub fn edmonds_karp_adaptor<G1, G2, C>(graph: G1) -> G2
where
    C: Default + Copy + Neg<Output = C>,
    G1: EdgeCollection<EdgeWeight = C> + AdaptEdges<G2, FlowBundle<C>> + EdgeIter,
    G1::EdgeWeight: Cost<C>,
    G2: EdgeCollection<EdgeWeight = FlowBundle<C>>,
{
    let edge_ids = graph.edge_ids().collect::<HashSet<_>>();

    graph.adapt_edges(|edges| {
        let mut adapted = Vec::new();

        for Edge { edge_id, weight } in edges {
            let cost = *weight.cost();

            let bundle = FlowBundle {
                capacity: cost,
                flow: C::default(),
                reverse: false,
            };

            adapted.push(Edge::new(edge_id, bundle));

            if !edge_ids.contains(&edge_id.rev()) {
                let bundle = FlowBundle {
                    capacity: cost,
                    flow: cost,
                    reverse: true,
                };

                adapted.push(Edge::new(edge_id.rev(), bundle));
            }
        }

        adapted.into_iter()
    })
}

pub fn edmonds_karp<C, G>(graph: &mut G, source: NodeId<G::Key>, sink: NodeId<G::Key>) -> C
where
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C> + Debug + TotalOrd,
    G: IndexEdge + IndexEdgeMut + EdgeAttribute + NodeAttribute + EdgeIterAdjacent,
    G::EdgeWeight: Flow<C> + Capacity<C>,
{
    ford_fulkerson(graph, source, sink, Bfs)
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
            let mut graph: AdjGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();

        b.iter(|| {
            let mut graph: AdjGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();

        b.iter(|| {
            let mut graph: AdjGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }
    // dense

    #[bench]
    fn edmonds_karp_g_1_2_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let mut graph: MatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();

        b.iter(|| {
            let mut graph: MatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();

        b.iter(|| {
            let mut graph: MatGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }

    // csr

    #[bench]
    fn edmonds_karp_g_1_2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let mut graph: CsrGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();

        b.iter(|| {
            let mut graph: CsrGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();

        b.iter(|| {
            let mut graph: CsrGraph<_, _, true> = edmonds_karp_adaptor(graph.clone());
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }
}
