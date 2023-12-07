pub use fixed::*;
use grax_core::{
    collections::NodeMap,
    index::{Identifier, NodeId},
    node::{Node, NodeMut},
};
pub use optional::*;
pub use ordered::*;
pub use slab::*;
pub use stable::*;
pub use unstable::*;

mod fixed;
mod optional;
mod ordered;
mod slab;
mod stable;
mod unstable;

pub trait NodeStorage<K: Identifier, W>:
    NodeMap<K, W> + IntoIterator<Item = Node<K, W>> + Sized
{
    type IndexedNodesMut<'a>: Iterator<Item = NodeMut<'a, K, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    fn new() -> Self;
    fn with_capacity(node_count: usize) -> Self;
    fn with_nodes(node_count: usize, nodes: impl IntoIterator<Item = W>) -> Self {
        let mut storage = Self::with_capacity(node_count);
        storage.extend_nodes(nodes);
        storage
    }
    fn clear(&mut self);

    fn iter_indexed_nodes_mut(
        &mut self,
        node_ids: Vec<NodeId<Self::Key>>,
    ) -> Self::IndexedNodesMut<'_>;
}
