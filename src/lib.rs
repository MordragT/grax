#![feature(adt_const_params)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(type_alias_impl_trait)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{AddAssign, Generator},
};

use deser::EdgeList;
use error::{GraphError, GraphResult};
use ordered_float::OrderedFloat;
use structure::{AdjacencyList, GraphDataProvider, GraphDataProviderExt};
use tree::UnionFind;

pub mod deser;
pub mod error;
pub mod structure;
pub mod tree;

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
pub struct Graph<const KIND: GraphKind, N, W, D: GraphDataProvider<N, W>> {
    data: D,
    node_kind: PhantomData<N>,
    weight_kind: PhantomData<W>,
}

impl<const KIND: GraphKind, N, W, D: GraphDataProvider<N, W>> Graph<KIND, N, W, D> {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            node_kind: PhantomData,
            weight_kind: PhantomData,
        }
    }

    pub fn with_data(data: D) -> Self {
        Self {
            data,
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

            for node in self.data.adjacent_indices(NodeIndex(root)) {
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

            for node in self.data.adjacent_indices(NodeIndex(root)) {
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

impl<const KIND: GraphKind, N: PartialEq, W: PartialEq, D: GraphDataProviderExt<N, W>>
    Graph<KIND, N, W, D>
{
    pub fn contains_node(&self, node: &N) -> Option<NodeIndex> {
        self.data.contains_node(node)
    }

    pub fn contains_edge(&self, left: NodeIndex, right: NodeIndex) -> Option<EdgeIndex> {
        self.data.contains_edge(left, right)
    }
}

impl<const KIND: GraphKind, N, W: Clone, D: GraphDataProvider<N, W>> Graph<KIND, N, W, D> {
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
                    .add_edge(left, right, weight.clone(), Direction::Incoming)?;
                self.data.add_edge(left, right, weight, Direction::Outgoing)
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MinOrderEdge<'a, W> {
    to: NodeIndex,
    weight: &'a W,
}

impl<'a, W> MinOrderEdge<'a, W> {
    pub fn new(to: NodeIndex, weight: &'a W) -> Self {
        Self { to, weight }
    }
}

impl<'a, W: Ord> Ord for MinOrderEdge<'a, W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse order
        other.weight.cmp(&self.weight)
    }
}

impl<'a, W: PartialOrd> PartialOrd for MinOrderEdge<'a, W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // reverse order
        other.weight.partial_cmp(&self.weight)
    }
}

impl<'a, W> From<&'a Edge<W>> for MinOrderEdge<'a, W> {
    fn from(edge: &'a Edge<W>) -> Self {
        Self {
            to: edge.to,
            weight: &edge.weight,
        }
    }
}

impl<'a, W> From<EdgeRef<'a, W>> for MinOrderEdge<'a, W> {
    fn from(edge: EdgeRef<'a, W>) -> Self {
        Self {
            to: edge.to,
            weight: edge.weight,
        }
    }
}

impl<
        const KIND: GraphKind,
        N,
        W: Ord + Default + AddAssign + ToOwned<Owned = W>,
        D: GraphDataProvider<N, W>,
    > Graph<KIND, N, W, D>
{
    pub fn kruskal(&self) -> W {
        let mut priority_queue = self
            .data
            .edges()
            .map(|edge| Reverse(edge))
            .collect::<BinaryHeap<_>>();

        let mut union_find = UnionFind::from(self.data.node_indices());
        let mut total_weight = W::default();

        while let Some(Reverse(EdgeRef { from, to, weight })) = priority_queue.pop() {
            if union_find.find(from) == union_find.find(to) {
                continue;
            }

            union_find.union(from, to);
            total_weight += weight.to_owned();
        }

        total_weight
    }

    pub fn prim(&self) -> W {
        match self.data.node_indices().next() {
            Some(start) => self.prim_inner(start),
            None => W::default(),
        }
    }

    fn prim_inner(&self, start: NodeIndex) -> W {
        let n = self.node_count();
        let mut visited = vec![false; n];
        let mut priority_queue = BinaryHeap::with_capacity(n / 2);
        let mut weights = vec![None; n];
        let mut total_weight = W::default();

        let default_weight = W::default();
        priority_queue.push(MinOrderEdge::new(start, &default_weight));

        while let Some(MinOrderEdge { to, weight }) = priority_queue.pop() {
            if visited[to.0] {
                continue;
            }
            visited[to.0] = true;
            total_weight += weight.to_owned();

            for edge in self.data.adjacent_edges(to) {
                if !visited[edge.to.0] {
                    if let Some(weight) = &mut weights[edge.to.0] {
                        if *weight > edge.weight {
                            *weight = edge.weight;
                            priority_queue.push(edge.into());
                        }
                    } else {
                        weights[edge.to.0] = Some(edge.weight);
                        priority_queue.push(edge.into());
                    }
                }
            }
        }

        total_weight
    }
}

impl<const KIND: GraphKind, N: Default, W: Default> From<AdjacencyList<KIND, N, W>>
    for AdjGraph<KIND, N, W>
{
    fn from(data: AdjacencyList<KIND, N, W>) -> Self {
        Self::with_data(data)
    }
}

impl<const KIND: GraphKind> TryFrom<EdgeList> for AdjGraph<KIND, usize, ()> {
    type Error = GraphError;

    fn try_from(edge_list: EdgeList) -> Result<Self, Self::Error> {
        let data = AdjacencyList::try_from(edge_list)?;
        Ok(Self::from(data))
    }
}

impl<const KIND: GraphKind> TryFrom<EdgeList> for AdjGraph<KIND, usize, OrderedFloat<f64>> {
    type Error = GraphError;

    fn try_from(edge_list: EdgeList) -> Result<Self, Self::Error> {
        let data = AdjacencyList::try_from(edge_list)?;
        Ok(Self::from(data))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct NodeIndex(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeIndex {
    pub(crate) to: NodeIndex,
    pub(crate) from: NodeIndex,
    pub(crate) depth: u32,
}

impl EdgeIndex {
    fn new(parent: NodeIndex, child: NodeIndex, depth: u32) -> Self {
        Self {
            to: parent,
            from: child,
            depth,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge<W> {
    pub from: NodeIndex,
    pub to: NodeIndex,
    pub weight: W,
}

impl<W> Edge<W> {
    pub fn new(from: NodeIndex, to: NodeIndex, weight: W) -> Self {
        Self { from, to, weight }
    }
}

impl<W: Ord> Ord for Edge<W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<W: PartialOrd> PartialOrd for Edge<W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<'a, W: ToOwned<Owned = W>> From<EdgeRef<'a, W>> for Edge<W> {
    fn from(edge_ref: EdgeRef<'a, W>) -> Self {
        Self {
            from: edge_ref.from,
            to: edge_ref.to,
            weight: edge_ref.weight.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EdgeRef<'a, W> {
    pub from: NodeIndex,
    pub to: NodeIndex,
    pub weight: &'a W,
}

impl<'a, W> EdgeRef<'a, W> {
    pub fn new(from: NodeIndex, to: NodeIndex, weight: &'a W) -> Self {
        Self { from, to, weight }
    }
}

impl<'a, W: Ord> Ord for EdgeRef<'a, W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(other.weight)
    }
}

impl<'a, W: PartialOrd> PartialOrd for EdgeRef<'a, W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(other.weight)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    /// left -> right
    Outgoing,
    /// left <- right
    Incoming,
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::{
        deser::{EdgeList, EdgeListOptions},
        UndirectedAdjGraph,
    };
    use ordered_float::OrderedFloat;
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
        let edge_list = EdgeList::new(edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

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
        let edge_list = EdgeList::new(edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

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
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph2(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph2.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph3(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph3.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_gross.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 222);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_ganz_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_ganzgross.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 9560);
        });
    }

    #[bench]
    fn breadth_search_connected_components_graph_ganz_ganz_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_ganzganzgross.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.breadth_search_connected_components();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph1(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph1.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 2);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph2(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph2.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph3(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph3.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 4);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_gross.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 222);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_ganz_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_ganzgross.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 9560);
        });
    }

    #[bench]
    fn depth_search_connected_components_graph_ganz_ganz_gross(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/Graph_ganzganzgross.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

        b.iter(|| {
            let (counter, _markers) = graph.depth_search_connected_components();
            assert_eq!(counter, 306);
        });
    }

    #[bench]
    fn prim_graph_1_2(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/G_1_2.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let graph = UndirectedAdjGraph::<usize, OrderedFloat<f64>>::try_from(edge_list).unwrap();

        b.iter(|| {
            let count = graph.prim().0 as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn prim_graph_1_20(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/G_1_20.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let graph = UndirectedAdjGraph::<usize, OrderedFloat<f64>>::try_from(edge_list).unwrap();

        b.iter(|| {
            let count = graph.prim().0 as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn prim_graph_1_200(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/G_1_200.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let graph = UndirectedAdjGraph::<usize, OrderedFloat<f64>>::try_from(edge_list).unwrap();

        b.iter(|| {
            let count = graph.prim().0 as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn prim_graph_10_20(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/G_10_20.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let graph = UndirectedAdjGraph::<usize, OrderedFloat<f64>>::try_from(edge_list).unwrap();

        b.iter(|| {
            let count = graph.prim().0 as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[bench]
    fn prim_graph_10_200(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/G_10_200.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let graph = UndirectedAdjGraph::<usize, OrderedFloat<f64>>::try_from(edge_list).unwrap();

        b.iter(|| {
            let count = graph.prim().0 as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[bench]
    fn prim_graph_100_200(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/G_100_200.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let graph = UndirectedAdjGraph::<usize, OrderedFloat<f64>>::try_from(edge_list).unwrap();

        b.iter(|| {
            let count = graph.prim().0 as f32;
            assert_eq!(count, 27550.51488);
        })
    }

    #[bench]
    fn kruskal_graph_1_2(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/G_1_2.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let graph = UndirectedAdjGraph::<usize, OrderedFloat<f64>>::try_from(edge_list).unwrap();

        b.iter(|| {
            let count = graph.kruskal().0 as f32;
            assert_eq!(count, 287.32286);
        })
    }

    #[bench]
    fn kruskal_graph_1_20(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/G_1_20.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let graph = UndirectedAdjGraph::<usize, OrderedFloat<f64>>::try_from(edge_list).unwrap();

        b.iter(|| {
            let count = graph.kruskal().0 as f32;
            assert_eq!(count, 36.86275);
        })
    }

    #[bench]
    fn kruskal_graph_1_200(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/G_1_200.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let graph = UndirectedAdjGraph::<usize, OrderedFloat<f64>>::try_from(edge_list).unwrap();

        b.iter(|| {
            let count = graph.kruskal().0 as f32;
            assert_eq!(count, 12.68182);
        })
    }

    #[bench]
    fn kruskal_graph_10_20(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/G_10_20.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let graph = UndirectedAdjGraph::<usize, OrderedFloat<f64>>::try_from(edge_list).unwrap();

        b.iter(|| {
            let count = graph.kruskal().0 as f32;
            assert_eq!(count, 2785.62417);
        })
    }

    #[bench]
    fn kruskal_graph_10_200(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/G_10_200.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let graph = UndirectedAdjGraph::<usize, OrderedFloat<f64>>::try_from(edge_list).unwrap();

        b.iter(|| {
            let count = graph.kruskal().0 as f32;
            assert_eq!(count, 372.14417);
        })
    }

    #[bench]
    fn kruskal_graph_100_200(b: &mut Bencher) {
        let edge_list = fs::read_to_string("data/G_100_200.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let graph = UndirectedAdjGraph::<usize, OrderedFloat<f64>>::try_from(edge_list).unwrap();

        b.iter(|| {
            let count = graph.kruskal().0 as f32;
            assert_eq!(count, 27550.51488);
        })
    }
}
