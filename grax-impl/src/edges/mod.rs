use grax_core::{
    collections::EdgeMap,
    edge::Edge,
    graph::{EdgeIterAdjacent, EdgeIterAdjacentMut},
    index::{Identifier, NodeId},
};

pub mod adj;
pub mod csr;
pub mod fixed;
pub mod hash;
pub mod mat;
#[cfg(test)]
pub mod test;

pub trait EdgeStorage<K: Identifier, W>:
    EdgeMap<K, W> + EdgeIterAdjacent + EdgeIterAdjacentMut + IntoIterator<Item = Edge<K, W>> + Sized
{
    /// Creates a new EdgeStorage
    fn new() -> Self;

    /// Creates a new EdgeStorage with the given capacity
    fn with_capacity(node_count: usize, edge_count: usize) -> Self;

    /// Creates a new EdgeStorage from the given edges
    fn with_edges(
        node_count: usize,
        edge_count: usize,
        edges: impl IntoIterator<Item = (NodeId<K>, NodeId<K>, W)>,
    ) -> Self;

    /// Allocates space for additional elements
    fn allocate(&mut self, additional: usize);

    /// Clears the storage, removing all elements
    fn clear(&mut self);

    fn remove_node(&mut self, node_id: NodeId<Self::Key>);
}
