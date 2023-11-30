use super::Matrix;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenseMatrix<T> {
    mat: Vec<Vec<Option<T>>>,
}

impl<T> DenseMatrix<T> {
    pub fn col(&self, col: usize) -> impl Iterator<Item = (usize, &T)> {
        self.mat.iter().enumerate().flat_map(move |(row, values)| {
            values.iter().enumerate().filter_map(move |(c, value)| {
                if let Some(val) = value && c == col {
                Some((row, val))
            } else {
                None
            }
            })
        })
    }
}

impl<T> IntoIterator for DenseMatrix<T> {
    type Item = (usize, usize, T);
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.mat.into_iter().enumerate().flat_map(|(row, values)| {
            values
                .into_iter()
                .enumerate()
                .filter_map(move |(col, value)| value.map(|val| (row, col, val)))
        })
    }
}

impl<T: Clone> Matrix<T> for DenseMatrix<T> {
    type Iter<'a> = impl Iterator<Item = (usize, usize, &'a T)> + 'a
    where T: 'a , Self: 'a;

    type IterMut<'a> = impl Iterator<Item = (usize, usize, &'a mut T)> + 'a
    where T: 'a , Self: 'a;

    type Row<'a> = impl Iterator<Item = (usize,  &'a T)> + 'a
    where T: 'a , Self: 'a;

    type RowMut<'a> = impl Iterator<Item = (usize,  &'a mut T)> + 'a
    where T: 'a , Self: 'a;

    fn new() -> Self {
        Self { mat: Vec::new() }
    }

    fn with_capacity(row_count: usize, col_count: usize) -> Self {
        let count = row_count.max(col_count);
        let mat = vec![vec![None; count]; count];

        Self { mat }
    }

    fn capacity(&self) -> usize {
        self.mat.capacity()
    }

    fn is_empty(&self) -> bool {
        self.mat
            .iter()
            .all(|rows| rows.iter().all(|item| item.is_none()))
    }

    fn nnz(&self) -> usize {
        self.mat
            .iter()
            .filter(|rows| !rows.iter().all(|item| item.is_none()))
            .count()
    }

    fn clear(&mut self) {
        for row in &mut self.mat {
            row.fill(None)
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.mat.iter().enumerate().flat_map(|(row, values)| {
            values
                .iter()
                .enumerate()
                .filter_map(move |(col, value)| value.as_ref().map(|val| (row, col, val)))
        })
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.mat.iter_mut().enumerate().flat_map(|(row, values)| {
            values
                .iter_mut()
                .enumerate()
                .filter_map(move |(col, value)| value.as_mut().map(|val| (row, col, val)))
        })
    }

    fn insert(&mut self, row: usize, col: usize, value: T) {
        let max = row.max(col);
        if self.mat.len() < max {
            self.mat.resize(max, vec![None; max]);
            for row in &mut self.mat {
                row.resize(max, None)
            }
        }

        self.mat[row][col] = Some(value);
    }

    fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.mat[row][col].as_ref()
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.mat[row][col].as_mut()
    }

    fn row(&self, row: usize) -> Self::Row<'_> {
        self.mat[row]
            .iter()
            .enumerate()
            .filter_map(|(col, value)| value.as_ref().map(|val| (col, val)))
    }

    fn row_mut(&mut self, row: usize) -> Self::RowMut<'_> {
        self.mat[row]
            .iter_mut()
            .enumerate()
            .filter_map(|(col, value)| value.as_mut().map(|val| (col, val)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dense_matrix_insert_and_get() {
        let mut matrix = DenseMatrix::with_capacity(5, 5);

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
    fn dense_matrix_row_and_col() {
        let mut matrix = DenseMatrix::with_capacity(10, 10);

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

    // #[test]
    // fn dense_matrix_transpose() {
    //     let mut matrix = DenseMatrix::with_capacity(10, 10);

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
