const INPUT: &str = include_str!("../../assets/p4.in");

type Range = std::ops::RangeInclusive<u8>;

fn into_range(section: &str) -> Range {
    let (s, e) = section.split_once('-').unwrap();
    s.parse().unwrap()..=e.parse().unwrap()
}

fn ranges(group: &str) -> (Range, Range) {
    let (a, b) = group.split_once(',').unwrap();
    (into_range(a), into_range(b))
}

fn contains(a: &Range, b: &Range) -> bool {
    a.start() <= b.start() && a.end() >= b.end()
}

fn overlap(a: &Range, b: &Range) -> bool {
    a.start() <= b.end() && a.end() >= b.start()
}

fn solve_1(input: &str) -> usize {
    input
        .lines()
        .map(ranges)
        .filter(|(a, b)| contains(a, b) || contains(b, a))
        .count()
}

fn solve_2(input: &str) -> usize {
    input
        .lines()
        .map(ranges)
        .filter(|(a, b)| overlap(a, b))
        .count()
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
        2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8\n";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 2);
        assert_eq!(super::solve_2(TEST_INPUT), 4);
    }
}
