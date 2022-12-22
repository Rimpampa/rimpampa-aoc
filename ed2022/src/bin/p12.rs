use std::cmp::Ordering::{Equal, Greater, Less};
use std::cmp::Reverse;
use std::iter::repeat_with;

const INPUT: &str = include_str!("../../assets/p12.in");

type Coord = [usize; 2];

#[derive(Default)]
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
            xpre.zip(Some(y)),
            Some(x).zip(ypre),
            Some(x).zip(ypost),
            xpost.zip(Some(y)),
        ]
        .into_iter()
        .flatten()
        .map(|(x, y)| [x, y])
    }
}

impl Map<u8> {
    fn is_climbable(&self, from: Coord, to: Coord) -> bool {
        let from = *self.at(from).unwrap();
        let to = *self.at(to).unwrap();
        to <= from + 1
    }

    fn is_descendable(&self, from: Coord, to: Coord) -> bool {
        let from = *self.at(from).unwrap();
        let to = *self.at(to).unwrap();
        to >= from - 1
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

fn map(input: &str) -> Map<u8> {
    let Some(line) = input.lines().next() else { return Map::default() };
    let width = line.len();
    let grid: Vec<u8> = input.lines().flat_map(str::bytes).collect();
    assert_eq!(grid.len() % width, 0);
    Map { grid, size: width }
}

type Weight = Reverse<(usize, usize)>;

#[derive(Clone, Copy)]
struct Step {
    at: Coord,
    count: usize,
    weight: Weight,
}

struct PathToEnd<WeightFn, ReachFn, FilterFn>
where
    WeightFn: Fn(&Map<u8>, Coord, usize) -> Weight,
    ReachFn: Fn(&Map<u8>, Coord) -> bool,
    FilterFn: Fn(&Map<u8>, Coord, Coord) -> bool,
{
    steps: Vec<Step>,
    map: Map<u8>,
    visited: Map<bool>,
    reach: ReachFn,
    weight: WeightFn,
    filter: FilterFn,
}

impl<WeightFn, ReachFn, FilterFn> PathToEnd<WeightFn, ReachFn, FilterFn>
where
    WeightFn: Fn(&Map<u8>, Coord, usize) -> Weight,
    ReachFn: Fn(&Map<u8>, Coord) -> bool,
    FilterFn: Fn(&Map<u8>, Coord, Coord) -> bool,
{
    fn start(
        map: Map<u8>,
        start: Coord,
        reach: ReachFn,
        weight: WeightFn,
        filter: FilterFn,
    ) -> Self {
        Self {
            steps: vec![Step {
                at: start,
                count: 0,
                weight: Reverse((0, 0)),
            }],
            visited: Map::empty(map.size),
            reach,
            map,
            weight,
            filter,
        }
    }

    fn step(&mut self) -> Option<usize> {
        let step = self.steps.pop().unwrap();
        if (self.reach)(&self.map, step.at) {
            return Some(step.count);
        }
        *self.visited.at_mut(step.at).unwrap() = true;

        let coords = self
            .map
            .neighbouring_coords(step.at)
            .filter(|&to| (self.filter)(&self.map, step.at, to))
            .filter(|&coord| !*self.visited.at(coord).unwrap());

        let count = step.count + 1;
        for at in coords {
            let same = self.steps.iter().position(|step| step.at == at);
            if let Some(true) = same.map(|i| self.steps[i].count <= count) {
                continue;
            }
            let weight = (self.weight)(&self.map, at, step.count);
            let (Ok(i) | Err(i)) = self.steps.binary_search_by_key(&weight, |step| step.weight);
            match same {
                None => self.steps.insert(i, Step { at, count, weight }),
                Some(other) => {
                    match i.cmp(&other) {
                        Less => self.steps.copy_within(i..other, i + 1),
                        Greater => self.steps.copy_within(other + 1..=i, other),
                        Equal => (),
                    }
                    self.steps[i] = Step { at, count, weight };
                }
            }
        }
        None
    }
}

fn solve_1(input: &str) -> usize {
    let mut map = map(input);
    let start = map.start();
    let end = map.end();
    *map.at_mut(start).unwrap() = b'a';
    *map.at_mut(end).unwrap() = b'z';
    let mut path = PathToEnd::start(
        map,
        start,
        |_, at| at == end,
        |_, [x, y], steps| {
            let x = x.abs_diff(end[0]);
            let y = y.abs_diff(end[1]);
            Reverse((steps, x * x + y * y))
        },
        Map::is_climbable,
    );
    loop {
        if let Some(count) = path.step() {
            break count;
        }
    }
}

fn solve_2(input: &str) -> usize {
    let mut map = map(input);
    let start = map.start();
    let end = map.end();
    *map.at_mut(start).unwrap() = b'a';
    *map.at_mut(end).unwrap() = b'z';
    let mut path = PathToEnd::start(
        map,
        end,
        |map, at| *map.at(at).unwrap() == b'a',
        |map, at, steps| Reverse((steps, *map.at(at).unwrap() as usize)),
        Map::is_descendable,
    );
    loop {
        if let Some(count) = path.step() {
            break count;
        }
    }
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
        assert_eq!(super::solve_2(TEST_INPUT), 29);
    }
}
