use super::PathFinder;
use crate::{parents::Parents, path::Path, tree::PathTree};

use grax_core::collections::GetNodeMut;
use grax_core::collections::NodeIter;
use grax_core::collections::VisitNodeMap;
use grax_core::graph::EdgeIterAdjacent;
use grax_core::graph::NodeAttribute;
use grax_core::graph::NodeIterAdjacent;
use grax_core::prelude::*;
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Dfs;

impl<G> PathFinder<G> for Dfs
where
    G: NodeAttribute + EdgeIterAdjacent,
{
    fn path_tree_where<F>(self, graph: &G, from: NodeId<G::Key>, filter: F) -> PathTree<G>
    where
        F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool,
    {
        dfs_where(graph, from, filter)
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
        dfs_to_where(graph, from, to, filter)
    }
}

pub fn dfs_scc<G>(graph: &G) -> (u32, G::FixedNodeMap<u32>)
where
    G: NodeAttribute + EdgeIterAdjacent + NodeIter,
{
    let mut counter = 0;
    let mut markers = graph.fixed_node_map(counter);

    for from in graph.node_ids() {
        if markers[from] == 0 {
            counter += 1;
            dfs_marker(graph, from, &mut markers, counter);
        }
    }

    (counter, markers)
}

pub fn dfs<G>(graph: &G, from: NodeId<G::Key>) -> G::FixedNodeMap<bool>
where
    G: NodeAttribute + EdgeIterAdjacent,
{
    let mut markers = graph.visit_node_map();
    dfs_marker(graph, from, &mut markers, true);
    markers
}

pub fn dfs_iter<G>(graph: &G, from: NodeId<G::Key>) -> impl Iterator<Item = NodeId<G::Key>> + '_
where
    G: NodeAttribute + NodeIterAdjacent,
{
    let mut visited = graph.visit_node_map();
    let mut stack = Vec::new();

    stack.push(from);
    visited.visit(from);

    std::iter::from_fn(move || {
        if let Some(from) = stack.pop() {
            for to in graph.adjacent_node_ids(from) {
                if !visited.is_visited(to) {
                    stack.push(to);
                    visited.visit(to);
                }
            }
            Some(from)
        } else {
            None
        }
    })
}

pub fn dfs_iter_edges<G>(
    graph: &G,
    from: NodeId<G::Key>,
) -> impl Iterator<Item = EdgeId<G::Key>> + '_
where
    G: NodeAttribute + EdgeIterAdjacent,
{
    let mut visited = graph.visit_node_map();
    let mut stack = Vec::new();

    stack.push(from);
    visited.visit(from);

    std::iter::from_fn(move || {
        if let Some(from) = stack.pop() {
            for edge_id in graph.adjacent_edge_ids(from) {
                let to = edge_id.to();
                if !visited.is_visited(to) {
                    stack.push(to);
                    visited.visit(to);
                    return Some(edge_id);
                }
            }
        }
        None
    })
}

pub fn dfs_where<F, G>(graph: &G, source: NodeId<G::Key>, filter: F) -> PathTree<G>
where
    F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool,
    G: NodeAttribute + EdgeIterAdjacent,
{
    let mut stack = Vec::new();
    let mut visited = graph.visit_node_map();
    let mut parents = Parents::new(graph);

    stack.push(source);
    visited.visit(source);

    while let Some(from) = stack.pop() {
        for edge in graph.iter_adjacent_edges(from) {
            let to = edge.edge_id.to();
            if !visited.is_visited(to) && filter(edge) {
                parents.insert(from, to);
                stack.push(to);
                visited.visit(to);
            }
        }
    }
    PathTree {
        from: source,
        parents,
    }
}

pub fn dfs_to_where<F, G>(
    graph: &G,
    source: NodeId<G::Key>,
    sink: NodeId<G::Key>,
    filter: F,
) -> Option<Path<G>>
where
    F: Fn(EdgeRef<G::Key, G::EdgeWeight>) -> bool,
    G: NodeAttribute + EdgeIterAdjacent,
{
    let mut stack = Vec::new();
    let mut visited = graph.visit_node_map();
    let mut parents = Parents::new(graph);

    stack.push(source);
    visited.visit(source);

    while let Some(from) = stack.pop() {
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
                stack.push(to);
                visited.visit(to);
            }
        }
    }
    None
}
pub(crate) fn dfs_marker<'a, G, M>(
    graph: &'a G,
    from: NodeId<G::Key>,
    markers: &mut G::FixedNodeMap<M>,
    mark: M,
) where
    G: NodeAttribute + EdgeIterAdjacent,
    M: Default + PartialEq + Copy + Debug,
{
    let mut stack = Vec::new();
    stack.push(from);
    markers.update_node(from, mark);

    while let Some(from) = stack.pop() {
        for edge_id in graph.adjacent_edge_ids(from) {
            let to = edge_id.to();
            if markers[to] == M::default() {
                stack.push(to);
                markers.update_node(to, mark);
            }
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::dfs_scc;
    use crate::test::weightless_undigraph;
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn dfs_scc_graph1_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph1.txt").unwrap();

        b.iter(|| {
            let (counter, _) = dfs_scc(&graph);
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn dfs_scc_graph2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph2.txt").unwrap();

        b.iter(|| {
            let (counter, _) = dfs_scc(&graph);
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn dfs_scc_graph3_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph3.txt").unwrap();

        b.iter(|| {
            let (counter, _) = dfs_scc(&graph);
            assert_eq!(counter, 4);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn dfs_scc_graph_gross_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let (counter, _) = dfs_scc(&graph);
            assert_eq!(counter, 222);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn dfs_scc_graph_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let (counter, _) = dfs_scc(&graph);
            assert_eq!(counter, 9560);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn dfs_scc_graph_ganz_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> =
            weightless_undigraph("../data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let (counter, _) = dfs_scc(&graph);
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn dfs_scc_graph1_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = weightless_undigraph("../data/Graph1.txt").unwrap();

        b.iter(|| {
            let (counter, _) = dfs_scc(&graph);
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn dfs_scc_graph2_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = weightless_undigraph("../data/Graph2.txt").unwrap();

        b.iter(|| {
            let (counter, _) = dfs_scc(&graph);
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn dfs_scc_graph3_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = weightless_undigraph("../data/Graph3.txt").unwrap();

        b.iter(|| {
            let (counter, _) = dfs_scc(&graph);
            assert_eq!(counter, 4);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn dfs_scc_graph_gross_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = weightless_undigraph("../data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let (counter, _) = dfs_scc(&graph);
            assert_eq!(counter, 222);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn dfs_scc_graph_ganz_gross_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> = weightless_undigraph("../data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let (counter, _) = dfs_scc(&graph);
            assert_eq!(counter, 9560);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn dfs_scc_graph_ganz_ganz_gross_csr_mat(b: &mut Bencher) {
        let graph: CsrGraph<_, _> =
            weightless_undigraph("../data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let (counter, _) = dfs_scc(&graph);
            assert_eq!(counter, 306);
        });
    }
}
