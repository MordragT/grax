#![feature(specialization)]
#![feature(type_alias_impl_trait)]
#![feature(associated_type_bounds)]
#![feature(array_windows)]
#![feature(is_some_and)]
#![feature(let_chains)]

// #![feature(adt_const_params)]
// #![feature(let_chains)]
// #![feature(if_let_guard)]

pub mod adaptor;
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
