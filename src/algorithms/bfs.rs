use crate::{
    graph::{Count, Index, IndexAdjacent},
    prelude::{EdgeIdentifier, NodeIdentifier, Tree},
};
use std::collections::VecDeque;

pub fn bfs_scc<G>(graph: &G) -> Vec<Tree<G>>
where
    G: Index + IndexAdjacent + Count,
{
    let mut counter = 0;
    let mut markers = vec![0; graph.node_count()];
    let mut components = Vec::new();

    for from in graph.node_ids() {
        if markers[from.as_usize()] == 0 {
            counter += 1;
            let comp = bfs_marker(graph, from, &mut markers, counter);
            components.push(comp);
        }
    }

    components
}

pub fn bfs<G>(graph: &G, from: G::NodeId) -> Tree<G>
where
    G: Index + IndexAdjacent + Count,
{
    let mut markers = vec![false; graph.node_count()];
    bfs_marker(graph, from, &mut markers, true)
}

pub fn bfs_iter<G>(graph: &G, from: G::NodeId) -> impl Iterator<Item = G::NodeId> + '_
where
    G: Index + IndexAdjacent + Count,
{
    let mut visited = vec![false; graph.node_count()];
    let mut queue = VecDeque::new();

    queue.push_back(from);
    visited[from.as_usize()] = true;

    std::iter::from_fn(move || {
        if let Some(from) = queue.pop_front() {
            for to in graph.adjacent_node_ids(from) {
                if !visited[to.as_usize()] {
                    queue.push_back(to);
                    visited[to.as_usize()] = true;
                }
            }
            Some(from)
        } else {
            None
        }
    })
}

pub fn bfs_iter_edges<G>(graph: &G, from: G::NodeId) -> impl Iterator<Item = G::EdgeId> + '_
where
    G: Index + IndexAdjacent + Count,
{
    let mut visited = vec![false; graph.node_count()];
    let mut queue = VecDeque::new();

    queue.push_back(from);
    visited[from.as_usize()] = true;

    std::iter::from_fn(move || {
        if let Some(from) = queue.pop_front() {
            for to in graph.adjacent_node_ids(from) {
                if !visited[to.as_usize()] {
                    queue.push_back(to);
                    visited[to.as_usize()] = true;
                    return Some(G::EdgeId::between(from, to));
                }
            }
        }
        None
    })
}

pub fn bfs_marker<'a, G, M>(
    graph: &'a G,
    from: G::NodeId,
    markers: &'a mut Vec<M>,
    mark: M,
) -> Tree<G>
where
    G: Index + IndexAdjacent + Count,
    M: Default + PartialEq + Copy,
{
    let mut tree = Tree::new(from, graph.node_count());
    let mut queue = VecDeque::new();
    queue.push_front(from);
    markers[from.as_usize()] = mark;

    while let Some(from) = queue.pop_front() {
        for to in graph.adjacent_node_ids(from) {
            if markers[to.as_usize()] == M::default() {
                queue.push_back(to);
                markers[to.as_usize()] = mark;
                tree.insert(from, to);
            }
        }
    }

    tree
}

#[cfg(test)]
mod test {
    extern crate test;
    use crate::{prelude::*, test::weightless_undigraph};
    use test::Bencher;

    #[bench]
    fn bfs_scc_graph1_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_scc().len();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn bfs_scc_graph2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_scc().len();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn bfs_scc_graph3_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_scc().len();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn bfs_scc_graph_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_scc().len();
            assert_eq!(counter, 222);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_scc().len();
            assert_eq!(counter, 9560);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_ganz_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> =
            weightless_undigraph("data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_scc().len();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn bfs_scc_graph1_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_scc().len();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn bfs_scc_graph2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_scc().len();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn bfs_scc_graph3_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = graph.bfs_scc().len();
            assert_eq!(counter, 4);
        });
    }
}
