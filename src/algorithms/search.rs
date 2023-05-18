use crate::graph::{GraphAdjacentTopology, GraphTopology};
use crate::indices::NodeIndex;
use std::collections::VecDeque;

use super::Tour;

pub fn depth_search_connected_components<N, W, G>(graph: &G) -> u32
where
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut counter = 0;
    let mut markers = vec![0; graph.node_count()];

    for root in graph.indices() {
        if markers[root.0] == 0 {
            counter += 1;
            _depth_search(graph, root, &mut markers, counter, |_| ());
        }
    }

    counter
}

pub fn breadth_search_connected_components<N, W, G>(graph: &G) -> u32
where
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut counter = 0;
    let mut markers = vec![0; graph.node_count()];

    for root in graph.indices() {
        if markers[root.0] == 0 {
            counter += 1;
            _breadth_search(graph, root, &mut markers, counter, |_| ());
        }
    }

    counter
}

pub fn depth_search_tour<N, W, G>(graph: &G, root: NodeIndex) -> Tour<()>
where
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut markers = vec![false; graph.node_count()];
    let mut route = vec![];

    _depth_search(graph, root, &mut markers, true, |index| route.push(index));

    Tour::new(route, ())
}

pub fn breadth_search_tour<N, W, G>(graph: &G, root: NodeIndex) -> Tour<()>
where
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
{
    let mut markers = vec![false; graph.node_count()];
    let mut route = vec![];

    _breadth_search(graph, root, &mut markers, true, |index| route.push(index));

    Tour::new(route, ())
}

pub(crate) fn _depth_search<N, W, G, M, F>(
    graph: &G,
    root: NodeIndex,
    markers: &mut Vec<M>,
    mark: M,
    mut f: F,
) where
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
    M: Default + PartialEq + Copy,
    F: FnMut(NodeIndex),
{
    let mut stack = Vec::new();
    stack.push(root);
    markers[root.0] = mark;

    while let Some(from) = stack.pop() {
        f(from);
        for to in graph.adjacent_indices(from) {
            if markers[to.0] == M::default() {
                stack.push(to);
                markers[to.0] = mark;
            }
        }
    }
}

pub(crate) fn _breadth_search<N, W, G, M, F>(
    graph: &G,
    root: NodeIndex,
    markers: &mut Vec<M>,
    mark: M,
    mut f: F,
) where
    G: GraphTopology<N, W> + GraphAdjacentTopology<N, W>,
    M: Default + PartialEq + Copy,
    F: FnMut(NodeIndex),
{
    let mut queue = VecDeque::new();
    queue.push_front(root);
    markers[root.0] = mark;

    while let Some(from) = queue.pop_front() {
        f(from);
        for to in graph.adjacent_indices(from) {
            if markers[to.0] == M::default() {
                queue.push_back(to);
                markers[to.0] = mark;
            }
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use crate::{adjacency_matrix::AdjacencyMatrix, prelude::*, test::weightless_undigraph};
    use test::Bencher;

    #[bench]
    fn breadth_search_connected_components_graph1_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = graph.breadth_search_connected_components();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = graph.breadth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph3_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = graph.breadth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let counter = graph.breadth_search_connected_components();
            assert_eq!(counter, 222);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.breadth_search_connected_components();
            assert_eq!(counter, 9560);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_ganz_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> =
            weightless_undigraph("data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.breadth_search_connected_components();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph1_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = graph.depth_search_connected_components();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = graph.depth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph3_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = graph.depth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let counter = graph.depth_search_connected_components();
            assert_eq!(counter, 222);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.depth_search_connected_components();
            assert_eq!(counter, 9560);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_ganz_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> =
            weightless_undigraph("data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.depth_search_connected_components();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph1_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = graph.breadth_search_connected_components();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = graph.breadth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph3_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = graph.breadth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_gross_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let counter = graph.breadth_search_connected_components();
            assert_eq!(counter, 222);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_ganz_gross_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> =
            weightless_undigraph("data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.breadth_search_connected_components();
            assert_eq!(counter, 9560);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_ganz_ganz_gross_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> =
            weightless_undigraph("data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.breadth_search_connected_components();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph1_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = graph.depth_search_connected_components();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = graph.depth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph3_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = graph.depth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_gross_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let counter = graph.depth_search_connected_components();
            assert_eq!(counter, 222);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_ganz_gross_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> =
            weightless_undigraph("data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.depth_search_connected_components();
            assert_eq!(counter, 9560);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_ganz_ganz_gross_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> =
            weightless_undigraph("data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.depth_search_connected_components();
            assert_eq!(counter, 306);
        });
    }
}
