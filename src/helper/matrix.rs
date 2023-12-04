use std::{
    cell::Cell,
    fmt::Debug,
    num::NonZeroUsize,
    ops::{Index, IndexMut},
    slice::{ChunksExact, ChunksExactMut},
};

pub struct Matrix<T> {
    inner: Vec<T>,
    width: Option<NonZeroUsize>,
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter_rows()).finish()
    }
}

impl<T> Matrix<T> {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            width: None,
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
        self.width.map(|w| w.get()).unwrap_or(0)
    }

    pub fn width(&self) -> usize {
        self.width.map(|w| w.get()).unwrap_or(0)
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

        if self.width() == 0 {
            self.width = NonZeroUsize::new(row.len());
        } else if row.len() != self.width() {
            panic!(
                "Tried to push row of length {} Matrix of width {}.",
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

        if self.width() == 0 {
            self.width = NonZeroUsize::new(row.len());
        } else if row.len() != self.width() {
            panic!(
                "Tried to insert row of length {} Matrix of width {}.",
                self.width(),
                row.len()
            );
        }

        self.inner.splice(at..at, row.iter().cloned());
    }

    #[track_caller]
    pub fn remove(&mut self, row: usize) {
        if row >= self.rows() {
            panic!(
                "Tried to remove row at index {} from Matrix with {} rows.",
                row,
                self.rows(),
            );
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
        RowsIter(self.inner.chunks_exact(self.columns()))
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
            start: 0,
            end: self.columns(),
        }
    }

    pub fn iter_columns_mut(&mut self) -> ColumnsIterMut<'_, T> {
        ColumnsIterMut::new(self)
    }
}

impl<T> Default for Matrix<T> {
    fn default() -> Self {
        Self::new()
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
                    "Tried to extend Matrix of width {} with iterator of width {}.",
                    self.width(),
                    change
                );
            }
        }
    }
}

pub struct RowsIter<'m, T>(ChunksExact<'m, T>);

impl<'m, T> Iterator for RowsIter<'m, T> {
    type Item = &'m [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'m, T> DoubleEndedIterator for RowsIter<'m, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<'m, T> ExactSizeIterator for RowsIter<'m, T> {
    fn len(&self) -> usize {
        self.0.len()
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

impl<'m, T> DoubleEndedIterator for RowsIterMut<'m, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<'m, T> ExactSizeIterator for RowsIterMut<'m, T> {
    fn len(&self) -> usize {
        self.0.len()
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

impl<'m, T> DoubleEndedIterator for ColumnIter<'m, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.rows.next_back() {
            Some(&next[self.column])
        } else {
            None
        }
    }
}

impl<'m, T> ExactSizeIterator for ColumnIter<'m, T> {
    fn len(&self) -> usize {
        self.rows.len()
    }
}

pub struct ColumnIterMut<'m, T> {
    slice: &'m [Cell<T>],
    columns: usize,
    start: usize,
    end: usize,
}

impl<'m, T> ColumnIterMut<'m, T> {
    fn new(matrix: &'m mut Matrix<T>, column: usize) -> Self {
        let columns = matrix.columns();
        let rows = matrix.rows();
        let slice = &mut matrix.inner[..];
        let slice = Cell::from_mut(slice).as_slice_of_cells();
        unsafe { Self::new_shared(slice, columns, rows, column) }
    }

    /// # Safety
    /// No two `ColumnIterMut`s can have the same `column` at the same time.
    unsafe fn new_shared(slice: &'m [Cell<T>], columns: usize, rows: usize, column: usize) -> Self {
        debug_assert!(column < columns);

        Self {
            slice,
            columns,
            start: column,
            end: columns * rows + column,
        }
    }
}

impl<'m, T> Iterator for ColumnIterMut<'m, T> {
    type Item = &'m mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let next = self.slice.get(self.start)?;
        self.start += self.columns;
        // SAFETY: No other `ColumnIterMut` has this column.
        Some(unsafe { &mut *next.as_ptr() })
    }
}

impl<'m, T> DoubleEndedIterator for ColumnIterMut<'m, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end <= self.start {
            return None;
        }

        self.end -= self.columns;
        let next = self.slice.get(self.end)?;
        // SAFETY: No other `ColumnIterMut` has this column.
        Some(unsafe { &mut *next.as_ptr() })
    }
}

impl<'m, T> ExactSizeIterator for ColumnIterMut<'m, T> {
    fn len(&self) -> usize {
        (self.end - self.start) / self.columns
    }
}

pub struct ColumnsIter<'m, T> {
    matrix: &'m Matrix<T>,
    start: usize,
    end: usize,
}

impl<'m, T> Iterator for ColumnsIter<'m, T> {
    type Item = ColumnIter<'m, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        let next = self.matrix.column(self.start)?;
        self.start += 1;

        Some(next)
    }
}

pub struct ColumnsIterMut<'m, T> {
    slice: &'m [Cell<T>],
    columns: usize,
    start: usize,
    end: usize,
}

impl<'m, T> ColumnsIterMut<'m, T> {
    fn new(matrix: &'m mut Matrix<T>) -> Self {
        let columns = matrix.columns();
        let slice = &mut matrix.inner[..];
        Self {
            slice: Cell::from_mut(slice).as_slice_of_cells(),
            columns,
            start: 0,
            end: columns,
        }
    }
}

impl<'m, T> Iterator for ColumnsIterMut<'m, T> {
    type Item = ColumnIterMut<'m, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.columns || self.start >= self.end {
            None
        } else {
            // SAFETY: `self.index` is different on each iteration.
            let next = unsafe {
                ColumnIterMut::new_shared(
                    self.slice,
                    self.columns,
                    self.slice.len() / self.columns,
                    self.start,
                )
            };
            self.start += 1;
            Some(next)
        }
    }
}

impl<'m, T> DoubleEndedIterator for ColumnsIter<'m, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end <= self.start {
            return None;
        }

        self.end -= 1;
        let next = self.matrix.column(self.end)?;

        Some(next)
    }
}

impl<'m, T> ExactSizeIterator for ColumnsIter<'m, T> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<'m, T> DoubleEndedIterator for ColumnsIterMut<'m, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end <= self.start {
            None
        } else {
            self.end -= 1;
            // SAFETY: `self.index` is different on each iteration.
            let next = unsafe {
                ColumnIterMut::new_shared(
                    self.slice,
                    self.columns,
                    self.slice.len() / self.columns,
                    self.end,
                )
            };
            Some(next)
        }
    }
}

impl<'m, T> ExactSizeIterator for ColumnsIterMut<'m, T> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_rows_mut() {
        let mut matrix = Matrix::new();
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
        let mut matrix = Matrix::new();
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
        let mut matrix = Matrix::new();
        matrix.push([0]);
        matrix.push([0]);
        let mut rows = matrix.iter_rows_mut();
        let row0 = rows.next().unwrap();
        let _row1 = rows.next().unwrap();
        row0[0] = 0;
    }

    #[test]
    fn double_ended_rows() {
        let mut matrix = Matrix::new();
        matrix.push([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        matrix.push([11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
        matrix.push([21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);

        let mut rows = matrix.iter_rows();
        assert_eq!(rows.len(), 3);
        rows.next();
        assert_eq!(rows.len(), 2);
        rows.next_back();
        assert_eq!(rows.len(), 1);
        rows.next_back();
        assert_eq!(rows.len(), 0);
        rows.next();
        assert_eq!(rows.len(), 0);

        assert!(rows.next().is_none());
        assert!(rows.next_back().is_none());
    }

    #[test]
    fn double_ended_columns() {
        let mut matrix = Matrix::new();
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

        let mut cols = matrix.iter_columns();
        assert_eq!(cols.len(), 3);
        cols.next();
        assert_eq!(cols.len(), 2);
        cols.next_back();
        assert_eq!(cols.len(), 1);
        cols.next_back();
        assert_eq!(cols.len(), 0);
        cols.next();
        assert_eq!(cols.len(), 0);

        assert!(cols.next().is_none());
        assert!(cols.next_back().is_none());
    }

    #[test]
    fn double_ended_column() {
        let mut matrix = Matrix::new();
        matrix.push([1]);
        matrix.push([2]);
        matrix.push([3]);

        let mut col = matrix.column(0).unwrap();
        assert_eq!(col.len(), 3);
        col.next();
        assert_eq!(col.len(), 2);
        col.next_back();
        assert_eq!(col.len(), 1);
        col.next_back();
        assert_eq!(col.len(), 0);
        col.next();
        assert_eq!(col.len(), 0);

        assert!(col.next().is_none());
        assert!(col.next_back().is_none());
    }
}
