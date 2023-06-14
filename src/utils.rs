/// A N*M sized sparse Matrix
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseMatrix<T> {
    values: Vec<T>,
    row_indices: Vec<usize>,
    col_indices: Vec<usize>,
    row_count: usize,
    col_count: usize,
}

const MODIFIER: usize = 4;

impl<T> SparseMatrix<T> {
    pub fn with_capacity(row_count: usize, col_count: usize) -> Self {
        Self {
            values: Vec::with_capacity(MODIFIER * (row_count + col_count)),
            row_indices: Vec::with_capacity(MODIFIER * row_count),
            col_indices: Vec::with_capacity(MODIFIER * col_count),
            row_count,
            col_count,
        }
    }

    /// Returns the number of rows in the matrix
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    /// Returns the number of columns in the matrix
    pub fn col_count(&self) -> usize {
        self.col_count
    }

    /// Returns true if the matrix is empty (has no elements), false otherwise
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the number of non-zero elements in the matrix
    pub fn nnz(&self) -> usize {
        self.values.len()
    }

    /// Clears the matrix, removing all elements
    pub fn clear(&mut self) {
        self.values.clear();
        self.row_indices.clear();
        self.col_indices.clear();
    }

    pub fn into_iter(self) -> impl Iterator<Item = (usize, usize, T)> {
        self.row_indices
            .into_iter()
            .zip(self.col_indices)
            .zip(self.values)
            .map(|((row, col), value)| (row, col, value))
    }

    /// Returns an iterator over the non-zero elements in the matrix
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.row_indices
            .iter()
            .zip(&self.col_indices)
            .zip(&self.values)
            .map(|((&row, &col), value)| (row, col, value))
    }

    /// Returns an mutable iterator over the non-zero elements in the matrix
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.row_indices
            .iter()
            .zip(&self.col_indices)
            .zip(&mut self.values)
            .map(|((&row, &col), value)| (row, col, value))
    }

    pub fn insert(&mut self, row: usize, col: usize, value: T) {
        assert!(row <= self.row_count);
        assert!(col <= self.col_count);

        self.values.push(value);
        self.row_indices.push(row);
        self.col_indices.push(col);
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        for i in 0..self.row_indices.len() {
            if self.row_indices[i] == row && self.col_indices[i] == col {
                return Some(&self.values[i]);
            }
        }
        None
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        for i in 0..self.row_indices.len() {
            if self.row_indices[i] == row && self.col_indices[i] == col {
                return Some(&mut self.values[i]);
            }
        }
        None
    }

    /// Returns the elements of a specific row
    pub fn row(&self, row: usize) -> impl Iterator<Item = (usize, &T)> {
        (0..self.row_indices.len()).filter_map(move |i| {
            if self.row_indices[i] == row {
                Some((self.col_indices[i], &self.values[i]))
            } else {
                None
            }
        })
    }

    /// Returns the elements of a specific row
    pub fn row_mut(&mut self, row: usize) -> impl Iterator<Item = (usize, &mut T)> {
        let (ids, col_ids): (Vec<_>, Vec<_>) = (0..self.row_indices.len())
            .filter_map(|i| {
                if &self.row_indices[i] == &row {
                    Some((i, *&self.col_indices[i]))
                } else {
                    None
                }
            })
            .unzip();

        let values = self
            .values
            .iter_mut()
            .enumerate()
            .filter_map(
                move |(i, val)| {
                    if ids.contains(&i) {
                        Some(val)
                    } else {
                        None
                    }
                },
            );

        col_ids.into_iter().zip(values)
    }

    /// Returns the elements of a specific column
    pub fn col(&self, col: usize) -> impl Iterator<Item = (usize, &T)> {
        (0..self.col_indices.len()).filter_map(move |i| {
            if self.col_indices[i] == col {
                Some((self.row_indices[i], &self.values[i]))
            } else {
                None
            }
        })
    }
}

impl<T: Clone> SparseMatrix<T> {
    /// Transposes the matrix, swapping the rows and columns
    pub fn transpose(&self) -> Self {
        let mut transposed = Self::with_capacity(self.col_count, self.row_count);
        for i in 0..self.values.len() {
            let row = self.col_indices[i];
            let col = self.row_indices[i];
            let value = self.values[i].clone();
            transposed.insert(row, col, value);
        }
        transposed
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sparse_matrix_insert_and_get() {
        let mut matrix = SparseMatrix::with_capacity(5, 5);

        matrix.insert(0, 0, 1);
        matrix.insert(1, 1, 2);
        matrix.insert(2, 2, 3);

        assert_eq!(matrix.get(0, 0), Some(&1));
        assert_eq!(matrix.get(1, 1), Some(&2));
        assert_eq!(matrix.get(2, 2), Some(&3));
        assert_eq!(matrix.get(0, 1), None);
        assert_eq!(matrix.get(1, 0), None);
    }

    #[test]
    fn sparse_matrix_row_and_col() {
        let mut matrix = SparseMatrix::with_capacity(10, 10);

        matrix.insert(0, 0, 1);
        matrix.insert(0, 1, 2);
        matrix.insert(1, 1, 3);
        matrix.insert(2, 0, 4);
        matrix.insert(2, 2, 5);

        let row_0 = matrix.row(0).collect::<Vec<_>>();
        let row_1 = matrix.row(1).collect::<Vec<_>>();
        let row_2 = matrix.row(2).collect::<Vec<_>>();
        let col_0 = matrix.col(0).collect::<Vec<_>>();
        let col_1 = matrix.col(1).collect::<Vec<_>>();
        let col_2 = matrix.col(2).collect::<Vec<_>>();

        assert_eq!(row_0, vec![(0, &1), (1, &2)]);
        assert_eq!(row_1, vec![(1, &3)]);
        assert_eq!(row_2, vec![(0, &4), (2, &5)]);
        assert_eq!(col_0, vec![(0, &1), (2, &4)]);
        assert_eq!(col_1, vec![(0, &2), (1, &3)]);
        assert_eq!(col_2, vec![(2, &5)]);
    }

    #[test]
    fn sparse_matrix_transpose() {
        let mut matrix = SparseMatrix::with_capacity(10, 10);

        matrix.insert(0, 0, 1);
        matrix.insert(0, 1, 2);
        matrix.insert(1, 1, 3);
        matrix.insert(2, 0, 4);
        matrix.insert(2, 2, 5);

        let transposed = matrix.transpose();

        assert_eq!(transposed.get(0, 0), Some(&1));
        assert_eq!(transposed.get(1, 0), Some(&2));
        assert_eq!(transposed.get(1, 1), Some(&3));
        assert_eq!(transposed.get(2, 0), None);
        assert_eq!(transposed.get(2, 1), None);
        assert_eq!(transposed.get(2, 2), Some(&5));
    }
}
