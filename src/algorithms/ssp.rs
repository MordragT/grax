use std::{
    cmp::min_by,
    fmt::Debug,
    ops::{AddAssign, Mul, Neg, Sub, SubAssign},
};

use num_traits::{Float, Pow};

use crate::{
    algorithms::{Mcf, _ford_fulkerson, bfs_sp},
    graph::{
        Count, EdgeCapacity, EdgeCost, EdgeDirection, EdgeFlow, Get, GetMut, Index, IndexAdjacent,
        Insert, Iter, NodeBalance, Remove, Sortable,
    },
    prelude::NodeId,
    structures::Parents,
};

/// successive shortest path
pub fn ssp<N, W, C, G>(graph: &G) -> C
where
    N: Default + NodeBalance<Balance = C>,
    W: EdgeCapacity<Capacity = C>
        + EdgeCost<Cost = C>
        + Default
        + EdgeDirection
        + EdgeFlow<Flow = C>,
    C: Default
        + PartialOrd
        + Copy
        + Neg<Output = C>
        + AddAssign
        + SubAssign
        + Debug
        + Sub<C, Output = C>
        + Mul<C, Output = C>,
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
    let Mcf {
        mut residual_graph,
        source,
        sink,
    } = Mcf::init(graph);

    _ford_fulkerson(&mut residual_graph, source, sink, |graph, source, sink| {
        bfs_sp(graph, source, sink, |weight: &W| {
            (*weight.capacity() - *weight.flow()) > C::default()
        })
    });

    let cost = residual_graph
        .iter_edges()
        .fold(C::default(), |mut akku, edge| {
            let weight = edge.weight;
            if !weight.is_reverse() {
                akku += *weight.flow() * *weight.cost();
            }

            akku
        });

    cost
}

#[cfg(test)]
mod test {
    use super::ssp;
    use crate::{prelude::AdjacencyList, test::bgraph};

    #[test]
    fn ssp_kostenminimal_1() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal1.txt").unwrap();
        let cost = ssp(&graph);
        assert_eq!(cost, 3.0);
    }

    #[test]
    fn ssp_kostenminimal_2() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal2.txt").unwrap();
        let cost = ssp(&graph);
        assert_eq!(cost, 0.0);
    }

    #[test]
    #[should_panic]
    fn ssp_kostenminimal_3() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal3.txt").unwrap();
        let _cost = ssp(&graph);
    }

    #[test]
    #[should_panic]
    fn ssp_kostenminimal_4() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal4.txt").unwrap();
        let _cost = ssp(&graph);
    }

    #[test]
    fn ssp_kostenminimal_gross_1() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal_gross1.txt").unwrap();
        let cost = ssp(&graph);
        assert_eq!(cost, 1537.0);
    }

    #[test]
    fn ssp_kostenminimal_gross_2() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal_gross2.txt").unwrap();
        let cost = ssp(&graph);
        assert_eq!(cost, 1838.0);
    }

    #[test]
    #[should_panic]
    fn ssp_kostenminimal_gross_3() {
        let graph: AdjacencyList<_, _, true> = bgraph("data/Kostenminimal_gross3.txt").unwrap();
        let _cost = ssp(&graph);
    }
}
