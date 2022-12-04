const INPUT: &str = include_str!("../../assets/p3.in");

fn find_duplicate(rucksack: &str) -> Option<u8> {
    let (a, b) = rucksack.as_bytes().split_at(rucksack.len() / 2);
    a.iter().find_map(|v| b.contains(v).then_some(*v))
}

fn groups<'a>(rucksacks: impl Iterator<Item = &'a str>) -> impl Iterator<Item = [&'a [u8]; 3]> {
    let mut rucksacks = rucksacks.map(str::as_bytes);
    std::iter::from_fn(move || Some([rucksacks.next()?, rucksacks.next()?, rucksacks.next()?]))
}

fn find_common(group: [&[u8]; 3]) -> Option<u8> {
    group[0]
        .iter()
        .filter(|v| group[1].contains(v))
        .filter(|v| group[2].contains(v))
        .copied()
        .next()
}

fn into_priority(item: u8) -> usize {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => unreachable!(),
    }
    .into()
}

fn solve_1(input: &str) -> usize {
    input
        .lines()
        .map(find_duplicate)
        .map(Option::unwrap)
        .map(into_priority)
        .sum()
}

fn solve_2(input: &str) -> usize {
    groups(input.lines())
        .map(find_common)
        .map(Option::unwrap)
        .map(into_priority)
        .sum()
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
    vJrwpWtwJgWrhcsFMMfFFhFp\n\
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
    PmmdzqPrVvPwwTWBwg\n\
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
    ttgJtRGJQctTZtZT\n\
    CrZsJsPPZsGzwwsLwLmpwMDw\n";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 157);
        assert_eq!(super::solve_2(TEST_INPUT), 70);
    }
}
