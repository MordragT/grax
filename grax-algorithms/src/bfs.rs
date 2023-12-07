use grax_core::collections::FixedNodeMap;
use grax_core::collections::GetNodeMut;
use grax_core::collections::NodeIter;
use grax_core::collections::VisitNodeMap;
use grax_core::graph::EdgeAttribute;
use grax_core::graph::EdgeIterAdjacent;
use grax_core::graph::NodeAttribute;
use grax_core::graph::NodeIterAdjacent;
use grax_core::prelude::*;
use grax_core::view::FilterEdgeView;
use grax_core::view::Parents;
use std::collections::VecDeque;
use std::fmt::Debug;

pub fn bfs_scc<G>(graph: &G) -> Vec<FilterEdgeView<G>>
where
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent + NodeIter,
{
    let mut counter = 0;
    let mut markers = graph.fixed_node_map(counter);
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

pub fn bfs<G>(graph: &G, from: NodeId<G::Key>) -> FilterEdgeView<G>
where
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent,
{
    let mut markers = graph.fixed_node_map(false);
    bfs_marker(graph, from, &mut markers, true)
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

pub fn bfs_sp<F, G>(
    graph: &G,
    source: NodeId<G::Key>,
    sink: NodeId<G::Key>,
    mut cost_fn: F,
) -> Option<Parents<G>>
where
    F: FnMut(&G::EdgeWeight) -> bool,
    G: EdgeAttribute + NodeAttribute + EdgeIterAdjacent,
{
    let mut queue = VecDeque::new();
    let mut visited = graph.visit_node_map();
    let mut parents = Parents::new(graph);

    queue.push_front(source);
    visited.visit(source);

    while let Some(from) = queue.pop_front() {
        if from == sink {
            return Some(parents);
        }

        for EdgeRef { edge_id, weight } in graph.iter_adjacent_edges(from) {
            let to = edge_id.to();
            if !visited.is_visited(to) && cost_fn(weight) {
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
) -> FilterEdgeView<G>
where
    G: NodeAttribute + EdgeAttribute + EdgeIterAdjacent,
    M: Default + PartialEq + Copy + Debug,
{
    let mut filter = FilterEdgeView::new(graph);
    let mut queue = VecDeque::new();
    queue.push_front(from);
    markers.update_node(from, mark);

    while let Some(from) = queue.pop_front() {
        for edge_id in graph.adjacent_edge_ids(from) {
            let to = edge_id.to();
            if markers.get(to) == &M::default() {
                queue.push_back(to);
                markers.update_node(to, mark);
                filter.keep(edge_id);
            }
        }
    }

    filter
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
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn bfs_scc_graph2_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn bfs_scc_graph3_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 4);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_gross_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 222);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> = weightless_undigraph("../data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 9560);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn bfs_scc_graph_ganz_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjGraph<_, _> =
            weightless_undigraph("../data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let counter = bfs_scc(&graph).len();
            assert_eq!(counter, 306);
        });
    }
}
