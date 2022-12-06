use std::collections::{HashMap, HashSet};

pub trait Unique {
    fn unique(&self) -> bool;
}

impl<const N: usize> Unique for [u8; N] {
    fn unique(&self) -> bool {
        let mut found = (0u128, 0u128);

        for c in self {
            if *c < 128 {
                let val = 1 << c;
                if (found.0 & val) != 0 {
                    return false;
                }
                found.0 |= val;
            } else {
                let val = 1 << (c - 128);
                if (found.1 & val) != 0 {
                    return false;
                }
                found.1 |= val;
            }
        }
        true
    }
}

impl Unique for [u8] {
    fn unique(&self) -> bool {
        let mut found = (0u128, 0u128);

        for c in self {
            if *c < 128 {
                let val = 1 << c;
                if (found.0 & val) != 0 {
                    return false;
                }
                found.0 |= val;
            } else {
                let val = 1 << (c - 128);
                if (found.1 & val) != 0 {
                    return false;
                }
                found.1 |= val;
            }
        }
        true
    }
}

mod tests {
    use std::array;

    use crate::helper::util::Unique;

    #[test]
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
