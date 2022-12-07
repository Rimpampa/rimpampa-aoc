const INPUT: &str = include_str!("../../assets/p6.in");

#[derive(Clone, Copy)]
struct Window<const SIZE: usize> {
    window: [u8; SIZE],
    size: usize,
}

impl<const SIZE: usize> Default for Window<SIZE> {
    fn default() -> Self {
        Self {
            window: [0; SIZE],
            size: 0,
        }
    }
}

impl<const SIZE: usize> Window<SIZE> {
    fn index_of(&self, value: u8) -> Option<usize> {
        self.window[..self.size]
            .into_iter()
            .position(|v| *v == value)
    }

    fn push(&mut self, value: u8) -> usize {
        if let Some(idx) = self.index_of(value) {
            self.window.copy_within(idx + 1..self.size, 0);
            self.size -= idx + 1;
        }
        if self.size < SIZE {
            self.window[self.size] = value;
            self.size += 1;
        }
        self.size
    }
}

fn solve<const SIZE: usize>(input: &str) -> usize {
    let mut window = Window::<SIZE>::default();
    input
        .as_bytes()
        .iter()
        .position(|&v| window.push(v) == SIZE)
        .unwrap()
        + 1
}

fn main() {
    println!("Answer 1: {}", solve::<4>(INPUT));
    println!("Answer 2: {}", solve::<14>(INPUT));
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
        assert_eq!(TEST_INPUTS.map(super::solve::<4>), [7, 5, 6, 10, 11]);
        assert_eq!(TEST_INPUTS.map(super::solve::<14>), [19, 23, 23, 29, 26]);
    }
}
