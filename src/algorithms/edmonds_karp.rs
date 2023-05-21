use crate::{
    algorithms::_bfs_augmenting_path,
    prelude::{
        EdgeIndex, GraphAccess, GraphAdjacentTopology, GraphCompare, GraphTopology, NodeIndex,
        Sortable,
    },
};
use std::{
    collections::{BTreeMap, HashSet},
    ops::{Add, AddAssign, Sub},
};

// pub(crate) fn ___bfs_augmenting_path<N, W, G>(
//     graph: &G,
//     source: NodeIndex,
//     sink: NodeIndex,
//     capacities: &mut BTreeMap<EdgeIndex, (W, W)>,
// ) -> Option<Tour<W>>
// where
//     N: PartialEq,
//     W: Sortable + Default + Clone + Sub<W, Output = W> + AddAssign,
//     G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
// {
//     let mut bottleneck: Option<W> = None;
//     let mut queue = VecDeque::new();
//     let mut route = Vec::new();

//     queue.push_front(source);

//     while let Some(from) = queue.pop_front() {
//         let mut edges = graph.adjacent_edges(from).collect::<Vec<_>>();
//         edges.sort_by(|edge, other| edge.weight.sort(other.weight));
//         route.push(from);

//         for EdgeRef { from, to, weight } in edges {
//             let index = EdgeIndex::new(from, to);

//             let residual_capacity = if let Some((current, max)) = capacities.get(&index).cloned() {
//                 if current >= max {
//                     continue;
//                 } else {
//                     max - current
//                 }
//             } else {
//                 capacities.insert(index, (W::default(), weight.clone()));
//                 weight.clone()
//             };

//             match bottleneck {
//                 Some(ref b) if b < &residual_capacity => (),
//                 _ => bottleneck = Some(residual_capacity),
//             }

//             queue.push_back(to);

//             if to == sink {
//                 route.push(to);
//                 queue.clear();
//                 break;
//             }
//         }
//     }

//     bottleneck.map(|b| Tour::new(route, b))
// }

pub fn edmonds_karp<N, W, G>(graph: &G, source: NodeIndex, sink: NodeIndex) -> W
where
    N: PartialEq,
    W: Sortable + Default + Clone + Sub<W, Output = W> + Add<W, Output = W> + AddAssign,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W> + GraphCompare<N, W> + GraphAccess<N, W>,
{
    enum AugmentedEdge {
        Forward(EdgeIndex),
        Backward(EdgeIndex),
    }

    let mut capacities: BTreeMap<EdgeIndex, (W, W)> = BTreeMap::new();
    let mut full_edges = HashSet::new();

    while let Some(tour) = _bfs_augmenting_path(graph, source, sink, &mut full_edges) {
        let mut bottleneck = None;
        let mut edges = Vec::new();

        // compute bottleneck and edge direction
        for (from, to) in tour.edges() {
            if let Some(index) = graph.contains_edge(*from, *to) {
                let residual_capacity = if let Some((residual_capacity, _)) = capacities.get(&index)
                {
                    residual_capacity.clone()
                } else {
                    let residual_capacity = graph.weight(index).clone();
                    capacities.insert(index, (W::default(), residual_capacity.clone()));
                    residual_capacity
                };

                match bottleneck {
                    Some(ref b) if b < &residual_capacity => (),
                    _ => bottleneck = Some(residual_capacity),
                }

                edges.push(AugmentedEdge::Forward(index));
            } else if let Some(index) = graph.contains_edge(*to, *from) {
                if !capacities.contains_key(&index) {
                    todo!()
                }

                edges.push(AugmentedEdge::Backward(index));
            } else {
                todo!()
            }
        }

        // for edge in edges {

        // }
    }

    // let flow = capacities
    //     .into_iter()
    //     .filter_map(|(edge, (current, _))| {
    //         if edge.contains(sink) {
    //             Some(current)
    //         } else {
    //             None
    //         }
    //     })
    //     .fold(W::default(), |mut accu, weight| {
    //         accu += weight;
    //         accu
    //     });

    // flow

    todo!()
}

#[cfg(test)]
mod test {
    extern crate test;

    use crate::{adjacency_matrix::AdjacencyMatrix, prelude::*, test::digraph};
    use test::Bencher;

    #[bench]
    fn edmonds_karp_g_1_2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _, true> = digraph("data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 5.0)
        })
    }

    #[bench]
    fn edmonds_karp_g_1_2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/G_1_2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 0.75447)
        })
    }

    #[bench]
    fn edmonds_karp_fluss_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Fluss.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 4.0)
        })
    }

    #[bench]
    fn edmonds_karp_fluss2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _, true> = digraph("data/Fluss2.txt").unwrap();

        b.iter(|| {
            let total = graph.edmonds_karp(NodeIndex(0), NodeIndex(7));
            assert_eq!(total as f32, 5.0)
        })
    }
}
