use ed2020 as base;

use collections::hash_map::{Entry, HashMap};
use std::{collections, fmt, iter, ops, str};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Border {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Rotation {
    Straight,
    Once,
    Twice,
    Trice,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Flip {
    None,
    Horizontal,
}

const TRASFORMATIONS: [(Rotation, Flip); 8] = [
    (Rotation::Straight, Flip::None),
    (Rotation::Once, Flip::None),
    (Rotation::Twice, Flip::None),
    (Rotation::Trice, Flip::None),
    (Rotation::Straight, Flip::Horizontal),
    (Rotation::Once, Flip::Horizontal),
    (Rotation::Twice, Flip::Horizontal),
    (Rotation::Trice, Flip::Horizontal),
];

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    data: Vec<bool>,
    side: usize,
    rot: Rotation,
    flip: Flip,
}

impl Tile {
    fn match_borders(&self, other: &Self) -> Option<Border> {
        assert!(self.side == other.side);
        let s = self.side;
        // Origin is at upper left

        // top border
        if (0..s).all(|x| self[(x, 0)] == other[(x, 0)]) {
            return Some(Border::Top);
        }
        // left border
        else if (0..s).all(|y| self[(0, y)] == other[(0, y)]) {
            return Some(Border::Left);
        }
        // bottom border
        else if (0..s).all(|x| self[(x, s - 1)] == other[(x, s - 1)]) {
            return Some(Border::Bottom);
        }
        // right border
        else if (0..s).all(|y| self[(s - 1, y)] == other[(s - 1, y)]) {
            return Some(Border::Right);
        }
        None
    }
}

impl ops::Index<(usize, usize)> for Tile {
    type Output = bool;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let s = self.side;
        // let idx = match self.orientation {
        //     Orientation::Straight => x + s * y,
        //     Orientation::RotatedOnce => s * x + s - 1 - y,
        //     Orientation::RotatedTwice => s * s - 1 - x - s * y,
        //     Orientation::RotatedTrice => s * (s - 1 - x) + y,
        //     Orientation::FilppedVertically => x + s * (s - 1 - y),
        //     Orientation::FilppedHorizontally => s - 1 - x + s * y,
        // };
        let (x, y) = match self.rot {
            Rotation::Straight => (x, y),
            Rotation::Once => (s - y - 1, x),
            Rotation::Twice => (s - x - 1, s - y - 1),
            Rotation::Trice => (y, s - x - 1),
        };
        let idx = match self.flip {
            Flip::None => x + y * s,
            Flip::Horizontal => s - x - 1 + y * s,
            // Flip::Vertical => x + (s - y - 1) * s,
        };
        &self.data[idx]
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        for y in 0..self.side {
            for x in 0..self.side {
                write!(f, "{}", if self[(x, y)] { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl str::FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let tile = lines.next().unwrap(); //.ok_or(())?;
        if tile.starts_with("Tile ") && tile.ends_with(':') {
            let id = tile[5..tile.len() - 1].parse().unwrap(); //.map_err(|_| ())?;
            let first = lines.next().unwrap(); //.ok_or(())?;
            let width = first.len();
            let mut vec = Vec::with_capacity(width * width);
            for line in iter::once(first).chain(lines) {
                vec.extend(line.chars().map(|c| c == '#'))
            }
            if vec.len() == vec.capacity() {
                return Ok(Self {
                    rot: Rotation::Straight,
                    flip: Flip::None,
                    id,
                    data: vec,
                    side: width,
                });
            }
        }
        Err(())
    }
}

fn main() {
    let input = base::get_input(20).unwrap();
    let mut tiles = Vec::<Tile>::new();
    for tile in input.split("\n\n") {
        tiles.push(tile.parse().unwrap());
    }
    let mut mul = 1;
    for i in 0..tiles.len() {
        let mut matching = 0;
        for j in (0..i).chain(i + 1..tiles.len()) {
            't: for (rot, flip) in &TRASFORMATIONS {
                tiles[i].rot = *rot;
                tiles[i].flip = *flip;
                if tiles[i].match_borders(&tiles[j]).is_some() {
                    matching += 1;
                    break 't;
                }
            }
        }
        if matching == 2 {
            mul *= tiles[i].id;
        }
    }
    println!("Mul: {}", mul);

    // origin at lower left

    let mut map = HashMap::<(isize, isize), usize>::new();
    map.insert((0, 0), 0);

    // I'm adding a tile to the map on every loop
    'add: for _ in 1..tiles.len() {
        // for every tile already in the map
        for (&(x, y), &i) in map.iter() {
            // Match it against every other tile
            for j in (0..i).chain(i + 1..tiles.len()) {
                // Don't care about tiles already matched
                if map.values().any(|idx| *idx == j) {
                    continue;
                }
                // Check for every possible transformation if it matches
                'tra: for (rot, flip) in &TRASFORMATIONS {
                    // apply the transformation
                    tiles[j].rot = *rot;
                    tiles[j].flip = *flip;

                    // check the border
                    if let Some(border) = tiles[i].match_borders(&tiles[j]) {
                        // Remember: the border is relative to the tile already in the map
                        let coord = match border {
                            Border::Top => (x, y + 1),
                            Border::Bottom => (x, y - 1),
                            Border::Left => (x - 1, y),
                            Border::Right => (x + 1, y),
                        };
                        // if it's already there continue (even if kinda strange idk)
                        if map.contains_key(&coord) {
                            continue 'tra;
                        }
                        // Now check if it also matches the adjacent tiles

                        if border != Border::Top {
                            if let Some(&idx) = map.get(&(coord.0, coord.1 - 1)) {
                                if tiles[idx].match_borders(&tiles[j]).is_none() {
                                    continue 'tra;
                                }
                            }
                        }
                        if border != Border::Bottom {
                            if let Some(&idx) = map.get(&(coord.0, coord.1 + 1)) {
                                if tiles[idx].match_borders(&tiles[j]).is_none() {
                                    continue 'tra;
                                }
                            }
                        }
                        if border != Border::Right {
                            if let Some(&idx) = map.get(&(coord.0 - 1, coord.1)) {
                                if tiles[idx].match_borders(&tiles[j]).is_none() {
                                    continue 'tra;
                                }
                            }
                        }
                        if border != Border::Left {
                            if let Some(&idx) = map.get(&(coord.0 + 1, coord.1)) {
                                if tiles[idx].match_borders(&tiles[j]).is_none() {
                                    continue 'tra;
                                }
                            }
                        }
                        // add only one tile on each cycle
                        map.insert(coord, j);
                        continue 'add;
                    }
                }
            }
        }
    }
    println!("Map: {:?}", map);
    println!("Map: {:?}", tiles)
}
