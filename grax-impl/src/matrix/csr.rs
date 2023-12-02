use super::Matrix;

struct CsrMatrixIterator<T> {
    row_offsets: Vec<(usize, RowOffset)>,
    nodes: Vec<CsrNode<T>>,
}

impl<T> CsrMatrixIterator<T> {
    pub fn new(matrix: CsrMatrix<T>) -> Self {
        let CsrMatrix { row_offsets, nodes } = matrix;

        let row_offsets = row_offsets
            .into_iter()
            .enumerate()
            .filter_map(|(row, offset)| {
                if let Some(offset) = offset {
                    Some((row, offset))
                } else {
                    None
                }
            })
            .collect();

        Self { row_offsets, nodes }
    }
}

impl<T> Iterator for CsrMatrixIterator<T> {
    type Item = (usize, Vec<CsrNode<T>>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((row, offset)) = self.row_offsets.pop() {
            let range = offset.start..offset.end;
            // TODO check if preallocated buffering might be faster
            let row_nodes = self.nodes.drain(range).collect();
            Some((row, row_nodes))
        } else {
            None
        }
    }
}

struct CsrMatrixIterMut<'a, T> {
    row_offsets: Vec<(usize, RowOffset)>,
    nodes: &'a mut Vec<CsrNode<T>>,
}

impl<'a, T> CsrMatrixIterMut<'a, T> {
    pub fn new(matrix: &'a mut CsrMatrix<T>) -> Self {
        let row_offsets = matrix
            .row_offsets
            .iter()
            .enumerate()
            .filter_map(|(row, offset)| {
                if let Some(offset) = offset {
                    Some((row, offset.clone()))
                } else {
                    None
                }
            })
            .collect();

        Self {
            row_offsets,
            nodes: &mut matrix.nodes,
        }
    }
}

impl<'a, T> Iterator for CsrMatrixIterMut<'a, T> {
    type Item = (usize, &'a mut [CsrNode<T>]);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((row, offset)) = self.row_offsets.pop() {
            let range = offset.start..offset.end;
            let row_nodes = &mut self.nodes[range] as *mut _;
            Some((row, unsafe { &mut *row_nodes }))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RowOffset {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CsrNode<T> {
    row_index: usize,
    col_index: usize,
    value: T,
}

impl<T> CsrNode<T> {
    pub fn new(row_index: usize, col_index: usize, value: T) -> Self {
        Self {
            row_index,
            col_index,
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CsrMatrix<T> {
    row_offsets: Vec<Option<RowOffset>>,
    nodes: Vec<CsrNode<T>>,
}

impl<T> CsrMatrix<T> {
    pub fn col(&self, col: usize) -> impl Iterator<Item = (usize, &T)> {
        self.nodes.iter().filter_map(move |node| {
            if node.col_index == col {
                Some((node.row_index, &node.value))
            } else {
                None
            }
        })
    }
}

impl<T> IntoIterator for CsrMatrix<T> {
    type Item = (usize, usize, T);
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        CsrMatrixIterator::new(self)
            .into_iter()
            .flat_map(|(row, row_nodes)| {
                row_nodes
                    .into_iter()
                    .map(move |node| (row, node.col_index, node.value))
            })
    }
}

impl<T> Matrix<T> for CsrMatrix<T> {
    type Iter<'a> = impl Iterator<Item = (usize, usize, &'a T)> + 'a
    where T: 'a , Self: 'a;

    type IterMut<'a> = impl Iterator<Item = (usize, usize, &'a mut T)> + 'a
    where T: 'a , Self: 'a;

    type Row<'a> = impl Iterator<Item = (usize,  &'a T)> + 'a
    where T: 'a , Self: 'a;

    type RowMut<'a> = impl Iterator<Item = (usize,  &'a mut T)> + 'a
    where T: 'a , Self: 'a;

    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            row_offsets: Vec::new(),
        }
    }

    fn with_capacity(row_count: usize, col_count: usize) -> Self {
        let value_count_prediction = 2 * (row_count + col_count);

        Self {
            nodes: Vec::with_capacity(value_count_prediction),
            row_offsets: Vec::with_capacity(row_count),
        }
    }

    fn capacity(&self) -> usize {
        self.nodes.capacity()
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn nnz(&self) -> usize {
        self.nodes.len()
    }

    fn clear(&mut self) {
        self.nodes.clear();
        self.row_offsets.clear()
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.row_offsets
            .iter()
            .enumerate()
            .filter_map(|(row, offset)| {
                if let Some(offset) = offset {
                    let values = &self.nodes[offset.start..offset.end];
                    Some((row, values))
                } else {
                    None
                }
            })
            .flat_map(|(row, values)| {
                values
                    .into_iter()
                    .map(move |node| (row, node.col_index, &node.value))
            })
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        CsrMatrixIterMut::new(self)
            .into_iter()
            .flat_map(|(row, row_nodes)| {
                row_nodes
                    .into_iter()
                    .map(move |node| (row, node.col_index, &mut node.value))
            })
    }

    fn insert(&mut self, row: usize, col: usize, value: T) {
        if row >= self.row_offsets.len() {
            self.row_offsets.resize(row + 1, None);

            let node_index = self.nodes.len();
            self.nodes.push(CsrNode::new(row, col, value));

            let start = node_index;
            let end = start + 1;
            self.row_offsets[row] = Some(RowOffset { start, end });
            return;
        }

        if let Some(offset) = &mut self.row_offsets[row] {
            let node_index = offset.end;
            self.nodes.insert(node_index, CsrNode::new(row, col, value));
            // end not included in row
            offset.end += 1;
        } else {
            let mut row_index = row;

            // fit node inbetween rows
            while self.row_offsets[row_index].is_none() {
                row_index += 1;
            }
            let next_offset = self.row_offsets[row_index].as_ref().unwrap();
            let start = next_offset.start;
            let end = start + 1;

            self.nodes.insert(start, CsrNode::new(row, col, value));
            self.row_offsets[row] = Some(RowOffset { start, end });
        }

        for offset in &mut self.row_offsets[row + 1..] {
            if let Some(offset) = offset {
                offset.start += 1;
                offset.end += 1;
            }
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<&T> {
        if let Some(Some(offset)) = self.row_offsets.get(row) {
            self.nodes[offset.start..offset.end]
                .iter()
                .find_map(|node| {
                    if node.col_index == col {
                        Some(&node.value)
                    } else {
                        None
                    }
                })
        } else {
            None
        }
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if let Some(Some(offset)) = self.row_offsets.get(row) {
            self.nodes[offset.start..offset.end]
                .iter_mut()
                .find_map(|node| {
                    if node.col_index == col {
                        Some(&mut node.value)
                    } else {
                        None
                    }
                })
        } else {
            None
        }
    }

    fn row(&self, row: usize) -> Self::Row<'_> {
        self.row_offsets
            .get(row)
            .into_iter()
            .flatten()
            .flat_map(|offset| {
                self.nodes[offset.start..offset.end]
                    .iter()
                    .map(|node| (node.col_index, &node.value))
            })
    }

    fn row_mut(&mut self, row: usize) -> Self::RowMut<'_> {
        if let Some(Some(offset)) = self.row_offsets.get(row) {
            Some(
                self.nodes[offset.start..offset.end]
                    .iter_mut()
                    .map(|node| (node.col_index, &mut node.value)),
            )
            .into_iter()
            .flatten()
        } else {
            None.into_iter().flatten()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn csr_matrix_insert_and_get() {
        let mut matrix = CsrMatrix::with_capacity(5, 5);

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
    fn csr_matrix_row_and_col() {
        let mut matrix = CsrMatrix::with_capacity(10, 10);

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
    // fn csr_matrix_transpose() {
    //     let mut matrix = CsrMatrix::with_capacity(10, 10);

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
