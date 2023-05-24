use crate::{
    graph::{Edge, Graph, Node, Weight},
    prelude::{EdgeIdentifier, NodeIdentifier},
};
use std::{
    collections::{HashMap, VecDeque},
    marker::PhantomData,
};

pub struct ResidualGraph<N: Node, W: Weight, G: Graph<N, W>> {
    pub(crate) graph: G,
    pub(crate) capacity: HashMap<G::EdgeId, W>,
    phantom: PhantomData<N>,
}

impl<N: Node, W: Weight, G: Graph<N, W> + Clone> From<&G> for ResidualGraph<N, W, G> {
    fn from(graph: &G) -> Self {
        let mut graph = graph.clone();
        let mut capacity = HashMap::new();

        let edges: Vec<Edge<G::EdgeId, W>> = graph
            .iter_edges()
            .map(|edge| edge.to_owned())
            .collect::<Vec<_>>();

        for Edge { edge_id, weight } in edges {
            if !graph.contains_edge_id(edge_id.rev()) {
                graph.insert_edge(edge_id.rev(), weight);
                capacity.insert(edge_id.rev(), W::default());
            }

            capacity.insert(edge_id, weight);
        }

        Self {
            graph,
            capacity,
            phantom: PhantomData,
        }
    }
}

impl<N: Node, W: Weight, G: Graph<N, W>> ResidualGraph<N, W, G> {
    pub fn edmonds_karp(&mut self, source: G::NodeId, sink: G::NodeId) -> W {
        let mut total_flow = W::default();
        let mut parent = vec![None; self.graph.node_count()];

        while self.bfs_augmenting_path(source, sink, &mut parent) {
            let mut to = sink;
            let mut bottleneck = None;

            while to != source {
                let from = parent[to.as_usize()].unwrap();
                let edge_id = G::EdgeId::between(from, to);
                let residual_capacity = self.capacity.get(&edge_id).unwrap();

                bottleneck = match bottleneck {
                    Some(b) => {
                        if b > residual_capacity {
                            Some(residual_capacity)
                        } else {
                            Some(b)
                        }
                    }
                    None => Some(residual_capacity),
                };

                to = from;
            }

            let bottleneck = *bottleneck.unwrap();
            total_flow += bottleneck;
            to = sink;

            while to != source {
                let from = parent[to.as_usize()].unwrap();

                let cap = self
                    .capacity
                    .get_mut(&G::EdgeId::between(from, to))
                    .unwrap();
                *cap -= bottleneck;

                assert!(*cap >= W::default());

                let cap_rev = self
                    .capacity
                    .get_mut(&G::EdgeId::between(to, from))
                    .unwrap();
                *cap_rev += bottleneck;

                assert!(*cap_rev >= W::default());

                to = from;
            }
        }

        total_flow
    }

    fn bfs_augmenting_path<'a>(
        &self,
        source: G::NodeId,
        sink: G::NodeId,
        parent: &mut Vec<Option<G::NodeId>>,
    ) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.graph.node_count()];

        queue.push_front(source);
        visited[source.as_usize()] = true;

        while let Some(from) = queue.pop_front() {
            if from == sink {
                return true;
            }

            for to in self.graph.adjacent_node_ids(from) {
                let index = G::EdgeId::between(from, to);
                if !visited[to.as_usize()] && self.capacity.get(&index).unwrap() > &W::default() {
                    parent[to.as_usize()] = Some(from);
                    queue.push_back(to);
                    visited[to.as_usize()] = true;
                }
            }
        }
        false
    }
}
