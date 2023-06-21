use crate::{
    graph::{Base, Count, Get, Index, IndexAdjacent},
    prelude::{EdgeId, NodeId, Tree},
    structures::Parents,
};

pub fn dfs_scc<G>(graph: &G) -> Vec<Tree<G>>
where
    G: Index + IndexAdjacent + Count,
{
    let mut counter = 0;
    let mut markers = vec![0; graph.node_count()];
    let mut components = Vec::new();

    for from in graph.node_ids() {
        if markers[from.as_usize()] == 0 {
            counter += 1;
            let comp = dfs_marker(graph, from, &mut markers, counter);
            components.push(comp);
        }
    }

    components
}

pub fn dfs<G>(graph: &G, from: NodeId<G::Id>) -> Tree<G>
where
    G: IndexAdjacent + Count,
{
    let mut markers = vec![false; graph.node_count()];
    dfs_marker(graph, from, &mut markers, true)
}

pub fn dfs_iter<G>(graph: &G, from: NodeId<G::Id>) -> impl Iterator<Item = NodeId<G::Id>> + '_
where
    G: IndexAdjacent + Count,
{
    let mut visited = vec![false; graph.node_count()];
    let mut stack = Vec::new();

    stack.push(from);
    visited[from.as_usize()] = true;

    std::iter::from_fn(move || {
        if let Some(from) = stack.pop() {
            for to in graph.adjacent_node_ids(from) {
                if !visited[to.as_usize()] {
                    stack.push(to);
                    visited[to.as_usize()] = true;
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
    G: IndexAdjacent + Count,
{
    let mut visited = vec![false; graph.node_count()];
    let mut stack = Vec::new();

    stack.push(from);
    visited[from.as_usize()] = true;

    std::iter::from_fn(move || {
        if let Some(from) = stack.pop() {
            for to in graph.adjacent_node_ids(from) {
                if !visited[to.as_usize()] {
                    stack.push(to);
                    visited[to.as_usize()] = true;
                    return Some(EdgeId::new_unchecked(from, to));
                }
            }
        }
        None
    })
}

pub fn dfs_sp<N, W, F, G>(
    graph: &G,
    source: NodeId<G::Id>,
    sink: NodeId<G::Id>,
    mut cost: F,
) -> Option<Parents<G>>
where
    F: FnMut(&W) -> bool,
    G: IndexAdjacent + Count + Get + Base<Node = N, Weight = W>,
{
    let count = graph.node_count();
    let mut stack = Vec::new();
    let mut visited = vec![false; count];
    let mut parents = Parents::with_count(count);

    stack.push(source);
    visited[source.as_usize()] = true;

    while let Some(from) = stack.pop() {
        if from == sink {
            return Some(parents);
        }

        for to in graph.adjacent_node_ids(from) {
            let weight = graph.weight(EdgeId::new_unchecked(from, to)).unwrap();

            if !visited[to.as_usize()] && cost(weight) {
                parents.insert(from, to);
                stack.push(to);
                visited[to.as_usize()] = true;
            }
        }
    }
    None
}

pub(crate) fn dfs_marker<'a, G, M>(
    graph: &'a G,
    from: NodeId<G::Id>,
    markers: &mut Vec<M>,
    mark: M,
) -> Tree<'a, G>
where
    G: IndexAdjacent + Count,
    M: Default + PartialEq + Copy,
{
    let mut tree = Tree::new(from, graph);
    let mut stack = Vec::new();
    stack.push(from);
    markers[from.as_usize()] = mark;

    while let Some(from) = stack.pop() {
        for to in graph.adjacent_node_ids(from) {
            if markers[to.as_usize()] == M::default() {
                stack.push(to);
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
    fn dfs_scc_graph1_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_scc().len();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn dfs_scc_graph2_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_scc().len();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn dfs_scc_graph3_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_scc().len();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn dfs_scc_graph_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph_gross.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_scc().len();
            assert_eq!(counter, 222);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn dfs_scc_graph_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> = weightless_undigraph("data/Graph_ganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_scc().len();
            assert_eq!(counter, 9560);
        });
    }

    #[cfg(feature = "extensive")]
    #[bench]
    fn dfs_scc_graph_ganz_ganz_gross_adj_list(b: &mut Bencher) {
        let graph: AdjacencyList<_, _> =
            weightless_undigraph("data/Graph_ganzganzgross.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_scc().len();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn dfs_scc_graph1_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph1.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_scc().len();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn dfs_scc_graph2_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph2.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_scc().len();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn dfs_scc_graph3_adj_mat(b: &mut Bencher) {
        let graph: AdjacencyMatrix<_, _> = weightless_undigraph("data/Graph3.txt").unwrap();

        b.iter(|| {
            let counter = graph.dfs_scc().len();
            assert_eq!(counter, 4);
        });
    }
}
