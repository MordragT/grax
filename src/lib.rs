#![feature(adt_const_params)]
#![feature(generators, generator_trait)]
#![feature(test)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Add, AddAssign, Generator},
};

use error::{GraphError, GraphResult};
use structure::{AdjacencyList, Direction, EdgeRef, GraphDataProvider, GraphDataProviderExt};

pub mod error;
pub mod structure;

// pub trait Weight: Debug + Clone + Copy + Eq + PartialEq + Hash {}

// impl<T: Debug + Clone + Copy + Eq + PartialEq + Hash> Weight for T {}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum GraphKind {
    #[default]
    Undirected,
    Directed,
}

pub type DirectedAdjGraph<N, W> = AdjGraph<{ GraphKind::Directed }, N, W>;
pub type UndirectedAdjGraph<N, W> = AdjGraph<{ GraphKind::Undirected }, N, W>;
pub type AdjGraph<const KIND: GraphKind, N, W> = Graph<KIND, N, W, AdjacencyList<KIND, N, W>>;

#[derive(Debug, Default)]
pub struct Graph<const KIND: GraphKind, N: Debug, W: Debug, D: GraphDataProvider<N, W>> {
    data: D,
    node_kind: PhantomData<N>,
    weight_kind: PhantomData<W>,
}

impl<const KIND: GraphKind> AdjGraph<KIND, usize, ()> {
    pub fn from_edge_list(edge_list: &str) -> GraphResult<Self> {
        let data = AdjacencyList::from_edge_list(edge_list)?;

        Ok(Self {
            data,
            node_kind: PhantomData,
            weight_kind: PhantomData,
        })
    }
}

impl<const KIND: GraphKind, N: Debug, W: Debug, D: GraphDataProvider<N, W>> Graph<KIND, N, W, D> {
    pub fn with_data(data: D) -> Self {
        Self {
            data,
            node_kind: PhantomData,
            weight_kind: PhantomData,
        }
    }
}

impl<
        const KIND: GraphKind,
        N: PartialEq + Debug,
        W: PartialEq + Debug,
        D: GraphDataProviderExt<N, W>,
    > Graph<KIND, N, W, D>
{
    pub fn contains_node(&self, node: &N) -> Option<NodeIndex> {
        self.data.contains_node(node)
    }

    pub fn contains_edge(&self, left: NodeIndex, right: NodeIndex) -> Option<EdgeIndex> {
        self.data.contains_edge(left, right)
    }
}

impl<const KIND: GraphKind, N: Debug, W: Debug + Copy, D: GraphDataProvider<N, W>>
    Graph<KIND, N, W, D>
{
    pub fn add_edge(
        &mut self,
        left: NodeIndex,
        right: NodeIndex,
        weight: W,
    ) -> GraphResult<EdgeIndex> {
        match KIND {
            GraphKind::Directed => self.data.add_edge(left, right, weight, Direction::Outgoing),
            GraphKind::Undirected => {
                self.data
                    .add_edge(left, right, weight, Direction::Incoming)?;
                self.data.add_edge(left, right, weight, Direction::Outgoing)
            }
        }
    }
}

impl<
        const KIND: GraphKind,
        N: Debug,
        W: Debug + PartialOrd + Default + AddAssign + Copy,
        D: GraphDataProvider<N, W>,
    > Graph<KIND, N, W, D>
{
    pub fn prim(&self, start: NodeIndex) -> HashSet<NodeIndex> {
        let mut combined_weight = W::default();
        let mut visited = HashSet::new();
        let mut edges = VecDeque::new();

        edges.extend(self.data.adjacent_indices(start));

        while let Some(edge) = edges.pop_front() {
            visited.insert(index);

            let mut current_ref: Option<EdgeRef<W>> = None;
            for adj_ref in self.data.adjacent_edges(index) {
                if !visited.contains(&adj_ref.child) {
                    if let Some(curr_ref) = &current_ref {
                        if curr_ref.weight > adj_ref.weight {
                            current_ref = Some(adj_ref)
                        }
                    } else {
                        current_ref = Some(adj_ref);
                    }
                }
            }
            if let Some(curr_ref) = current_ref {
                combined_weight += *curr_ref.weight;
            }
        }

        todo!()
    }
}

impl<const KIND: GraphKind, N: Debug, W: Debug, D: GraphDataProvider<N, W>> Graph<KIND, N, W, D> {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            node_kind: PhantomData,
            weight_kind: PhantomData,
        }
    }

    pub fn add_node(&mut self, node: N) -> NodeIndex {
        self.data.add_node(node)
    }

    pub fn update_node(&mut self, index: NodeIndex, node: N) -> N {
        self.data.update_node(index, node)
    }

    pub fn update_edge(&mut self, index: EdgeIndex, weight: W) -> W {
        self.data.update_edge(index, weight)
    }

    pub fn get(&self, index: NodeIndex) -> &N {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: NodeIndex) -> &mut N {
        self.data.get_mut(index)
    }

    pub fn weight(&self, index: EdgeIndex) -> &W {
        self.data.weight(index)
    }

    pub fn weight_mut(&mut self, index: EdgeIndex) -> &mut W {
        self.data.weight_mut(index)
    }

    // TODO fix double loop
    fn depth_search_generator(&self, root: usize) -> impl Generator<Yield = &N> + '_ {
        move || {
            let mut visited = vec![false; self.data.node_count()];
            let mut stack = Vec::new();
            visited[root] = true;

            for node in self.data.adjacent_indices(NodeIndex(root)).into_iter() {
                stack.push(node.0)
            }

            while let Some(idx) = stack.pop() {
                visited[idx] = true;
                yield self.get(NodeIndex(idx));
                for node in self.data.adjacent_indices(NodeIndex(idx)) {
                    if visited[idx] == false {
                        stack.push(node.0);
                    }
                }
            }
        }
    }

    // TODO fix double loop
    fn _breadth_search_generator(&self, root: usize) -> impl Generator<Yield = &N> + '_ {
        move || {
            let mut visited = vec![false; self.data.node_count()];
            let mut queue = VecDeque::new();
            visited[root] = true;

            for node in self.data.adjacent_indices(NodeIndex(root)).into_iter() {
                queue.push_back(node.0)
            }

            while let Some(idx) = queue.pop_front() {
                visited[idx] = true;
                yield self.get(NodeIndex(idx));
                for node in self.data.adjacent_indices(NodeIndex(idx)) {
                    if visited[idx] == false {
                        queue.push_back(node.0);
                    }
                }
            }
        }
    }

    fn depth_search(&self, root: usize, markers: &mut Vec<u32>, counter: u32) {
        let mut stack = Vec::new();
        stack.push(root);
        markers[root] = counter;

        while let Some(idx) = stack.pop() {
            for node in self.data.adjacent_indices(NodeIndex(idx)) {
                if markers[node.0] == 0 {
                    stack.push(node.0);
                    markers[node.0] = counter;
                }
            }
        }
    }

    fn breadth_search(&self, root: usize, markers: &mut Vec<u32>, counter: u32) {
        let mut queue = VecDeque::new();
        queue.push_front(root);
        markers[root] = counter;

        while let Some(idx) = queue.pop_front() {
            for node in self.data.adjacent_indices(NodeIndex(idx)) {
                if markers[node.0] == 0 {
                    queue.push_back(node.0);
                    markers[node.0] = counter;
                }
            }
        }
    }

    fn search_connected_components(
        &self,
        search: impl Fn(&Self, usize, &mut Vec<u32>, u32),
    ) -> (u32, Vec<u32>) {
        let mut counter = 0;
        let mut markers = vec![0; self.data.node_count()];

        for root in 0..self.data.node_count() {
            if markers[root] == 0 {
                counter += 1;
                search(&self, root, &mut markers, counter)
            }
        }

        (counter, markers)
    }

    pub fn depth_search_connected_components(&self) -> (u32, Vec<u32>) {
        self.search_connected_components(Self::depth_search)
    }

    pub fn breadth_search_connected_components(&self) -> (u32, Vec<u32>) {
        self.search_connected_components(Self::breadth_search)
    }

    pub fn neighbors(&self, index: NodeIndex) -> impl Generator<Yield = &N> + '_ {
        self.depth_search_generator(index.0)
    }

    pub fn node_count(&self) -> usize {
        self.data.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.data.edge_count()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct NodeIndex(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeIndex {
    parent: NodeIndex,
    child: NodeIndex,
    depth: u32,
}

impl EdgeIndex {
    fn new(parent: NodeIndex, child: NodeIndex, depth: u32) -> Self {
        Self {
            parent,
            child,
            depth,
        }
    }
}

// #[derive(Debug)]
// pub struct Edge<W> {
//     nodes_between: Option<Vec<NodeIndex>>,
//     weight: W,
// }

// impl<W> Edge<W> {
//     pub fn new(weight: W) -> Self {
//         Self {
//             nodes_between: None,
//             weight,
//         }
//     }

//     pub fn with_nodes(nodes_betweeen: Vec<NodeIndex>, weight: W) -> Self {
//         Self {
//             nodes_between: Some(nodes_betweeen),
//             weight,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::UndirectedAdjGraph;
    use std::{
        collections::HashSet,
        fs,
        ops::{Generator, GeneratorState},
        pin::Pin,
    };
    use test::Bencher;

    #[test]
    fn add_node() {
        let mut graph = UndirectedAdjGraph::<u32, ()>::new();
        let _idx1 = graph.add_node(1);
        let _idx2 = graph.add_node(2);
        let _idx3 = graph.add_node(3);

        graph.contains_node(&1).unwrap();
        graph.contains_node(&2).unwrap();
        graph.contains_node(&3).unwrap();

        assert!(graph.contains_node(&100).is_none());
    }

    #[test]
    fn update_node() {
        let mut graph = UndirectedAdjGraph::<u32, ()>::new();
        let idx1 = graph.add_node(1);

        assert_eq!(graph.update_node(idx1, 5), 1);

        graph.contains_node(&5).unwrap();
        assert!(graph.contains_node(&1).is_none());
    }

    #[test]
    fn add_edge() {
        let mut graph = UndirectedAdjGraph::<u32, ()>::new();
        let idx1 = graph.add_node(1);
        let idx2 = graph.add_node(2);
        let idx3 = graph.add_node(3);

        let _ = graph.add_edge(idx1, idx2, ()).unwrap();

        graph.contains_edge(idx1, idx2).unwrap();
        graph.contains_edge(idx2, idx1).unwrap();

        assert!(graph.contains_edge(idx3, idx2).is_none());
    }

    #[test]
    fn update_edge() {
        let mut graph = UndirectedAdjGraph::<u32, u32>::new();
        let idx1 = graph.add_node(1);
        let idx2 = graph.add_node(2);

        let edge = graph.add_edge(idx1, idx2, 2).unwrap();

        assert_eq!(graph.update_edge(edge, 5), 2);
        assert_eq!(graph.weight(edge), &5);
    }

    #[test]
    fn from_edge_list() {
        let edge_list = "4
        0 2
        1 2
        2 3
        3 1";
        let graph = UndirectedAdjGraph::from_edge_list(edge_list).unwrap();

        assert_eq!(graph.node_count(), 4);

        let idx0 = graph.contains_node(&0).unwrap();
        let idx1 = graph.contains_node(&1).unwrap();
        let idx2 = graph.contains_node(&2).unwrap();
        let idx3 = graph.contains_node(&3).unwrap();

        graph.contains_edge(idx0, idx2).unwrap();
        graph.contains_edge(idx1, idx2).unwrap();
        graph.contains_edge(idx2, idx3).unwrap();
        graph.contains_edge(idx3, idx1).unwrap();

        graph.contains_edge(idx2, idx0).unwrap();
        graph.contains_edge(idx2, idx1).unwrap();
        graph.contains_edge(idx3, idx2).unwrap();
        graph.contains_edge(idx1, idx3).unwrap();

        assert!(graph.contains_edge(idx1, idx0).is_none());
    }

    #[test]
    fn neighbors() {
        let edge_list = "4
        0 2
        1 2
        2 3
        3 1";
        let graph = UndirectedAdjGraph::from_edge_list(edge_list).unwrap();

        let idx2 = graph.contains_node(&2).unwrap();

        let mut gen = graph.neighbors(idx2);
        let mut neighbors = HashSet::new();

        while let GeneratorState::Yielded(neighbor) = Pin::new(&mut gen).resume(()) {
            neighbors.insert(*neighbor);
        }

        let mut expected = HashSet::new();
        expected.insert(0);
        expected.insert(1);
        expected.insert(3);

        assert_eq!(neighbors, expected);
    }

    #[bench]
    fn breadth_search_connected_components_graph1(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph1.txt").unwrap();
        let graph = UndirectedAdjGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph2(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph2.txt").unwrap();
        let graph = UndirectedAdjGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph3(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph3.txt").unwrap();
        let graph = UndirectedAdjGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_gross.txt").unwrap();
        let graph = UndirectedAdjGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 222);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_ganz_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_ganzgross.txt").unwrap();
        let graph = UndirectedAdjGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 9560);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_ganz_ganz_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_ganzganzgross.txt").unwrap();
        let graph = UndirectedAdjGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph1(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph1.txt").unwrap();
        let graph = UndirectedAdjGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph2(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph2.txt").unwrap();
        let graph = UndirectedAdjGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph3(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph3.txt").unwrap();
        let graph = UndirectedAdjGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_gross.txt").unwrap();
        let graph = UndirectedAdjGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 222);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_ganz_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_ganzgross.txt").unwrap();
        let graph = UndirectedAdjGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 9560);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_ganz_ganz_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_ganzganzgross.txt").unwrap();
        let graph = UndirectedAdjGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 306);
        });
    }
}
