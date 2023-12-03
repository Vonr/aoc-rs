use std::{
    cell::Cell,
    fmt::Debug,
    ops::{Index, IndexMut},
    slice::ChunksExactMut,
};

pub struct Matrix<T> {
    inner: Vec<T>,
    width: usize,
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter_rows()).finish()
    }
}

impl<T> Matrix<T> {
    pub fn new(width: usize) -> Self {
        Self {
            inner: Vec::new(),
            width,
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn elements(&self) -> usize {
        self.inner.len()
    }

    pub fn columns(&self) -> usize {
        self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn rows(&self) -> usize {
        self.elements() / self.columns()
    }

    pub fn height(&self) -> usize {
        self.elements() / self.columns()
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.inner.get(row * self.columns() + column)
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        let columns = self.columns();
        self.inner.get_mut(row * columns + column)
    }

    pub fn reserve(&mut self, additional_rows: usize) {
        self.inner.reserve(additional_rows * self.columns());
    }

    #[track_caller]
    pub fn push(&mut self, row: impl AsRef<[T]>)
    where
        T: Clone,
    {
        let row = row.as_ref();
        if row.len() != self.width() {
            panic!(
                "Tried to push row with different width. {} != {}",
                self.width(),
                row.len()
            );
        }

        self.inner.extend_from_slice(row);
    }

    #[track_caller]
    pub fn insert(&mut self, at: usize, row: impl AsRef<[T]>)
    where
        T: Clone,
    {
        let row: &[T] = row.as_ref();

        if row.len() != self.width() {
            panic!("Tried to push row with different width.")
        }

        self.inner.splice(at..at, row.iter().cloned());
    }

    #[track_caller]
    pub fn remove(&mut self, row: usize) {
        if row >= self.rows() {
            panic!("Tried to remove non-existent row.");
        }

        self.inner
            .drain(row * self.columns()..(row + 1) * self.columns());
    }

    pub fn row(&self, row: usize) -> Option<&[T]> {
        if row >= self.rows() {
            None
        } else {
            Some(&self.inner[row * self.columns()..(row + 1) * self.columns()])
        }
    }

    pub fn row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        if row >= self.rows() {
            None
        } else {
            let columns = self.columns();
            Some(&mut self.inner[row * columns..(row + 1) * columns])
        }
    }

    pub fn iter_rows(&self) -> RowsIter<'_, T> {
        RowsIter {
            matrix: self,
            index: 0,
        }
    }

    pub fn iter_rows_mut(&mut self) -> RowsIterMut<'_, T> {
        let columns = self.columns();
        RowsIterMut(self.inner.chunks_exact_mut(columns))
    }

    pub fn column(&self, column: usize) -> Option<ColumnIter<'_, T>> {
        if column >= self.columns() {
            None
        } else {
            Some(ColumnIter::new(self, column))
        }
    }

    pub fn column_mut(&mut self, column: usize) -> Option<ColumnIterMut<'_, T>> {
        if column >= self.columns() {
            None
        } else {
            Some(ColumnIterMut::new(self, column))
        }
    }

    pub fn iter_columns(&self) -> ColumnsIter<'_, T> {
        ColumnsIter {
            matrix: self,
            index: 0,
        }
    }

    pub fn iter_columns_mut(&mut self) -> ColumnsIterMut<'_, T> {
        ColumnsIterMut::new(self)
    }
}

impl<'m, T> Index<usize> for &'m Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        self.row(index).unwrap()
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        self.row(index).unwrap()
    }
}

impl<'m, T> Index<usize> for &'m mut Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        self.row(index).unwrap()
    }
}

impl<'m, T> IndexMut<usize> for &'m mut Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.row_mut(index).unwrap()
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.row_mut(index).unwrap()
    }
}

impl<T, I> Extend<I> for Matrix<T>
where
    I: Iterator<Item = T>,
{
    #[track_caller]
    fn extend<O: IntoIterator<Item = I>>(&mut self, iter: O) {
        for iter in iter {
            let initial_len = self.elements();
            self.inner.extend(iter);
            let change = self.elements() - initial_len;
            if change != self.width() {
                panic!(
                    "Tried to extend with iterator of different width. {} != {}",
                    self.width(),
                    change
                );
            }
        }
    }
}

pub struct RowsIter<'m, T> {
    matrix: &'m Matrix<T>,
    index: usize,
}

impl<'m, T> Iterator for RowsIter<'m, T> {
    type Item = &'m [T];

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.matrix.row(self.index);
        if next.is_some() {
            self.index += 1;
        }

        next
    }
}

pub struct RowsIterMut<'m, T>(ChunksExactMut<'m, T>);

impl<'m, T> Iterator for RowsIterMut<'m, T> {
    type Item = &'m mut [T];

    fn next(&mut self) -> Option<Self::Item>
    where
        for<'a> Self: 'm,
    {
        self.0.next()
    }
}

pub struct ColumnIter<'m, T: 'm> {
    rows: RowsIter<'m, T>,
    column: usize,
}

impl<'m, T> ColumnIter<'m, T> {
    fn new(matrix: &'m Matrix<T>, column: usize) -> Self {
        Self {
            rows: matrix.iter_rows(),
            column,
        }
    }
}

impl<'m, T> Iterator for ColumnIter<'m, T> {
    type Item = &'m T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.rows.next() {
            Some(&next[self.column])
        } else {
            None
        }
    }
}

pub struct ColumnIterMut<'m, T> {
    slice: &'m [Cell<T>],
    columns: usize,
    index: usize,
}

impl<'m, T> ColumnIterMut<'m, T> {
    fn new(matrix: &'m mut Matrix<T>, column: usize) -> Self {
        let columns = matrix.columns();
        let slice = &mut matrix.inner[..];
        let slice = Cell::from_mut(slice).as_slice_of_cells();
        unsafe { Self::new_shared(slice, columns, column) }
    }

    /// # Safety
    /// No two `ColumnIterMut`s can have the same `column` at the same time.
    unsafe fn new_shared(slice: &'m [Cell<T>], columns: usize, column: usize) -> Self {
        debug_assert!(column < columns);

        Self {
            slice,
            columns,
            index: column,
        }
    }
}

impl<'m, T> Iterator for ColumnIterMut<'m, T> {
    type Item = &'m mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.slice.get(self.index)?;
        self.index += self.columns;
        // SAFETY: No other `ColumnIterMut` has this column.
        Some(unsafe { &mut *next.as_ptr() })
    }
}

pub struct ColumnsIter<'m, T> {
    matrix: &'m Matrix<T>,
    index: usize,
}

impl<'m, T> Iterator for ColumnsIter<'m, T> {
    type Item = ColumnIter<'m, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.matrix.column(self.index)?;
        self.index += 1;

        Some(next)
    }
}

pub struct ColumnsIterMut<'m, T> {
    slice: &'m [Cell<T>],
    columns: usize,
    index: usize,
}

impl<'m, T> ColumnsIterMut<'m, T> {
    fn new(matrix: &'m mut Matrix<T>) -> Self {
        let columns = matrix.columns();
        let slice = &mut matrix.inner[..];
        Self {
            slice: Cell::from_mut(slice).as_slice_of_cells(),
            columns,
            index: 0,
        }
    }
}

impl<'m, T> Iterator for ColumnsIterMut<'m, T> {
    type Item = ColumnIterMut<'m, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.columns {
            None
        } else {
            // SAFETY: `self.index` is different on each iteration.
            let next = unsafe { ColumnIterMut::new_shared(self.slice, self.columns, self.index) };
            self.index += 1;
            Some(next)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_rows_mut() {
        let mut matrix = Matrix::new(10);
        matrix.push([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        matrix.push([11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
        matrix.push([21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);

        for row in matrix.iter_rows_mut() {
            for item in row {
                *item += 1;
            }
        }

        let vec = matrix.into_vec();
        for (idx, elem) in vec.into_iter().enumerate() {
            assert_eq!(elem, idx + 2);
        }
    }

    #[test]
    fn iter_columns_mut() {
        let mut matrix = Matrix::new(3);
        matrix.push([1, 11, 21]);
        matrix.push([2, 12, 22]);
        matrix.push([3, 13, 23]);
        matrix.push([4, 14, 24]);
        matrix.push([5, 15, 25]);
        matrix.push([6, 16, 26]);
        matrix.push([7, 17, 27]);
        matrix.push([8, 18, 28]);
        matrix.push([9, 19, 29]);
        matrix.push([10, 20, 30]);

        for row in matrix.iter_columns_mut() {
            for item in row {
                *item += 1;
            }
        }

        let vec = matrix.into_vec();
        for (idx, elem) in vec.into_iter().enumerate() {
            assert_eq!(elem, (idx / 3 + 2) + idx % 3 * 10);
        }
    }

    #[test]
    fn fixed_soundness_hole() {
        let mut matrix = Matrix::new(1);
        matrix.push([0]);
        matrix.push([0]);
        let mut rows = matrix.iter_rows_mut();
        let row0 = rows.next().unwrap();
        let _row1 = rows.next().unwrap();
        row0[0] = 0;
    }
}
