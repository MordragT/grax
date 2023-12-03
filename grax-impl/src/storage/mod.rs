use grax_core::{
    edge::{Edge, EdgeRef, EdgeRefMut},
    index::EdgeId,
};

pub mod adj;
pub mod csr;
pub mod csr_simd;
pub mod dense;
// pub mod ellpack;
pub mod hash;
pub mod sparse;
#[cfg(test)]
pub mod test;

pub trait EdgeStorage<W> {
    type IntoIter: Iterator<Item = Edge<usize, W>>;

    type Iter<'a>: Iterator<Item = EdgeRef<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    type IterMut<'a>: Iterator<Item = EdgeRefMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    type Adjacent<'a>: Iterator<Item = EdgeRef<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    type AdjacentMut<'a>: Iterator<Item = EdgeRefMut<'a, usize, W>> + 'a
    where
        W: 'a,
        Self: 'a;

    type Indices<'a>: Iterator<Item = EdgeId<usize>> + 'a
    where
        Self: 'a;

    /// Creates a new EdgeStorage
    fn new() -> Self;

    /// Creates a new EdgeStorage with the given capacity
    fn with_capacity(edge_count: usize) -> Self;

    /// Returns the current capacity
    fn capacity(&self) -> usize;

    /// Returns the number of elements in the storage
    fn count(&self) -> usize;

    /// Returns true if the storage has no elements, false otherwise
    fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Allocates space for additional elements
    fn allocate(&mut self, additional: usize);

    /// Clears the storage, removing all elements
    fn clear(&mut self);

    /// Returns an owning iterator over the elements in the storage in no particular order
    fn into_iter_unstable(self) -> Self::IntoIter;

    /// Returns an iterator over the elements in the storage in no particular order
    fn iter_unstable(&self) -> Self::Iter<'_>;

    /// Returns an mutable iterator over the elements in the storage in no particular order
    fn iter_mut_unstable(&mut self) -> Self::IterMut<'_>;

    /// Returns the adjacent elements to the specified index in no particular order
    fn iter_adjacent_unstable(&self, node_id: usize) -> Self::Adjacent<'_>;

    /// Returns the mutable adjacent elements to the specified index in no particular order
    fn iter_adjacent_mut_unstable(&mut self, node_id: usize) -> Self::AdjacentMut<'_>;

    /// Returns the edge indices this storage contains
    fn indices(&self) -> Self::Indices<'_>;

    /// Inserts a new edge in the storage, assumes that both indices are < count
    fn insert(&mut self, from: usize, to: usize, weight: W) -> EdgeId<usize>;

    /// Extends the storage with multiple edges assuming all indices are < count
    fn extend(&mut self, edges: impl IntoIterator<Item = (usize, usize, W)>);

    /// Removes an edge from the storage
    fn remove(&mut self, from: usize, to: usize) -> Option<Edge<usize, W>>;

    /// Get the weight at a specific position in the storage
    fn get(&self, from: usize, to: usize) -> Option<EdgeRef<'_, usize, W>>;

    /// Gets a mutable reference to the weight at a specific position in the storage
    fn get_mut(&mut self, from: usize, to: usize) -> Option<EdgeRefMut<'_, usize, W>>;
}
