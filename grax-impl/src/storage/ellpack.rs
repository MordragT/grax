use super::Matrix;

#[derive(Debug, Clone, PartialEq, Eq)]
struct EllpackNode<T> {
    value: T,
    // index for the next value of the row inside the values list
    next: Option<usize>,
}

pub struct EllpackMatrixIterator<T> {
    row_index: usize,
    index: usize,
    row_count: usize,
    col_count: usize,
    row_offsets: Vec<usize>,
    col_indices: Vec<usize>,
    values: Vec<Option<EllpackNode<T>>>,
}

impl<T> EllpackMatrixIterator<T> {
    pub fn new(row_index: usize, matrix: EllpackMatrix<T>) -> Self {
        let index = matrix.row_offsets[row_index];
        let EllpackMatrix {
            row_count,
            col_count,
            row_offsets,
            col_indices,
            values,
        } = matrix;

        let values = values.into_iter().map(Some).collect();

        Self {
            row_index,
            index,
            row_count,
            col_count,
            row_offsets,
            col_indices,
            values,
        }
    }
}

impl<T> Iterator for EllpackMatrixIterator<T> {
    type Item = (usize, usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index < self.row_count {
            if let Some(current_node) = std::mem::replace(&mut self.values[self.index], None) {
                let col_index = self.col_indices[self.index];

                if let Some(next_index) = current_node.next {
                    self.index = next_index;
                } else {
                    self.row_index += 1;
                    self.index = self.row_offsets[self.row_index];
                }

                return Some((self.row_index, col_index, current_node.value));
            }
        }
        None
    }
}

pub struct EllpackMatrixIter<'a, T> {
    row_index: usize,
    index: usize,
    matrix: &'a EllpackMatrix<T>,
}

impl<'a, T> EllpackMatrixIter<'a, T> {
    pub fn new(row_index: usize, matrix: &'a EllpackMatrix<T>) -> Self {
        let index = matrix.row_offsets[row_index];

        Self {
            row_index,
            index,
            matrix,
        }
    }
}

impl<'a, T> Iterator for EllpackMatrixIter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index < self.matrix.row_count {
            let col_index = self.matrix.col_indices[self.index];
            let current_node = &self.matrix.values[self.index];

            if let Some(next_index) = current_node.next {
                self.index = next_index;
            } else {
                self.row_index += 1;
                self.index = self.matrix.row_offsets[self.row_index];
            }

            return Some((self.row_index, col_index, &current_node.value));
        }
        None
    }
}

pub struct EllpackMatrixIterMut<'a, T> {
    row_index: usize,
    index: usize,
    matrix: &'a mut EllpackMatrix<T>,
}

impl<'a, T> EllpackMatrixIterMut<'a, T> {
    pub fn new(row_index: usize, matrix: &'a mut EllpackMatrix<T>) -> Self {
        let index = matrix.row_offsets[row_index];

        Self {
            row_index,
            index,
            matrix,
        }
    }
}

impl<'a, T> Iterator for EllpackMatrixIterMut<'a, T> {
    type Item = (usize, usize, &'a mut T);

    fn next(&mut self) -> Option<(usize, usize, &'a mut T)> {
        if self.row_index < self.matrix.row_count {
            let col_index = self.matrix.col_indices[self.index];
            let current_index = self.index;

            if let Some(next_index) = self.matrix.values[current_index].next {
                self.index = next_index;
            } else {
                self.row_index += 1;
                self.index = self.matrix.row_offsets[self.row_index];
            }

            let value = &mut self.matrix.values[current_index].value as *mut T;

            return Some((self.row_index, col_index, unsafe { &mut *value }));
        }
        None
    }
}

pub struct EllpackRowIter<'a, T> {
    current: Option<usize>,
    matrix: &'a EllpackMatrix<T>,
}

impl<'a, T> EllpackRowIter<'a, T> {
    pub fn new(row_index: usize, matrix: &'a EllpackMatrix<T>) -> Self {
        let next = Some(matrix.row_offsets[row_index]);

        Self {
            current: next,
            matrix,
        }
    }
}

impl<'a, T> Iterator for EllpackRowIter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            let current_node = &self.matrix.values[current];
            let col_index = self.matrix.col_indices[current];
            self.current = current_node.next;
            Some((col_index, &current_node.value))
        } else {
            None
        }
    }
}

pub struct EllpackRowIterMut<'a, T> {
    current: Option<usize>,
    matrix: &'a mut EllpackMatrix<T>,
}

impl<'a, T> EllpackRowIterMut<'a, T> {
    pub fn new(row_index: usize, matrix: &'a mut EllpackMatrix<T>) -> Self {
        let next = Some(matrix.row_offsets[row_index]);

        Self {
            current: next,
            matrix,
        }
    }
}

impl<'a, T> Iterator for EllpackRowIterMut<'a, T> {
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            let col_index = self.matrix.col_indices[current];
            self.current = self.matrix.values[current].next;
            let value = &mut self.matrix.values[current].value as *mut T;

            Some((col_index, unsafe { &mut *value }))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EllpackMatrix<T> {
    row_count: usize,
    col_count: usize,
    // contains the index for the first element of a row at the position of the row
    row_offsets: Vec<usize>,
    // col indices at same position of values
    col_indices: Vec<usize>,
    values: Vec<EllpackNode<T>>,
}

impl<T> EllpackMatrix<T> {
    /// Returns the number of rows in the matrix
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    /// Returns the number of columns in the matrix
    pub fn col_count(&self) -> usize {
        self.col_count
    }

    // Returns the elements of a specific column
    // pub fn col(&self, col: usize) -> impl Iterator<Item = (usize, &T)> {
    //     let mut row_index = 0;

    //     std::iter::from_generator(move || {
    //         while row_index < self.row_count {
    //             let mut node_index = self.row_offsets[row_index];

    //             while let Some(next_node_index) = self.values[node_index].next {
    //                 if self.col_indices[node_index] == col {
    //                     yield (row_index, &self.values[node_index].value);
    //                 }

    //                 node_index = next_node_index;
    //             }

    //             row_index += 1;
    //         }
    //     })
    // }
}

impl<T> IntoIterator for EllpackMatrix<T> {
    type Item = (usize, usize, T);
    type IntoIter = EllpackMatrixIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        EllpackMatrixIterator::new(0, self)
    }
}

impl<T> Matrix<T> for EllpackMatrix<T> {
    type Iter<'a> = EllpackMatrixIter<'a, T> where T: 'a;
    type IterMut<'a> = EllpackMatrixIterMut<'a,  T> where T: 'a;
    type Row<'a> = EllpackRowIter<'a, T> where T: 'a;
    type RowMut<'a> = EllpackRowIterMut<'a, T> where T: 'a;

    fn new() -> Self {
        Self {
            row_count: 0,
            col_count: 0,
            row_offsets: vec![],
            col_indices: vec![],
            values: vec![],
        }
    }

    fn with_capacity(row_count: usize, col_count: usize) -> Self {
        Self {
            row_count,
            col_count,
            row_offsets: vec![0; row_count],
            col_indices: vec![],
            values: vec![],
        }
    }

    fn capacity(&self) -> usize {
        self.values.capacity()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn nnz(&self) -> usize {
        self.values.len()
    }

    fn clear(&mut self) {
        self.values.clear();
        self.row_offsets.clear();
        self.col_indices.clear();
        self.row_count = 0;
        self.col_count = 0;
    }

    fn iter(&self) -> Self::Iter<'_> {
        EllpackMatrixIter::new(0, self)
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        EllpackMatrixIterMut::new(0, self)
    }

    fn insert(&mut self, row: usize, col: usize, value: T) {
        let node_index = self.values.len();

        self.values.push(EllpackNode { value, next: None });
        // TODO no check if element already exists ?
        self.col_indices.push(col);

        self.row_count = row.max(self.row_offsets.len());
        self.row_offsets.resize(self.row_count, 0);
        let row_start = self.row_offsets[row];

        if row_start != 0 {
            // row exists
            // TODO this adds to front even if element might not be col 0
            // therefore the col_index must always be asked for by col_indices
            // and iterating over this matrix returns elements unsorted
            self.values[node_index].next = Some(row_start);
        } else if row == 0 && col != 0 {
            self.values[node_index].next = Some(0);
        }
        self.row_offsets[row] = node_index;
    }

    fn get(&self, row: usize, col: usize) -> Option<&T> {
        // row start
        let mut node_index = self.row_offsets[row];

        loop {
            if self.col_indices[node_index] == col {
                return Some(&self.values[node_index].value);
            } else if let Some(idx) = self.values[node_index].next {
                node_index = idx;
            } else {
                break;
            }
        }

        None
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        let mut node_index = self.row_offsets[row];

        loop {
            if self.col_indices[node_index] == col {
                return Some(&mut self.values[node_index].value);
            } else if let Some(idx) = self.values[node_index].next {
                node_index = idx;
            } else {
                break;
            }
        }

        None
    }

    fn row(&self, row: usize) -> Self::Row<'_> {
        EllpackRowIter::new(row, self)
    }

    fn row_mut(&mut self, row: usize) -> Self::RowMut<'_> {
        EllpackRowIterMut::new(row, self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ellpack_matrix_insert_and_get() {
        let mut matrix = EllpackMatrix::with_capacity(5, 5);

        matrix.insert(0, 0, 1);
        matrix.insert(0, 1, 12);
        matrix.insert(1, 1, 2);
        matrix.insert(2, 2, 3);

        dbg!(&matrix);

        assert_eq!(matrix.get(0, 0), Some(&1));
        assert_eq!(matrix.get(1, 1), Some(&2));
        assert_eq!(matrix.get(2, 2), Some(&3));
        assert_eq!(matrix.get(0, 1), Some(&12));
        assert_eq!(matrix.get(1, 0), None);
    }

    #[test]
    fn ellpack_matrix_row_and_col() {
        let mut matrix = EllpackMatrix::with_capacity(10, 10);

        matrix.insert(0, 0, 1);
        matrix.insert(0, 1, 2);
        matrix.insert(1, 1, 3);
        matrix.insert(2, 0, 4);
        matrix.insert(2, 2, 5);

        let row_0 = matrix.row(0).collect::<Vec<_>>();
        let row_1 = matrix.row(1).collect::<Vec<_>>();
        let row_2 = matrix.row(2).collect::<Vec<_>>();
        // let col_0 = matrix.col(0).collect::<Vec<_>>();
        // let col_1 = matrix.col(1).collect::<Vec<_>>();
        // let col_2 = matrix.col(2).collect::<Vec<_>>();

        assert!(row_0.contains(&(0, &1)));
        assert!(row_0.contains(&(1, &2)));
        assert!(row_1.contains(&(1, &3)));
        assert!(row_2.contains(&(0, &4)));
        assert!(row_2.contains(&(2, &5)));
        // assert_eq!(col_0, vec![(0, &1), (2, &4)]);
        // assert_eq!(col_1, vec![(0, &2), (1, &3)]);
        // assert_eq!(col_2, vec![(2, &5)]);
    }

    // #[test]
    // fn ellpack_matrix_transpose() {
    //     let mut matrix = EllpackMatrix::with_capacity(10, 10);

    //     matrix.insert(0, 0, 1);
    //     matrix.insert(0, 1, 2);
    //     matrix.insert(1, 1, 3);
    //     matrix.insert(2, 0, 4);
    //     matrix.insert(2, 2, 5);

    //     let transposed = matrix.transpose();

    //     assert_eq!(transposed.get(0, 0), Some(&1));
    //     assert_eq!(transposed.get(1, 0), Some(&2));
    //     assert_eq!(transposed.get(1, 1), Some(&3));
    //     assert_eq!(transposed.get(2, 0), None);
    //     assert_eq!(transposed.get(2, 1), None);
    //     assert_eq!(transposed.get(2, 2), Some(&5));
    // }
}
