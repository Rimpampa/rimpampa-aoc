const INPUT: &str = include_str!("../../assets/p1.in");

struct SlidingWindow<const SIZE: usize> {
    window: [Option<usize>; SIZE],
}

impl<const SIZE: usize> SlidingWindow<SIZE> {
    fn new() -> Self {
        Self {
            window: [None; SIZE],
        }
    }

    fn get(&self) -> Option<usize> {
        self.window[0]?;
        Some(self.window.into_iter().map(Option::unwrap).sum())
    }

    fn push(&mut self, new: usize) {
        self.window.copy_within(1.., 0);
        self.window[SIZE - 1] = Some(new);
    }
}

fn solve_1(input: &str) -> usize {
    let mut count = 0;
    let mut last_line: Option<usize> = None;
    for line in input.lines() {
        let n = line.parse().unwrap();
        count += last_line.map(|prev| prev < n).unwrap_or(false) as usize;
        last_line = Some(n);
    }
    count
}

fn solve_2(input: &str) -> usize {
    let mut window = SlidingWindow::<3>::new();
    let mut count = 0;
    let mut last: Option<usize> = None;
    for line in input.lines() {
        window.push(line.parse().unwrap());
        if let Some(curr) = window.get() {
            if let Some(last) = last {
                count += (curr > last) as usize;
            }
            last = Some(curr);
        }
    }
    count
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 7);
        assert_eq!(super::solve_2(TEST_INPUT), 5);
    }
}
