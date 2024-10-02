use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fmt::Display,
    iter::{repeat, repeat_with},
};

const INPUT: &str = include_str!("../../assets/p12.in");

type Coord = [usize; 2];

#[derive(Default, Clone)]
struct Map<T> {
    grid: Vec<T>,
    size: usize,
}

impl<T> Map<T> {
    fn at(&self, [x, y]: Coord) -> Option<&T> {
        self.grid
            .get(y * self.size..)
            .and_then(|s| s.get(..self.size))
            .and_then(|s| s.get(x))
    }

    fn at_mut(&mut self, [x, y]: Coord) -> Option<&mut T> {
        self.grid
            .get_mut(y * self.size..)
            .and_then(|s| s.get_mut(..self.size))
            .and_then(|s| s.get_mut(x))
    }

    fn neighbouring_coords(&self, [x, y]: Coord) -> impl Iterator<Item = [usize; 2]> + '_ {
        let xpre = x.checked_sub(1);
        let xpost = (0..self.size).nth(x + 1);
        let ypre = y.checked_sub(1);
        let ypost = (0..self.grid.len() / self.size).nth(y + 1);
        [
            // xpre.zip(ypre),
            xpre.zip(Some(y)),
            // xpre.zip(ypost),
            Some(x).zip(ypre),
            // Some(x).zip(Some(y)),
            Some(x).zip(ypost),
            // xpost.zip(ypre),
            xpost.zip(Some(y)),
            // xpost.zip(ypost),
        ]
        .into_iter()
        .flatten()
        .map(|(x, y)| [x, y])
    }

    fn map<O>(self, f: impl Fn(T) -> O) -> Map<O> {
        Map {
            grid: self.grid.into_iter().map(f).collect(),
            size: self.size,
        }
    }
}

impl Map<u8> {
    fn walkable_coords(&self, coord: Coord) -> impl Iterator<Item = [usize; 2]> + '_ {
        let at = self.at(coord).unwrap();
        self.neighbouring_coords(coord)
            .filter(move |&coord| at + 1 >= *self.at(coord).unwrap())
    }

    fn end(&self) -> Coord {
        self.coord_of(b'E').unwrap()
    }

    fn start(&self) -> Coord {
        self.coord_of(b'S').unwrap()
    }
}

impl<T: Default> Map<T> {
    fn empty(size: usize) -> Self {
        Self {
            grid: Vec::from_iter(repeat_with(T::default).take(size * size)),
            size,
        }
    }
}

impl<T: PartialEq> Map<T> {
    fn coord_of(&self, v: T) -> Option<Coord> {
        let p = self.grid.iter().position(|i| *i == v)?;
        Some([p % self.size, p / self.size])
    }
}

impl Display for Map<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+{:-<w$}+", "", w = self.size)?;
        self.grid
            .chunks_exact(self.size)
            .try_for_each(|s| writeln!(f, "|{}|", String::from_iter(s)))?;
        writeln!(f, "+{:-<w$}+", "", w = self.size)
    }
}

fn map(input: &str) -> Map<u8> {
    let Some(line) = input.lines().next() else { return Map::default() };
    let width = line.len();
    let grid: Vec<u8> = input.lines().flat_map(str::bytes).collect();
    assert_eq!(grid.len() % width, 0);
    Map { grid, size: width }
}

fn direction([fx, fy]: [usize; 2], prev: char, [tx, ty]: [usize; 2]) -> char {
    let dir = match [fx.cmp(&tx), fy.cmp(&ty)] {
        [Greater, Equal] => '←',
        [Less, Equal] => '→',
        [Equal, Greater] => '↑',
        [Equal, Less] => '↓',
        v => panic!("{v:?}"),
    };
    match [prev, dir] {
        ['↑' | '⮤' | '⮥', '←'] => '⮢',
        ['→' | '⮡' | '⮣', '↓'] => '⮧',
        ['↓' | '⮧' | '⮦', '←'] => '⮠',
        ['→' | '⮡' | '⮣', '↑'] => '⮥',
        ['↑' | '⮤' | '⮥', '→'] => '⮣',
        ['←' | '⮢' | '⮠', '↓'] => '⮦',
        ['↓' | '⮧' | '⮦', '→'] => '⮡',
        ['←' | '⮢' | '⮠', '↑'] => '⮤',
        _ => dir,
    }
}

fn map_arrow(arrow: char) -> char {
    match arrow {
        '⮢' | '⮧' => '┐',
        '⮠' | '⮥' => '┘',
        '⮣' | '⮦' => '┌',
        '⮡' | '⮤' => '└',
        '←' | '→' => '─',
        '↑' | '↓' => '│',
        _ => arrow,
    }
}

#[derive(Debug)]
struct Maps<'a, T: 'a, I: Iterator<Item = &'a Map<T>> + Clone>(I);

impl<'a, I: Iterator<Item = &'a Map<char>> + Clone> Display for Maps<'a, char, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self.0.clone();
        let lines: Vec<Vec<_>> = lines
            .map(|map| {
                [format!("+{:-<w$}+", "", w = map.size)]
                    .into_iter()
                    .chain(
                        map.grid
                            .chunks_exact(map.size)
                            .map(|s| format!("|{}|", String::from_iter(s))),
                    )
                    .chain([format!("+{:-<w$}+", "", w = map.size)])
                    .collect()
            })
            .collect();
        for i in 0..lines[0].len() {
            for s in lines.iter().map(|v| &v[i]) {
                write!(f, "{s} ")?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Step {
    at: Coord,
    count: usize,
    map: Map<char>,
    prev: char,
    weight: (usize, usize),
}

struct State {
    steps: Vec<Step>,
    map: Map<u8>,
    visited: Map<bool>,
    end: Coord,
}

impl State {
    fn start(mut map: Map<u8>) -> Self {
        let at = map.start();
        let end = map.end();
        *map.at_mut(at).unwrap() = b'a';
        *map.at_mut(end).unwrap() = b'z';
        Self {
            steps: vec![Step {
                at,
                count: 0,
                map: map.clone().map(char::from),
                prev: '?',
                weight: (0, 0),
            }],
            visited: Map::empty(map.size),
            end,
            map,
        }
    }

    fn step(&mut self) -> Option<usize> {
        let Step {
            at,
            count,
            mut map,
            prev,
            ..
        } = self.steps.pop()?;

        map.at_mut(at).map(|v| *v = v.to_ascii_uppercase());
        // println!("{map}");

        if at == self.end {
            self.steps.push(Step {
                at,
                count,
                map,
                prev,
                weight: (0, 0),
            });
            return Some(count);
        }
        *self.visited.at_mut(at).unwrap() = true;

        let coords = self
            .map
            .walkable_coords(at)
            .filter(|&coord| !*self.visited.at(coord).unwrap());

        let from = at;

        let [ex, ey] = self.end;
        let weight = |[x, y]: [usize; 2]| {
            let x = x.abs_diff(ex);
            let y = y.abs_diff(ey);
            (usize::MAX - count, x * x + y * y)
        };
        let count = count + 1;

        for coord in coords {
            let weight = weight(coord);
            let (Ok(i) | Err(i)) = self.steps.binary_search_by_key(&weight, |step| step.weight);
            let mut map = map.clone();
            let dir = direction(from, prev, coord);
            *map.at_mut(from).unwrap() = map_arrow(dir);
            let step = Step {
                at: coord,
                count,
                map,
                prev: dir,
                weight,
            };
            self.steps.insert(i, step);
        }
        None
    }
}

fn solve_1(input: &str) -> usize {
    let mut state = State::start(map(input));
    println!("{}", state.map.clone().map(char::from));
    'l: loop {
        for _ in 0..100 {
            if let Some(count) = state.step() {
                // println!("{}", state.steps.last().unwrap().map);
                break 'l count;
            }
        }
        let maps = Maps(state.steps.iter().map(|v| &v.map));
        println!("{maps}");
    }
}

fn solve_2(input: &str) -> usize {
    todo!()
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
        Sabqponm\n\
        abcryxxl\n\
        accszExk\n\
        acctuvwj\n\
        abdefghi\n";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 31);
        assert_eq!(super::solve_2(TEST_INPUT), todo!());
    }
}
