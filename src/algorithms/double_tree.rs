use std::ops::{Add, AddAssign};

use crate::prelude::{GraphAccess, GraphAdjacentTopology, GraphCompare, GraphTopology, Sortable};

use super::{depth_search_tour, dijkstra_between, kruskal_mst, MinimumSpanningTree, Tour};

pub fn double_tree<N, W, G>(graph: &G) -> Option<Tour<W>>
where
    N: PartialEq,
    W: Default + Sortable + Copy + AddAssign + Add<W, Output = W>,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W> + GraphCompare<N, W>,
{
    let MinimumSpanningTree { tree: mst, root } = kruskal_mst(graph);

    let mut route = depth_search_tour(&mst, root).route;
    route.push(root);

    if route.len() != graph.node_count() + 1 {
        return None;
    }

    let mut total_weight = W::default();
    for [from, to] in route.array_windows::<2>() {
        let weight = match mst.contains_edge(*from, *to) {
            Some(index) => *mst.weight(index),
            None if let Some(weight) = dijkstra_between(graph, *from, *to) => weight,
            _ => return None,
        };
        total_weight += weight;
    }

    Some(Tour::new(route, total_weight))
}
