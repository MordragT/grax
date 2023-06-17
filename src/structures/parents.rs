use crate::{
    graph::Base,
    prelude::{EdgeIdentifier, NodeIdentifier},
};

// TODO parents wieder zurück in structures gerade adjacent nodes und adjacent indices sind nicht performant
// stattdessen tree vernünftig als "subgraph" implementieren
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parents<G: Base>(Vec<Option<G::NodeId>>);

impl<G: Base> Parents<G> {
    pub fn new(parents: Vec<Option<G::NodeId>>) -> Self {
        Self(parents)
    }

    pub fn with_count(count: usize) -> Self {
        Self(vec![None; count])
    }

    pub fn with_parents(parents: Vec<Option<G::NodeId>>) -> Self {
        Self(parents)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, from: G::NodeId, to: G::NodeId) -> Option<G::NodeId> {
        std::mem::replace(&mut self.0[to.as_usize()], Some(from))
    }

    pub fn parent(&self, child: G::NodeId) -> Option<G::NodeId> {
        if let Some(Some(parent)) = self.0.get(child.as_usize()) {
            Some(*parent)
        } else {
            None
        }
    }

    pub unsafe fn parent_unchecked(&self, child: G::NodeId) -> G::NodeId {
        self.0[child.as_usize()].unwrap()
    }

    pub fn edge_ids(&self) -> impl Iterator<Item = G::EdgeId> + '_ {
        self.0.iter().enumerate().filter_map(|(to, parent)| {
            if let Some(from) = parent {
                Some(G::EdgeId::between(*from, to.into()))
            } else {
                None
            }
        })
    }
}

// impl<NodeId: NodeIdentifier> Parents<NodeId> {
//     pub fn node_id_cycle<'a>(&self) -> Vec<NodeId> {
//         let mut node = self.source;
//         let mut visited = vec![false; self.count()];
//         let mut cycle = Vec::new();
//         let mut is_inner = false;

//         while let Some(parent) = self.parents[node.as_usize()] && parent != self.source {
//             cycle.push(parent);

//             // inner cycle
//             if visited[parent.as_usize()] {
//                 let pos = cycle.iter().position(|&p| p == parent).unwrap();
//                 cycle = cycle[pos..].to_vec();
//                 is_inner = true;
//                 break;
//             }

//             visited[parent.as_usize()] = true;
//             node = parent;
//         }

//         cycle = if cycle.len() == self.count() {
//             // complete cycle
//             cycle.push(cycle[0]);
//             cycle
//         } else if is_inner {
//             cycle
//         } else {
//             panic!()
//         };
//         cycle.reverse();
//         cycle
//     }

//     pub fn edge_id_cycle<G: Base<NodeId = NodeId>>(&self) -> Vec<G::EdgeId> {
//         let mut node = self.source;
//         let mut visited = vec![false; self.count()];
//         // let mut cycle = Vec::new();
//         let mut is_inner = false;

//         todo!()
//     }

//     pub fn nodes<'a, N, W, G: Get<N, W> + Base<NodeId = NodeId>>(
//         &'a self,
//         graph: &'a G,
//     ) -> Vec<&'a N> {
//         let mut tour = self
//             .iter()
//             .map(|node_id| graph.node(node_id).unwrap())
//             .collect::<Vec<_>>();
//         tour.reverse();
//         tour
//     }

//     pub fn edges<N, W, G: Get<N, W>>(&self, graph: &G) -> Vec<EdgeRef<NodeId, W>> {
//         todo!()
//     }

//     pub fn node_ids(&self) -> Vec<NodeId> {
//         let mut node_ids = self.iter().collect::<Vec<_>>();
//         node_ids.reverse();
//         node_ids
//     }

//     pub fn iter<'a>(&'a self) -> impl Iterator<Item = NodeId> + 'a {
//         let mut node = self.source;

//         std::iter::from_fn(move || {
//             if let Some(parent) = self.parents[node.as_usize()] && parent != self.source {
//                 node = parent;
//                 return Some(parent);
//             }
//             None
//         })
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::Parents;

//     #[test]
//     fn parents_node_cycle() {
//         let parents = Parents::<u32>::new(0, vec![None, Some(3), Some(0), None, Some(2)]);

//         dbg!(parents.node_id_cycle());
//     }
// }
