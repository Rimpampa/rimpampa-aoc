const INPUT: &str = include_str!("../../assets/p2.in");

#[derive(Default)]
struct Submarine {
    depth: usize,
    pos: usize,
    aim: usize,
}

impl Submarine {
    fn new() -> Self {
        Self::default()
    }

    fn move_down(&mut self, val: usize) {
        self.depth -= val
    }

    fn move_up(&mut self, val: usize) {
        self.depth += val
    }

    fn aim_up(&mut self, val: usize) {
        self.aim -= val
    }

    fn aim_down(&mut self, val: usize) {
        self.aim += val
    }

    fn move_forward(&mut self, val: usize) {
        self.pos += val;
        self.depth += self.aim * val;
    }

    fn position(&self) -> usize {
        self.depth * self.pos
    }

    fn exec(
        &mut self,
        cmd: &str,
        val: usize,
        forward: impl Fn(&mut Self, usize),
        down: impl Fn(&mut Self, usize),
        up: impl Fn(&mut Self, usize),
    ) {
        match cmd {
            "forward" => forward(self, val),
            "down" => up(self, val),
            "up" => down(self, val),
            s => unreachable!(s),
        }
    }
}

use Submarine as Sub;

fn solve_1(input: &str) -> usize {
    let mut sub = Sub::new();
    for line in input.lines() {
        let (cmd, n) = line.split_once(' ').unwrap();
        let n = n.parse().unwrap();
        sub.exec(cmd, n, Sub::move_forward, Sub::move_up, Sub::move_down);
    }
    sub.position()
}

fn solve_2(input: &str) -> usize {
    let mut sub = Sub::new();
    for line in input.lines() {
        let (cmd, n) = line.split_once(' ').unwrap();
        let n: usize = n.parse().unwrap();
        sub.exec(cmd, n, Sub::move_forward, Sub::aim_up, Sub::aim_down);
    }
    sub.position()
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 150);
        assert_eq!(super::solve_2(TEST_INPUT), 900);
    }
}
