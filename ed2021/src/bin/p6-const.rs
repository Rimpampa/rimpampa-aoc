use std::str::FromStr;

const INPUT: &str = include_str!("../../assets/p6.in");

struct Childs<const DAYS: usize>;

impl<const DAYS: usize> Childs<DAYS> {
    const fn of_gen(gen: usize, start: usize) -> usize {
        const fn inner(x: u128, y: u128) -> u128 {
            let mut accum = 1;
            let mut i = 0;
            while i < x {
                accum *= y + i;
                i += 1;
            }
            i = 0;
            while i < x {
                i += 1;
                accum /= i;
            }
            accum
        }
        let prev = gen.saturating_sub(1);
        let childs = (DAYS + 6).saturating_sub(start + prev * 9) / 7;
        inner(gen as u128, childs as u128) as usize
    }

    const fn all(start: usize) -> usize {
        let mut gen = 0;
        let mut sum = 0;
        let mut res;
        while {
            res = Self::of_gen(gen, start);
            res > 0
        } {
            gen += 1;
            sum += res;
        }
        sum
    }

    const PRECOMPUTED: [usize; 7] = [
        Self::all(0),
        Self::all(1),
        Self::all(2),
        Self::all(3),
        Self::all(4),
        Self::all(5),
        Self::all(6),
    ];
}

fn solve<const DAYS: usize>(input: &str) -> usize {
    input
        .split(',')
        .map(str::trim)
        .map(usize::from_str)
        .map(Result::unwrap)
        .map(|n| Childs::<DAYS>::PRECOMPUTED[n])
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
