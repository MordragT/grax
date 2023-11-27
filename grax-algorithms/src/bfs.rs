use grax_core::adaptor::partial::PartialGraphAdaptor;
use grax_core::prelude::*;
use grax_core::traits::*;
use grax_core::view::{AttrMap, Parents, VisitMap};
use std::collections::VecDeque;
use std::fmt::Debug;

pub fn bfs_scc<G>(graph: &G) -> Vec<PartialGraphAdaptor<G>>
where
    G: Index + IndexAdjacent + Count + Viewable + Contains,
{
    let mut counter = 0;
    let mut markers = graph.node_map();
    let mut components = Vec::new();

    for from in graph.node_ids() {
        if markers.get(from) == &0 {
            counter += 1;
            let comp = bfs_marker(graph, from, &mut markers, counter);
            components.push(comp);
        }
    }

    components
}

pub fn bfs<G>(graph: &G, from: NodeId<G::Id>) -> PartialGraphAdaptor<'_, G>
where
    G: IndexAdjacent + Count + Viewable + Contains,
{
    let mut markers = graph.node_map();
    bfs_marker(graph, from, &mut markers, true)
}

pub fn bfs_iter<G>(graph: &G, from: NodeId<G::Id>) -> impl Iterator<Item = NodeId<G::Id>> + '_
where
    G: IndexAdjacent + Count + Visitable,
{
    let mut visited = graph.visit_map();
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

pub fn bfs_iter_edges<G>(graph: &G, from: NodeId<G::Id>) -> impl Iterator<Item = EdgeId<G::Id>> + '_
where
    G: IndexAdjacent + Count + Visitable,
{
    let mut visited = graph.visit_map();
    let mut queue = VecDeque::new();

    queue.push_back(from);
    visited.visit(from);

    std::iter::from_fn(move || {
        if let Some(from) = queue.pop_front() {
            for to in graph.adjacent_node_ids(from) {
                if !visited.is_visited(to) {
                    queue.push_back(to);
                    visited.visit(to);
                    return Some(EdgeId::new_unchecked(from, to));
                }
            }
        }
        None
    })
}

pub fn bfs_sp<C, F, G>(
    graph: &G,
    source: NodeId<G::Id>,
    sink: NodeId<G::Id>,
    mut cost_fn: F,
) -> Option<Parents<G>>
where
    F: FnMut(&G::EdgeFlow) -> bool,
    G: IndexAdjacent + Flow<C> + Visitable + Viewable,
{
    let mut queue = VecDeque::new();
    let mut visited = graph.visit_map();
    let mut parents = graph.parents();

    queue.push_front(source);
    visited.visit(source);

    while let Some(from) = queue.pop_front() {
        if from == sink {
            return Some(parents);
        }

        for to in graph.adjacent_node_ids(from) {
            let flow = graph.flow(EdgeId::new_unchecked(from, to)).unwrap();

            if !visited.is_visited(to) && cost_fn(flow) {
                parents.insert(from, to);
                queue.push_back(to);
                visited.visit(to);
            }
        }
    }
    None
}

pub(crate) fn bfs_marker<'a, G, M>(
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
    let mut queue = VecDeque::new();
    tree.keep_node_id(from);
    queue.push_front(from);
    markers.insert(from, mark);

    while let Some(from) = queue.pop_front() {
        for to in graph.adjacent_node_ids(from) {
            if markers.get(to) == &M::default() {
                queue.push_back(to);
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
    use super::bfs_scc;
    use crate::test::weightless_undigraph;
    use grax_impl::*;
    use test::Bencher;

    #[bench]
    fn bfs_scc_graph1_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("../data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn bfs_scc_graph2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("../data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn bfs_scc_graph3_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("../data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 4);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("../data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 222);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> =
            weightless_undigraph("../data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 9560);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_ganz_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> =
            weightless_undigraph("../data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn bfs_scc_graph1_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("../data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn bfs_scc_graph2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("../data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn bfs_scc_graph3_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("../data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 4);
        });
    }
}
