use std::collections::{HashMap, HashSet};

pub trait Unique {
    fn unique(&self) -> bool;
}

impl<const N: usize> Unique for [u8; N] {
    fn unique(&self) -> bool {
        let mut found = (0u128, 0u128);
        let mut ones = 0;

        for (i, c) in self.iter().enumerate() {
            if *c < 128 {
                let val = 1 << c;
                ones += 1 - ((found.0 & val) >> c);
                found.0 |= val;
            } else {
                let val = 1 << (c - 128);
                ones += 1 - ((found.1 & val) >> (c - 128));
                found.1 |= val;
            }
            if ones as usize != i + 1 {
                return false;
            }
        }
        true
    }
}

impl Unique for [u8] {
    fn unique(&self) -> bool {
        let mut found = (0u128, 0u128);
        let mut ones = 0;

        for (i, c) in self.iter().enumerate() {
            if *c < 128 {
                let val = 1 << c;
                ones += 1 - ((found.0 & val) >> c);
                found.0 |= val;
            } else {
                let val = 1 << (c - 128);
                ones += 1 - ((found.1 & val) >> (c - 128));
                found.1 |= val;
            }
            if ones as usize != i + 1 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::array;

    use crate::helper::util::Unique;

    #[test]
    #[cfg(not(miri))]
    fn unique() {
        let mut all: [u8; 256] = array::from_fn(|i| i as u8);
        assert!(all.unique());

        for i in 0..=255 {
            for j in 0..=255u8 {
                if i as u8 == j {
                    continue;
                }
                all[i] = j;
                assert!(!all.unique());
            }
            all[i] = i as u8;
            assert!(all.unique());
        }
    }
}
