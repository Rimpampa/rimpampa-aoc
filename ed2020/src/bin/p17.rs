use ed2020 as base;

use std::{iter, mem, str};

#[derive(Clone)]
struct EnergyCube {
    curr: Vec<bool>,
    next: Vec<bool>,
    len: usize,
}

#[rustfmt::skip]
impl EnergyCube {
    fn cycle3d(&mut self) {
        let len = self.len;
        let curr = &self.curr;

        for x in 0..len {
            for y in 0..len {
                for z in 0..13 {
                    let idx = |x, y, z| x + y * len + z * len * len + 6 * len * len * 13;
                    let mut active = 0;
                    base::neighbours![
                        [x] as x in 0..len,
                        [y] as y in 0..len,
                        [z] as z in 0..13
                        => active += curr[idx(x, y, z)] as u8
                    ];
                    let i = idx(x, y, z);
                    if self.curr[i] {
                        self.next[i] = matches!(active, 2 | 3);
                    } else {
                        self.next[i] = active == 3;
                    }
                }
            }
        }
        mem::swap(&mut self.curr, &mut self.next);
    }
    
    fn cycle4d(&mut self) {
        let len = self.len;
        let curr = &self.curr;

        for x in 0..len {
            for y in 0..len {
                for z in 0..13 {
                    for w in 0..13 {
                        let idx = |x, y, z, w| x + y * len + z * len * len + w * len * len * 13;
                        let mut active = 0;
                        base::neighbours![
                            [x] as x in 0..len,
                            [y] as y in 0..len,
                            [z] as z in 0..13,
                            [w] as w in 0..13
                            => active += curr[idx(x, y, z, w)] as u8
                        ];
                        let i = idx(x, y, z, w);
                        if self.curr[i] {
                            self.next[i] = matches!(active, 2 | 3);
                        } else {
                            self.next[i] = active == 3;
                        }
                    }
                }
            }
        }
        mem::swap(&mut self.curr, &mut self.next);
    }

    fn count_active(&self) -> usize {
        self.curr.iter().filter(|&v| *v).count()
    }
}

impl str::FromStr for EnergyCube {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let st = lines.next().ok_or(())?;
        let len = st.len() + 12;
        let mut vec = Vec::with_capacity(len * len * 13);
        let pad_tessrct = vec![false; len * len * 13 * 6];
        let pad_cube = vec![false; len * len * 6];
        let pad_plane = vec![false; 6 * len];
        let pad_line = vec![false; 6];

        vec.extend_from_slice(&pad_tessrct);
        vec.extend_from_slice(&pad_cube);
        vec.extend_from_slice(&pad_plane);
        for line in iter::once(st).chain(lines) {
            vec.extend_from_slice(&pad_line);
            for ch in line.chars() {
                vec.push(ch == '#');
            }
            vec.extend_from_slice(&pad_line);
        }
        vec.extend_from_slice(&pad_plane);
        vec.extend_from_slice(&pad_cube);
        vec.extend_from_slice(&pad_tessrct);

        assert_eq!(vec.len(), len * len * 13 * 13);
        Ok(EnergyCube {
            curr: vec.clone(),
            next: vec,
            len,
        })
    }
}

fn main() {
    let input = base::get_input(17).unwrap();
    let mut cube: EnergyCube = input.parse().unwrap();
    let mut tessrct: EnergyCube = cube.clone();
    for _ in 0..6 {
        cube.cycle3d();
        tessrct.cycle4d();
    }
    println!("Active: {}", cube.count_active());
    println!("Active 3D: {}", tessrct.count_active());
}
