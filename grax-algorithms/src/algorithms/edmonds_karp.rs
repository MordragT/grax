use crate::weight::TotalOrd;

use super::{insert_residual_edges, Bfs, _ford_fulkerson, remove_residual_edges};

use grax_core::{
    collections::{EdgeIter, GetEdge, IndexEdge, IndexEdgeMut, InsertEdge, RemoveEdge},
    edge::weight::*,
    graph::{EdgeAttribute, EdgeIterAdjacent, NodeAttribute},
    prelude::*,
};
use std::{
    fmt::Debug,
    ops::{AddAssign, Sub, SubAssign},
};

pub fn edmonds_karp<C, G>(graph: &mut G, source: NodeId<G::Key>, sink: NodeId<G::Key>) -> C
where
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C> + Debug + TotalOrd,
    G: IndexEdge
        + IndexEdgeMut
        + EdgeAttribute
        + NodeAttribute
        + EdgeIterAdjacent
        + InsertEdge
        + RemoveEdge
        + GetEdge
        + EdgeIter,
    G::EdgeWeight: Flow<C> + Capacity<C> + Reverse,
{
    insert_residual_edges(graph);

    let total_flow = _ford_fulkerson(graph, source, sink, Bfs);

    remove_residual_edges(graph);

    total_flow
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::edmonds_karp;
    use crate::algorithms::{empty_flow, flow_adaptor};
    use crate::test::{digraph, id};
    use grax_impl::*;
    use test::Bencher;

    // adj

    #[bench]
    fn edmonds_karp_g_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();
        let mut graph: AdjGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();
        let mut graph: AdjGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();
        let mut graph: AdjGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0);
            empty_flow(&mut graph);
        })
    }
    // dense

    #[bench]
    fn edmonds_karp_g_1_2_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();
        let mut graph: MatGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn edmonds_karp_fluss_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();
        let mut graph: MatGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();
        let mut graph: MatGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0);
            empty_flow(&mut graph);
        })
    }

    // csr

    #[bench]
    fn edmonds_karp_g_1_2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();
        let mut graph: CsrGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn edmonds_karp_fluss_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();
        let mut graph: CsrGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();
        let mut graph: CsrGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = edmonds_karp(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0);
            empty_flow(&mut graph);
        })
    }
}
