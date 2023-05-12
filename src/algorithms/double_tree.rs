use std::ops::{Add, AddAssign};

use crate::{
    adjacency_list::AdjacencyOptions,
    prelude::{
        AdjacencyList, GraphAccess, GraphAdjacentTopology, GraphCompare, GraphTopology, Sortable,
    },
};

use super::{_depth_search, _kruskal, dijkstra, Tour};

pub fn double_tree<N, W, G>(graph: &G) -> Option<Tour<W>>
where
    N: PartialEq,
    W: Default + Sortable + Copy + AddAssign + Add<W, Output = W>,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W> + GraphCompare<N, W>,
{
    let mut mst = AdjacencyList::with(AdjacencyOptions {
        directed: graph.directed(),
        nodes: Some(graph.nodes().collect()),
    });

    let union_find = _kruskal(graph, |edge| {
        mst.add_edge(edge.from, edge.to, *edge.weight).unwrap();
        mst.add_edge(edge.to, edge.from, *edge.weight).unwrap();
    });
    let root = union_find.root();

    let mut route = vec![];
    let mut visited = vec![false; graph.node_count()];

    _depth_search(&mst, root, &mut visited, true, |index| {
        route.push(index);
    });

    route.push(root);

    let mut total_weight = W::default();
    for [from, to] in route.array_windows::<2>() {
        let weight = match mst.contains_edge(*from, *to) {
            Some(index) => *mst.weight(index),
            None if let Some(weight) = dijkstra(graph, *from, *to) => weight,
            _ => return None,
        };
        total_weight += weight;
    }

    if visited.into_iter().all(|visit| visit == true) {
        Some(Tour::new(route, total_weight))
    } else {
        None
    }
}
