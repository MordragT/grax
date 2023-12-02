pub mod csr;
pub mod dense;
pub mod ellpack;
pub mod sparse;

// TODO Storage trait which is implemented by DenseMat, EllpackMat, SparseMat and AdjacencyList
// one single graph impl generic over storage

// TODO new one

// pub struct CsrMatrix {
//     row_start: Vec<usize>,
//     row_end: Vec<usize>,
//     values: Vec<T>
// }

pub trait Matrix<T>: IntoIterator<Item = (usize, usize, T)> {
    type Iter<'a>: Iterator<Item = (usize, usize, &'a T)> + 'a
    where
        T: 'a,
        Self: 'a;
    type IterMut<'a>: Iterator<Item = (usize, usize, &'a mut T)> + 'a
    where
        T: 'a,
        Self: 'a;

    type Row<'a>: Iterator<Item = (usize, &'a T)> + 'a
    where
        T: 'a,
        Self: 'a;

    type RowMut<'a>: Iterator<Item = (usize, &'a mut T)> + 'a
    where
        T: 'a,
        Self: 'a;

    /// Creates a new Matrix
    fn new() -> Self;

    /// Creates a new Matrix with the given capacities
    fn with_capacity(row_count: usize, col_count: usize) -> Self;

    fn capacity(&self) -> usize;

    /// Returns true if the matrix is empty (has no elements), false otherwise
    fn is_empty(&self) -> bool;

    /// Returns the number of non-zero elements in the matrix
    fn nnz(&self) -> usize;

    /// Clears the matrix, removing all elements
    fn clear(&mut self);

    /// Returns an iterator over the non-zero elements in the matrix
    fn iter(&self) -> Self::Iter<'_>;

    /// Returns an mutable iterator over the non-zero elements in the matrix
    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    /// Inserts a new element in the matrix
    fn insert(&mut self, row: usize, col: usize, value: T);

    /// Get the value at a specific position in the matrix
    fn get(&self, row: usize, col: usize) -> Option<&T>;

    /// Gets a mutable reference to the value at a specific position in the matrix
    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T>;

    /// Returns the elements of a specific row
    fn row(&self, row: usize) -> Self::Row<'_>;

    /// Returns the elements of a specific row
    fn row_mut(&mut self, row: usize) -> Self::RowMut<'_>;

    // /// Returns the elements of a specific column
    // fn col(&self, col: usize) -> impl Iterator<Item = (usize, &T)>;

    // Consumes the matrix returning its transposed version
    // fn transpose(self) -> Self;
}
