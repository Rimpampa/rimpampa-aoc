use std::str::FromStr;

macro_rules! childs {
    () => { sum };
    ($_:tt $($t:tt)*) => { |i| (1..=i).map(childs!($($t)*)).sum::<usize>() };
}

const INPUT: &str = include_str!("../../assets/p6.in");

const fn sum(val: usize) -> usize {
    val * (val + 1) / 2
}

fn childs_of_gen<const DAYS: usize>(gen: usize, start: usize) -> usize {
    fn inner(gen: usize, childs: usize) -> usize {
        match gen {
            0 => 1,
            // redundant: speeds up
            1 => childs,
            2 => sum(childs),
            3 => childs!(?)(childs),
            4 => childs!(??)(childs),
            5 => childs!(???)(childs),
            6 => childs!(????)(childs),
            7 => childs!(?????)(childs),
            8 => childs!(??????)(childs),
            _ => (1..=childs).map(|i| inner(gen - 1, i)).sum(),
        }
    }
    let prev = gen.saturating_sub(1);
    let childs = (DAYS + 6).saturating_sub(start + prev * 9) / 7;
    inner(gen, childs)
}

fn childs<const DAYS: usize>(start: usize) -> usize {
    (0..)
        .map(|gen| childs_of_gen::<DAYS>(gen, start))
        .take_while(|n| n > &0)
        .sum()
}

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
