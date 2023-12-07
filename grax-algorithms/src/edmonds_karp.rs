use std::{
    collections::HashSet,
    fmt::Debug,
    ops::{AddAssign, Neg, Sub, SubAssign},
};

use grax_core::{
    adaptor::flow::FlowBundle,
    collections::{EdgeCollection, EdgeIter, GetEdge, GetEdgeMut},
    edge::*,
    graph::{AdaptEdge, Cost, EdgeAttribute, EdgeIterAdjacent, Flow, NodeAttribute},
    prelude::*,
    view::Parents,
    weight::Maximum,
};

use super::{_ford_fulkerson, bfs_sp};

pub fn edmonds_karp_adaptor<G1, G2, W1, C>(graph: G1) -> G2
where
    C: Default + Copy + Neg<Output = C>,
    W1: Clone + Maximum + Default,
    G1: EdgeCollection<EdgeWeight = W1> + AdaptEdge<G2, FlowBundle<W1, C>> + EdgeIter + Cost<C>,
    G2: EdgeCollection<EdgeWeight = FlowBundle<W1, C>>,
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

pub fn edmonds_karp<C, G>(graph: &mut G, source: NodeId<G::Key>, sink: NodeId<G::Key>) -> C
where
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C> + Debug,
    G: GetEdge + GetEdgeMut + EdgeAttribute + NodeAttribute + EdgeIterAdjacent + Flow<C> + Cost<C>,
{
    fn shortest_path<C, G>(
        graph: &G,
        source: NodeId<G::Key>,
        sink: NodeId<G::Key>,
    ) -> Option<Parents<G>>
    where
        C: Default + Sub<C, Output = C> + PartialOrd + Copy + Debug,
        G: EdgeAttribute + NodeAttribute + EdgeIterAdjacent + Flow<C> + Cost<C>,
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
