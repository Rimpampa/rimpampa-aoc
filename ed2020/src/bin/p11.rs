use ed2020 as base;

use std::{fmt, ops};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Occupied,
    Empty,
    Floor,
}

impl State {
    fn is_occupied(&self) -> bool {
        *self == State::Occupied
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Floor => write!(f, "."),
            State::Occupied => write!(f, "#"),
            State::Empty => write!(f, "L"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    state: State,
    change: bool,
}

impl Tile {
    pub fn new(c: char) -> Self {
        Self {
            state: match c {
                '.' => State::Floor,
                'L' => State::Empty,
                '#' => State::Occupied,
                _ => unreachable!(),
            },
            change: false,
        }
    }

    fn change(&mut self) -> bool {
        if self.change {
            self.state = match self.state {
                State::Empty => State::Occupied,
                State::Occupied => State::Empty,
                State::Floor => State::Floor,
            };
            self.change = false;
            true
        } else {
            false
        }
    }
}

impl ops::Deref for Tile {
    type Target = State;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.change {
            write!(f, ">{}<", self.state)
        } else {
            write!(f, " {} ", self.state)
        }
    }
}

#[derive(Debug)]
struct Ferry {
    tiles: Vec<Vec<Tile>>,
}

impl Ferry {
    fn new(s: &str) -> Self {
        let mut tiles = Vec::new();
        let mut row = Vec::new();
        for line in s.lines() {
            for ch in line.chars() {
                row.push(Tile::new(ch));
            }
            let len = row.len();
            tiles.push(row);
            row = Vec::with_capacity(len);
        }
        Self { tiles }
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn predict_next(&mut self) -> bool {
        let (h, w) = (self.height(), self.width());
        for (y, x) in (0..h).flat_map(|y| (0..w).map(move |x| (y, x))) {
            let change = match self.tiles[y][x] {
                Tile { change: true, .. } => true,
                Tile {
                    state: State::Empty,
                    ..
                } => {
                    let mut b = false;
                    base::neighbours![
                        [x] as x in 0..w,
                        [y] as y in 0..h
                        => b = b || self.tiles[y][x].is_occupied()
                    ];
                    !b
                }
                Tile {
                    state: State::Occupied,
                    ..
                } => {
                    let mut occ = 0;
                    base::neighbours![
                        [x] as x in 0..w,
                        [y] as y in 0..h
                        => occ += self.tiles[y][x].is_occupied() as usize
                    ];
                    occ > 3
                }
                _ => false,
            };
            self.tiles[y][x].change = change;
        }
        self.tiles
            .iter_mut()
            .flat_map(|v| v.iter_mut())
            .fold(false, |p, t| t.change() || p)
    }

    fn sees_occupied(&self, mut x: usize, mut y: usize, dir: (isize, isize)) -> bool {
        while ((x > 0 && x + 1 < self.width()) || dir.0 == 0)
            && ((y > 0 && y + 1 < self.height()) || dir.1 == 0)
            && !matches!(*self.tiles[y][x], State::Occupied | State::Empty)
        {
            if dir.0.is_positive() {
                x += dir.0 as usize;
            } else {
                x -= dir.0.abs() as usize;
            }
            if dir.1.is_positive() {
                y += dir.1 as usize;
            } else {
                y -= dir.1.abs() as usize;
            }
        }
        self.tiles[y][x].is_occupied()
    }

    fn predict_next_better(&mut self) -> bool {
        let (h, w) = (self.height(), self.width());
        for (y, x) in (0..h).flat_map(|y| (0..w).map(move |x| (y, x))) {
            let change = match self.tiles[y][x] {
                Tile { change: true, .. } => true,
                Tile {
                    state: State::Empty,
                    ..
                } => {
                    let mut b = false;
                    base::neighbours![
                        [x, ox] as x in 0..w,
                        [y, oy] as y in 0..h
                        => b = b || self.sees_occupied(x, y, (ox, oy))
                    ];
                    !b
                }
                Tile {
                    state: State::Occupied,
                    ..
                } => {
                    let mut occ = 0;
                    base::neighbours![
                        [x, ox] as x in 0..w,
                        [y, oy] as y in 0..h
                        => occ += self.sees_occupied(x, y, (ox, oy)) as usize
                    ];
                    occ > 4
                }
                _ => false,
            };
            self.tiles[y][x].change = change;
        }
        self.tiles
            .iter_mut()
            .flat_map(|v| v.iter_mut())
            .fold(false, |p, t| t.change() || p)
    }

    fn occupied(&self) -> usize {
        self.tiles
            .iter()
            .flat_map(|v| v.iter())
            .filter(|t| t.is_occupied())
            .count()
    }
}

impl fmt::Display for Ferry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let input = base::get_input(11).unwrap();

    let mut ferry = Ferry::new(&input);
    let mut furry = Ferry::new(&input);

    while ferry.predict_next() {}
    println!("Occupied: {}", ferry.occupied());

    while furry.predict_next_better() {
        // println!("{}", furry)
    }
    println!("Occupied: {}", furry.occupied());
}
