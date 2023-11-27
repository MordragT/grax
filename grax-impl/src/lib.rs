#![feature(type_alias_impl_trait)]
#![feature(test)]

pub mod directory;
mod edge_list;
pub mod error;
mod memory;

pub use edge_list::EdgeList;
pub use memory::*;
#[cfg(test)]
mod test;
