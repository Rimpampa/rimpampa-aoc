const INPUT: &str = include_str!("../../assets/p1.in");

fn split_elves(elves: &str) -> impl Iterator<Item = &str> {
    let eol = elves
        .contains("\n\n")
        .then_some("\n\n")
        .unwrap_or("\r\n\r\n");
    elves.split(eol)
}

fn split_calories(elf: &str) -> impl Iterator<Item = usize> + '_ {
    elf.lines().map(str::parse).map(Result::unwrap)
}

fn solve_1(input: &str) -> usize {
    split_elves(input)
        .map(|s| split_calories(s).sum())
        .max()
        .unwrap()
}

fn solve_2(input: &str) -> usize {
    split_elves(input)
        .map(|s| split_calories(s).sum())
        .fold([0; 3], |mut max, next: usize| {
            let Some(idx) = max.iter().position(|&v| next > v) else { return max };
            max.copy_within(idx..2, idx + 1);
            max[idx] = next;
            max
        })
        .into_iter()
        .sum::<usize>()
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
        1000\n\
        2000\n\
        3000\n\
        \n\
        4000\n\
        \n\
        5000\n\
        6000\n\
        \n\
        7000\n\
        8000\n\
        9000\n\
        \n\
        10000\n";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 24000);
        assert_eq!(super::solve_2(TEST_INPUT), 45000);
    }
}
