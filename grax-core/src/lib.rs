#![feature(min_specialization)]
#![feature(let_chains)]
#![feature(impl_trait_in_assoc_type)]

pub mod collections;
pub mod edge;
pub mod graph;
pub mod index;
pub mod node;
pub mod weight;

pub mod prelude {
    pub use crate::{
        edge::{Edge, EdgeMut, EdgeRef},
        graph::{ImGraph, MutGraph},
        index::*,
        node::{Node, NodeMut, NodeRef},
    };
}
