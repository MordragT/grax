#![feature(specialization)]
// #![feature(associated_type_bounds)]
#![feature(array_windows)]
#![feature(let_chains)]
#![feature(impl_trait_in_assoc_type)]

// #![feature(adt_const_params)]
// #![feature(let_chains)]
// #![feature(if_let_guard)]

pub mod adaptor;
pub mod algorithms;
pub mod collections;
pub mod edge;
pub mod graph;
pub mod index;
pub mod node;
// pub mod variant;
pub mod view;
pub mod weight;

pub mod prelude {
    pub use crate::{
        edge::{Edge, EdgeMut, EdgeRef},
        graph::{ImGraph, MutGraph},
        index::*,
        node::{Node, NodeMut, NodeRef},
    };
}
