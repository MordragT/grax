#![feature(type_alias_impl_trait)]
#![feature(test)]
#![feature(let_chains)]
// #![feature(iter_from_generator)]
// #![feature(generators)]
// #![feature(return_position_impl_trait_in_trait)]

pub mod directory;
mod edge_list;
pub mod error;
pub mod flow;
pub mod matrix;
mod memory;

pub use edge_list::EdgeList;
pub use memory::*;
#[cfg(test)]
mod test;
