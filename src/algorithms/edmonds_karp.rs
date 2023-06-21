use std::{
    fmt::Debug,
    ops::{AddAssign, Neg, Sub, SubAssign},
};

use crate::{
    graph::{
        Base, Count, Create, EdgeCapacity, EdgeCost, EdgeFlow, FlowWeight, Get, GetMut,
        IndexAdjacent, Insert, Iter,
    },
    prelude::{AdjacencyList, EdgeRef, NodeId},
    structures::Parents,
};

use super::{_ford_fulkerson, bfs_sp};

// TODO fix edmonds karp to not use adj list but graph representation itself

pub fn edmonds_karp<N, W, C, G>(graph: &G, source: NodeId<G::Id>, sink: NodeId<G::Id>) -> C
where
    N: Debug,
    C: Default
        + PartialOrd
        + Copy
        + AddAssign
        + SubAssign
        + Neg<Output = C>
        + Sub<C, Output = C>
        + Debug,
    W: EdgeCost<Cost = C>,
    G: Iter + Base<Id = usize> + Get + Base<Node = N, Weight = W> + Debug,
{
    let mut residual_graph = AdjacencyList::<_, _, true>::with_nodes(graph.iter_nodes());
    for EdgeRef { edge_id, weight } in graph.iter_edges() {
        let cost = *weight.cost();

        let capacity = FlowWeight::new(cost, cost, C::default());
        residual_graph.insert_edge(edge_id.from(), edge_id.to(), capacity);

        if !graph.contains_edge_id(edge_id.rev()) {
            residual_graph.insert_edge(
                edge_id.to(),
                edge_id.from(),
                FlowWeight::new(cost, -cost, cost),
            );
        }
    }

    _edmonds_karp(&mut residual_graph, source, sink)
}

pub(crate) fn _edmonds_karp<N, W, C, G>(
    graph: &mut G,
    source: NodeId<G::Id>,
    sink: NodeId<G::Id>,
) -> C
where
    C: Default + PartialOrd + Copy + AddAssign + SubAssign + Sub<C, Output = C>,
    W: EdgeCapacity<Capacity = C> + EdgeFlow<Flow = C>,
    G: Count + IndexAdjacent + Get + GetMut + Base<Node = N, Weight = W> + Debug,
{
    fn shortest_path<N, W, C, G>(
        graph: &G,
        source: NodeId<G::Id>,
        sink: NodeId<G::Id>,
    ) -> Option<Parents<G>>
    where
        C: Default + Sub<C, Output = C> + PartialOrd + Copy,
        W: EdgeCapacity<Capacity = C> + EdgeFlow<Flow = C>,
        G: IndexAdjacent + Count + Get + Base<Node = N, Weight = W>,
    {
        bfs_sp(graph, source, sink, |weight: &W| {
            (*weight.capacity() - *weight.flow()) > C::default()
        })
    }

    _ford_fulkerson(graph, source, sink, shortest_path)
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::edmonds_karp;
    use crate::{
        prelude::*,
        test::{digraph, id},
    };
    use test::Bencher;

    #[bench]
    fn edmonds_karp_g_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }

    #[bench]
    fn edmonds_karp_g_1_2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = edmonds_karp(&graph, id(0), id(7));
            assert_eq!(total as f32, 5.0)
        })
    }
}
