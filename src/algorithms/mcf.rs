use crate::{
    algorithms::_edmonds_karp,
    graph::{
        Base, Count, EdgeCapacity, EdgeCost, EdgeDirection, EdgeFlow, Get, GetMut, Index,
        IndexAdjacent, Insert, Iter, NodeBalance, Remove,
    },
    prelude::{EdgeRef, NodeId},
};
use std::{
    fmt::Debug,
    ops::{AddAssign, Neg, Sub, SubAssign},
};

pub struct Mcf<G: Base> {
    pub residual_graph: G,
    pub source: NodeId<G::Id>,
    pub sink: NodeId<G::Id>,
}

impl<G: Base> Mcf<G> {
    pub fn init<N, W, C>(graph: &G) -> Self
    where
        N: Default + NodeBalance<Balance = C>,
        W: EdgeCapacity<Capacity = C>
            + EdgeCost<Cost = C>
            + Default
            + EdgeDirection
            + EdgeFlow<Flow = C>,
        C: Default + PartialOrd + Copy + Neg<Output = C> + AddAssign + SubAssign + Debug,
        G: Index
            + Get<N, W>
            + GetMut<N, W>
            + Insert<N, W>
            + Remove<N, W>
            + Count
            + IndexAdjacent
            + Iter<N, W>
            + Clone,
    {
        let mut residual_graph = graph.clone();
        let source = residual_graph.insert_node(N::default());
        let sink = residual_graph.insert_node(N::default());

        for node_id in graph.node_ids() {
            let node = residual_graph.node(node_id).unwrap();
            let balance = *node.balance();

            let mut weight = W::default();
            let mut rev_weight = W::default();
            rev_weight.reverse();

            if balance > C::default() {
                // supply
                *weight.capacity_mut() = balance;
                *rev_weight.capacity_mut() = balance;
                *rev_weight.flow_mut() = balance;

                residual_graph.insert_edge(source, node_id, weight);
                residual_graph.insert_edge(node_id, source, rev_weight);
            } else if node.balance() < &C::default() {
                // demand
                *weight.capacity_mut() = -balance;
                *rev_weight.capacity_mut() = -balance;
                *rev_weight.flow_mut() = -balance;

                residual_graph.insert_edge(node_id, sink, weight);
                residual_graph.insert_edge(sink, node_id, rev_weight);
            }
        }

        for EdgeRef { edge_id, weight } in graph.iter_edges() {
            if !residual_graph.contains_edge_id(edge_id.rev()) {
                let mut w = W::default();
                *w.cost_mut() = -*weight.cost();
                *w.capacity_mut() = *weight.capacity();
                *w.flow_mut() = *weight.capacity();
                w.reverse();

                residual_graph.insert_edge(edge_id.to(), edge_id.from(), w);
            }
        }

        Self {
            source,
            sink,
            residual_graph,
        }
    }

    pub fn solvable<N, W, C>(&mut self) -> bool
    where
        N: Default + NodeBalance<Balance = C>,
        C: Default
            + PartialOrd
            + Copy
            + Neg<Output = C>
            + AddAssign
            + SubAssign
            + Debug
            + Sub<C, Output = C>,
        W: EdgeCapacity<Capacity = C> + EdgeFlow<Flow = C>,
        G: Index
            + Get<N, W>
            + GetMut<N, W>
            + Insert<N, W>
            + Remove<N, W>
            + Count
            + IndexAdjacent
            + Iter<N, W>
            + Clone,
    {
        let total_flow = _edmonds_karp(&mut self.residual_graph, self.source, self.sink);
        let expected = self
            .residual_graph
            .iter_nodes()
            .fold(C::default(), |mut akku, node| {
                if node.balance() > &C::default() {
                    akku += *node.balance();
                }
                akku
            });

        total_flow == expected
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use more_asserts::assert_ge;

    use crate::{
        algorithms::Mcf,
        graph::{Get, IndexAdjacent, Iter},
        prelude::{AdjacencyList, EdgeId, EdgeRef},
        test::{bgraph, id},
    };

    fn mcf_residual_graph(path: &str) {
        let graph: AdjacencyList<_, _, true> = bgraph(path).unwrap();
        let Mcf {
            residual_graph,
            source,
            sink,
        } = Mcf::init(&graph);

        let mut nodes = HashSet::new();
        for node_id in residual_graph.adjacent_node_ids(source) {
            let weight = residual_graph
                .weight(EdgeId::new_unchecked(source, node_id))
                .unwrap();
            let balance = residual_graph.node(node_id).unwrap();

            nodes.insert(node_id);
            assert_eq!(balance.balance, weight.capacity)
        }

        let expected = graph
            .iter_nodes()
            .enumerate()
            .filter_map(|(i, node)| {
                if node.balance > 0.0 {
                    Some(id(i))
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();
        assert_eq!(nodes, expected);

        nodes.clear();

        for node_id in residual_graph.adjacent_node_ids(sink) {
            let weight = residual_graph
                .weight(EdgeId::new_unchecked(node_id, sink))
                .unwrap();
            let balance = residual_graph.node(node_id).unwrap();

            assert_eq!(-balance.balance, weight.capacity);
            nodes.insert(node_id);
        }

        let expected = graph
            .iter_nodes()
            .enumerate()
            .filter_map(|(i, node)| {
                if node.balance < 0.0 {
                    Some(id(i))
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();
        assert_eq!(nodes, expected);

        for EdgeRef { edge_id: _, weight } in residual_graph.iter_edges() {
            assert_ge!(weight.capacity, 0.0);
        }
    }

    #[test]
    fn mcf_residual_graph_kostenminimal_1() {
        mcf_residual_graph("data/Kostenminimal1.txt")
    }

    #[test]
    fn mcf_residual_graph_kostenminimal_gross_1() {
        mcf_residual_graph("data/Kostenminimal_gross1.txt")
    }

    #[test]
    fn mcf_residual_graph_kostenminimal_gross_2() {
        mcf_residual_graph("data/Kostenminimal_gross2.txt")
    }

    #[test]
    fn mcf_solvable_kostenminimal_1() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal1.txt").unwrap();
        let mut mcf = Mcf::init(&graph);

        assert!(mcf.solvable())
    }

    #[test]
    fn mcf_solvable_kostenminimal_2() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal2.txt").unwrap();
        let mut mcf = Mcf::init(&graph);

        assert!(mcf.solvable())
    }

    #[test]
    fn mcf_solvable_kostenminimal_3() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal3.txt").unwrap();
        let mut mcf = Mcf::init(&graph);

        assert!(!mcf.solvable())
    }

    #[test]
    fn mcf_solvable_kostenminimal_4() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal4.txt").unwrap();
        let mut mcf = Mcf::init(&graph);

        assert!(!mcf.solvable())
    }

    #[test]
    fn mcf_solvable_kostenminimal_gross_1() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal_gross1.txt").unwrap();
        let mut mcf = Mcf::init(&graph);

        assert!(mcf.solvable())
    }

    #[test]
    fn mcf_solvable_kostenminimal_gross_2() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal_gross2.txt").unwrap();
        let mut mcf = Mcf::init(&graph);

        assert!(mcf.solvable())
    }

    #[test]
    fn mcf_solvable_kostenminimal_gross_3() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal_gross3.txt").unwrap();
        let mut mcf = Mcf::init(&graph);

        assert!(!mcf.solvable())
    }
}
