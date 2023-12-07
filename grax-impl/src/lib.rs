#![feature(type_alias_impl_trait)]
#![feature(test)]
#![feature(let_chains)]
#![feature(int_roundings)]

pub mod edges;
pub mod error;
mod graph;
pub mod nodes;

pub use graph::*;
