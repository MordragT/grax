use crate::{
    algorithms::{AugmentedPath, ParentPath},
    graph::{Edge, Graph, Node, Weight},
    prelude::{EdgeIndex, NodeIndex},
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    marker::PhantomData,
};

pub struct ResidualGraph<N: Node, W: Weight, G: Graph<N, W>> {
    pub(crate) graph: G,
    pub(crate) backward_edges: HashSet<EdgeIndex>,
    pub(crate) full_edges: HashSet<EdgeIndex>,
    pub(crate) flow: HashMap<EdgeIndex, W>,
    pub(crate) total_flow: W,
    phantom: PhantomData<N>,
}

impl<N: Node, W: Weight, G: Graph<N, W>> From<G> for ResidualGraph<N, W, G> {
    fn from(mut graph: G) -> Self {
        let mut backward_edges = HashSet::new();
        let mut flow = HashMap::new();

        let edges: Vec<Edge<G::EdgeId, W>> = graph
            .iter_edges()
            .map(|edge| edge.to_owned())
            .collect::<Vec<_>>();

        for Edge { edge_id, weight } in edges {
            graph.insert_edge(edge_id, weight).unwrap();
            backward_edges.insert(edge_id.rev());

            // init residual capacity as 0
            flow.insert(index.rev(), W::default());
        }

        Self {
            graph,
            backward_edges,
            full_edges: HashSet::new(),
            flow,
            phantom: PhantomData,
            total_flow: W::default(),
        }
    }
}

impl<N: Node, W: Weight, G: Graph<N, W>> ResidualGraph<N, W, G> {
    pub fn edmonds_karp(&mut self, source: NodeIndex, sink: NodeIndex) -> W {
        while let Some(path) = self.bfs_augmenting_path(source, sink) {
            let mut edges = Vec::new();
            let mut from = sink;

            while let Some(to) = path.parent[from.0] {
                edges.push(EdgeIndex::new(to, from));
                if to == source {
                    break;
                }
                from = to;
            }

            self.apply(AugmentedPath { edges });
        }

        self.total_flow
    }

    fn flow(&self, index: &EdgeIndex) -> &W {
        if self.backward_edges.contains(index) {
            self.flow.get(&index.rev()).unwrap()
        } else {
            self.flow.get(index).unwrap()
        }
    }

    fn flow_mut(&mut self, index: &EdgeIndex) -> &mut W {
        if self.backward_edges.contains(index) {
            self.flow.get_mut(&index.rev()).unwrap()
        } else {
            self.flow.get_mut(index).unwrap()
        }
    }

    fn apply(&mut self, augmented_path: AugmentedPath) {
        let mut capacities = Vec::new();

        // find residual capacites of edges
        // if forward edge then it is simply the residual capacity (max - flow)
        // if it is a backward edge then it is its flow
        for edge in &augmented_path.edges {
            let capacity = if self.backward_edges.contains(edge) {
                *self.flow.get(&edge.rev()).unwrap()
            } else {
                *self.graph.weight(*edge) - *self.flow.get(&edge).unwrap()
            };
            capacities.push(capacity);
        }

        // update edges
        if let Some(bottleneck) = capacities
            .into_iter()
            .min_by(|this, other| this.sort(other))
        {
            self.total_flow += bottleneck;

            for edge in augmented_path.edges {
                let max = *self.graph.weight(edge);

                let flow = if self.backward_edges.contains(&edge) {
                    let flow = self.flow_mut(&edge);
                    *flow = if *flow - bottleneck <= W::default() {
                        W::default()
                    } else {
                        *flow - bottleneck
                    };
                    *flow
                } else {
                    let flow = self.flow_mut(&edge);
                    *flow += bottleneck;
                    *flow
                };

                if flow >= max {
                    self.full_edges.insert(edge);
                } else {
                    self.full_edges.remove(&edge);
                }
            }
        }
    }

    fn bfs_augmenting_path<'a>(&self, source: NodeIndex, sink: NodeIndex) -> Option<ParentPath> {
        let mut queue = VecDeque::new();
        let mut parent = vec![None; self.graph.node_count()];
        let mut visited = vec![false; self.graph.node_count()];

        queue.push_front(source);
        visited[source.0] = true;

        while let Some(from) = queue.pop_front() {
            if from == sink {
                return Some(ParentPath { parent });
            }

            for to in self.graph.adjacent_indices(from) {
                let index = EdgeIndex::new(from, to);
                if !visited[to.0] {
                    if self.full_edges.contains(&index) && !self.backward_edges.contains(&index) {
                        continue;
                    }
                    if self.backward_edges.contains(&index)
                        && *self.flow.get(&index.rev()).unwrap() <= W::default()
                    {
                        continue;
                    }

                    parent[to.0] = Some(from);
                    queue.push_back(to);
                    visited[to.0] = true;
                }
            }
        }
        None
    }
}
