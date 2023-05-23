use super::{ConnectedComponents, Tour};
use crate::{
    graph::{Count, Index, IndexAdjacent},
    prelude::NodeIdentifier,
};
use std::collections::VecDeque;

pub fn dfs_connected_components<G>(graph: &G) -> ConnectedComponents<G::NodeId>
where
    G: Index + IndexAdjacent + Count,
{
    let mut counter = 0;
    let mut markers = vec![0; graph.node_count()];
    let mut components = Vec::new();

    for from in graph.node_ids() {
        if markers[from.as_usize()] == 0 {
            counter += 1;
            let comp = _dfs(graph, from, &mut markers, counter).collect();
            components.push(comp);
        }
    }

    ConnectedComponents::new(components)
}

pub fn bfs_connected_components<G>(graph: &G) -> ConnectedComponents<G::NodeId>
where
    G: Index + IndexAdjacent + Count,
{
    let mut counter = 0;
    let mut markers = vec![0; graph.node_count()];
    let mut components = Vec::new();

    for from in graph.node_ids() {
        if markers[from.as_usize()] == 0 {
            counter += 1;
            let comp = _bfs(graph, from, &mut markers, counter).collect();
            components.push(comp);
        }
    }

    ConnectedComponents::new(components)
}

pub fn dfs_tour<G>(graph: &G, from: G::NodeId) -> Tour<G::NodeId, ()>
where
    G: Index + IndexAdjacent + Count,
{
    let mut markers = vec![false; graph.node_count()];
    let route = _dfs(graph, from, &mut markers, true).collect();

    Tour::new(route, ())
}

pub fn bfs_tour<G>(graph: &G, from: G::NodeId) -> Tour<G::NodeId, ()>
where
    G: Index + IndexAdjacent + Count,
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

pub(crate) fn _dfs<'a, G, M>(
    graph: &'a G,
    from: G::NodeId,
    markers: &'a mut Vec<M>,
    mark: M,
) -> impl Iterator<Item = G::NodeId> + 'a
where
    G: Index + IndexAdjacent,
    M: Default + PartialEq + Copy,
{
    let mut stack = Vec::new();
    stack.push(from);
    markers[from.as_usize()] = mark;

    std::iter::from_fn(move || {
        while let Some(from) = stack.pop() {
            for to in graph.adjacent_node_ids(from) {
                if markers[to.as_usize()] == M::default() {
                    stack.push(to);
                    markers[to.as_usize()] = mark;
                }
            }
            return Some(from);
        }
        None
    })
}

pub(crate) fn _bfs<'a, G, M>(
    graph: &'a G,
    from: G::NodeId,
    markers: &'a mut Vec<M>,
    mark: M,
) -> impl Iterator<Item = G::NodeId> + 'a
where
    G: Index + IndexAdjacent,
    M: Default + PartialEq + Copy,
{
    let mut queue = VecDeque::new();
    queue.push_front(from);
    markers[from.as_usize()] = mark;

    std::iter::from_fn(move || {
        while let Some(from) = queue.pop_front() {
            for to in graph.adjacent_node_ids(from) {
                if markers[to.as_usize()] == M::default() {
                    queue.push_back(to);
                    markers[to.as_usize()] = mark;
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
    use crate::{prelude::*, test::weightless_undigraph};
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
