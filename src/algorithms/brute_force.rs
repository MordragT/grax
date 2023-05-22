use crate::prelude::{Count, Get, Maximum, NodeIndex};
use std::ops::AddAssign;

use super::Tour;

pub fn brute_force<N, W, G>(graph: &G) -> Option<Tour<W>>
where
    N: PartialEq,
    W: Default + Maximum + PartialOrd + AddAssign + Copy,
    G: Get<N, W> + Count,
{
    let mut best_path = Vec::new();
    let mut best_weight = W::max();

    let start = (0..graph.node_count()).map(NodeIndex).collect::<Vec<_>>();

    for perm in permute::permutations_of(&start) {
        let mut perm = perm.map(ToOwned::to_owned).collect::<Vec<_>>();
        perm.push(perm[0]);

        let edges = perm
            .array_windows::<2>()
            .map(|w| graph.contains_edge_id(w[0], w[1]))
            .collect::<Option<Vec<_>>>();

        if let Some(edges) = edges {
            let total_weight = edges.into_iter().map(|edge| *graph.weight(edge)).fold(
                W::default(),
                |mut accu, w| {
                    accu += w;
                    accu
                },
            );

            if total_weight < best_weight {
                best_path = perm.clone();
                best_weight = total_weight;
            }
        }
    }

    if best_weight == W::max() {
        None
    } else {
        Some(Tour::new(best_path, best_weight))
    }
}
