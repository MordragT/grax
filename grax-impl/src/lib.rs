#![feature(test)]
#![feature(let_chains)]
#![feature(int_roundings)]
#![feature(impl_trait_in_assoc_type)]

pub mod edges;
pub mod error;
mod graph;
pub mod nodes;

pub use graph::*;
