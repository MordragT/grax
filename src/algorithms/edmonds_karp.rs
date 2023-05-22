use crate::{
    edge::{Edge, EdgeRef},
    prelude::{
        EdgeIndex, GraphAccess, GraphAdjacentTopology, GraphCompare, GraphTopology, NodeIndex,
        Sortable,
    },
};
use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

/*

lieber residualen graph erstellen
dann edges entfernen wenn die full sind
 */

pub fn edmonds_karp<N, W, G>(graph: &mut G, source: NodeIndex, sink: NodeIndex) -> W
where
    N: PartialEq,
    W: Sortable
        + Default
        + Copy
        + Sub<W, Output = W>
        + Add<W, Output = W>
        + AddAssign
        + Debug
        + SubAssign,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W> + GraphCompare<N, W> + GraphAccess<N, W>,
{
    let mut capacities: BTreeMap<EdgeIndex, (W, W)> = BTreeMap::new();
    let mut full_edges = HashSet::new();
    let mut flow = W::default();

    // create residual graph
    let backward_edge_map = graph
        .edges()
        .map(|edge| {
            let edge: Edge<W> = edge.into();
            (edge.index(), edge.weight)
        })
        .collect::<HashMap<_, _>>();
    let mut backward_edges = HashSet::new();
    for (index, weight) in backward_edge_map {
        let index = graph.add_edge(index.to, index.from, weight).unwrap();
        backward_edges.insert(index);
    }

    while let Some(path) = _bfs_augmenting_path(graph, source, sink, &full_edges, &backward_edges) {
        let mut residual_capacities = Vec::new();

        // find maxium capacity and residual capacites of edges
        // raw_edges transforms reverse edges automatically to the original
        for edge in path.raw_edges() {
            let maximum_capacity = graph.weight(edge);

            if let Some((residual_capacity, _)) = capacities.get(&edge) {
                residual_capacities.push(*residual_capacity);
            } else {
                capacities.insert(edge, (*maximum_capacity, *maximum_capacity));
                residual_capacities.push(*maximum_capacity)
            }
        }

        dbg!(&residual_capacities);

        // update edges
        if let Some(bottleneck) = residual_capacities
            .into_iter()
            .min_by(|this, other| this.sort(other))
        {
            dbg!(&bottleneck);

            // if bottleneck <= W::default() {
            //     break;
            // }

            flow += bottleneck;

            for edge in path.edges {
                let index = edge.raw();

                let (residual_capacity, max) = capacities
                    .get_mut(&index)
                    .expect("INTERNAL: Edge should be already inside capacities");

                if edge.is_rev() {
                    if *residual_capacity - bottleneck <= W::default() {
                        *residual_capacity = W::default();
                    } else {
                        *residual_capacity -= bottleneck;
                    }
                } else {
                    *residual_capacity += bottleneck;
                }

                if residual_capacity >= max {
                    full_edges.insert(index);
                } else {
                    full_edges.remove(&index);
                }
            }
        }
    }

    flow
}

#[cfg(test)]
mod test {
    extern crate test;

    use crate::{prelude::*, test::digraph};
    use test::Bencher;

    #[bench]
    fn edmonds_karp_g_1_2_adj_list(b: &mut Bencher) {
        let mut graph: AdjacencyList<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_list(b: &mut Bencher) {
        let mut graph: AdjacencyList<_, _, true> = digraph("data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_list(b: &mut Bencher) {
        let mut graph: AdjacencyList<_, _, true> = digraph("data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 5.0)
        })
    }

    #[bench]
    fn edmonds_karp_g_1_2_adj_mat(b: &mut Bencher) {
        let mut graph: AdjacencyMatrix<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_mat(b: &mut Bencher) {
        let mut graph: AdjacencyMatrix<_, _, true> = digraph("data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_mat(b: &mut Bencher) {
        let mut graph: AdjacencyMatrix<_, _, true> = digraph("data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 5.0)
        })
    }
}
