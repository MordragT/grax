use crate::{
    adjacency_list::AdjacencyList,
    edge::EdgeRef,
    edge_list::EdgeList,
    error::{GraphError, GraphResult},
    tree::{Tree, UnionFind},
    Direction, EdgeIndex, NodeIndex,
};
use priq::PriorityQueue;
use std::{cmp::Ordering, collections::VecDeque, fmt::Debug, marker::PhantomData, ops::AddAssign};

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

    fn depth_search<M: Default + PartialEq + Copy, F>(
        &self,
        root: NodeIndex,
        markers: &mut Vec<M>,
        mark: M,
        mut f: F,
    ) where
        F: FnMut(EdgeIndex),
    {
        let mut stack = Vec::new();
        stack.push(root);
        markers[root.0] = mark;

        while let Some(from) = stack.pop() {
            for to in self.data.adjacent_indices(from) {
                if markers[to.0] == M::default() {
                    stack.push(to);
                    markers[to.0] = mark;
                    f(EdgeIndex::new(from, to));
                }
            }
        }
    }

    fn breadth_search<M: Default + PartialEq + Copy, F>(
        &self,
        root: NodeIndex,
        markers: &mut Vec<M>,
        mark: M,
        mut f: F,
    ) where
        F: FnMut(EdgeIndex),
    {
        let mut queue = VecDeque::new();
        queue.push_front(root);
        markers[root.0] = mark;

        while let Some(from) = queue.pop_front() {
            for to in self.data.adjacent_indices(from) {
                if markers[to.0] == M::default() {
                    queue.push_back(to);
                    markers[to.0] = mark;
                    f(EdgeIndex::new(from, to));
                }
            }
        }
    }

    pub fn depth_search_connected_components(&self) -> u32 {
        let mut counter = 0;
        let mut markers = vec![0; self.data.node_count()];

        for root in self.data.indices() {
            if markers[root.0] == 0 {
                counter += 1;
                self.depth_search(root, &mut markers, counter, |_| ());
            }
        }

        counter
    }

    pub fn breadth_search_connected_components(&self) -> u32 {
        let mut counter = 0;
        let mut markers = vec![0; self.data.node_count()];

        for root in self.data.indices() {
            if markers[root.0] == 0 {
                counter += 1;
                self.breadth_search(root, &mut markers, counter, |_| ());
            }
        }

        counter
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
        let mut total_weight = W::default();
        self.inner_kruskal::<true, _>(|edge| total_weight += edge.weight.to_owned());
        total_weight
    }

    /// Returns the root node of union find
    fn inner_kruskal<const PATH_COMPRESSION: bool, F>(
        &self,
        mut f: F,
    ) -> UnionFind<PATH_COMPRESSION>
    where
        F: FnMut(EdgeRef<W>),
    {
        let mut priority_queue = self.data.edges().collect::<Vec<_>>();
        priority_queue.sort_unstable_by(|this, other| this.weight.sort(other.weight));

        let mut union_find = UnionFind::<PATH_COMPRESSION>::from(self.data.indices());

        for edge in priority_queue {
            if union_find.find(edge.from) == union_find.find(edge.to) {
                continue;
            }
            union_find.union(edge.from, edge.to);
            f(edge);
        }

        union_find
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

impl<
        const KIND: GraphKind,
        N: PartialEq,
        W: Sortable + PartialOrd + PartialEq + Default + AddAssign + ToOwned<Owned = W>,
        D: GraphDataProviderExt<N, W>,
    > Graph<KIND, N, W, D>
{
    pub fn nearest_neighbor(&self) -> GraphResult<W> {
        match self.data.indices().next() {
            Some(start) => self.nearest_neighbor_inner(start),
            None => Ok(W::default()),
        }
    }

    fn nearest_neighbor_inner(&self, start: NodeIndex) -> GraphResult<W> {
        let mut visited = vec![false; self.node_count()];
        let mut total_weight = W::default();

        let mut target = (start, W::default());

        loop {
            visited[target.0 .0] = true;
            total_weight += target.1;

            let mut next = None;

            for edge in self.data.adjacent_edges(target.0) {
                if !visited[edge.to.0] {
                    if let Some((_, weight)) = next {
                        if weight > edge.weight {
                            next = Some((edge.to, edge.weight));
                        }
                    } else {
                        next = Some((edge.to, edge.weight));
                    }
                }
            }

            target = match next {
                Some((to, weight)) => (to, weight.to_owned()),
                None => break,
            };
        }

        if visited.into_iter().all(|visit| visit == true) {
            if let Some(edge_index) = self.data.contains_edge(target.0, start) {
                total_weight += self.data.weight(edge_index).to_owned();
                return Ok(total_weight);
            }
        }
        Err(GraphError::NNAbort)
    }

    pub fn double_tree(&self) -> GraphResult<W> {
        // TODO return Tree instead of just root.
        let tree = self
            .inner_kruskal::<false, _>(|_| ())
            .expect("INTERNAL ERROR");
        // let mut visited = vec![false; self.node_count()];
        // let mut total_weight = W::default();

        // self.depth_search(root, &mut visited, true, |index| {
        //     let weight = self.weight(index);
        //     total_weight += weight.to_owned();
        // });

        // if visited.into_iter().all(|visit| visit == true) {
        //     Ok(total_weight)
        // } else {
        //     Err(GraphError::NoCycle)
        // }
        todo!()
    }

    pub fn branch_bound(&self) -> W {
        todo!()
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
