use crate::indices::NodeIndex;

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<NodeIndex>,
    rank: Vec<usize>,
    path: Vec<NodeIndex>,
}

impl UnionFind {
    pub fn root(&self) -> NodeIndex {
        self.parent[0]
    }

    pub fn rank(&self, index: NodeIndex) -> usize {
        self.rank[index.0]
    }

    pub fn find(&mut self, needle: NodeIndex) -> NodeIndex {
        let mut root = needle;

        self.path.clear();

        while self.parent[root.0] != root {
            self.path.push(root);
            root = self.parent[root.0];
        }

        // set root of every cached index in path to "root"
        // when union find is run for a longer time the
        // performance might degrade as find must traverse
        // more parents in the former loop
        // this allows to skip intermediate nodes and improves the performance
        for index in &self.path {
            self.parent[index.0] = root;
        }
        root
    }

    pub fn union(&mut self, x: NodeIndex, y: NodeIndex) {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x == root_y {
            return;
        }

        // keep depth of trees small by appending small tree to big tree
        // ensures find operation is not doing effectively a linked list search
        if self.rank[root_x.0] < self.rank[root_y.0] {
            std::mem::swap(&mut root_x, &mut root_y);
        }
        self.parent[root_y.0] = root_x;
        self.rank[root_x.0] += self.rank[root_y.0];
    }
}

// Set every parent of each tree to itself
// Meaning that every tree == 1 node
impl<T: Iterator<Item = NodeIndex>> From<T> for UnionFind {
    fn from(nodes: T) -> Self {
        let parent: Vec<NodeIndex> = nodes.collect();
        //parent.sort();

        let rank = vec![1; parent.len()];

        Self {
            parent,
            rank,
            path: Vec::new(),
        }
    }
}
