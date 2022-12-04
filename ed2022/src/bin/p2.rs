const INPUT: &str = include_str!("../../assets/p2.in");

#[derive(PartialEq, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissor,
}

impl Rps {
    const LIST: [Self; 3] = [Self::Rock, Self::Paper, Self::Scissor];

    fn round_score(line: &str) -> usize {
        let [them, b' ', mine] = line.as_bytes() else { panic!() };
        let them = Self::LIST[index_of(b"ABC", them).unwrap()];
        let mine = Self::LIST[index_of(b"XYZ", mine).unwrap()];
        let shape = mine.index();
        // 0 = draw, 1 = lose, 2 = win
        let result = index_of(rotated_right(Self::LIST, 3 - shape), them).unwrap();
        shape + 1 + [3, 0, 6][result]
    }

    fn outcome_score(line: &str) -> usize {
        let [a, b' ', b] = line.as_bytes() else { panic!() };
        // 0 = draw, 1 = win, 2 = lose
        let result = index_of(b"YZX", b).unwrap();
        let them = index_of(b"ABC", a).unwrap();
        // 0 = draw, 1 = win, 2 = lose
        let list = rotated_right(Self::LIST, 3 - them);
        [3, 6, 0][result] + list[result].index() + 1
    }

    fn index(self) -> usize {
        index_of(Self::LIST, self).unwrap()
    }
}

fn rotated_right<T, const S: usize>(mut arr: [T; S], amount: usize) -> [T; S] {
    arr.rotate_right(amount);
    arr
}

fn index_of<I: IntoIterator>(list: I, val: I::Item) -> Option<usize>
where
    I::Item: PartialEq,
{
    list.into_iter().position(|item| item == val)
}

fn solve_1(input: &str) -> usize {
    input.lines().map(Rps::round_score).sum()
}

fn solve_2(input: &str) -> usize {
    input.lines().map(Rps::outcome_score).sum()
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
        A Y\n\
        B X\n\
        C Z\n";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 15);
        assert_eq!(super::solve_2(TEST_INPUT), 12);
    }
}
