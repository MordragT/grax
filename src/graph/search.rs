use std::collections::VecDeque;

use crate::indices::{EdgeIndex, NodeIndex};

use super::topology::{GraphAdjacentTopology, GraphTopology};

pub trait GraphSearch<N, W>: GraphTopology<N, W> + GraphAdjacentTopology<N, W> + Sized {
    fn depth_search_connected_components(&self) -> u32 {
        let mut counter = 0;
        let mut markers = vec![0; self.node_count()];

        for root in self.indices() {
            if markers[root.0] == 0 {
                counter += 1;
                self.depth_search(root, &mut markers, counter, |_| ());
            }
        }

        counter
    }

    fn breadth_search_connected_components(&self) -> u32 {
        let mut counter = 0;
        let mut markers = vec![0; self.node_count()];

        for root in self.indices() {
            if markers[root.0] == 0 {
                counter += 1;
                self.breadth_search(root, &mut markers, counter, |_| ());
            }
        }

        counter
    }
}

impl<N, W, T: PrivateGraphSearch<N, W>> GraphSearch<N, W> for T {}

pub(crate) trait PrivateGraphSearch<N, W>:
    GraphTopology<N, W> + GraphAdjacentTopology<N, W>
{
    fn depth_search<M: Default + PartialEq + Copy, F>(
        &self,
        root: NodeIndex,
        markers: &mut Vec<M>,
        mark: M,
        mut f: F,
    ) where
        F: FnMut(EdgeIndex),
    {
        let mut stack = Vec::new();
        stack.push(root);
        markers[root.0] = mark;

        while let Some(from) = stack.pop() {
            for to in self.adjacent_indices(from) {
                if markers[to.0] == M::default() {
                    stack.push(to);
                    markers[to.0] = mark;
                    f(EdgeIndex::new(from, to));
                }
            }
        }
    }

    fn breadth_search<M: Default + PartialEq + Copy, F>(
        &self,
        root: NodeIndex,
        markers: &mut Vec<M>,
        mark: M,
        mut f: F,
    ) where
        F: FnMut(EdgeIndex),
    {
        let mut queue = VecDeque::new();
        queue.push_front(root);
        markers[root.0] = mark;

        while let Some(from) = queue.pop_front() {
            for to in self.adjacent_indices(from) {
                if markers[to.0] == M::default() {
                    queue.push_back(to);
                    markers[to.0] = mark;
                    f(EdgeIndex::new(from, to));
                }
            }
        }
    }
}

impl<N, W, T: GraphTopology<N, W> + GraphAdjacentTopology<N, W>> PrivateGraphSearch<N, W> for T {}
