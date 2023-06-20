use crate::{graph::Base, prelude::NodeId, structures::Parents};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionFind<G: Base> {
    parents: Parents<G>,
    rank: Vec<u32>,
}

impl<G: Base> Deref for UnionFind<G> {
    type Target = Parents<G>;

    fn deref(&self) -> &Self::Target {
        &self.parents
    }
}

impl<G: Base> DerefMut for UnionFind<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parents
    }
}

impl<G: Base> From<Parents<G>> for UnionFind<G> {
    fn from(parents: Parents<G>) -> Self {
        let count = parents.count();
        Self {
            parents,
            rank: vec![1; count],
        }
    }
}

impl<G: Base> UnionFind<G> {
    pub fn with_count(count: usize) -> Self {
        Self {
            parents: Parents::with_count(count),
            rank: vec![1; count],
        }
    }

    pub fn with_node_ids(node_ids: impl IntoIterator<Item = NodeId<G::Id>>) -> Self {
        let parents = node_ids
            .into_iter()
            .map(|node_id| Some(node_id))
            .collect::<Vec<_>>();
        let count = parents.len();

        Self {
            parents: Parents::new(parents),
            rank: vec![1; count],
        }
    }

    pub fn root(&self) -> NodeId<G::Id> {
        // self.find(0.into())
        unsafe {
            self.parents
                .parent_unchecked(NodeId::new_unchecked(0.into()))
        }
    }

    pub fn rank(&self, index: NodeId<G::Id>) -> u32 {
        self.rank[index.as_usize()]
    }

    pub fn find(&mut self, needle: NodeId<G::Id>) -> NodeId<G::Id> {
        let mut path = Vec::new();
        let mut node = needle;

        while let Some(from) = self.parent(node) && from != node {
            path.push(node);
            node = from;
        }

        // set root of every cached index in path to "root"
        // when union find is run for a longer time the
        // performance might degrade as find must traverse
        // more parents in the former loop
        // this allows to skip intermediate nodes and improves the performance
        for to in path {
            self.insert(node, to);
        }
        node
    }

    pub fn union(&mut self, x: NodeId<G::Id>, y: NodeId<G::Id>) -> NodeId<G::Id> {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x == root_y {
            return root_x;
        }

        // keep depth of trees small by appending small tree to big tree
        // ensures find operation is not doing effectively a linked list search
        if self.rank[root_x.as_usize()] < self.rank[root_y.as_usize()] {
            std::mem::swap(&mut root_x, &mut root_y);
        }
        self.insert(root_x, root_y);
        self.rank[root_x.as_usize()] += self.rank[root_y.as_usize()];
        root_x
    }
}

#[cfg(test)]
mod tests {
    use crate::test::{id, PhantomGraph};

    use super::*;

    #[test]
    fn union_find() {
        let count = 8;

        // Create a UnionFind with 5 elements
        let mut union_find =
            UnionFind::<PhantomGraph>::with_node_ids((0..count).map(NodeId::new_unchecked));

        // Initially, each element is in its own set
        assert_eq!(union_find.count(), count);
        for i in 0..count {
            let i = NodeId::new_unchecked(i);
            assert_eq!(union_find.find(i), i);
            assert_eq!(union_find.rank(i), 1);
        }

        // Perform some union operations
        assert_eq!(union_find.union(id(0), id(1)), id(0));
        assert_eq!(union_find.union(id(2), id(3)), id(2));
        assert_eq!(union_find.union(id(3), id(4)), id(2));

        // Check the root and rank of each element
        assert_eq!(union_find.find(id(0)), id(0));
        assert_eq!(union_find.rank(id(0)), 2);
        assert_eq!(union_find.find(id(1)), id(0));
        assert_eq!(union_find.rank(id(1)), 1);
        assert_eq!(union_find.find(id(2)), id(2));
        assert_eq!(union_find.rank(id(2)), 3);
        assert_eq!(union_find.find(id(3)), id(2));
        assert_eq!(union_find.rank(id(3)), 1);
        assert_eq!(union_find.find(id(4)), id(2));
        assert_eq!(union_find.rank(id(4)), 1);

        assert_eq!(union_find.union(id(3), id(5)), id(2));
        assert_eq!(union_find.union(id(6), id(0)), id(0));

        assert_eq!(union_find.find(id(0)), id(0));
        assert_eq!(union_find.rank(id(0)), 3);
        assert_eq!(union_find.find(id(2)), id(2));
        assert_eq!(union_find.rank(id(2)), 4);
        assert_eq!(union_find.find(id(6)), id(0));
        assert_eq!(union_find.rank(id(6)), 1);
    }
}
