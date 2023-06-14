// no more sink and source (st) -> no st-flow
// b (balance)
// b-flow: sum of all b-flow in the graph is 0
// edge cannot carry more flow than its capacity
// the difference in flow entering and leaving a node
// must be equal b(v) (flow balance)

// wenn capacity von edge == 0 -> edge ignorieren
// supply und demand nodes
// wenn balance < 0 demand node, wenn balance > 0 supply node
// flow kann nicht mehr als die capacities sein aber auch nicht weniger als 0
// differenz zwischen flow der raus geht und reingeht muss gleich dem supply oder demand sein
// wenn flow durch nodes geht, dann muss bei demand-nodes der demand vom flow abgezogen werden,
// analog bei supply-nodes wird der supply addiert.
// möglicherweise nicht solvable wenn im netzwerk weniger supply als demand vorhanden ist, oder mehr supply als demand
// wir können checken ob MCF möglich wenn wir das problem in ein max flow problem überführen

// neuer graph g'

use std::ops::{AddAssign, Neg, SubAssign};

use crate::{
    algorithms::_edmonds_karp,
    graph::{BalancedNode, CapacityWeight, Count, Get, GetMut, Index, IndexAdjacent, Insert, Iter},
    prelude::{Edge, EdgeIdentifier, EdgeRef},
};

fn mcf_solvable<G, N, W>(graph: &G) -> bool
where
    N: Default,
    W: Default + PartialOrd + Copy + Neg<Output = W> + AddAssign + SubAssign,
    G: Index
        + Get<BalancedNode<N, W>, CapacityWeight<W>>
        + GetMut<BalancedNode<N, W>, CapacityWeight<W>>
        + Insert<BalancedNode<N, W>, CapacityWeight<W>>
        + Count
        + IndexAdjacent
        + Iter<BalancedNode<N, W>, CapacityWeight<W>>
        + Clone,
{
    let mut residual_graph = graph.clone();

    let source = residual_graph.add_node(BalancedNode::new(N::default(), W::default()));
    let sink = residual_graph.add_node(BalancedNode::new(N::default(), W::default()));

    for node_id in graph.node_ids() {
        let node = residual_graph.node(node_id).unwrap();

        if node.balance > W::default() {
            // supply
            let edge_id = G::EdgeId::between(source, node_id);
            residual_graph.insert_edge(edge_id, CapacityWeight::new(node.balance, W::default()));
            residual_graph.insert_edge(edge_id.rev(), CapacityWeight::default());
        } else {
            // demand
            let edge_id = G::EdgeId::between(node_id, sink);
            residual_graph.insert_edge(edge_id, CapacityWeight::new(-node.balance, W::default()));
            residual_graph.insert_edge(edge_id.rev(), CapacityWeight::default());
        }
    }

    for EdgeRef { edge_id, weight } in graph.iter_edges() {
        if !residual_graph.contains_edge_id(edge_id.rev()) {
            residual_graph.insert_edge(
                edge_id.rev(),
                CapacityWeight::new(W::default(), weight.weight),
            );
        }
    }

    let total_flow = _edmonds_karp(&mut residual_graph, source, sink);
    let expected = graph.iter_nodes().fold(W::default(), |mut akku, node| {
        if node.balance > W::default() {
            akku += node.balance;
        }
        akku
    });

    total_flow == expected
}

#[cfg(test)]
mod test {
    use crate::{prelude::AdjacencyList, test::bgraph};

    use super::mcf_solvable;

    #[test]
    fn cycle_canceling_kostenminimal_1() {
        let graph: AdjacencyList<_, _> = bgraph("data/Kostenminimal1.txt").unwrap();

        assert!(mcf_solvable(&graph))
    }

    #[test]
    fn cycle_canceling_kostenminimal_2() {
        let graph: AdjacencyList<_, _> = bgraph("data/Kostenminimal2.txt").unwrap();

        assert!(mcf_solvable(&graph))
    }

    #[test]
    fn cycle_canceling_kostenminimal_3() {
        let graph: AdjacencyList<_, _> = bgraph("data/Kostenminimal3.txt").unwrap();

        assert!(!mcf_solvable(&graph))
    }

    #[test]
    fn cycle_canceling_kostenminimal_4() {
        let graph: AdjacencyList<_, _> = bgraph("data/Kostenminimal4.txt").unwrap();

        assert!(!mcf_solvable(&graph))
    }

    #[test]
    fn cycle_canceling_kostenminimal_gross_1() {}

    #[test]
    fn cycle_canceling_kostenminimal_gross_2() {}

    #[test]
    fn cycle_canceling_kostenminimal_gross_3() {}
}
