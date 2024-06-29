use crate::parents::Parents;

use grax_core::{collections::GetNodeMut, graph::NodeAttribute, prelude::NodeId};
use std::ops::{Add, AddAssign, Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rank(u32);

impl Default for Rank {
    fn default() -> Self {
        Rank(1)
    }
}

impl AddAssign<Rank> for Rank {
    fn add_assign(&mut self, rhs: Rank) {
        self.0 += rhs.0
    }
}

impl Add<Rank> for Rank {
    type Output = Rank;
    fn add(self, rhs: Rank) -> Self::Output {
        Rank(self.0 + rhs.0)
    }
}

#[derive(Debug)]
pub struct UnionFind<G: NodeAttribute> {
    pub(crate) parents: Parents<G>,
    pub(crate) rank: G::FixedNodeMap<Rank>,
}

impl<G: NodeAttribute> Deref for UnionFind<G> {
    type Target = Parents<G>;

    fn deref(&self) -> &Self::Target {
        &self.parents
    }
}

impl<G: NodeAttribute> DerefMut for UnionFind<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parents
    }
}

impl<G: NodeAttribute> UnionFind<G> {
    pub fn new(graph: &G) -> Self {
        let parents = Parents::new(graph);
        let rank = graph.fixed_node_map(Rank::default());

        Self { parents, rank }
    }

    pub fn rank(&self, node_id: NodeId<G::Key>) -> u32 {
        self.rank[node_id].0
    }

    pub fn find(&mut self, needle: NodeId<G::Key>) -> NodeId<G::Key> {
        // let mut path = Vec::new();
        let mut node = needle;

        while let Some(from) = self.parent(node) {
            // path.push(node);
            node = from;
        }

        // set root of every cached index in path to "root"
        // when union find is run for a longer time the
        // performance might degrade as find must traverse
        // more parents in the former loop
        // this allows to skip intermediate nodes and improves the performance
        // for to in path {
        //     self.insert(node, to);
        // }
        // disabled because benchmarks showed that overhead of allocation seems to
        // outweigh benefits of path compression
        // also this allows kruskal to reuse the parents struct from union find
        node
    }

    pub fn union(&mut self, x: NodeId<G::Key>, y: NodeId<G::Key>) -> NodeId<G::Key> {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x == root_y {
            return root_x;
        }

        // keep depth of trees small by appending small tree to big tree
        // ensures find operation is not doing effectively a linked list search
        if self.rank(root_x) < self.rank(root_y) {
            std::mem::swap(&mut root_x, &mut root_y);
        }
        self.insert(root_x, root_y);

        let rank_y = Rank(self.rank(root_y));
        self.rank
            .update_node(root_x, Rank(self.rank(root_x)) + rank_y);
        root_x
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::test::id;
//     use grax_impl::memory::AdjacencyList;

//     use super::*;

//     #[test]
//     fn union_find() {
//         let count = 8;

//         // Create a UnionFind with 5 elements
//         let mut union_find = UnionFind::<AdjacencyList<usize, ()>>::with_node_ids(
//             (0..count).map(NodeId::new_unchecked),
//         );

//         // Initially, each element is in its own set
//         assert_eq!(union_find.count(), count);
//         for i in 0..count {
//             let i = NodeId::new_unchecked(i);
//             assert_eq!(union_find.find(i), i);
//             assert_eq!(union_find.rank(i), 1);
//         }

//         // Perform some union operations
//         assert_eq!(union_find.union(id(0), id(1)), id(0));
//         assert_eq!(union_find.union(id(2), id(3)), id(2));
//         assert_eq!(union_find.union(id(3), id(4)), id(2));

//         // Check the root and rank of each element
//         assert_eq!(union_find.find(id(0)), id(0));
//         assert_eq!(union_find.rank(id(0)), 2);
//         assert_eq!(union_find.find(id(1)), id(0));
//         assert_eq!(union_find.rank(id(1)), 1);
//         assert_eq!(union_find.find(id(2)), id(2));
//         assert_eq!(union_find.rank(id(2)), 3);
//         assert_eq!(union_find.find(id(3)), id(2));
//         assert_eq!(union_find.rank(id(3)), 1);
//         assert_eq!(union_find.find(id(4)), id(2));
//         assert_eq!(union_find.rank(id(4)), 1);

//         assert_eq!(union_find.union(id(3), id(5)), id(2));
//         assert_eq!(union_find.union(id(6), id(0)), id(0));

//         assert_eq!(union_find.find(id(0)), id(0));
//         assert_eq!(union_find.rank(id(0)), 3);
//         assert_eq!(union_find.find(id(2)), id(2));
//         assert_eq!(union_find.rank(id(2)), 4);
//         assert_eq!(union_find.find(id(6)), id(0));
//         assert_eq!(union_find.rank(id(6)), 1);
//     }
// }
