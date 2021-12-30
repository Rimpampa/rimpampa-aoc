use ed2020 as base;

use collections::HashMap;
use std::{collections, str};

#[derive(Copy, Clone)]
struct BitMask {
    and: u64,
    or: u64,
}

impl BitMask {
    fn new() -> Self {
        Self { and: 0, or: 0 }
    }

    fn apply(&self, v: u64) -> u64 {
        (v & self.and) | self.or
    }
}

impl str::FromStr for BitMask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 36 {
            return Err(());
        }
        let mut and = 0;
        let mut or = 0;
        for ch in s.chars() {
            and <<= 1;
            or <<= 1;
            match ch {
                'X' => and |= 1,
                '1' => or |= 1,
                '0' => (),
                _ => return Err(()),
            }
        }
        Ok(Self { and, or })
    }
}

impl IntoIterator for BitMask {
    type IntoIter = BitMaskIterator;
    type Item = BitMask;

    fn into_iter(self) -> Self::IntoIter {
        BitMaskIterator {
            mask: self.and,
            v: self.or,
            i: 0,
        }
    }
}

struct BitMaskIterator {
    v: u64,
    mask: u64,
    i: u64,
}

impl Iterator for BitMaskIterator {
    type Item = BitMask;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= 1 << self.mask.count_ones() {
            None
        } else {
            let mut a = 0;
            let mut b = 1;
            let mut c = 1 << self.mask.trailing_zeros();
            while b <= self.i {
                if b & self.i > 0 {
                    a |= c;
                }
                c <<= 1;
                while c <= self.mask && c & self.mask == 0 {
                    c <<= 1;
                }
                b <<= 1;
            }
            self.i += 1;
            Some(BitMask {
                and: !(self.v + self.mask),
                or: a + self.v,
            })
        }
    }
}

fn main() {
    let input = base::get_input(14).unwrap();
    let mut mem = HashMap::new();
    let mut mem_mapped = HashMap::new();

    let mut mask = BitMask::new();
    for line in input.lines() {
        let (istr, val) = base::split_at(line, '=').unwrap();
        let (istr, val) = (&istr[..istr.len() - 1], &val[1..]);
        match istr {
            "mask" => mask = val.parse().unwrap(),
            _ if &istr[..4] == "mem[" => {
                let e = istr.find(']').unwrap();
                let idx: u64 = istr[4..e].parse().unwrap();
                let value = val.parse().unwrap();
                let x = mask.apply(value);
                if x == 0 {
                    mem.remove(&idx);
                } else {
                    mem.insert(idx, x);
                }
                if value == 0 {
                    for idx_mask in mask {
                        mem_mapped.remove(&idx_mask.apply(idx));
                    }
                } else {
                    for idx_mask in mask {
                        mem_mapped.insert(idx_mask.apply(idx), value);
                    }
                }
            }
            _ => panic!(),
        }
    }
    let sum = mem.values().sum::<u64>();
    let sum_mapped = mem_mapped.values().sum::<u64>();
    println!("Sum: {}", sum);
    println!("Mapped: {}", sum_mapped);
}
