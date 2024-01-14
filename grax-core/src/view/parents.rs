use super::Route;
use crate::{
    collections::{GetNode, GetNodeMut, NodeCount, NodeIter, VisitNodeMap},
    graph::NodeAttribute,
    prelude::{EdgeId, NodeId},
};

// only use this if parents is known to have cycle and "node" is in it
pub fn parents_cycle<G: NodeAttribute>(
    graph: &G,
    parents: &Parents<G>,
    start: NodeId<G::Key>,
) -> Route<G> {
    let mut visited = graph.visit_node_map();
    let mut path = Vec::new();
    let mut node = start;

    loop {
        let ancestor = match parents.parent(node) {
            Some(predecessor_node) => predecessor_node,
            None => node, // no predecessor, self cycle
        };
        // We have only 2 ways to find the cycle and break the loop:
        // 1. start is reached
        if ancestor == start {
            path.push(ancestor);
            break;
        }
        // 2. some node was reached twice
        else if visited.is_visited(ancestor) {
            // Drop any node in path that is before the first ancestor
            let pos = path
                .iter()
                .position(|&p| p == ancestor)
                .expect("we should always have a position");
            path = path[pos..path.len()].to_vec();

            break;
        }

        // None of the above, some middle path node
        path.push(ancestor);
        visited.visit(ancestor);
        node = ancestor;
    }
    path.reverse();

    Route::new(path)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parents<G: NodeAttribute>(G::FixedNodeMap<Option<NodeId<G::Key>>>);

impl<G: NodeAttribute> Parents<G> {
    pub fn new(graph: &G) -> Self {
        let parents = graph.fixed_node_map(None);
        Self(parents)
    }

    pub fn count(&self) -> usize {
        self.0.node_count()
    }

    pub fn is_empty(&self) -> bool {
        self.0.nodes_empty()
    }

    pub fn insert(&mut self, from: NodeId<G::Key>, to: NodeId<G::Key>) -> Option<NodeId<G::Key>> {
        self.0.update_node(to, Some(from)).flatten()
    }

    pub fn parent(&self, child: NodeId<G::Key>) -> Option<NodeId<G::Key>> {
        self.0.node(child).and_then(|parent| *parent.weight)
    }

    pub fn edge_ids(&self) -> impl Iterator<Item = EdgeId<G::Key>> + '_ {
        self.0.iter_nodes().filter_map(|node| {
            if let Some(parent) = node.weight {
                let child = node.node_id;
                Some(EdgeId::new_unchecked(*parent, child))
            } else {
                None
            }
        })
    }

    /// Panics if there is no connection between source and sink
    pub fn iter_parents(
        &self,
        source: NodeId<G::Key>,
        sink: NodeId<G::Key>,
    ) -> impl Iterator<Item = NodeId<G::Key>> + '_
    where
        G::Key: 'static,
    {
        let mut to = sink;

        std::iter::from_fn(move || {
            while to != source {
                let from = self.parent(to).unwrap();
                to = from;
                return Some(from);
            }
            None
        })
    }

    /// Panics if there is no connection between source and sink
    pub fn iter_parent_edges(
        &self,
        source: NodeId<G::Key>,
        sink: NodeId<G::Key>,
    ) -> impl Iterator<Item = EdgeId<G::Key>> + '_
    where
        G::Key: 'static,
    {
        let mut to = sink;

        std::iter::from_fn(move || {
            while to != source {
                let from = self.parent(to).unwrap();
                to = from;
                return Some(EdgeId::new_unchecked(from, to));
            }
            None
        })
    }
}

#[cfg(test)]
mod tests {
    // use crate::test::{id, PhantomGraph};

    // use super::*;

    // // Helper function to create a Parents struct from a given list of parent nodes
    // fn create_parents(parents: Vec<Option<usize>>) -> Parents<PhantomGraph> {
    //     Parents(parents.into_iter().map(|opt| opt.map(id)).collect())
    // }

    // #[test]
    // fn parents_find_cycle_no_cycle() {
    //     let parents = create_parents(vec![Some(1), Some(2), Some(3), Some(4), None]);
    //     let start = 0;
    //     let cycle = parents.find_cycle(id(start));

    //     assert_eq!(cycle.count(), 0); // No cycle should be found
    // }

    // #[test]
    // fn parents_find_cycle_self_cycle() {
    //     let parents = create_parents(vec![Some(0), Some(2), Some(0), Some(3), Some(4)]);
    //     let start = 0;
    //     let cycle = parents.find_cycle(id(start));

    //     assert_eq!(cycle.count(), 1); // Self cycle should be found
    //     assert_eq!(cycle.into_raw(), vec![id(0)]);
    // }

    // #[test]
    // fn parents_find_cycle_single_cycle() {
    //     let parents = create_parents(vec![Some(1), Some(2), Some(3), Some(0), Some(4)]);
    //     let start = 0;
    //     let cycle = parents.find_cycle(id(start));

    //     assert_eq!(cycle.count(), 4); // Cycle with 4 nodes should be found
    //     assert_eq!(cycle.into_raw(), vec![id(0), id(3), id(2), id(1)]);
    // }

    // #[test]
    // fn parents_find_cycle_multiple_cycles() {
    //     let parents = create_parents(vec![Some(1), Some(2), Some(3), Some(0), Some(5), Some(4)]);
    //     let start = 0;
    //     let cycle = parents.find_cycle(id(start));

    //     assert_eq!(cycle.count(), 4); // Cycle with 4 nodes should be found
    //     assert_eq!(cycle.into_raw(), vec![id(0), id(3), id(2), id(1)]);
    // }
}
