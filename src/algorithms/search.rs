use crate::edge::EdgeRef;
use crate::graph::{GraphAdjacentTopology, GraphTopology};
use crate::indices::NodeIndex;
use crate::prelude::{EdgeIndex, Sortable};
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::ops::{AddAssign, Sub};

use super::{ConnectedComponents, Tour};

pub fn dfs_connected_components<N, W, G>(graph: &G) -> ConnectedComponents
where
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut counter = 0;
    let mut markers = vec![0; graph.node_count()];
    let mut components = Vec::new();

    for from in graph.indices() {
        if markers[from.0] == 0 {
            counter += 1;
            let comp = _dfs(graph, from, &mut markers, counter).collect();
            components.push(comp);
        }
    }

    ConnectedComponents::new(components)
}

pub fn bfs_connected_components<N, W, G>(graph: &G) -> ConnectedComponents
where
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut counter = 0;
    let mut markers = vec![0; graph.node_count()];
    let mut components = Vec::new();

    for from in graph.indices() {
        if markers[from.0] == 0 {
            counter += 1;
            let comp = _bfs(graph, from, &mut markers, counter).collect();
            components.push(comp);
        }
    }

    ConnectedComponents::new(components)
}

pub fn dfs_tour<N, W, G>(graph: &G, from: NodeIndex) -> Tour<()>
where
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut markers = vec![false; graph.node_count()];
    let route = _dfs(graph, from, &mut markers, true).collect();

    Tour::new(route, ())
}

pub fn bfs_tour<N, W, G>(graph: &G, from: NodeIndex) -> Tour<()>
where
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut markers = vec![false; graph.node_count()];
    let route = _bfs(graph, from, &mut markers, true).collect();

    Tour::new(route, ())
}

// pub fn dfs<'a, N, W, G>(graph: &'a G, from: NodeIndex) -> impl Iterator<Item = NodeIndex> + 'a
// where
//     G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
// {
//     let mut markers = vec![false; graph.node_count()];
//     _dfs(graph, from, &mut markers, true)
// }

// pub fn bfs<'a, N, W, G>(graph: &'a G, from: NodeIndex) -> impl Iterator<Item = NodeIndex> + 'a
// where
//     G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
// {
//     let mut markers = vec![false; graph.node_count()];
//     _bfs(graph, from, &mut markers, true)
// }

pub(crate) fn _bfs_augmenting_path<N, W, G>(
    graph: &G,
    source: NodeIndex,
    sink: NodeIndex,
    full_edges: &HashSet<EdgeIndex>,
) -> Option<Tour<()>>
where
    N: PartialEq,
    W: Sortable + Default + Clone + Sub<W, Output = W> + AddAssign,
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut queue = VecDeque::new();
    let mut route = Vec::new();
    let mut visited = HashSet::new();

    queue.push_front(source);

    while let Some(from) = queue.pop_front() {
        route.push(from);
        if from == sink {
            return Some(Tour::new(route, ()));
        }

        let mut edges = graph.adjacent_edges(from).collect::<Vec<_>>();
        edges.sort_by(|edge, other| other.weight.sort(edge.weight));

        for EdgeRef { from, to, weight } in edges {
            let index = EdgeIndex::new(from, to);

            if !visited.contains(&index) && !full_edges.contains(&index) {
                queue.push_back(to);
                visited.insert(index);
            }
        }
    }

    None
}

pub(crate) fn _dfs<'a, N, W, G, M>(
    graph: &'a G,
    from: NodeIndex,
    markers: &'a mut Vec<M>,
    mark: M,
) -> impl Iterator<Item = NodeIndex> + 'a
where
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
    M: Default + PartialEq + Copy,
{
    let mut stack = Vec::new();
    stack.push(from);
    markers[from.0] = mark;

    std::iter::from_fn(move || {
        while let Some(from) = stack.pop() {
            for to in graph.adjacent_indices(from) {
                if markers[to.0] == M::default() {
                    stack.push(to);
                    markers[to.0] = mark;
                }
            }
            return Some(from);
        }
        None
    })
}

pub(crate) fn _bfs<'a, N, W, G, M>(
    graph: &'a G,
    from: NodeIndex,
    markers: &'a mut Vec<M>,
    mark: M,
) -> impl Iterator<Item = NodeIndex> + 'a
where
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
    M: Default + PartialEq + Copy,
{
    let mut queue = VecDeque::new();
    queue.push_front(from);
    markers[from.0] = mark;

    std::iter::from_fn(move || {
        while let Some(from) = queue.pop_front() {
            for to in graph.adjacent_indices(from) {
                if markers[to.0] == M::default() {
                    queue.push_back(to);
                    markers[to.0] = mark;
                }
            }
            return Some(from);
        }
        None
    })
}

#[cfg(test)]
mod test {
    extern crate test;
    use crate::{adjacency_matrix::AdjacencyMatrix, prelude::*, test::weightless_undigraph};
    use test::Bencher;

    #[bench]
    fn bfs_connected_components_graph1_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_connected_components().count();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn bfs_connected_components_graph2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_connected_components().count();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn bfs_connected_components_graph3_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_connected_components().count();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn bfs_connected_components_graph_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_connected_components().count();
            assert_eq!(counter, 222);
        });
    }

    #[bench]
    fn bfs_connected_components_graph_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_connected_components().count();
            assert_eq!(counter, 9560);
        });
    }

    #[bench]
    fn bfs_connected_components_graph_ganz_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> =
            weightless_undigraph("data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_connected_components().count();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn dfs_connected_components_graph1_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_connected_components().count();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn dfs_connected_components_graph2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_connected_components().count();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn dfs_connected_components_graph3_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_connected_components().count();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn dfs_connected_components_graph_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_connected_components().count();
            assert_eq!(counter, 222);
        });
    }

    #[bench]
    fn dfs_connected_components_graph_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_connected_components().count();
            assert_eq!(counter, 9560);
        });
    }

    #[bench]
    fn dfs_connected_components_graph_ganz_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> =
            weightless_undigraph("data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_connected_components().count();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn bfs_connected_components_graph1_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_connected_components().count();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn bfs_connected_components_graph2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_connected_components().count();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn bfs_connected_components_graph3_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_connected_components().count();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn dfs_connected_components_graph1_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_connected_components().count();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn dfs_connected_components_graph2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_connected_components().count();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn dfs_connected_components_graph3_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_connected_components().count();
            assert_eq!(counter, 4);
        });
    }
}
