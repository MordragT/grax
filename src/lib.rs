#![feature(adt_const_params)]
#![feature(generators, generator_trait)]
#![feature(test)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
    num::ParseIntError,
    ops::Generator,
};
use thiserror::Error;

pub type GraphResult<T> = Result<T, GraphError>;

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("The edge between {left:?} and {right:?} already exists.")]
    EdgeAlreadyExists { left: NodeIndex, right: NodeIndex },
    #[error("Two sided edge forbidden between {left:?} and {right:?} in directed graph.")]
    TwoSidedEdgeForbidden { left: NodeIndex, right: NodeIndex },
    #[error("The given edge list has a bad format")]
    BadEdgeListFormat,
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] ParseIntError),
}

pub trait Weight: Debug + Clone + Copy + Eq + PartialEq + Hash {}

impl<T: Debug + Clone + Copy + Eq + PartialEq + Hash> Weight for T {}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum GraphKind {
    #[default]
    Undirected,
    Directed,
}

pub type DirectedGraph<N, W> = Graph<{ GraphKind::Directed }, N, W>;
pub type UndirectedGraph<N, W> = Graph<{ GraphKind::Undirected }, N, W>;

#[derive(Debug)]
pub struct Graph<const KIND: GraphKind, N: Debug, W: Weight> {
    nodes: Vec<N>,
    adjacencies: Vec<HashSet<NodeIndex>>,
    edges: HashMap<EdgeIndex, Edge<W>>,
}

impl<const KIND: GraphKind> Graph<KIND, usize, ()> {
    pub fn from_edge_list(edge_list: &str) -> GraphResult<Self> {
        let mut lines = edge_list.lines();

        let nodes_len = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
        let nodes_len = usize::from_str_radix(nodes_len, 10)?;

        let mut graph = Graph {
            nodes: vec![0; nodes_len],
            adjacencies: vec![HashSet::new(); nodes_len],
            edges: HashMap::new(),
        };

        for line in lines {
            let mut split = line.split_whitespace();
            let left = split.next().ok_or(GraphError::BadEdgeListFormat)?;
            let right = split.next().ok_or(GraphError::BadEdgeListFormat)?;

            let left = usize::from_str_radix(left, 10)?;
            let right = usize::from_str_radix(right, 10)?;
            let left_idx = NodeIndex(left);
            let right_idx = NodeIndex(right);

            // panics if out of range
            graph.nodes[left] = left;
            graph.nodes[right] = right;

            let _ = graph.add_edge(left_idx, right_idx, ())?;
        }

        Ok(graph)
    }
}

impl<const KIND: GraphKind, N: Debug + PartialEq, W: Weight> Graph<KIND, N, W> {
    pub fn contains_node(&self, node: N) -> Option<NodeIndex> {
        for (i, other) in self.nodes.iter().enumerate() {
            if &node == other {
                return Some(NodeIndex(i));
            }
        }
        None
    }
}

impl<const KIND: GraphKind, N: Debug, W: Weight> Graph<KIND, N, W> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            adjacencies: Vec::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: N) -> NodeIndex {
        let index = NodeIndex(self.nodes.len());
        self.nodes.push(node);
        self.adjacencies.push(HashSet::new());
        index
    }

    pub fn update_node(&mut self, index: NodeIndex, node: N) -> N {
        std::mem::replace(&mut self.nodes[index.0], node)
    }

    pub fn get(&self, index: NodeIndex) -> &N {
        &self.nodes[index.0]
    }

    pub fn get_mut(&mut self, index: NodeIndex) -> &mut N {
        &mut self.nodes[index.0]
    }

    pub fn add_edge(
        &mut self,
        left: NodeIndex,
        right: NodeIndex,
        weight: W,
    ) -> GraphResult<EdgeIndex> {
        // no need to check right side, as in undirected graph both edges will be added
        if self.adjacencies[left.0].contains(&right) {
            return Err(GraphError::EdgeAlreadyExists { left, right });
        }

        if KIND == GraphKind::Directed && self.adjacencies[right.0].contains(&left) {
            return Err(GraphError::TwoSidedEdgeForbidden {
                left: right,
                right: left,
            });
        }
        self.adjacencies[left.0 as usize].insert(right);

        if KIND == GraphKind::Undirected {
            self.adjacencies[right.0 as usize].insert(left);
            assert!(self
                .edges
                .insert(EdgeIndex::new(right, left, 0), Edge::new(weight))
                .is_none());
        }

        let idx = EdgeIndex::new(left, right, 0);

        assert!(self.edges.insert(idx, Edge::new(weight)).is_none());

        Ok(idx)
    }

    pub fn update_edge(&mut self, index: EdgeIndex, weight: W) -> W {
        let edge = self
            .edges
            .get_mut(&index)
            .expect("INTERNAL: Broken EdgeIndex: cannot get edge.");
        let wght = edge.weight;
        edge.weight = weight;
        wght
    }

    pub fn contains_edge(&self, left: NodeIndex, right: NodeIndex) -> Option<EdgeIndex> {
        let index = EdgeIndex::new(left, right, 0);
        if self.edges.contains_key(&index) {
            Some(index)
        } else {
            None
        }
    }

    pub fn weight(&self, index: EdgeIndex) -> &W {
        &self.edges[&index].weight
    }

    pub fn weight_mut(&mut self, index: EdgeIndex) -> &mut W {
        &mut self
            .edges
            .get_mut(&index)
            .expect("INTERNAL: Broken EdgeIndex: cannot get weight")
            .weight
    }

    fn depth_search_generator(&self, root: usize) -> impl Generator<Yield = &N> + '_ {
        move || {
            let mut visited = vec![false; self.nodes.len()];
            let mut stack = Vec::new();
            visited[root] = true;

            for node in &self.adjacencies[root] {
                stack.push(node.0)
            }

            while let Some(idx) = stack.pop() {
                visited[idx] = true;
                yield &self.nodes[idx];
                for node in &self.adjacencies[idx] {
                    if visited[idx] == false {
                        stack.push(node.0);
                    }
                }
            }
        }
    }

    fn _breadth_search_generator(&self, root: usize) -> impl Generator<Yield = &N> + '_ {
        move || {
            let mut visited = vec![false; self.nodes.len()];
            let mut queue = VecDeque::new();
            visited[root] = true;

            for node in &self.adjacencies[root] {
                queue.push_back(node.0)
            }

            while let Some(idx) = queue.pop_front() {
                visited[idx] = true;
                yield &self.nodes[idx];
                for node in &self.adjacencies[idx] {
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
            for node in &self.adjacencies[idx] {
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
            for node in &self.adjacencies[idx] {
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
        let mut markers = vec![0; self.nodes.len()];

        for root in 0..self.nodes.len() {
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
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adjacencies
            .iter()
            .map(|adjs| adjs.len())
            .fold(0, |a, b| a + b)
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

#[derive(Debug)]
pub struct Edge<W: Weight> {
    nodes_between: Option<Vec<NodeIndex>>,
    weight: W,
}

impl<W: Weight> Edge<W> {
    pub fn new(weight: W) -> Self {
        Self {
            nodes_between: None,
            weight,
        }
    }

    pub fn with_nodes(nodes_betweeen: Vec<NodeIndex>, weight: W) -> Self {
        Self {
            nodes_between: Some(nodes_betweeen),
            weight,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::UndirectedGraph;
    use std::{
        collections::HashSet,
        fs,
        ops::{Generator, GeneratorState},
        pin::Pin,
    };
    use test::Bencher;

    #[test]
    fn add_node() {
        let mut graph = UndirectedGraph::<u32, ()>::new();
        let _idx1 = graph.add_node(1);
        let _idx2 = graph.add_node(2);
        let _idx3 = graph.add_node(3);

        graph.contains_node(1).unwrap();
        graph.contains_node(2).unwrap();
        graph.contains_node(3).unwrap();

        assert!(graph.contains_node(100).is_none());
    }

    #[test]
    fn update_node() {
        let mut graph = UndirectedGraph::<u32, ()>::new();
        let idx1 = graph.add_node(1);

        assert_eq!(graph.update_node(idx1, 5), 1);

        graph.contains_node(5).unwrap();
        assert!(graph.contains_node(1).is_none());
    }

    #[test]
    fn add_edge() {
        let mut graph = UndirectedGraph::<u32, ()>::new();
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
        let mut graph = UndirectedGraph::<u32, u32>::new();
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
        let graph = UndirectedGraph::from_edge_list(edge_list).unwrap();

        assert_eq!(graph.node_count(), 4);

        let idx0 = graph.contains_node(0).unwrap();
        let idx1 = graph.contains_node(1).unwrap();
        let idx2 = graph.contains_node(2).unwrap();
        let idx3 = graph.contains_node(3).unwrap();

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
        let graph = UndirectedGraph::from_edge_list(edge_list).unwrap();

        let idx2 = graph.contains_node(2).unwrap();

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
        let graph = UndirectedGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph2(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph2.txt").unwrap();
        let graph = UndirectedGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph3(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph3.txt").unwrap();
        let graph = UndirectedGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_gross.txt").unwrap();
        let graph = UndirectedGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 222);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_ganz_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_ganzgross.txt").unwrap();
        let graph = UndirectedGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 9560);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_ganz_ganz_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_ganzganzgross.txt").unwrap();
        let graph = UndirectedGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph1(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph1.txt").unwrap();
        let graph = UndirectedGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph2(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph2.txt").unwrap();
        let graph = UndirectedGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph3(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph3.txt").unwrap();
        let graph = UndirectedGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_gross.txt").unwrap();
        let graph = UndirectedGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 222);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_ganz_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_ganzgross.txt").unwrap();
        let graph = UndirectedGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 9560);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_ganz_ganz_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_ganzganzgross.txt").unwrap();
        let graph = UndirectedGraph::from_edge_list(&edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 306);
        });
    }
}
