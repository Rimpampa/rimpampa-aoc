use std::str::FromStr;

const INPUT: &str = include_str!("../../assets/p4.in");

macro_rules! impl_iter {
    ($ty:ty) => { impl Iterator<Item = $ty> + Clone };
}

pub fn max_opt(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    a.zip(b).map(|(a, b)| a.max(b))
}

struct Board<'a> {
    nums: [[&'a str; 5]; 5],
}

impl<'a> Board<'a> {
    pub fn new(numbers: &'a str) -> Self {
        let mut nums = [[""; 5]; 5];
        numbers
            .split_ascii_whitespace()
            .zip(nums.iter_mut().flatten())
            .for_each(|(n, m)| *m = n);
        Self { nums }
    }

    pub fn check_win<'b>(&'a self, numbers: impl_iter!(&'b str)) -> Option<usize> {
        match (self.check_win_h(numbers.clone()), self.check_win_v(numbers)) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (v @ Some(_), None) | (None, v @ Some(_)) => v,
            (None, None) => None,
        }
    }

    pub fn check_win_h<'b>(&'a self, numbers: impl_iter!(&'b str)) -> Option<usize> {
        self.nums
            .iter()
            .map(|row| row.map(|n| numbers.clone().position(|m| n == m)))
            .filter_map(|row| row.into_iter().fold(Some(0), max_opt))
            .min()
    }

    pub fn check_win_v<'b>(&'a self, numbers: impl_iter!(&'b str)) -> Option<usize> {
        (0..5)
            .map(|i| {
                self.nums
                    .map(|row| numbers.clone().position(|n| n == row[i]))
            })
            .filter_map(|row| row.into_iter().fold(Some(0), max_opt))
            .min()
    }

    pub fn sum_unmarked<'b>(&'a self, numbers: impl_iter!(&'b str)) -> usize {
        self.nums
            .into_iter()
            .flatten()
            .filter(|&n| numbers.clone().all(|m| n != m))
            .map(usize::from_str)
            .map(Result::unwrap)
            .sum()
    }
}

fn solve<const FIRST_OR_LAST: bool>(input: &str) -> usize {
    let (first, rest) = input.split_once("\n\n").unwrap();
    let numbers = first.split(",");
    let boards = rest.split("\n\n").map(Board::new);
    let win_boards = boards
        .clone()
        .filter_map(|b| b.check_win(numbers.clone()).zip(Some(b)));

    let (win_turn, winning) = match FIRST_OR_LAST {
        true => win_boards.min_by_key(|(turns, _)| *turns),
        false => win_boards.max_by_key(|(turns, _)| *turns),
    }
    .unwrap();

    let sum: usize = winning.sum_unmarked(numbers.clone().take(win_turn + 1));
    sum * numbers
        .clone()
        .nth(win_turn)
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

fn solve_1(input: &str) -> usize {
    solve::<true>(input)
}

fn solve_2(input: &str) -> usize {
    solve::<false>(input)
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../assets/p4-test.in");

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 4512);
        assert_eq!(super::solve_2(TEST_INPUT), 1924);
    }
}
