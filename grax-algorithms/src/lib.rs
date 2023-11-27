#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]
#![feature(test)]
#![feature(let_chains)]
#![feature(array_windows)]

pub use bellman_ford::*;
pub use bfs::*;
pub use branch_bound::*;
pub use brute_force::*;
// pub use capacity_scaling::*;
// pub use cycle_canceling::*;
pub use dfs::*;
pub use dijkstra::*;
pub use double_tree::*;
pub use edmonds_karp::*;
pub use ford_fulkerson::*;
pub use kruskal::*;
// pub use mcf::*;
pub use nearest_neighbor::*;
pub use prim::*;
// pub use ssp::*;

mod bellman_ford;
mod bfs;
mod branch_bound;
mod brute_force;
// mod capacity_scaling;
// mod cycle_canceling;
mod dfs;
mod dijkstra;
mod double_tree;
mod edmonds_karp;
mod ford_fulkerson;
mod kruskal;
// mod mcf;
mod nearest_neighbor;
mod prim;
// mod ssp;

#[cfg(test)]
mod test;
