use std::{collections::HashSet, iter::successors, ops::Add, str::FromStr};

const INPUT: &str = include_str!("../../assets/p9.in");

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Point<T>(T, T);

impl<T> Point<T> {
    fn map<O>(self, f: impl FnOnce(T) -> O + Copy) -> Point<O> {
        Point(f(self.0), f(self.1))
    }

    fn zip<U>(self, other: Point<U>) -> Point<(T, U)> {
        Point((self.0, other.0), (self.1, other.1))
    }

    fn reduce<O>(self, f: impl FnOnce(T, T) -> O) -> O {
        f(self.0, self.1)
    }
}

impl Point<Position> {
    fn to_direction(self) -> Option<Direction<Move>> {
        use Direction::*;
        use Move::*;
        use Position::*;
        match self {
            Point(Same, After) => Some(Vertical(Forward)),
            Point(Same, Before) => Some(Vertical(Backward)),
            Point(After, Same) => Some(Horizontal(Forward)),
            Point(Before, Same) => Some(Horizontal(Backward)),
            _ => None,
        }
    }
}

impl<T: Add> Add for Point<T> {
    type Output = Point<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

trait Directional: Copy {
    fn opposite(self) -> Self;
    fn is_same(self, other: Self) -> bool;
    fn is_opposite(self, other: Self) -> bool {
        self.opposite().is_same(other)
    }
    fn to_unit(self) -> isize;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Position {
    Before,
    Same,
    After,
}

impl Position {
    fn moved(self, m: Self) -> Self {
        use Position::*;
        match (self, m) {
            (Before, After) | (After, Before) => Same,
            (Same, After) => After,
            (Same, Before) => Before,
            _ => self,
        }
    }
}

impl Directional for Position {
    fn opposite(self) -> Self {
        match self {
            Self::After => Self::Before,
            Self::Before => Self::After,
            Self::Same => Self::Same,
        }
    }

    fn is_same(self, other: Self) -> bool {
        self == other
    }

    fn is_opposite(self, other: Self) -> bool {
        use Position::*;
        matches!((self, other), (After, Before) | (Before, After))
    }

    fn to_unit(self) -> isize {
        match self {
            Self::Before => -1,
            Self::Same => 0,
            Self::After => 1,
        }
    }
}

impl From<Move> for Position {
    fn from(m: Move) -> Self {
        match m {
            Move::Forward => Self::After,
            Move::Backward => Self::Before,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Move {
    Forward,
    Backward,
}

impl Directional for Move {
    fn opposite(self) -> Self {
        match self {
            Self::Forward => Self::Backward,
            Self::Backward => Self::Forward,
        }
    }

    fn is_same(self, other: Self) -> bool {
        self == other
    }

    fn to_unit(self) -> isize {
        match self {
            Move::Forward => 1,
            Move::Backward => -1,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction<T> {
    Vertical(T),
    Horizontal(T),
}

impl<T> Direction<T> {
    fn map<O>(self, f: impl FnOnce(T) -> O) -> Direction<O> {
        match self {
            Direction::Vertical(v) => Direction::Vertical(f(v)),
            Direction::Horizontal(v) => Direction::Horizontal(f(v)),
        }
    }
}

impl Direction<isize> {
    fn moves(self) -> impl Iterator<Item = Direction<Move>> {
        use Direction::*;
        let filter0 = |d| (!matches!(d, Vertical(0) | Horizontal(0))).then_some(d);
        successors(filter0(self), move |&dir| match dir {
            Vertical(0) | Horizontal(0) => unreachable!(),
            Vertical(1..) | Horizontal(1..) => filter0(dir.map(|n| n - 1)),
            Vertical(_) | Horizontal(_) => filter0(dir.map(|n| n + 1)),
        })
        .map(|dir| match dir {
            Vertical(0) | Horizontal(0) => unreachable!(),
            Vertical(1..) | Horizontal(1..) => dir.map(|_| Move::Forward),
            Vertical(_) | Horizontal(_) => dir.map(|_| Move::Backward),
        })
    }
}

impl FromStr for Direction<isize> {
    type Err = <isize as FromStr>::Err;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some(n) = line.strip_prefix("R ") {
            Ok(Self::Horizontal(isize::from_str(n)?))
        } else if let Some(n) = line.strip_prefix("U ") {
            Ok(Self::Vertical(isize::from_str(n)?))
        } else if let Some(n) = line.strip_prefix("L ") {
            Ok(Self::Horizontal(-isize::from_str(n)?))
        } else if let Some(n) = line.strip_prefix("D ") {
            Ok(Self::Vertical(-isize::from_str(n)?))
        } else {
            isize::from_str("").map(|_| unreachable!())
        }
    }
}

fn directions(input: &str) -> impl Iterator<Item = Direction<isize>> + '_ {
    input.lines().map(str::parse).map(Result::unwrap)
}

struct Rope<const LENGTH: usize> {
    head_at: Point<isize>,
    visited: HashSet<Point<isize>>,
    knots: [Point<Position>; LENGTH],
}

impl<const LENGTH: usize> Rope<LENGTH> {
    fn start() -> Self {
        Self {
            head_at: Point(0, 0),
            visited: HashSet::from([Point(0, 0)]),
            knots: [Point(Position::Same, Position::Same); LENGTH],
        }
    }

    fn head_to_tail_offset(&self, tail: usize) -> Point<isize> {
        self.knots[..=tail]
            .into_iter()
            .map(|pos| pos.map(<_>::to_unit))
            .fold(Point(0, 0), Add::add)
    }

    fn visited_at(&mut self, at: usize) {
        if at == LENGTH - 1 {
            let offset = self.head_to_tail_offset(LENGTH - 1);
            self.visited
                .insert(Point(self.head_at.0 + offset.0, self.head_at.1 + offset.1));
        }
    }

    fn move_tail(&mut self, at: usize, direction: Point<Position>) {
        use Direction::*;
        use Position::*;
        if at >= LENGTH || direction == Point(Same, Same) {
            return;
        }
        if let Some(direction) = direction.to_direction() {
            match (direction, &mut self.knots[at]) {
                (Horizontal(m), Point(x, y)) if x.is_opposite(m.into()) => {
                    let ymove = y.opposite();
                    *y = Same;
                    self.move_tail(at + 1, Point(m.into(), ymove));
                    self.visited_at(at);
                }
                (Vertical(m), Point(x, y)) if y.is_opposite(m.into()) => {
                    let xmove = x.opposite();
                    *x = Same;
                    self.move_tail(at + 1, Point(xmove, m.into()));
                    self.visited_at(at);
                }
                (Horizontal(m), Point(x, _)) => *x = x.moved(m.opposite().into()),
                (Vertical(m), Point(_, y)) => *y = y.moved(m.opposite().into()),
            };
        }
        self.move_tail(at + 1, direction);
        self.visited_at(at);
        // let x = self.knots[at].0.is_opposite(direction.0);
        // let y = self.knots[at].1.is_opposite(direction.1);
        // use Position::*;
        // let opposite = knot
        //     .zip(direction)
        //     .map(|(k, d)| k.is_opposite(d.opposite()));
        // if opposite.reduce(|k, d| k || d) {
        //     self.move_tail(at + 1, direction);
        //     self.visited_at(at);
        // } else {
        //     *knot = knot.zip(direction).map(|(k, d)| k.moved(d.opposite()));
        // }
    }

    fn move_head(&mut self, direction: Direction<Move>) {
        use Direction::*;
        use Position::*;
        match direction {
            Horizontal(m) => self.head_at.0 += m.to_unit(),
            Vertical(m) => self.head_at.1 += m.to_unit(),
        }
        match (direction, &mut self.knots[0]) {
            (Horizontal(m), Point(x, y)) if x.is_opposite(m.into()) => {
                let ymove = y.opposite();
                *y = Same;
                self.move_tail(1, Point(m.into(), ymove));
                self.visited_at(0);
            }
            (Vertical(m), Point(x, y)) if y.is_opposite(m.into()) => {
                let xmove = x.opposite();
                *x = Same;
                self.move_tail(1, Point(xmove, m.into()));
                self.visited_at(0);
            }
            (Horizontal(m), Point(x, _)) => *x = x.moved(m.opposite().into()),
            (Vertical(m), Point(_, y)) => *y = y.moved(m.opposite().into()),
        };
    }

    fn print(&self) {
        let mut visited = self.visited.clone();
        let tails = (0..LENGTH).map(|i| self.head_to_tail_offset(i) + self.head_at);
        visited.insert(self.head_at);
        visited.extend(tails.clone());

        let xs = visited.iter().map(|Point(x, _)| *x);
        let ys = visited.iter().map(|Point(_, y)| *y);

        let min_x = xs.clone().min().unwrap();
        let min_y = ys.clone().min().unwrap();
        let max_x = xs.clone().max().unwrap();
        let max_y = ys.clone().max().unwrap();

        let line = ['+']
            .into_iter()
            .chain((min_x..=max_x).map(|_| '-'))
            .chain(['+', '\n']);
        line.clone().for_each(|c| print!("{c}"));
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                (x == min_x).then(|| print!("|"));
                if Point(x, y) == self.head_at {
                    print!("H");
                } else if let Some(i) = tails.clone().position(|p| p == Point(x, y)) {
                    print!("{i}");
                } else if visited.contains(&Point(x, y)) {
                    print!("o");
                } else {
                    print!(".");
                }
                (x == max_x).then(|| print!("|"));
            }
            println!();
        }
        line.clone().for_each(|c| print!("{c}"));
    }
}

fn solve_1(input: &str) -> usize {
    let mut state = Rope::<1>::start();
    for m in directions(input).flat_map(Direction::moves) {
        state.move_head(m)
    }
    state.print();
    state.visited.len()
}

fn solve_2(input: &str) -> usize {
    let mut state = Rope::<9>::start();
    for d in directions(input) {
        for m in dbg!(d).moves() {
            state.print();
            state.move_head(m)
        }
    }
    state.print();
    state.visited.len()
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
        R 4\n\
        U 4\n\
        L 3\n\
        D 1\n\
        R 4\n\
        D 1\n\
        L 5\n\
        R 2\n";

    const SECOND_TEST_INPUT: &str = "\
        R 5\n\
        U 8\n\
        L 8\n\
        D 3\n\
        R 17\n\
        D 10\n\
        L 25\n\
        U 20\n";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 13);
        assert_eq!(super::solve_2(TEST_INPUT), 1);
        assert_eq!(super::solve_2(SECOND_TEST_INPUT), 36);
    }
}
