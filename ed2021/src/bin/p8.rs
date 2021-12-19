use std::{fmt::Debug, iter::Copied, ops::Deref, slice::Iter, str::FromStr};

macro_rules! impl_iter {
    ($ty:ty) => { impl Iterator<Item = $ty> + Clone };
}

const INPUT: &str = include_str!("../../assets/p8.in");

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(usize)]
enum Signal {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
}

impl Signal {
    const SIGNALS: usize = 7;

    const ALL: [Signal; Signal::SIGNALS] = {
        use Signal::*;
        [A, B, C, D, E, F, G]
    };

    fn except(signals: impl_iter!(Signal)) -> impl_iter!(Signal) {
        Self::ALL
            .into_iter()
            .filter(move |&s| signals.clone().all(|v| v != s))
    }
}

impl TryFrom<usize> for Signal {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::ALL.get(value).copied().ok_or(())
    }
}

#[derive(Clone, Copy)]
struct Signals {
    signals: [Signal; Signal::SIGNALS],
    len: usize,
}

impl Signals {
    fn mapped(self, map: [Signal; Signal::SIGNALS]) -> Self {
        Self {
            signals: self.signals.map(|v| map[v as usize]),
            len: self.len,
        }
    }

    fn convert(&self) -> Option<usize> {
        fn try_get<const N: usize>(signals: &[Signal]) -> Option<usize>
        where
            SignalsOf<N>: HasSignals,
        {
            let n_sigs = SignalsOf::<N>::SIGNALS;
            let a = n_sigs.iter().all(|s| signals.contains(s));
            let b = signals.iter().all(|s| n_sigs.contains(s));
            (a && b).then(|| N)
        }

        try_get::<0>(self)
            .or_else(|| try_get::<1>(self))
            .or_else(|| try_get::<2>(self))
            .or_else(|| try_get::<3>(self))
            .or_else(|| try_get::<4>(self))
            .or_else(|| try_get::<5>(self))
            .or_else(|| try_get::<6>(self))
            .or_else(|| try_get::<7>(self))
            .or_else(|| try_get::<8>(self))
            .or_else(|| try_get::<9>(self))
    }

    fn inverted(&self) -> impl_iter!(Signal) {
        Signal::except(self.into_iter())
    }
}

impl FromStr for Signals {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut signals = [Signal::A; Signal::SIGNALS];
        let len = s.len();
        for (s, v) in s.bytes().zip(signals.iter_mut()) {
            *v = (s.checked_sub(b'a').ok_or(())? as usize).try_into()?;
        }
        Ok(Self { signals, len })
    }
}

impl Deref for Signals {
    type Target = [Signal];

    fn deref(&self) -> &Self::Target {
        &self.signals[..self.len]
    }
}

impl IntoIterator for Signals {
    type Item = Signal;
    type IntoIter = std::iter::Take<std::array::IntoIter<Signal, 7>>;
    fn into_iter(self) -> Self::IntoIter {
        self.signals.into_iter().take(self.len)
    }
}

macro_rules! signals_of {
    ($n:tt = [$($v:ident),* $(,)?]) => {
        impl HasSignals for SignalsOf<$n> {
            const SIGNALS: &'static [Signal] = &[$(Signal::$v),*];
        }
    };

    ($n:tt) => { SignalsOf::<$n>::signals() };
    ($n:tt + $m:tt $(+ $o:tt)*) => {
        signals_of!($n).chain(signals_of!($m $(+ $o)*))
    }
}

struct SignalsOf<const N: usize>;

trait HasSignals {
    const SIGNALS: &'static [Signal];

    fn signals() -> Copied<Iter<'static, Signal>> {
        Self::SIGNALS.iter().copied()
    }
}

signals_of!(1 = [C, F]);
signals_of!(7 = [A, C, F]);
signals_of!(4 = [B, C, D, F]);
signals_of!(2 = [A, C, D, E, G]);
signals_of!(3 = [A, C, D, F, G]);
signals_of!(5 = [A, B, D, F, G]);
signals_of!(0 = [A, B, C, E, F, G]);
signals_of!(6 = [A, B, D, E, F, G]);
signals_of!(9 = [A, B, C, D, F, G]);
signals_of!(8 = [A, B, C, D, E, F, G]);

#[derive(Clone, Copy, PartialEq)]
struct Choices([bool; Signal::SIGNALS]);

impl Choices {
    fn unknown() -> Self {
        Self([true; Signal::SIGNALS])
    }

    fn remove(&mut self, signal: Signal) {
        self.0[signal as usize] = false;
    }

    fn remove_all(&mut self, signals: impl_iter!(Signal)) {
        signals.for_each(|s| self.remove(s))
    }

    fn keep(&mut self, signals: impl_iter!(Signal)) {
        self.remove_all(Signal::except(signals));
    }

    fn possibles(&self) -> impl_iter!(Signal) {
        let poss = self.0;
        Signal::ALL.into_iter().filter(move |&s| poss[s as usize])
    }
}

impl Debug for Choices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let iter = Signal::ALL.into_iter().filter(|&s| self.0[s as usize]);
        write!(f, "{:?}", iter.collect::<Vec<_>>())
    }
}

struct SignalsMapper([Choices; Signal::SIGNALS]);

impl SignalsMapper {
    fn unknown() -> Self {
        Self([Choices::unknown(); Signal::SIGNALS])
    }

    fn keep(&mut self, on: impl_iter!(Signal), with: impl_iter!(Signal)) {
        on.for_each(|s| self.0[s as usize].keep(with.clone()))
    }

    fn remove(&mut self, on: impl_iter!(Signal), with: impl_iter!(Signal)) {
        on.for_each(|s| self.0[s as usize].remove_all(with.clone()))
    }

    fn input(&mut self, signals: Signals) {
        match signals.len() {
            2 => {
                self.keep(signals.into_iter(), signals_of!(1));
                self.remove(signals.inverted(), signals_of!(1))
            }
            3 => {
                self.keep(signals.into_iter(), signals_of!(7));
                self.remove(signals.inverted(), signals_of!(7))
            }
            4 => {
                self.keep(signals.into_iter(), signals_of!(4));
                self.remove(signals.inverted(), signals_of!(4))
            }
            5 => self.keep(signals.into_iter(), signals_of!(2 + 3 + 5)),
            6 => self.keep(signals.into_iter(), signals_of!(0 + 6 + 9)),
            7 => {
                self.keep(signals.into_iter(), signals_of!(8));
                self.remove(signals.inverted(), signals_of!(8))
            }
            _ => unreachable!(),
        }
    }

    fn complete(&self, numbers: [Signals; 10]) -> Option<[Signal; Signal::SIGNALS]> {
        let mut map = [Signal::A; Signal::SIGNALS];

        let permutations = [
            [false, false, false],
            [false, false, true],
            [false, true, false],
            [false, true, true],
            [true, false, false],
            [true, false, true],
            [true, true, false],
            [true, true, true],
        ];
        for perm in permutations {
            let mut at_perm = 0;
            for (at, choices) in self.0.iter().enumerate() {
                let mut possibs = choices.possibles();
                let first = possibs.next().unwrap();
                let second = possibs.next();

                match second {
                    Some(v) if map[..at].contains(&first) => map[at] = v,
                    Some(v) if map[..at].contains(&v) => map[at] = first,
                    Some(v) => {
                        match perm[at_perm] {
                            true => map[at] = v,
                            false => map[at] = first,
                        }
                        at_perm += 1;
                    }
                    None => map[at] = first,
                }
            }
            if numbers.iter().all(|n| n.mapped(map).convert().is_some()) {
                return Some(map);
            }
        }
        None
    }
}

impl Debug for SignalsMapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignalsMap")
            .field("A", &self.0[Signal::A as usize])
            .field("B", &self.0[Signal::B as usize])
            .field("C", &self.0[Signal::C as usize])
            .field("D", &self.0[Signal::D as usize])
            .field("E", &self.0[Signal::E as usize])
            .field("F", &self.0[Signal::F as usize])
            .field("G", &self.0[Signal::G as usize])
            .finish()
    }
}

fn solve_1(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.split_once(" | "))
        .map(Option::unwrap)
        .map(|(_, s)| s)
        .map(str::split_ascii_whitespace)
        .flatten()
        .map(str::len)
        .filter(|n| [2, 3, 4, 7].contains(n)) // 1 7 4 8
        .count()
}

fn solve_2(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.split_once(" | "))
        .map(Option::unwrap)
        .map(|(ins, outs)| {
            let mut map = SignalsMapper::unknown();

            let mut signals = ins
                .split_ascii_whitespace()
                .map(Signals::from_str)
                .map(Result::unwrap);
            let signals = [(); 10].map(|_| signals.next().unwrap());

            signals.into_iter().for_each(|s| map.input(s));

            let map = map.complete(signals).unwrap();

            outs.split_ascii_whitespace()
                .map(Signals::from_str)
                .map(Result::unwrap)
                .map(|s| s.mapped(map))
                .map(|s| s.convert().unwrap())
                .fold(0, |acc, n| acc * 10 + n)
        })
        .sum()
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 26);
        assert_eq!(super::solve_2(TEST_INPUT), 61229);
    }
}
