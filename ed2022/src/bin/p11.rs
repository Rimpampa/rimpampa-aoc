use std::{
    convert::identity,
    ops::{Add, Mul, Not},
    str::FromStr,
};

const INPUT: &str = include_str!("../../assets/p11.in");

#[derive(Clone, Copy)]
enum Param<T> {
    Old,
    Num(T),
}

impl<T> Param<T> {
    fn get(self, old: T) -> T {
        match self {
            Self::Old => old,
            Self::Num(n) => n,
        }
    }
}

impl FromStr for Param<usize> {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Param::Old),
            n => n.parse().map(Param::Num),
        }
    }
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn exec<T: Add + Mul<Output = <T as Add>::Output>>(self, lhs: T, rhs: T) -> <T as Add>::Output {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Mul => lhs * rhs,
        }
    }
}

#[derive(Clone, Copy)]
struct Expression<T> {
    params: [Param<T>; 2],
    op: Operation,
}

impl<T: Add + Mul<Output = <T as Add>::Output> + Copy> Expression<T> {
    fn exec(self, old: T) -> <T as Add>::Output {
        let [lhs, rhs] = self.params.map(|p| p.get(old));
        self.op.exec(lhs, rhs)
    }
}

impl FromStr for Expression<usize> {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || usize::from_str("").map(|_| unreachable!());

        let Some(s) = s.strip_prefix("new = ") else { err()? };
        let mut iter = s.split(" ");
        let [Some(lhs), Some(op), Some(rhs), None] = [(); 4].map(|()| iter.next()) else { err()? };
        let params = [lhs.parse()?, rhs.parse()?];
        let op = match op {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            _ => err()?,
        };
        Ok(Self { params, op })
    }
}

struct Monke {
    num: usize,
    items: Vec<usize>,
    op: Expression<usize>,
    test: usize,
    throw: (usize, usize),
    inspections: usize,
}

impl Monke {
    fn turn<const RELIEF: usize>(&mut self) -> Option<(usize, usize)> {
        let mut item = self.items.is_empty().not().then(|| self.items.remove(0))?;
        item = self.op.exec(item) / RELIEF;
        let throw = if item % self.test == 0 {
            self.throw.0
        } else {
            self.throw.1
        };
        self.inspections += 1;
        Some((throw, item))
    }
}

impl FromStr for Monke {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || usize::from_str("").map(|_| unreachable!());

        let mut lines = s.lines();
        let [Some(monkey), Some(items), Some(op), Some(test), Some(if_true), Some(if_false), None] =
            [(); 7].map(|()| lines.next()) else { err()? };

        let Some(monkey) = monkey.strip_prefix("Monkey ").and_then(|s| s.strip_suffix(":")) else { err()? };
        let Some(items) = items.strip_prefix("  Starting items: ") else { err()? };
        let Some(op) = op.strip_prefix("  Operation: ") else { err()? };
        let Some(test) = test.strip_prefix("  Test: divisible by ") else { err()? };
        let Some(if_true) = if_true.strip_prefix("    If true: throw to monkey ") else { err()? };
        let Some(if_false) = if_false.strip_prefix("    If false: throw to monkey ") else { err()? };

        let items = items
            .split(", ")
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self {
            num: monkey.parse()?,
            items,
            op: op.parse()?,
            test: test.parse()?,
            throw: (if_true.parse()?, if_false.parse()?),
            inspections: 0,
        })
    }
}

fn monkeys(input: &str) -> impl Iterator<Item = Monke> + '_ {
    let eol = input
        .contains("\n\n")
        .then_some("\n\n")
        .unwrap_or("\r\n\r\n");
    input.split(eol).map(str::parse).map(Result::unwrap)
}

fn monkey_business<const ROUNDS: usize, const RELIEF: usize>(input: &str) -> usize {
    let mut monkeys: Vec<_> = monkeys(input).collect();
    monkeys.sort_unstable_by_key(|m| m.num);
    let div: usize = monkeys.iter().map(|m| m.test).product();
    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            while let Some((i, item)) = monkeys[i].turn::<RELIEF>() {
                let item = (RELIEF == 1).then_some(item % div).unwrap_or(item);
                monkeys[i].items.push(item)
            }
        }
    }
    monkeys.sort_unstable_by_key(|m| m.inspections);
    let [.., a, b] = &monkeys[..] else { panic!() };
    a.inspections * b.inspections
}

fn main() {
    println!("Answer 1: {}", monkey_business::<20, 3>(INPUT));
    println!("Answer 2: {}", monkey_business::<10000, 1>(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = concat!(
        "Monkey 0:\n",
        "  Starting items: 79, 98\n",
        "  Operation: new = old * 19\n",
        "  Test: divisible by 23\n",
        "    If true: throw to monkey 2\n",
        "    If false: throw to monkey 3\n",
        "\n",
        "Monkey 1:\n",
        "  Starting items: 54, 65, 75, 74\n",
        "  Operation: new = old + 6\n",
        "  Test: divisible by 19\n",
        "    If true: throw to monkey 2\n",
        "    If false: throw to monkey 0\n",
        "\n",
        "Monkey 2:\n",
        "  Starting items: 79, 60, 97\n",
        "  Operation: new = old * old\n",
        "  Test: divisible by 13\n",
        "    If true: throw to monkey 1\n",
        "    If false: throw to monkey 3\n",
        "\n",
        "Monkey 3:\n",
        "  Starting items: 74\n",
        "  Operation: new = old + 3\n",
        "  Test: divisible by 17\n",
        "    If true: throw to monkey 0\n",
        "    If false: throw to monkey 1\n",
    );

    #[test]
    fn test() {
        assert_eq!(super::monkey_business::<20, 3>(TEST_INPUT), 10605);
        assert_eq!(super::monkey_business::<10000, 1>(TEST_INPUT), 2713310158);
    }
}
