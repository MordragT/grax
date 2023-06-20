use super::Route;
use crate::{
    graph::Base,
    prelude::{EdgeId, NodeId},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parents<G: Base>(Vec<Option<NodeId<G::Id>>>);

impl<G: Base> Parents<G> {
    pub fn new(parents: Vec<Option<NodeId<G::Id>>>) -> Self {
        Self(parents)
    }

    pub fn with_count(count: usize) -> Self {
        Self(vec![None; count])
    }

    pub fn with_parents(parents: Vec<Option<NodeId<G::Id>>>) -> Self {
        Self(parents)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, from: NodeId<G::Id>, to: NodeId<G::Id>) -> Option<NodeId<G::Id>> {
        std::mem::replace(&mut self.0[to.as_usize()], Some(from))
    }

    pub fn parent(&self, child: NodeId<G::Id>) -> Option<NodeId<G::Id>> {
        if let Some(Some(parent)) = self.0.get(child.as_usize()) {
            Some(*parent)
        } else {
            None
        }
    }

    pub unsafe fn parent_unchecked(&self, child: NodeId<G::Id>) -> NodeId<G::Id> {
        self.0[child.as_usize()].unwrap()
    }

    pub fn edge_ids(&self) -> impl Iterator<Item = EdgeId<G::Id>> + '_ {
        self.0.iter().enumerate().filter_map(|(to, parent)| {
            if let Some(from) = parent {
                let to = NodeId::new_unchecked(to.into());
                Some(EdgeId::new_unchecked(*from, to))
            } else {
                None
            }
        })
    }

    // only use this if parents is known to have cycle and "node" is in it
    pub(crate) fn find_cycle(&self, start: NodeId<G::Id>) -> Route<G> {
        let mut visited = vec![false; self.count()];
        let mut path = Vec::new();
        let mut node = start;

        loop {
            let ancestor = match self.parent(node) {
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
            else if visited[ancestor.as_usize()] {
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
            visited[ancestor.as_usize()] = true;
            node = ancestor;
        }
        path.reverse();

        Route::new(path)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::{id, PhantomGraph};

    use super::*;

    // Helper function to create a Parents struct from a given list of parent nodes
    fn create_parents(parents: Vec<Option<usize>>) -> Parents<PhantomGraph> {
        Parents(parents.into_iter().map(|opt| opt.map(id)).collect())
    }

    #[test]
    fn parents_find_cycle_no_cycle() {
        let parents = create_parents(vec![Some(1), Some(2), Some(3), Some(4), None]);
        let start = 0;
        let cycle = parents.find_cycle(id(start));

        assert_eq!(cycle.count(), 0); // No cycle should be found
    }

    #[test]
    fn parents_find_cycle_self_cycle() {
        let parents = create_parents(vec![Some(0), Some(2), Some(0), Some(3), Some(4)]);
        let start = 0;
        let cycle = parents.find_cycle(id(start));

        assert_eq!(cycle.count(), 1); // Self cycle should be found
        assert_eq!(cycle.into_raw(), vec![id(0)]);
    }

    #[test]
    fn parents_find_cycle_single_cycle() {
        let parents = create_parents(vec![Some(1), Some(2), Some(3), Some(0), Some(4)]);
        let start = 0;
        let cycle = parents.find_cycle(id(start));

        assert_eq!(cycle.count(), 4); // Cycle with 4 nodes should be found
        assert_eq!(cycle.into_raw(), vec![id(0), id(3), id(2), id(1)]);
    }

    #[test]
    fn parents_find_cycle_multiple_cycles() {
        let parents = create_parents(vec![Some(1), Some(2), Some(3), Some(0), Some(5), Some(4)]);
        let start = 0;
        let cycle = parents.find_cycle(id(start));

        assert_eq!(cycle.count(), 4); // Cycle with 4 nodes should be found
        assert_eq!(cycle.into_raw(), vec![id(0), id(3), id(2), id(1)]);
    }
}
