use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fmt::Debug,
    str::FromStr,
};

const INPUT: &str = include_str!("../../assets/p5.in");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point(u16, u16);

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(',').ok_or(())?;
        Ok(Self(a.parse().or(Err(()))?, b.parse().or(Err(()))?))
    }
}

#[derive(Clone, Copy)]
struct Segment(Point, Point, bool);

impl Iterator for Segment {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        use Segment as S;

        let p = self.0;
        match *self {
            S(.., true) => return None,
            S(a, b, ref mut e) if a == b => *e = true,
            S(ref mut a, Point(x, y), _) => {
                match a.0.cmp(&x) {
                    Less => a.0 += 1,
                    Greater => a.0 -= 1,
                    Equal => (),
                }
                match a.1.cmp(&y) {
                    Less => a.1 += 1,
                    Greater => a.1 -= 1,
                    Equal => (),
                }
            }
        }
        Some(p)
    }
}

impl FromStr for Segment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" -> ").ok_or(())?;
        Ok(Self(a.parse().or(Err(()))?, b.parse().or(Err(()))?, false))
    }
}

fn solve(input: &str, filter: impl Fn(&Segment) -> bool) -> usize {
    let mut points: Vec<Point> = input
        .lines()
        .map(Segment::from_str)
        .map(Result::unwrap)
        .filter(filter)
        .flatten()
        .collect();
    points.sort_unstable();
    points
        .windows(2)
        .filter(|v| v[0] == v[1])
        .fold((0, None), |(count, prev), next| {
            (count + (prev != Some(next[0])) as usize, Some(next[0]))
        })
        .0
}

fn solve_1(input: &str) -> usize {
    solve(input, |Segment(a, b, _)| a.0 == b.0 || a.1 == b.1)
}

fn solve_2(input: &str) -> usize {
    solve(input, |_| true)
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 5);
        assert_eq!(super::solve_2(TEST_INPUT), 12);
    }
}
