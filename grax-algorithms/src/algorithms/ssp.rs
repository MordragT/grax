// use std::{
//     cmp::min_by,
//     fmt::Debug,
//     ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
// };

// use num_traits::{Float, Pow};

// use super::{bellman_ford, dijkstra};
// use crate::algorithms::{Mcf, _ford_fulkerson, bfs_sp};
// use crate::view::Parents;
// use grax_core::prelude::*;
// use grax_core::traits::*;

use std::{
    fmt::Debug,
    ops::{Neg, Sub},
};

use grax_core::edge::{weight::*, *};
use grax_core::{
    collections::{
        EdgeCollection, EdgeIter, EdgeIterMut, GetEdgeMut, GetNodeMut, NodeCollection, NodeIter,
        NodeIterMut,
    },
    graph::NodeAttribute,
    node::{NodeMut, NodeRef},
};

/// successive shortest path
pub fn ssp<C, G>(graph: &mut G) -> Option<C>
where
    C: PartialOrd + Default + Copy + Debug + Neg<Output = C> + Sub<C, Output = C>,
    G: EdgeCollection<EdgeWeight = FlowCostBundle<C>>
        + NodeCollection<NodeWeight = C>
        + GetNodeMut
        + GetEdgeMut
        + EdgeIter
        + NodeAttribute,
{
    let (to_augment, flows): (Vec<_>, Vec<_>) = graph
        .iter_edges()
        .filter_map(|EdgeRef { edge_id, weight }| {
            if *weight.cost() < C::default() {
                let flow = *weight.capacity();
                Some((edge_id, flow))
            } else {
                None
            }
        })
        .unzip();

    let mut balances = graph.fixed_node_map(C::default());

    for (edge_id, flow) in to_augment.into_iter().zip(flows) {
        let mut weight = graph.edge_mut(edge_id).unwrap().weight;
        *weight.flow_mut() = flow;

        let source_id = edge_id.from();
        let mut source = graph.node_mut(edge_id.from()).unwrap();

        // balances.update_node(source_id, source.weight.clone());
        // *source.weight.balance_mut() += flow;
    }

    todo!()
}

// where
//     N: Default + NodeBalance<Balance = C>,
//     W: EdgeCapacity<Capacity = C>
//         + Cost<C>
//         + Default
//         + EdgeDirection
//         + EdgeFlow<Flow = C>,
//     C: Default
//         + PartialOrd
//         + Copy
//         + Neg<Output = C>
//         + AddAssign
//         + SubAssign
//         + Debug
//         + Sub<C, Output = C>
//         + Mul<C, Output = C>
//         + Add<C, Output = C>
//         + Sortable,
//     G: Index
//         + Get
//         + GetMut
//         + Insert
//         + Remove
//         + Count
//         + IndexAdjacent
//         + IterAdjacent
//         + Iter
//         + IterMut
//         + Clone
//         + Base<Node = N, Weight = W>
//         + Debug,
// {
//     // let Mcf {
//     //     mut residual_graph,
//     //     source,
//     //     sink,
//     // } = Mcf::init(graph);

//     // pseudo flow, erfüllt kapazitätsbedinungen aber verletzt ddie massenbalancebedingung
//     let mut delta = match graph
//         .iter_edges()
//         .map(|edge| edge.weight.capacity())
//         .cloned()
//         .reduce(C::min)
//     {
//         Some(d) => d,
//         None => return None,
//     };

//     let mut graphR = graph.clone();

//     for edge in graphR.iter_edges_mut() {
//         *edge.weight.flow_mut() += delta;
//     }

//     for EdgeRef { edge_id, weight } in graph.iter_edges() {
//         if !graphR.contains_edge_id(edge_id.rev()) {
//             let mut w = W::default();
//             *w.cost_mut() = -*weight.cost();
//             *w.capacity_mut() = *weight.capacity();
//             *w.flow_mut() = *weight.capacity() - *weight.flow();
//             w.reverse();

//             graphR.insert_edge(edge_id.to(), edge_id.from(), w);
//         }
//     }

//     let cost = graphR.iter_edges().fold(C::default(), |mut akku, edge| {
//         let weight = edge.weight;
//         if !weight.is_reverse() {
//             akku += *weight.flow() * *weight.cost();
//         }

//         akku
//     });

//     Some(cost)

#[cfg(test)]
mod test {
    use super::ssp;
    use crate::test::bgraph;
    use grax_impl::*;

    #[test]
    fn ssp_kostenminimal_1() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal1.txt").unwrap();
        let cost = ssp(&mut graph).unwrap();
        assert_eq!(cost, 3.0);
    }

    #[test]
    fn ssp_kostenminimal_2() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal2.txt").unwrap();
        let cost = ssp(&mut graph).unwrap();
        assert_eq!(cost, 0.0);
    }

    #[test]
    #[should_panic]
    fn ssp_kostenminimal_3() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal3.txt").unwrap();
        let _cost = ssp(&mut graph).unwrap();
    }

    #[test]
    #[should_panic]
    fn ssp_kostenminimal_4() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal4.txt").unwrap();
        let _cost = ssp(&mut graph).unwrap();
    }

    #[test]
    fn ssp_kostenminimal_gross_1() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross1.txt").unwrap();
        let cost = ssp(&mut graph).unwrap();
        assert_eq!(cost, 1537.0);
    }

    #[test]
    fn ssp_kostenminimal_gross_2() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross2.txt").unwrap();
        let cost = ssp(&mut graph).unwrap();
        assert_eq!(cost, 1838.0);
    }

    #[test]
    #[should_panic]
    fn ssp_kostenminimal_gross_3() {
        let mut graph: AdjGraph<_, _, true> = bgraph("../data/Kostenminimal_gross3.txt").unwrap();
        let _cost = ssp(&mut graph).unwrap();
    }
}
