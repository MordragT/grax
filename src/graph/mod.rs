use crate::{
    adjacency_list::AdjacencyList,
    edge_list::EdgeList,
    error::{GraphError, GraphResult},
    tree::UnionFind,
    Direction, EdgeIndex, NodeIndex,
};
use priq::PriorityQueue;
use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::Debug,
    marker::PhantomData,
    ops::{AddAssign, Generator},
};

use self::data_provider::{GraphDataProvider, GraphDataProviderExt};

pub mod data_provider;
#[cfg(test)]
mod test;

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

impl<const KIND: GraphKind, N, W, D: GraphDataProvider<N, W> + Default> Graph<KIND, N, W, D> {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            node_kind: PhantomData,
            weight_kind: PhantomData,
        }
    }
}

impl<const KIND: GraphKind, N, W, D: GraphDataProvider<N, W>> Graph<KIND, N, W, D> {
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
    ) -> u32 {
        let mut counter = 0;
        let mut markers = vec![0; self.data.node_count()];

        for root in 0..self.data.node_count() {
            if markers[root] == 0 {
                counter += 1;
                search(&self, root, &mut markers, counter)
            }
        }

        counter
    }

    pub fn depth_search_connected_components(&self) -> u32 {
        self.search_connected_components(Self::depth_search)
    }

    pub fn breadth_search_connected_components(&self) -> u32 {
        self.search_connected_components(Self::breadth_search)
    }

    pub fn depth_search_connected_nodes(&self, root: NodeIndex) -> impl Generator<Yield = &N> + '_ {
        move || {
            let mut visited = vec![false; self.data.node_count()];
            let mut stack = Vec::new();
            visited[root.0] = true;
            stack.push(root);

            while let Some(idx) = stack.pop() {
                yield self.get(idx);
                for node in self.data.adjacent_indices(idx) {
                    if visited[node.0] == false {
                        stack.push(node);
                        visited[node.0] = true;
                    }
                }
            }
        }
    }

    pub fn breadth_search_connected_nodes(
        &self,
        root: NodeIndex,
    ) -> impl Generator<Yield = &N> + '_ {
        move || {
            let mut visited = vec![false; self.data.node_count()];
            let mut queue = VecDeque::new();
            visited[root.0] = true;
            queue.push_back(root);

            while let Some(idx) = queue.pop_front() {
                yield self.get(idx);
                for node in self.data.adjacent_indices(idx) {
                    if visited[node.0] == false {
                        queue.push_back(node);
                        visited[node.0] = true;
                    }
                }
            }
        }
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

pub trait Sortable: PartialOrd {
    fn sort(&self, other: &Self) -> Ordering;
}

default impl<T: PartialOrd> Sortable for T {
    default fn sort(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Sortable for f64 {
    fn sort(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

impl Sortable for f32 {
    fn sort(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

impl<
        const KIND: GraphKind,
        N,
        W: Sortable + Default + AddAssign + ToOwned<Owned = W>,
        D: GraphDataProvider<N, W>,
    > Graph<KIND, N, W, D>
{
    pub fn kruskal(&self) -> W {
        let mut priority_queue = self
            .data
            .edges()
            .map(|edge| (edge.weight, (edge.from, edge.to)))
            .collect::<Vec<_>>();
        priority_queue.sort_unstable_by(|this, other| this.0.sort(other.0));

        let mut union_find = UnionFind::from(self.data.indices());
        let mut total_weight = W::default();

        for (weight, (from, to)) in priority_queue {
            if union_find.find(from) == union_find.find(to) {
                continue;
            }

            union_find.union(from, to);
            total_weight += weight.to_owned();
        }

        total_weight
    }

    pub fn prim(&self) -> W {
        match self.data.indices().next() {
            Some(start) => self.prim_inner(start),
            None => W::default(),
        }
    }

    fn prim_inner(&self, start: NodeIndex) -> W {
        let n = self.node_count();
        let mut visited = vec![false; n];
        let mut priority_queue = PriorityQueue::with_capacity(n);
        // einfach mit W::max init
        let mut weights = vec![None; n];
        let mut total_weight = W::default();

        priority_queue.put(W::default(), start);

        while let Some((weight, to)) = priority_queue.pop() {
            if visited[to.0] {
                continue;
            }
            visited[to.0] = true;
            total_weight += weight;

            for edge in self.data.adjacent_edges(to) {
                if !visited[edge.to.0] {
                    if let Some(weight) = &mut weights[edge.to.0] {
                        if *weight > edge.weight {
                            *weight = edge.weight;
                            priority_queue.put(edge.weight.to_owned(), edge.to);
                        }
                    } else {
                        weights[edge.to.0] = Some(edge.weight);
                        priority_queue.put(edge.weight.to_owned(), edge.to);
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

impl<const KIND: GraphKind, N: Default, W: Default> TryFrom<EdgeList<N, W>> for AdjGraph<KIND, N, W>
where
    AdjacencyList<KIND, N, W>: TryFrom<EdgeList<N, W>, Error = GraphError>,
{
    type Error = GraphError;

    fn try_from(edge_list: EdgeList<N, W>) -> Result<Self, Self::Error> {
        let data = AdjacencyList::try_from(edge_list)?;
        Ok(Self::from(data))
    }
}
