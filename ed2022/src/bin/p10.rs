use std::{iter::from_fn, str::FromStr};

const INPUT: &str = include_str!("../../assets/p10.in");

enum Instruction {
    Noop,
    Addx(isize),
}

impl FromStr for Instruction {
    type Err = <isize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(n) = s.strip_prefix("addx ") {
            Ok(Self::Addx(n.parse()?))
        } else if "noop" == s {
            Ok(Self::Noop)
        } else {
            isize::from_str("").map(|_| unreachable!())
        }
    }
}

fn instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    use Instruction::*;
    let mut iter = input.lines().map(str::parse).map(Result::unwrap);
    let mut temp = None;
    from_fn(move || {
        temp.take().or_else(|| {
            if let i @ Addx(_) = iter.next()? {
                temp = Some(i)
            }
            Some(Noop)
        })
    })
}

fn solve_1(input: &str) -> isize {
    let mut x = 1;
    let mut strength = 0;
    for (cycle, i) in instructions(input).enumerate() {
        let cycle: isize = cycle.try_into().unwrap();
        if cycle > 18 && (cycle - 19) % 40 == 0 {
            let add = x * (cycle + 1);
            strength += add;
        }
        if let Instruction::Addx(add) = i {
            x += add;
        }
    }
    strength
}

fn solve_2(input: &str) -> String {
    let mut x = 1;
    let mut display = [[b'.'; 40]; 6];
    for (cycle, i) in instructions(input).enumerate() {
        let pixel = (cycle % 40).try_into().unwrap();
        if x + 1 >= pixel && x - 1 <= pixel {
            display[cycle / 40][cycle % 40] = b'#';
        }
        if let Instruction::Addx(add) = i {
            x += add;
        }
    }
    let chars = display
        .into_iter()
        .flat_map(|row| row.into_iter().chain([b'\n']))
        .collect();
    String::from_utf8(chars).unwrap()
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: \n{}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "addx 15\n\
        addx -11\n\
        addx 6\n\
        addx -3\n\
        addx 5\n\
        addx -1\n\
        addx -8\n\
        addx 13\n\
        addx 4\n\
        noop\n\
        addx -1\n\
        addx 5\n\
        addx -1\n\
        addx 5\n\
        addx -1\n\
        addx 5\n\
        addx -1\n\
        addx 5\n\
        addx -1\n\
        addx -35\n\
        addx 1\n\
        addx 24\n\
        addx -19\n\
        addx 1\n\
        addx 16\n\
        addx -11\n\
        noop\n\
        noop\n\
        addx 21\n\
        addx -15\n\
        noop\n\
        noop\n\
        addx -3\n\
        addx 9\n\
        addx 1\n\
        addx -3\n\
        addx 8\n\
        addx 1\n\
        addx 5\n\
        noop\n\
        noop\n\
        noop\n\
        noop\n\
        noop\n\
        addx -36\n\
        noop\n\
        addx 1\n\
        addx 7\n\
        noop\n\
        noop\n\
        noop\n\
        addx 2\n\
        addx 6\n\
        noop\n\
        noop\n\
        noop\n\
        noop\n\
        noop\n\
        addx 1\n\
        noop\n\
        noop\n\
        addx 7\n\
        addx 1\n\
        noop\n\
        addx -13\n\
        addx 13\n\
        addx 7\n\
        noop\n\
        addx 1\n\
        addx -33\n\
        noop\n\
        noop\n\
        noop\n\
        addx 2\n\
        noop\n\
        noop\n\
        noop\n\
        addx 8\n\
        noop\n\
        addx -1\n\
        addx 2\n\
        addx 1\n\
        noop\n\
        addx 17\n\
        addx -9\n\
        addx 1\n\
        addx 1\n\
        addx -3\n\
        addx 11\n\
        noop\n\
        noop\n\
        addx 1\n\
        noop\n\
        addx 1\n\
        noop\n\
        noop\n\
        addx -13\n\
        addx -19\n\
        addx 1\n\
        addx 3\n\
        addx 26\n\
        addx -30\n\
        addx 12\n\
        addx -1\n\
        addx 3\n\
        addx 1\n\
        noop\n\
        noop\n\
        noop\n\
        addx -9\n\
        addx 18\n\
        addx 1\n\
        addx 2\n\
        noop\n\
        noop\n\
        addx 9\n\
        noop\n\
        noop\n\
        noop\n\
        addx -1\n\
        addx 2\n\
        addx -37\n\
        addx 1\n\
        addx 3\n\
        noop\n\
        addx 15\n\
        addx -21\n\
        addx 22\n\
        addx -6\n\
        addx 1\n\
        noop\n\
        addx 2\n\
        addx 1\n\
        noop\n\
        addx -10\n\
        noop\n\
        noop\n\
        addx 20\n\
        addx 1\n\
        addx 2\n\
        addx 2\n\
        addx -6\n\
        addx -11\n\
        noop\n\
        noop\n\
        noop\n";

    const OUTPUT: &str = "\
        ##..##..##..##..##..##..##..##..##..##..\n\
        ###...###...###...###...###...###...###.\n\
        ####....####....####....####....####....\n\
        #####.....#####.....#####.....#####.....\n\
        ######......######......######......####\n\
        #######.......#######.......#######.....\n";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 13140);
        assert_eq!(super::solve_2(TEST_INPUT), OUTPUT);
    }
}
