use std::fmt::Debug;

pub use edge::FilterEdgeView;
// pub use node::FilterNodeView;

use crate::graph::{EdgeAttribute, NodeAttribute};

mod edge;
// mod node;

struct FilterView<T: Debug + Clone, G: EdgeAttribute + NodeAttribute> {
    nodes: G::FixedNodeMap<T>,
    edges: G::FixedEdgeMap<T>,
}
