use super::{insert_residual_edges, remove_residual_edges, Dfs, PathFinder};
use crate::weight::TotalOrd;

use grax_core::collections::{EdgeIter, GetEdge, IndexEdge, IndexEdgeMut, InsertEdge, RemoveEdge};
use grax_core::edge::{weight::*, *};
use grax_core::graph::{EdgeIterAdjacent, NodeAttribute};
use grax_core::prelude::*;

use std::{
    fmt::Debug,
    ops::{AddAssign, Sub, SubAssign},
};

pub fn ford_fulkerson<C, G>(graph: &mut G, source: NodeId<G::Key>, sink: NodeId<G::Key>) -> C
where
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C> + Debug + TotalOrd,
    G: IndexEdge
        + IndexEdgeMut
        + Debug
        + NodeAttribute
        + InsertEdge
        + RemoveEdge
        + GetEdge
        + EdgeIter
        + EdgeIterAdjacent,
    G::EdgeWeight: Flow<C> + Capacity<C> + Reverse,
{
    insert_residual_edges(graph);

    let total_flow = _ford_fulkerson(graph, source, sink, Dfs);

    remove_residual_edges(graph);

    total_flow
}

pub(crate) fn _ford_fulkerson<C, G>(
    graph: &mut G,
    source: NodeId<G::Key>,
    sink: NodeId<G::Key>,
    path_finder: impl PathFinder<G>,
) -> C
where
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C> + Debug + TotalOrd,
    G: IndexEdge + IndexEdgeMut + Debug + NodeAttribute,
    G::EdgeWeight: Flow<C> + Capacity<C>,
{
    let mut total_flow = C::default();

    let filter = |edge: EdgeRef<_, G::EdgeWeight>| edge.weight.residual_capacity() > C::default();

    // loop while path_finder finds a path
    while let Some(path) = path_finder.path_where(graph, source, sink, filter) {
        let parents = path.parents;

        if parents.is_empty() {
            break;
        }

        let bottleneck = parents
            .iter_edges_to(source, sink)
            .map(|edge_id| {
                let weight = &graph[edge_id];
                *weight.capacity() - *weight.flow()
            })
            .min_by(TotalOrd::total_ord)
            .unwrap();

        total_flow += bottleneck;

        for edge_id in parents.iter_edges_to(source, sink) {
            *graph[edge_id].flow_mut() += bottleneck;
            *graph[edge_id.reverse()].flow_mut() -= bottleneck;
        }
    }
    total_flow
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::ford_fulkerson;
    use crate::algorithms::{empty_flow, flow_adaptor};
    use crate::test::{digraph, id};
    use grax_impl::*;
    use test::Bencher;

    // adj

    #[cfg(feature = "extensive")]
    #[bench]
    fn ford_fulkerson_g_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();
        let mut graph: AdjGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = ford_fulkerson(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn ford_fulkerson_fluss_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();
        let mut graph: AdjGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = ford_fulkerson(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn ford_fulkerson_fluss2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();
        let mut graph: AdjGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = ford_fulkerson(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0);
            empty_flow(&mut graph);
        })
    }
    // dense

    #[cfg(feature = "extensive")]
    #[bench]
    fn ford_fulkerson_g_1_2_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();
        let mut graph: MatGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = ford_fulkerson(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn ford_fulkerson_fluss_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();
        let mut graph: MatGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = ford_fulkerson(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn ford_fulkerson_fluss2_dense_mat(b: &mut Bencher) {
        let graph: MatGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();
        let mut graph: MatGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = ford_fulkerson(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0);
            empty_flow(&mut graph);
        })
    }

    // csr

    #[cfg(feature = "extensive")]
    #[bench]
    fn ford_fulkerson_g_1_2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/G_1_2.txt").unwrap();
        let mut graph: CsrGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = ford_fulkerson(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn ford_fulkerson_fluss_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Fluss.txt").unwrap();
        let mut graph: CsrGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = ford_fulkerson(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 4.0);
            empty_flow(&mut graph);
        })
    }

    #[bench]
    fn ford_fulkerson_fluss2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _, true> = digraph("../data/Fluss2.txt").unwrap();
        let mut graph: CsrGraph<_, _, true> = flow_adaptor(graph);

        b.iter(|| {
            let total = ford_fulkerson(&mut graph, id(0), id(7));
            assert_eq!(total as f32, 5.0);
            empty_flow(&mut graph);
        })
    }
}
