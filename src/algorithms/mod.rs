pub use branch_bound::*;
pub use brute_force::*;
pub use dijkstra::*;
pub use double_tree::*;
pub use kruskal::*;
pub use nearest_neighbor::*;
pub use prim::*;
pub use search::*;

use crate::prelude::{GraphAccess, NodeIndex};

mod branch_bound;
mod brute_force;
mod dijkstra;
mod double_tree;
mod kruskal;
mod nearest_neighbor;
mod prim;
mod search;

pub struct Tour<W> {
    pub route: Vec<NodeIndex>,
    pub weight: W,
}

impl<W> Tour<W> {
    pub fn new(route: Vec<NodeIndex>, weight: W) -> Self {
        Self { route, weight }
    }

    pub fn nodes<'a, N, G>(&'a self, graph: &'a G) -> impl Iterator<Item = &'a N> + 'a
    where
        G: GraphAccess<N, W>,
    {
        self.route.iter().map(|index| graph.node(*index))
    }
}
