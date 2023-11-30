use crate::traits::{Contains, Viewable};

use super::{AttrMap, View, ViewGraph};

pub use edge::FilterEdgeView;
pub use node::FilterNodeView;

mod edge;
mod node;

struct FilterView<G: Viewable> {
    nodes: G::NodeMap<bool>,
    edges: G::EdgeMap<bool>,
}
