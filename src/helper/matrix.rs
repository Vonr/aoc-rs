use std::mem::MaybeUninit;

pub struct Matrix<T, const ROWS: usize, const COLUMNS: usize>
where
    [(); ROWS * COLUMNS]:,
{
    rows_init: usize,
    data: [MaybeUninit<T>; ROWS * COLUMNS],
}

impl<T, const ROWS: usize, const COLUMNS: usize> Default for Matrix<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    pub fn new() -> Self {
        Self {
            rows_init: 0,
            data: MaybeUninit::uninit_array(),
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn get(&self, row: usize, column: usize) -> Option<T>
    where
        T: Copy,
    {
        if self.rows_init + 1 < row {
            return None;
        }
        unsafe { Some(self.data[row * COLUMNS + column].assume_init()) }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn get_cloned(&self, row: usize, column: usize) -> Option<T>
    where
        T: Clone,
    {
        if self.rows_init < row - 1 {
            return None;
        }
        unsafe { Some(self.data[row * COLUMNS + column].assume_init_ref().clone()) }
    }

    pub fn extend(&mut self, row: [T; COLUMNS]) {
        use std::ptr;

        if self.rows_init == ROWS {
            panic!("Attempted to extend Matrix over capacity of ROWS");
        }

        unsafe {
            ptr::copy_nonoverlapping(
                row.as_ptr(),
                self.data.as_mut_ptr().add(self.rows_init * COLUMNS) as *mut T,
                COLUMNS,
            );
        }
        self.rows_init += 1;
    }
}

impl<T, C, const ROWS: usize, const COLUMNS: usize> FromIterator<C> for Matrix<T, ROWS, COLUMNS>
where
    C: IntoIterator<Item = T>,
    [(); ROWS * COLUMNS]:,
{
    fn from_iter<I: IntoIterator<Item = C>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        if let Some(first) = iter.next() {
            let mut out = Self::new();
            for row in iter {
                let row = row.into_iter().collect::<Vec<T>>();
                if row.len() != COLUMNS {
                    panic!("Tried to extend Matrix with row of a different length");
                }
                let mut to_extend: [MaybeUninit<T>; COLUMNS] = MaybeUninit::uninit_array();
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        row.as_ptr(),
                        to_extend.as_mut_ptr() as *mut T,
                        COLUMNS,
                    );
                    out.extend(MaybeUninit::array_assume_init(to_extend));
                }
            }
            return out;
        }

        Self::new()
    }
}

mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let mut matrix = Matrix::<(usize, usize), 10, 10>::new();
        let row = [
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (0, 6),
            (0, 7),
            (0, 8),
            (0, 9),
        ];
        matrix.extend(row);

        for (i, e) in row.iter().enumerate() {
            assert_eq!(matrix.get(0, i), Some(*e));
        }

        matrix.extend(row);
        for (i, e) in row.iter().enumerate() {
            assert_eq!(matrix.get(1, i), Some(*e));
        }
    }
}
