use std::iter::{from_fn, repeat};

const INPUT: &str = include_str!("../../assets/p5.in");

type Stack = Vec<u8>;

fn split(elves: &str) -> (&str, &str) {
    let eol = elves
        .contains("\n\n")
        .then_some("\n\n")
        .unwrap_or("\r\n\r\n");
    elves.split_once(eol).unwrap()
}

fn crates(line: &str) -> impl Iterator<Item = Option<u8>> + '_ {
    let mut line = Some(line.as_bytes());
    from_fn(move || {
        let inner = line?;
        line = inner.get(4..);
        let [b'[', item, b']', ..] = inner else { return Some(None) };
        Some(Some(*item))
    })
}

fn stacks(stacks: &str) -> Vec<Stack> {
    stacks
        .lines()
        .flat_map(|line| crates(line).enumerate())
        .flat_map(|(i, opt)| opt.map(|v| (i, v)))
        .fold(vec![], |mut vec, (i, v)| {
            if let Some(extend) = i.checked_sub(vec.len()) {
                vec.extend(repeat(vec![]).take(extend + 1));
            }
            vec[i].insert(0, v);
            vec
        })
}

type Instruction = (usize, usize, usize);

fn instruction(rule: &str) -> Option<Instruction> {
    let (mov, rule) = rule.strip_prefix("move ")?.split_once(' ')?;
    let (from, rule) = rule.strip_prefix("from ")?.split_once(' ')?;
    let to = rule.strip_prefix("to ")?;
    let parse = |s: &str| s.parse::<usize>().ok();
    Some((parse(mov)?, parse(from)? - 1, parse(to)? - 1))
}

fn instructions(rules: &str) -> impl Iterator<Item = Instruction> + '_ {
    rules.lines().map(instruction).map(Option::unwrap)
}

fn execute<const RETAIN_ORDER: bool>(input: &str) -> impl Iterator<Item = u8> {
    let (stacks, instructions) = split(input);
    let mut stacks = self::stacks(stacks);

    for (mov, from, to) in self::instructions(instructions) {
        let [a, .., b] = &mut stacks[from.min(to)..=from.max(to)] else { continue };
        let (from, to) = if from < to { (a, b) } else { (b, a) };

        let retain = from.len() - mov;
        if RETAIN_ORDER {
            to.extend_from_slice(&from[retain..])
        } else {
            to.extend(from[retain..].iter().rev())
        }
        from.truncate(retain);
    }
    stacks.into_iter().map(|v| *v.last().unwrap())
}

fn solve_1(input: &str) -> String {
    String::from_utf8(execute::<false>(input).collect()).unwrap()
}

fn solve_2(input: &str) -> String {
    String::from_utf8(execute::<true>(input).collect()).unwrap()
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "    [D]    \n\
        [N] [C]    \n\
        [Z] [M] [P]\n\
         1   2   3 \n\
        \n\
        move 1 from 2 to 1\n\
        move 3 from 1 to 3\n\
        move 2 from 2 to 1\n\
        move 1 from 1 to 2\n";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), "CMZ");
        assert_eq!(super::solve_2(TEST_INPUT), "MCD");
    }
}
