#![feature(associated_type_bounds)]
#![feature(test)]
#![feature(let_chains)]
#![feature(array_windows)]
#![feature(iter_array_chunks)]
#![feature(impl_trait_in_assoc_type)]

pub use algorithms::*;

mod algorithms;
pub mod category;
#[cfg(test)]
mod test;
pub mod utility;
