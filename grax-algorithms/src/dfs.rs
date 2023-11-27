use grax_core::adaptor::partial::PartialGraphAdaptor;
use grax_core::prelude::*;
use grax_core::traits::*;
use grax_core::variant::connected::ConnectedGraph;
use grax_core::view::{AttrMap, Parents, VisitMap};
use std::fmt::Debug;

pub fn dfs_cycle<'a, G>(
    graph: &'a G,
    from: NodeId<G::Id>,
) -> Option<ConnectedGraph<PartialGraphAdaptor<'a, G>>>
where
    G: IndexAdjacent + Count + Visitable + Viewable,
{
    let mut tree = PartialGraphAdaptor::new(graph);
    let mut stack = Vec::new();
    let mut visited = graph.visit_map();
    let mut path = Vec::new();
    stack.push(from);
    visited.visit(from);

    while let Some(from) = stack.pop() {
        for to in graph.adjacent_node_ids(from) {
            if !visited.is_visited(to) {
                stack.push(to);
                path.push(to);
                visited.visit(to);
                tree.keep_edge_id(EdgeId::new_unchecked(from, to));
            } else if path.contains(&to) {
                return None;
            }
        }

        path.pop();
    }

    Some(ConnectedGraph::from_unchecked(tree, from))
}

pub fn dfs_scc<G>(graph: &G) -> Vec<PartialGraphAdaptor<G>>
where
    G: Index + IndexAdjacent + Count + Viewable + Contains,
{
    let mut counter = 0;
    let mut markers = graph.node_map();
    let mut components = Vec::new();

    for from in graph.node_ids() {
        if markers.get(from) == &0 {
            counter += 1;
            let comp = dfs_marker(graph, from, &mut markers, counter);
            components.push(comp);
        }
    }

    components
}

pub fn dfs<G>(graph: &G, from: NodeId<G::Id>) -> PartialGraphAdaptor<'_, G>
where
    G: IndexAdjacent + Count + Viewable + Contains,
{
    let mut markers = graph.node_map();
    dfs_marker(graph, from, &mut markers, true)
}

pub fn dfs_iter<G>(graph: &G, from: NodeId<G::Id>) -> impl Iterator<Item = NodeId<G::Id>> + '_
where
    G: IndexAdjacent + Count + Visitable,
{
    let mut visited = graph.visit_map();
    let mut queue = Vec::new();

    queue.push(from);
    visited.visit(from);

    std::iter::from_fn(move || {
        if let Some(from) = queue.pop() {
            for to in graph.adjacent_node_ids(from) {
                if !visited.is_visited(to) {
                    queue.push(to);
                    visited.visit(to);
                }
            }
            Some(from)
        } else {
            None
        }
    })
}

pub fn dfs_iter_edges<G>(graph: &G, from: NodeId<G::Id>) -> impl Iterator<Item = EdgeId<G::Id>> + '_
where
    G: IndexAdjacent + Count + Visitable,
{
    let mut visited = graph.visit_map();
    let mut queue = Vec::new();

    queue.push(from);
    visited.visit(from);

    std::iter::from_fn(move || {
        if let Some(from) = queue.pop() {
            for to in graph.adjacent_node_ids(from) {
                if !visited.is_visited(to) {
                    queue.push(to);
                    visited.visit(to);
                    return Some(EdgeId::new_unchecked(from, to));
                }
            }
        }
        None
    })
}

pub fn dfs_sp<C, F, G>(
    graph: &G,
    source: NodeId<G::Id>,
    sink: NodeId<G::Id>,
    mut cost_fn: F,
) -> Option<Parents<G>>
where
    F: FnMut(&G::EdgeFlow) -> bool,
    G: IndexAdjacent + Flow<C> + Visitable + Viewable,
{
    let mut queue = Vec::new();
    let mut visited = graph.visit_map();
    let mut parents = graph.parents();

    queue.push(source);
    visited.visit(source);

    while let Some(from) = queue.pop() {
        if from == sink {
            return Some(parents);
        }

        for to in graph.adjacent_node_ids(from) {
            let flow = graph.flow(EdgeId::new_unchecked(from, to)).unwrap();

            if !visited.is_visited(to) && cost_fn(flow) {
                parents.insert(from, to);
                queue.push(to);
                visited.visit(to);
            }
        }
    }
    None
}

pub(crate) fn dfs_marker<'a, G, M>(
    graph: &'a G,
    from: NodeId<G::Id>,
    markers: &mut G::NodeMap<M>,
    mark: M,
) -> PartialGraphAdaptor<'a, G>
where
    G: IndexAdjacent + Count + Viewable + Contains,
    M: Default + PartialEq + Copy + Debug,
{
    let mut tree = PartialGraphAdaptor::new(graph);
    let mut queue = Vec::new();
    tree.keep_node_id(from);
    queue.push(from);
    markers.insert(from, mark);

    while let Some(from) = queue.pop() {
        for to in graph.adjacent_node_ids(from) {
            if markers.get(to) == &M::default() {
                queue.push(to);
                markers.insert(to, mark);
                tree.keep_edge(from, to);
            }
        }
    }

    tree
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
        let graph: AdjacencyList<_, _> = weightless_undigraph("../data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = dfs_scc(&graph).len();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn dfs_scc_graph2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("../data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = dfs_scc(&graph).len();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn dfs_scc_graph3_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("../data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = dfs_scc(&graph).len();
            assert_eq!(counter, 4);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn dfs_scc_graph_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("../data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let counter = dfs_scc(&graph).len();
            assert_eq!(counter, 222);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn dfs_scc_graph_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> =
            weightless_undigraph("../data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let counter = dfs_scc(&graph).len();
            assert_eq!(counter, 9560);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn dfs_scc_graph_ganz_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> =
            weightless_undigraph("../data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let counter = dfs_scc(&graph).len();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn dfs_scc_graph1_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("../data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = dfs_scc(&graph).len();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn dfs_scc_graph2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("../data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = dfs_scc(&graph).len();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn dfs_scc_graph3_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("../data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = dfs_scc(&graph).len();
            assert_eq!(counter, 4);
        });
    }
}
