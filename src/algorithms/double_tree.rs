use std::ops::{Add, AddAssign};

use crate::{
    adjacency_list::AdjacencyOptions,
    error::GraphResult,
    prelude::{
        AdjacencyList, GraphAccess, GraphAdjacentTopology, GraphCompare, GraphError, GraphTopology,
        Sortable,
    },
};

use super::{_depth_search, _kruskal, dijkstra};

pub fn double_tree<N, W, G>(graph: &G) -> GraphResult<W>
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
        mst.add_edge(edge.from, edge.to, edge.weight.clone())
            .unwrap();
        mst.add_edge(edge.to, edge.from, edge.weight.clone())
            .unwrap();
    });
    let root = union_find.root();

    let mut euler_tour = vec![];
    let mut visited = vec![false; graph.node_count()];

    _depth_search(&mst, root, &mut visited, true, |index| {
        euler_tour.push(index);
    });

    euler_tour.push(root);

    let mut total_weight = W::default();
    for [from, to] in euler_tour.array_windows::<2>() {
        let weight = match mst.contains_edge(*from, *to) {
            Some(index) => mst.weight(index).clone(),
            None => dijkstra(graph, *from, *to).ok_or(GraphError::NoCycle)?,
        };
        total_weight += weight;
    }

    if visited.into_iter().all(|visit| visit == true) {
        Ok(total_weight)
    } else {
        Err(GraphError::NoCycle)
    }
}
