use crate::NodeIndex;

pub struct Tree {
    root: NodeIndex,
    parent: Vec<NodeIndex>,
}

impl Tree {
    pub fn new(union_find: UnionFind<false>) -> Option<Self> {
        let root = union_find.parent[0];
        let parent = union_find.parent;
    }
}

impl From<UnionFind<false>> for Tree {
    fn from(mut union_find: UnionFind<false>) -> Self {
        let root = union_find.find(NodeIndex(0));
        Self {
            root,
            parent: union_find.parent,
        }
    }
}

pub struct UnionFind<const PATH_COMPRESSION: bool> {
    parent: Vec<NodeIndex>,
    rank: Vec<usize>,
    path: Vec<NodeIndex>,
}

impl<const PATH_COMPRESSION: bool> UnionFind<PATH_COMPRESSION> {
    pub fn find(&mut self, needle: NodeIndex) -> NodeIndex {
        let mut root = needle;

        if PATH_COMPRESSION {
            self.path.clear();
        }

        while self.parent[root.0] != root {
            if PATH_COMPRESSION {
                self.path.push(root);
            }
            root = self.parent[root.0];
        }

        // set root of every cached index in path to "root"
        // when union find is run for a longer time the
        // performance might degrade as find must traverse
        // more parents in the former loop
        // this allows to skip intermediate nodes and improves the performance
        if PATH_COMPRESSION {
            for index in &self.path {
                self.parent[index.0] = root;
            }
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
impl<const PATH_COMPRESSION: bool, T: Iterator<Item = NodeIndex>> From<T>
    for UnionFind<PATH_COMPRESSION>
{
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
