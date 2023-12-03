#![feature(type_alias_impl_trait)]
#![feature(test)]
#![feature(let_chains)]

mod edge_list;
pub mod error;
mod graph;
pub mod storage;

pub use edge_list::EdgeList;
pub use graph::*;
