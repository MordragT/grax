#![feature(test)]
#![feature(iter_array_chunks)]
#![feature(let_chains)]
#![feature(int_roundings)]
#![feature(impl_trait_in_assoc_type)]
// #![cfg_attr(feature = "nightly", feature(f16, f128))]

pub mod edges;
pub mod error;
mod graph;
pub mod nodes;

pub use graph::*;
