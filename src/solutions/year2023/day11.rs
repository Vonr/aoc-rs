use std::fmt::Display;

use bitvec::{bitvec, order::LocalBits, vec::BitVec};
use bstr::ByteSlice;
use rustc_hash::FxHashSet;
use set_builder::set;

use crate::helper::matrix::Matrix;

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut mat = Matrix::from_iter(input.lines());

    let mut empty_rows = bitvec![usize, LocalBits; 0; mat.rows()];
    let mut empty_columns = bitvec![usize, LocalBits; 0; mat.columns()];

    for (idx, row) in mat.iter_rows().enumerate() {
        if row.find_byte(b'#').is_none() {
            empty_rows.set(idx, true);
        }
    }

    for (idx, mut col) in mat.iter_columns().enumerate() {
        if !col.any(|&b| b == b'#') {
            empty_columns.set(idx, true);
        }
    }

    let mut galaxies = mat
        .iter_elements()
        .filter(|(_pos, &val)| val == b'#')
        .collect::<Vec<_>>();

    let mut sum = 0;

    let mut pairs = set![ (a.0, b.0) : (ai, a) <- galaxies.iter().enumerate(), b <- galaxies.iter().skip(ai+1) ];

    for ((ar, ac), (br, bc)) in pairs {
        for r in (ar.min(br) + 1)..(ar.max(br)) {
            if empty_rows[r] {
                sum += 1;
            }
        }

        for c in (ac.min(bc) + 1)..(ac.max(bc)) {
            if empty_columns[c] {
                sum += 1;
            }
        }

        sum += ar.abs_diff(br) + ac.abs_diff(bc);
    }

    sum
}

pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut mat = Matrix::from_iter(input.lines());

    let mut empty_rows = bitvec![usize, LocalBits; 0; mat.rows()];
    let mut empty_columns = bitvec![usize, LocalBits; 0; mat.columns()];

    for (idx, row) in mat.iter_rows().enumerate() {
        if row.find_byte(b'#').is_none() {
            empty_rows.set(idx, true);
        }
    }

    for (idx, mut col) in mat.iter_columns().enumerate() {
        if !col.any(|&b| b == b'#') {
            empty_columns.set(idx, true);
        }
    }

    let mut galaxies = mat
        .iter_elements()
        .filter(|(_pos, &val)| val == b'#')
        .collect::<Vec<_>>();

    let mut sum = 0;

    let mut pairs = set![ (a.0, b.0) : (ai, a) <- galaxies.iter().enumerate(), b <- galaxies.iter().skip(ai+1) ];

    for ((ar, ac), (br, bc)) in pairs {
        for r in (ar.min(br) + 1)..(ar.max(br)) {
            if empty_rows[r] {
                sum += 1000000 - 1;
            }
        }

        for c in (ac.min(bc) + 1)..(ac.max(bc)) {
            if empty_columns[c] {
                sum += 1000000 - 1;
            }
        }

        sum += ar.abs_diff(br) + ac.abs_diff(bc);
    }

    sum
}
