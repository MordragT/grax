//! A view refers to customized or filtered perspectives of a graph.
//! These views are created to present specific subsets of the graph
//! or to apply specific filters, transformations, or analyses to the graph's data.

pub use distances::Distances;
pub use parents::Parents;
pub use route::Route;
pub use tree::*;

mod distances;
mod parents;
mod route;
mod tree;
