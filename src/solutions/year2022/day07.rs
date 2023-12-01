use std::{collections::HashMap, fmt::Display, hint::unreachable_unchecked};

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
    let mut done = 0;

    for i in 0..eights {
        let i = i * 8;
        let end = len - i;
        let m = usizex8::from_array(POW_10[..8].try_into().unwrap_unchecked()).reverse();
        let v =
            u8x8::from_array(arr[end - 8..end].try_into().unwrap_unchecked()) & u8x8::splat(0xf);
        out += (m * v.cast()).reduce_sum() * POW_10.get_unchecked(i + done);
    }
    done += eights * 8;

    for i in 0..fours {
        let i = i * 4;
        let end = len - done - i;
        let m = usizex4::from_array(POW_10[..4].try_into().unwrap_unchecked()).reverse();
        let v =
            u8x4::from_array(arr[end - 4..end].try_into().unwrap_unchecked()) & u8x4::splat(0xf);
        out += (m * v.cast()).reduce_sum() * POW_10.get_unchecked(i + done);
    }

    let mut temp = 0;
    for e in arr.iter().take(rem) {
        temp *= 10;
        temp += (e & 0xf) as usize;
    }

    out + temp * POW_10.get_unchecked(len - len % 4)
}

mod tests {
    use super::as_usize;

    #[allow(clippy::inconsistent_digit_grouping)]
    #[test]
    fn u64_parse() {
        unsafe {
            unsafe fn correct(s: &str) {
                assert_eq!(as_usize(s.as_bytes()), s.parse().unwrap());
            }
            correct("123");
            correct("1234");
            correct("12345");
            if cfg!(target_pointer_width = "16") {
                return;
            }

            correct("123456");
            correct("1234567");
            correct("12345678");
            correct("123456789");
            correct("1234567898");
            if cfg!(target_pointer_width = "32") {
                return;
            }

            correct("12345678987");
            correct("123456789876");
            correct("1234567898765");
            correct("12345678987654");
            correct("123456789876543");
            correct("1234567898765432");
            correct("12345678987654321");
            correct("123456789876543212");
            correct("1234567898765432123");
            correct("12345678987654321234");
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
                dir.extend_from_slice(after.trim_ascii_start());
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
