#![feature(test)]
#![feature(let_chains)]
#![feature(array_windows)]
#![feature(iter_array_chunks)]

pub use algorithms::*;

mod algorithms;
#[cfg(test)]
mod test;
pub mod util;
