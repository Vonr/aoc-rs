use std::{collections::HashMap, fmt::Display};

use bstr::{io::BufReadExt, ByteSlice};

use crate::helper::parsing::BytesAsNumber;

unsafe fn as_usize(arr: &[u8]) -> usize {
    use std::simd::*;

    const POW_10: [usize; 20] = [
        1,
        10,
        100,
        1000,
        10000,
        100000,
        1000000,
        10000000,
        100000000,
        1000000000,
        10000000000,
        100000000000,
        1000000000000,
        10000000000000,
        100000000000000,
        1000000000000000,
        10000000000000000,
        100000000000000000,
        1000000000000000000,
        10000000000000000000,
    ];
    let mut out = 0;
    let len = arr.len();
    let eights = len / 8;
    let fours = (len % 8) / 4;
    let rem = len % 4;

    for i in 0..eights {
        let m = usizex8::from_array([1000_0000, 100_0000, 10_0000, 1_0000, 1000, 100, 10, 1]);
        let v = u8x8::from_slice(&arr[len - 8 * (i + 1)..len - 8 * i]) - u8x8::splat(b'0');
        out += (m * v.cast()).reduce_sum() * POW_10.get_unchecked(i * 8);
    }

    for i in 0..fours {
        let m = usizex4::from_array([1000, 100, 10, 1]);
        let v = u8x4::from_slice(&arr[len - eights * 8 - 4 * (i + 1)..len - eights * 8 - 4 * i])
            - u8x4::splat(b'0');
        out += (m * v.cast()).reduce_sum() * POW_10.get_unchecked(i * 4 + eights * 8);
    }

    let mut temp = 0;
    for e in arr.iter().take(rem) {
        temp *= 10;
        temp += (e - b'0') as usize;
    }

    out + temp * POW_10.get_unchecked(len - len % 4)
}

mod tests {
    use super::as_usize;

    #[allow(clippy::inconsistent_digit_grouping)]
    #[test]
    fn u64_parse() {
        assert_eq!(unsafe { as_usize(b"123") }, 123);
        assert_eq!(unsafe { as_usize(b"1234") }, 1234);
        assert_eq!(unsafe { as_usize(b"12345") }, 1_2345);
        if cfg!(target_pointer_width = "16") {
            return;
        }
        assert_eq!(unsafe { as_usize(b"123456") }, 12_3456);
        assert_eq!(unsafe { as_usize(b"1234567") }, 123_4567);
        assert_eq!(unsafe { as_usize(b"12345678") }, 1234_5678);
        assert_eq!(unsafe { as_usize(b"123456789") }, 1_23456789);
        assert_eq!(unsafe { as_usize(b"1234567898") }, 12_34567898);
        if cfg!(target_pointer_width = "64") {
            assert_eq!(unsafe { as_usize(b"12345678987") }, 123_45678987);
            assert_eq!(unsafe { as_usize(b"123456789876") }, 1234_56789876);
            assert_eq!(unsafe { as_usize(b"1234567898765") }, 1_2345_67898765);
            assert_eq!(unsafe { as_usize(b"12345678987654") }, 12_3456_78987654);
            assert_eq!(unsafe { as_usize(b"123456789876543") }, 123_4567_89876543);
            assert_eq!(unsafe { as_usize(b"1234567898765432") }, 12345678_98765432);
            assert_eq!(
                unsafe { as_usize(b"12345678987654321") },
                1_23456789_87654321
            );
            assert_eq!(
                unsafe { as_usize(b"123456789876543212") },
                12_34567898_76543212
            );
            assert_eq!(
                unsafe { as_usize(b"1234567898765432123") },
                123_45678987_65432123
            );
            assert_eq!(
                unsafe { as_usize(b"12345678987654321234") },
                1234_56789876_54321234
            );
        }
    }
}

fn get_sizes(input: &str) -> HashMap<Vec<u8>, usize> {
    let mut sizes: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut dir = Vec::new();
    let mut len: usize = 0;
    let mut size: usize = 0;
    let mut i = 0;

    input.as_bytes().for_byte_line(|line| {
        if line[0] == b'$' {
            if len != 0 {
                sizes
                    .iter_mut()
                    .filter(|(k, _)| dir.starts_with(k))
                    .for_each(|(_, v)| *v += size);
                sizes.insert(dir.clone(), size);
                dir.splice(len.., []);
                len = 0;
                size = 0;
            }

            if let Some(after) = line.strip_prefix(b"$ cd ") {
                match after {
                    b"/" => {
                        dir.splice(.., [b'/']);
                    }
                    b".." => {
                        dir.splice(dir.rfind_byte(b'/').unwrap().., []);
                        if dir.is_empty() {
                            dir.push(b'/');
                        }
                    }
                    _ => {
                        if dir.last().map(|l| *l != b'/').unwrap_or_default() {
                            dir.push(b'/');
                        }
                        dir.extend_from_slice(after);
                    }
                }
            } else if let Some(after) = line.strip_prefix(b"$ ls") {
                len = dir.len();
                if !dir.last().unwrap() == b'/' {
                    dir.push(b'/');
                }
                dir.extend_from_slice(after.trim_start());
            }
        } else if line[0] >= b'0' && line[0] <= b'9' {
            size += unsafe { as_usize(&line[..line.find_byte(b' ').unwrap()]) };
        }
        i += 1;
        Ok(true)
    });
    sizes
}

pub fn part1(input: &str) -> impl Display {
    get_sizes(input)
        .values()
        .filter(|v| **v <= 100000)
        .sum::<usize>()
}

pub fn part2(input: &str) -> impl Display {
    let sizes = get_sizes(input);
    let sum = sizes.get([b'/'].as_slice()).unwrap();
    let free = 70000000 - *sum;
    *sizes
        .values()
        .filter(|v| free + *v > 30000000)
        .min()
        .unwrap()
}
