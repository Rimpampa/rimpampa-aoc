const INPUT: &str = include_str!("../../assets/p6.in");

#[derive(Clone, Copy)]
struct Window<const SIZE: usize> {
    window: [u8; SIZE],
    size: usize,
}

const fn split_left(mut slice: &[u8], at: usize) -> &[u8] {
    while slice.len() != at {
        let [s @ .., _] = slice else { unreachable!() };
        slice = s;
    }
    slice
}

const fn split_right(mut slice: &[u8], at: usize) -> &[u8] {
    let len = slice.len() - at - 1;
    while slice.len() != len {
        let [_, s @ ..] = slice else { unreachable!() };
        slice = s;
    }
    slice
}

impl<const SIZE: usize> Window<SIZE> {
    const fn window(&self) -> &[u8] {
        split_left(self.window.as_slice(), self.size)
    }

    const fn index_of(&self, value: u8) -> Option<usize> {
        let mut slice = self.window();
        loop {
            match slice {
                [s @ .., v] if *v == value => return Some(s.len()),
                [s @ .., _] => slice = s,
                [] => return None,
            }
        }
    }

    const fn shifted(self, shift: usize) -> Self {
        let mut new = Self {
            window: [0; SIZE],
            size: self.size - shift,
        };
        let mut window = split_right(self.window.as_slice(), shift);
        while !window.is_empty() {
            let [v, w @ ..] = window else { unreachable!() };
            new.window[new.size - window.len()] = *v;
            window = w;
        }
        new
    }

    const fn push(mut self, value: u8) -> Self {
        if let Some(idx) = self.index_of(value) {
            self = self.shifted(idx + 1);
            self.size -= idx + 1;
        }
        if self.size < SIZE {
            self.window[self.size] = value;
            self.size += 1;
        }
        self
    }
}

const fn solve<const SIZE: usize>(input: &str) -> usize {
    let mut window = Window {
        window: [0; SIZE],
        size: 0,
    };
    let mut slice = input.as_bytes();
    let len = slice.len();
    loop {
        match window.push(slice[0]) {
            Window { size, .. } if size == SIZE => return len - slice.len() + 1,
            w => window = w,
        }
        let [_, s @ ..] = slice else { panic!() };
        slice = s;
    }
}

pub const fn solve_1(input: &str) -> usize {
    solve::<4>(input)
}

pub const fn solve_2(input: &str) -> usize {
    solve::<14>(input)
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUTS: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn test() {
        const SOLVE1: [usize; 5] = [
            super::solve_1(TEST_INPUTS[0]),
            super::solve_1(TEST_INPUTS[1]),
            super::solve_1(TEST_INPUTS[2]),
            super::solve_1(TEST_INPUTS[3]),
            super::solve_1(TEST_INPUTS[4]),
        ];
        const SOLVE2: [usize; 5] = [
            super::solve_1(TEST_INPUTS[0]),
            super::solve_1(TEST_INPUTS[1]),
            super::solve_1(TEST_INPUTS[2]),
            super::solve_1(TEST_INPUTS[3]),
            super::solve_1(TEST_INPUTS[4]),
        ];
        assert_eq!(SOLVE1, [7, 5, 6, 10, 11]);
        assert_eq!(SOLVE2, [19, 23, 23, 29, 26]);
    }
}
