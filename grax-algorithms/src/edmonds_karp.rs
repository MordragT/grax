use std::{
    fmt::Debug,
    ops::{AddAssign, Neg, Sub, SubAssign},
};

use grax_core::{
    edge::*,
    prelude::*,
    traits::*,
    variant::flow::{FlowBundle, FlowGraph},
    view::{AttrMap, Parents},
};

use super::{_ford_fulkerson, bfs_sp};

pub fn edmonds_karp<C, G>(graph: &G, source: NodeId<G::Id>, sink: NodeId<G::Id>) -> C
where
    C: Default
        + PartialOrd
        + Copy
        + AddAssign
        + SubAssign
        + Neg<Output = C>
        + Sub<C, Output = C>
        + Debug,
    G: Iter
        + Get
        + Clone
        + Viewable
        + Cost<C>
        + Insert
        + Count
        + IndexAdjacent
        + GetMut
        + Debug
        + Visitable
        + Base<Weight: Copy>,
{
    let mut flow_map = graph.edge_map();
    let mut residual_graph = graph.clone();

    for EdgeRef { edge_id, weight } in graph.iter_edges() {
        let cost = *graph.cost(edge_id).unwrap().cost();

        let bundle = FlowBundle {
            capacity: cost,
            cost,
            flow: C::default(),
            reverse: false,
        };

        *flow_map.get_mut(edge_id) = Some(bundle);

        if !residual_graph.contains_edge_id(edge_id.rev()) {
            residual_graph.insert_edge(edge_id.to(), edge_id.from(), *weight);
            let bundle = FlowBundle {
                capacity: cost,
                cost: -cost,
                flow: cost,
                reverse: true,
            };

            // TODO potentially heavy computation see update impl in respective graph impls

            // residual_graph.update_edge_map(&mut flow_map);
            flow_map.insert(edge_id.rev(), Some(bundle));
        }
    }

    let mut flow_graph = FlowGraph::from_unchecked(residual_graph, flow_map);

    _edmonds_karp(&mut flow_graph, source, sink)
}

pub(crate) fn _edmonds_karp<C, G>(graph: &mut G, source: NodeId<G::Id>, sink: NodeId<G::Id>) -> C
where
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C> + Debug,
    G: Count + IndexAdjacent + Get + GetMut + Viewable + Debug + Flow<C> + Cost<C> + Visitable,
{
    fn shortest_path<C, G>(
        graph: &G,
        source: NodeId<G::Id>,
        sink: NodeId<G::Id>,
    ) -> Option<Parents<G>>
    where
        C: Default + Sub<C, Output = C> + PartialOrd + Copy + Debug,
        G: IndexAdjacent + Count + Get + Viewable + Flow<C> + Cost<C> + Visitable,
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

    use super::edmonds_karp;
    use crate::test::{digraph, id};
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn edmonds_karp_g_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("../data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("../data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }

    #[bench]
    fn edmonds_karp_g_1_2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("../data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("../data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }
}
