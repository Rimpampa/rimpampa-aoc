use std::{iter::repeat, str::FromStr};

const INPUT: &str = include_str!("../../assets/p7.in");

const fn sum(val: usize) -> usize {
    val * (val + 1) / 2
}

fn solve(input: &str, sum: impl Fn((&usize, &usize)) -> usize) -> usize {
    let pos: Vec<_> = input
        .split(',')
        .map(str::trim)
        .map(usize::from_str)
        .map(Result::unwrap)
        .collect();
    let min = *pos.iter().min().unwrap();
    let max = *pos.iter().max().unwrap();
    (min..=max)
        .map(|ref x| pos.iter().zip(repeat(x)).map(&sum).sum())
        .min()
        .unwrap()
}

fn solve_1(input: &str) -> usize {
    solve(input, |(a, b)| a.max(b) - a.min(b))
}

fn solve_2(input: &str) -> usize {
    solve(input, |(a, b)| sum(a.max(b) - a.min(b)))
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 37);
        assert_eq!(super::solve_2(TEST_INPUT), 168);
    }
}
