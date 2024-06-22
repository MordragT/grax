use crate::problems::{Path, PathFinder, PathTree};
use crate::util::Parents;

use grax_core::collections::*;
use grax_core::graph::*;
use grax_core::prelude::*;
use std::collections::VecDeque;
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Bfs;

impl<G> PathFinder<G> for Bfs
where
    G: NodeAttribute + EdgeIterAdjacent,
{
    fn path_tree_where<F>(self, graph: &G, from: NodeId<G::Key>, filter: F) -> PathTree<G>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool,
    {
        bfs_where(graph, from, filter)
    }

    fn path_where<F>(
        self,
        graph: &G,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
        filter: F,
    ) -> Option<Path<G>>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool,
    {
        bfs_to_where(graph, from, to, filter)
    }
}

pub fn bfs_scc<G>(graph: &G) -> (u32, G::FixedNodeMap<u32>)
where
    G: NodeAttribute + EdgeIterAdjacent + NodeIter,
{
    let mut counter = 0;
    let mut markers = graph.fixed_node_map(counter);

    for from in graph.node_ids() {
        if markers.get(from) == &0 {
            counter += 1;
            bfs_marker(graph, from, &mut markers, counter);
        }
    }

    (counter, markers)
}

pub fn bfs_bipartite<G>(graph: &G, from: NodeId<G::Key>) -> (bool, G::FixedNodeMap<u8>)
where
    G: NodeAttribute + EdgeIterAdjacent + NodeIter,
{
    let mut color = graph.fixed_node_map(0);
    let mut queue = VecDeque::new();

    queue.push_front(from);
    color.update_node(from, 1);

    while let Some(from) = queue.pop_front() {
        for edge_id in graph.adjacent_edge_ids(from) {
            let to = edge_id.to();
            if color.get(to) == &0 {
                queue.push_back(to);
                color.update_node(to, !color.get(from));
            } else if color.get(to) == color.get(from) {
                return (false, color);
            }
        }
    }

    (true, color)
}

pub fn bfs<G>(graph: &G, from: NodeId<G::Key>) -> G::FixedNodeMap<bool>
where
    G: NodeAttribute + EdgeIterAdjacent,
{
    let mut markers = graph.visit_node_map();
    bfs_marker(graph, from, &mut markers, true);
    markers
}

pub fn bfs_iter<G>(graph: &G, from: NodeId<G::Key>) -> impl Iterator<Item = NodeId<G::Key>> + '_
where
    G: NodeAttribute + NodeIterAdjacent,
{
    let mut visited = graph.visit_node_map();
    let mut queue = VecDeque::new();

    queue.push_back(from);
    visited.visit(from);

    std::iter::from_fn(move || {
        if let Some(from) = queue.pop_front() {
            for to in graph.adjacent_node_ids(from) {
                if !visited.is_visited(to) {
                    queue.push_back(to);
                    visited.visit(to);
                }
            }
            Some(from)
        } else {
            None
        }
    })
}

pub fn bfs_iter_edges<G>(
    graph: &G,
    from: NodeId<G::Key>,
) -> impl Iterator<Item = EdgeId<G::Key>> + '_
where
    G: NodeAttribute + EdgeIterAdjacent,
{
    let mut visited = graph.visit_node_map();
    let mut queue = VecDeque::new();

    queue.push_back(from);
    visited.visit(from);

    std::iter::from_fn(move || {
        if let Some(from) = queue.pop_front() {
            for edge_id in graph.adjacent_edge_ids(from) {
                let to = edge_id.to();
                if !visited.is_visited(to) {
                    queue.push_back(to);
                    visited.visit(to);
                    return Some(edge_id);
                }
            }
        }
        None
    })
}

pub fn bfs_where<F, G>(graph: &G, from: NodeId<G::Key>, filter: F) -> PathTree<G>
where
    F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool,
    G: NodeAttribute + EdgeIterAdjacent,
{
    let mut queue = VecDeque::new();
    let mut visited = graph.visit_node_map();
    let mut parents = Parents::new(graph);

    queue.push_front(from);
    visited.visit(from);

    while let Some(from) = queue.pop_front() {
        for edge in graph.iter_adjacent_edges(from) {
            let to = edge.edge_id.to();
            if !visited.is_visited(to) && filter(edge) {
                parents.insert(from, to);
                queue.push_back(to);
                visited.visit(to);
            }
        }
    }
    PathTree { from, parents }
}

pub fn bfs_to_where<F, G>(
    graph: &G,
    source: NodeId<G::Key>,
    sink: NodeId<G::Key>,
    filter: F,
) -> Option<Path<G>>
where
    F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool,
    G: NodeAttribute + EdgeIterAdjacent,
{
    let mut queue = VecDeque::new();
    let mut visited = graph.visit_node_map();
    let mut parents = Parents::new(graph);

    queue.push_front(source);
    visited.visit(source);

    while let Some(from) = queue.pop_front() {
        if from == sink {
            return Some(Path {
                from: source,
                to: sink,
                parents,
            });
        }

        for edge in graph.iter_adjacent_edges(from) {
            let to = edge.edge_id.to();
            if !visited.is_visited(to) && filter(edge) {
                parents.insert(from, to);
                queue.push_back(to);
                visited.visit(to);
            }
        }
    }
    None
}

pub(crate) fn bfs_marker<G, M>(
    graph: &G,
    from: NodeId<G::Key>,
    markers: &mut G::FixedNodeMap<M>,
    mark: M,
) where
    G: NodeAttribute + EdgeIterAdjacent,
    M: Default + PartialEq + Copy + Debug,
{
    let mut queue = VecDeque::new();
    queue.push_front(from);
    markers.update_node(from, mark);

    while let Some(from) = queue.pop_front() {
        for edge_id in graph.adjacent_edge_ids(from) {
            let to = edge_id.to();
            if markers.get(to) == &M::default() {
                queue.push_back(to);
                markers.update_node(to, mark);
            }
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::{bfs_bipartite, bfs_scc};
    use crate::test::{id, weightless_undigraph};
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn bfs_bipartite_test(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = AdjGraph::with_edges(
            [
                (0, 1, ()),
                (0, 3, ()),
                (1, 0, ()),
                (1, 2, ()),
                (2, 1, ()),
                (2, 3, ()),
                (3, 0, ()),
                (3, 2, ()),
            ],
            4,
        );

        b.iter(|| {
            let (is_bipartite, _) = bfs_bipartite(&graph, id(0));
            assert!(is_bipartite);
        });
    }

    #[bench]
    fn bfs_scc_graph1_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph1.txt").unwrap();

        b.iter(|| {
            let (counter, _) = bfs_scc(&graph);
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn bfs_scc_graph2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph2.txt").unwrap();

        b.iter(|| {
            let (counter, _) = bfs_scc(&graph);
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn bfs_scc_graph3_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph3.txt").unwrap();

        b.iter(|| {
            let (counter, _) = bfs_scc(&graph);
            assert_eq!(counter, 4);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_gross_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let (counter, _) = bfs_scc(&graph);
            assert_eq!(counter, 222);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let (counter, _) = bfs_scc(&graph);
            assert_eq!(counter, 9560);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_ganz_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> =
            weightless_undigraph("../data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let (counter, _) = bfs_scc(&graph);
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn bfs_scc_graph1_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = weightless_undigraph("../data/Graph1.txt").unwrap();

        b.iter(|| {
            let (counter, _) = bfs_scc(&graph);
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn bfs_scc_graph2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = weightless_undigraph("../data/Graph2.txt").unwrap();

        b.iter(|| {
            let (counter, _) = bfs_scc(&graph);
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn bfs_scc_graph3_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = weightless_undigraph("../data/Graph3.txt").unwrap();

        b.iter(|| {
            let (counter, _) = bfs_scc(&graph);
            assert_eq!(counter, 4);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_gross_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = weightless_undigraph("../data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let (counter, _) = bfs_scc(&graph);
            assert_eq!(counter, 222);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_ganz_gross_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = weightless_undigraph("../data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let (counter, _) = bfs_scc(&graph);
            assert_eq!(counter, 9560);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_ganz_ganz_gross_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> =
            weightless_undigraph("../data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let (counter, _) = bfs_scc(&graph);
            assert_eq!(counter, 306);
        });
    }
}
