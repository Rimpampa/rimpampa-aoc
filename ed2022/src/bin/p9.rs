use std::{
    collections::HashSet,
    iter::successors,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

const INPUT: &str = include_str!("../../assets/p9.in");

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Knot<T>(T, T);

impl<T> Knot<T> {
    fn map<O>(self, f: impl FnOnce(T) -> O + Copy) -> Knot<O> {
        Knot(f(self.0), f(self.1))
    }

    fn zip<U>(self, other: Knot<U>) -> Knot<(T, U)> {
        Knot((self.0, other.0), (self.1, other.1))
    }
}

impl<A, B> Knot<(A, B)> {
    fn map_unzip<O>(self, f: impl FnOnce(A, B) -> O + Copy) -> Knot<O> {
        Knot(f(self.0 .0, self.0 .1), f(self.1 .0, self.1 .1))
    }
}

impl<T: Add> Add for Knot<T> {
    type Output = Knot<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip(rhs).map_unzip(T::add)
    }
}

impl<T: Add + Copy + From<T::Output>> AddAssign for Knot<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self + rhs).map(<_>::from)
    }
}

impl<T: Sub> Sub for Knot<T> {
    type Output = Knot<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip(rhs).map_unzip(T::sub)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Move {
    Forward,
    Backward,
}

impl Move {
    fn to_unit(self) -> isize {
        match self {
            Self::Forward => 1,
            Self::Backward => -1,
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
    head: Knot<isize>,
    visited: HashSet<Knot<isize>>,
    knots: [Knot<isize>; LENGTH],
}

impl<const LENGTH: usize> Rope<LENGTH> {
    fn start() -> Self {
        Self {
            head: Knot(0, 0),
            visited: HashSet::from([Knot(0, 0)]),
            knots: [Knot(0, 0); LENGTH],
        }
    }

    fn move_tail(&mut self, at: usize, prev: Knot<isize>) {
        let k = &mut self.knots[at];
        match prev - *k {
            o @ Knot(2 | -2, 0) => k.0 += o.0 / 2,
            o @ Knot(0, 2 | -2) => k.1 += o.1 / 2,
            o @ Knot(2 | -2, 1 | -1) => *k += Knot(o.0 / 2, o.1),
            o @ Knot(1 | -1, 2 | -2) => *k += Knot(o.0, o.1 / 2),
            o @ Knot(2 | -2, 2 | -2) => *k += o.map(|v| v / 2),
            _ => return,
        }
        if at + 1 < LENGTH {
            self.move_tail(at + 1, self.knots[at])
        } else {
            self.visited.insert(self.knots[at]);
        }
    }

    fn move_head(&mut self, direction: Direction<Move>) {
        match direction {
            Direction::Horizontal(m) => self.head.0 += m.to_unit(),
            Direction::Vertical(m) => self.head.1 += m.to_unit(),
        }
        self.move_tail(0, self.head)
    }
}

fn solve<const LENGTH: usize>(input: &str) -> usize {
    let mut state = Rope::<LENGTH>::start();
    for m in directions(input).flat_map(Direction::moves) {
        state.move_head(m)
    }
    state.visited.len()
}

fn main() {
    println!("Answer 1: {}", solve::<1>(INPUT));
    println!("Answer 2: {}", solve::<9>(INPUT));
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
        assert_eq!(super::solve::<1>(TEST_INPUT), 13);
        assert_eq!(super::solve::<9>(TEST_INPUT), 1);
        assert_eq!(super::solve::<9>(SECOND_TEST_INPUT), 36);
    }
}
