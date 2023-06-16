use crate::{
    graph::{Base, Get},
    prelude::{EdgeRef, NodeIdentifier},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parents<NodeId> {
    pub parents: Vec<Option<NodeId>>,
    pub source: NodeId,
}

impl<NodeId> Parents<NodeId> {
    pub fn new(source: NodeId, parents: Vec<Option<NodeId>>) -> Self {
        Self { source, parents }
    }

    pub fn count(&self) -> usize {
        self.parents.len()
    }
}

impl<NodeId: NodeIdentifier> Parents<NodeId> {
    pub fn node_cycle(&self) -> Vec<NodeId> {
        let mut node = self.source;
        let mut visited = vec![false; self.count()];
        let mut path = vec![];

        loop {
            let ancestor = match self.parents[node.as_usize()] {
                Some(parent) => parent,
                None => node,
            };

            if ancestor == self.source {
                path.push(ancestor);
                break;
            }

            if visited[ancestor.as_usize()] {
                let pos = path.iter().position(|&p| p == ancestor).unwrap();
                path = path[pos..path.len()].to_vec();
                break;
            }

            path.push(ancestor);
            visited[ancestor.as_usize()] = true;
            node = ancestor;
        }
        path.reverse();
        path
    }

    pub fn edge_cycle<G: Base<NodeId = NodeId>>(&self, graph: &G) -> Vec<G::EdgeId> {
        todo!()
    }

    pub fn nodes<N, W, G: Get<N, W> + Base<NodeId = NodeId>>(&self, graph: &G) -> Vec<&N> {
        let mut tour = self
            .iter()
            .map(|node_id| graph.node(node_id).unwrap())
            .collect::<Vec<_>>();
        tour.reverse();
        tour
    }

    pub fn edges<N, W, G: Get<N, W>>(&self, graph: &G) -> Vec<EdgeRef<NodeId, W>> {
        todo!()
    }

    pub fn node_ids(&self) -> Vec<NodeId> {
        let mut node_ids = self.iter().collect::<Vec<_>>();
        node_ids.reverse();
        node_ids
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = NodeId> + 'a {
        std::iter::from_fn(|| {
            let mut node = self.source;

            while let Some(parent) = self.parents[node.as_usize()] && parent != self.source {
                node = parent;
                return Some(parent);
            }
            None
        })
    }
}
