#![feature(test)]
#![feature(let_chains)]
#![feature(iter_array_chunks)]

pub mod algorithms;
pub mod cycle;
pub mod distances;
pub mod flow;
pub mod parents;
pub mod path;
pub mod tree;
pub mod weight;

pub mod prelude {
    pub use crate::algorithms::*;
    pub use crate::cycle::*;
    pub use crate::distances::*;
    pub use crate::parents::*;
    pub use crate::path::*;
    pub use crate::tree::*;
    pub use crate::weight::*;
}

#[cfg(test)]
mod test;
