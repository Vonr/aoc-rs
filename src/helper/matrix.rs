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

    pub fn with_width(width: usize) -> Self {
        Self {
            inner: Vec::new(),
            width: NonZeroUsize::new(width),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            width: None,
        }
    }

    pub fn with_width_and_capacity(width: usize, capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            width: NonZeroUsize::new(width),
        }
    }

    /// Get inner Vec that backs the Matrix.
    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    pub fn iter_elements(&self) -> ElementIter<'_, T> {
        ElementIter {
            matrix: self,
            start: 0,
            end: self.elements(),
        }
    }

    pub fn iter_elements_mut(&mut self) -> ElementIterMut<'_, T> {
        ElementIterMut::new(self)
    }

    /// Get a mutable reference to the inner Vec that backs the Matrix.
    ///
    /// # Safety
    /// Caller must not mutate the Vec in such a way that the Matrix becomes invalid.
    pub unsafe fn as_mut_vec(&mut self) -> &mut Vec<T> {
        &mut self.inner
    }

    /// The number of elements - not rows, that the Matrix can still fit.
    ///
    /// If you want the capacity of rows that the Matrix can still fit instead, use [`Matrix::row_capacity`].
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// The number of rows - not elements, that the Matrix can still fit.
    ///
    /// If you want the capacity of elements that the Matrix can still fit instead, use [`Matrix::capacity`].
    pub fn row_capacity(&self) -> usize {
        if self.inner.capacity() == 0 || self.width() == 0 {
            return 0;
        }

        self.inner.capacity() / self.width()
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

    /// Pushes a row without checking if the new row is of the correct length or if there is
    /// enough capacity in the Matrix for the new row.
    ///
    /// # Safety
    ///
    /// Caller must not give a row with a length different than the Matrix's width.
    /// `matrix.width() == row.len()`
    ///
    /// Caller must ensure that there is enough capacity for the new row.
    /// `matrix.row_capacity() >= 1`
    pub unsafe fn push_unchecked(&mut self, row: impl AsRef<[T]>)
    where
        T: Clone,
    {
        let row = row.as_ref();

        if self.width() == 0 {
            self.width = NonZeroUsize::new(row.len());
        }

        self.inner.extend_from_slice_unchecked(row);
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

    pub fn neighbours<'a: 'b, 'b>(&'a self, row: usize, col: usize) -> [Option<&T>; 4] {
        let (r, c) = (row, col);
        [
            // Top
            r.checked_sub(1).and_then(|r| self.get(r, c)),
            // Left
            c.checked_sub(1).and_then(|c| self.get(r, c)),
            // Right
            c.checked_add(1).and_then(|c| {
                if c < self.columns() {
                    self.get(r, c)
                } else {
                    None
                }
            }),
            // Bottom
            r.checked_add(1).and_then(|r| {
                if r < self.rows() {
                    self.get(r, c)
                } else {
                    None
                }
            }),
        ]
    }

    pub fn orthogonal_neighbours<'a: 'b, 'b>(&'a self, row: usize, col: usize) -> [Option<&T>; 8] {
        let (r, c) = (row, col);
        [
            // Top-left
            r.checked_sub(1)
                .and_then(|r| c.checked_sub(1).and_then(|c| self.get(r, c))),
            // Top-middle
            r.checked_sub(1).and_then(|r| self.get(r, c)),
            // Top-right
            r.checked_sub(1).and_then(|r| {
                c.checked_add(1).and_then(|c| {
                    if c < self.columns() {
                        self.get(r, c)
                    } else {
                        None
                    }
                })
            }),
            // Middle-left
            c.checked_sub(1).and_then(|c| self.get(r, c)),
            // Middle-right
            c.checked_add(1).and_then(|c| {
                if c < self.columns() {
                    self.get(r, c)
                } else {
                    None
                }
            }),
            // Bottom-left
            r.checked_add(1).and_then(|r| {
                if r < self.rows() {
                    c.checked_sub(1).and_then(|c| self.get(r, c))
                } else {
                    None
                }
            }),
            // Bottom-middle
            r.checked_add(1).and_then(|r| {
                if r < self.rows() {
                    self.get(r, c)
                } else {
                    None
                }
            }),
            // Bottom-right
            r.checked_add(1).and_then(|r| {
                if r < self.rows() {
                    c.checked_add(1).and_then(|c| {
                        if c < self.columns() {
                            self.get(r, c)
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            }),
        ]
    }

    pub fn neighbours_with_indices<'a: 'b, 'b>(
        &'a self,
        row: usize,
        col: usize,
    ) -> [Option<((usize, usize), &T)>; 4] {
        let (r, c) = (row, col);
        [
            // Top
            r.checked_sub(1)
                .and_then(|r| self.get(r, c).map(|v| ((r, c), v))),
            // Left
            c.checked_sub(1)
                .and_then(|c| self.get(r, c).map(|v| ((r, c), v))),
            // Right
            c.checked_add(1).and_then(|c| {
                if c < self.columns() {
                    self.get(r, c).map(|v| ((r, c), v))
                } else {
                    None
                }
            }),
            // Bottom
            r.checked_add(1).and_then(|r| {
                if r < self.rows() {
                    self.get(r, c).map(|v| ((r, c), v))
                } else {
                    None
                }
            }),
        ]
    }

    pub fn orthogonal_neighbours_with_indices<'a: 'b, 'b>(
        &'a self,
        row: usize,
        col: usize,
    ) -> [Option<((usize, usize), &T)>; 8] {
        let (r, c) = (row, col);
        [
            // Top-left
            r.checked_sub(1).and_then(|r| {
                c.checked_sub(1)
                    .and_then(|c| self.get(r, c).map(|v| ((r, c), v)))
            }),
            // Top-middle
            r.checked_sub(1)
                .and_then(|r| self.get(r, c).map(|v| ((r, c), v))),
            // Top-right
            r.checked_sub(1).and_then(|r| {
                c.checked_add(1).and_then(|c| {
                    if c < self.columns() {
                        self.get(r, c).map(|v| ((r, c), v))
                    } else {
                        None
                    }
                })
            }),
            // Middle-left
            c.checked_sub(1)
                .and_then(|c| self.get(r, c).map(|v| ((r, c), v))),
            // Middle-right
            c.checked_add(1).and_then(|c| {
                if c < self.columns() {
                    self.get(r, c).map(|v| ((r, c), v))
                } else {
                    None
                }
            }),
            // Bottom-left
            r.checked_add(1).and_then(|r| {
                if r < self.rows() {
                    c.checked_sub(1)
                        .and_then(|c| self.get(r, c).map(|v| ((r, c), v)))
                } else {
                    None
                }
            }),
            // Bottom-middle
            r.checked_add(1).and_then(|r| {
                if r < self.rows() {
                    self.get(r, c).map(|v| ((r, c), v))
                } else {
                    None
                }
            }),
            // Bottom-right
            r.checked_add(1).and_then(|r| {
                if r < self.rows() {
                    c.checked_add(1).and_then(|c| {
                        if c < self.columns() {
                            self.get(r, c).map(|v| ((r, c), v))
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            }),
        ]
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

impl<'m, T> Index<(usize, usize)> for &'m Matrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.get(row, col).unwrap()
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.get(row, col).unwrap()
    }
}

impl<'m, T> Index<(usize, usize)> for &'m mut Matrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.get(row, col).unwrap()
    }
}

impl<'m, T> IndexMut<(usize, usize)> for &'m mut Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.get_mut(row, col).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.get_mut(row, col).unwrap()
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
        self.rows.len() - self.column
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

trait ExtendFromSliceUnchecked<T> {
    unsafe fn extend_from_slice_unchecked(&mut self, slice: &[T]);
}

impl<T> ExtendFromSliceUnchecked<T> for Vec<T> {
    #[inline]
    unsafe fn extend_from_slice_unchecked(&mut self, slice: &[T]) {
        let len = self.len();
        let amt = slice.len();

        std::ptr::copy_nonoverlapping(slice.as_ptr(), self.as_mut_ptr().add(len), amt);
        self.set_len(len + amt);
    }
}

pub struct ElementIter<'m, T> {
    matrix: &'m Matrix<T>,
    start: usize,
    end: usize,
}

impl<'m, T> Iterator for ElementIter<'m, T> {
    type Item = ((usize, usize), &'m T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let width = self.matrix.width();
        let row = self.start / width;
        let col = self.start % width;
        let next = self.matrix.get(row, col)?;
        self.start += 1;

        Some(((row, col), next))
    }
}

impl<'m, T> DoubleEndedIterator for ElementIter<'m, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end <= self.start {
            return None;
        }

        let width = self.matrix.width();
        let row = self.end / width;
        let col = self.end % width;
        let next = self.matrix.get(row, col)?;
        self.end -= 1;

        Some(((row, col), next))
    }
}

impl<'m, T> ExactSizeIterator for ElementIter<'m, T> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

pub struct ElementIterMut<'m, T> {
    slice: &'m [Cell<T>],
    width: usize,
    start: usize,
    end: usize,
}

impl<'m, T> ElementIterMut<'m, T> {
    fn new(matrix: &'m mut Matrix<T>) -> Self {
        let width = matrix.width();
        let slice = &mut matrix.inner[..];
        let slice = Cell::from_mut(slice).as_slice_of_cells();
        unsafe { Self::new_shared(slice, width) }
    }

    /// # Safety
    /// No two `ColumnIterMut`s can have the same `column` at the same time.
    unsafe fn new_shared(slice: &'m [Cell<T>], width: usize) -> Self {
        Self {
            slice,
            width,
            start: 0,
            end: slice.len(),
        }
    }
}

impl<'m, T> Iterator for ElementIterMut<'m, T> {
    type Item = ((usize, usize), &'m mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let width = self.width;
        let row = self.start / width;
        let col = self.start % width;

        let next = self.slice.get(self.start)?;
        self.start += 1;

        // SAFETY: No other `ElementIterMut` has this column.
        Some(((row, col), unsafe { &mut *next.as_ptr() }))
    }
}

impl<'m, T> DoubleEndedIterator for ElementIterMut<'m, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end <= self.start {
            return None;
        }

        let width = self.width;
        let row = self.end / width;
        let col = self.end % width;

        let next = self.slice.get(self.end)?;
        self.end -= 1;

        Some(((row, col), unsafe { &mut *next.as_ptr() }))
    }
}

impl<'m, T> ExactSizeIterator for ElementIterMut<'m, T> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<T, R> FromIterator<R> for Matrix<T>
where
    T: Clone,
    R: AsRef<[T]>,
{
    fn from_iter<I: IntoIterator<Item = R>>(iter: I) -> Self {
        let mut matrix = Matrix::new();

        for row in iter {
            matrix.push(row);
        }

        matrix
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

    #[test]
    fn iter_elements_mut() {
        let mut matrix = Matrix::new();
        matrix.push([0]);
        matrix.push([0]);
        let mut eles = matrix.iter_elements_mut();
        let ele0 = eles.next().unwrap();
        let _ele1 = eles.next().unwrap();
        *ele0.1 = 2;
        matrix[0][0] = 3;
    }
}
