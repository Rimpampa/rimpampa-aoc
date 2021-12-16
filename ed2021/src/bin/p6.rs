use std::str::FromStr;

const INPUT: &str = include_str!("../../assets/p6.in");

fn childs_of_gen<const DAYS: usize>(gen: usize, start: usize) -> usize {
    /// Number of childs made by the genaration x if the first child of the
    /// previous generation made y children
    fn inner(x: u128, y: u128) -> u128 {
        (0..x).map(|v| v + y).product::<u128>() / (1..=x).product::<u128>()
    }
    let prev = gen.saturating_sub(1);
    // number of children made by the first child of the previous generation
    let childs = (DAYS + 6).saturating_sub(start + prev * 9) / 7;
    usize::try_from(inner(gen as u128, childs as u128)).unwrap()
}

/// Total number of children made in DAYS days by starting from the state start
fn childs<const DAYS: usize>(start: usize) -> usize {
    (0..)
        .map(|gen| childs_of_gen::<DAYS>(gen, start))
        .take_while(|n| n > &0)
        .sum()
}

/// Precompute the number of childern made in DAYS days for every starting state
fn precompute<const DAYS: usize>() -> [usize; 7] {
    [0, 1, 2, 3, 4, 5, 6].map(childs::<DAYS>)
}

fn solve<const DAYS: usize>(input: &str) -> usize {
    let precomputed = precompute::<DAYS>();
    input
        .split(',')
        .map(str::trim)
        .map(usize::from_str)
        .map(Result::unwrap)
        .map(|n| precomputed[n])
        .sum::<usize>()
}

fn solve_1(input: &str) -> usize {
    solve::<80>(input)
}

fn solve_2(input: &str) -> usize {
    solve::<256>(input)
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test() {
        assert_eq!(super::solve::<18>(TEST_INPUT), 26);
        assert_eq!(super::solve_1(TEST_INPUT), 5934);
        assert_eq!(super::solve_2(TEST_INPUT), 26984457539);
    }
}
