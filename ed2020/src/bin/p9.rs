use ed2020 as base;

fn find(numbers: &[usize]) -> Option<usize> {
    const PREAMBLE: usize = 25;
    for line in numbers.windows(PREAMBLE + 1) {
        let prev = &line[..PREAMBLE];
        let n = line[PREAMBLE];
        if !prev
            .iter()
            .enumerate()
            .any(|(i, a)| prev[i + 1..].iter().any(|b| a + b == n))
        {
            return Some(n);
        }
    }
    None
}

use std::cmp::Ordering;
use std::ops::Range;
fn exploit(numbers: &[usize], n: usize) -> Option<Range<usize>> {
    let mut range = 0..1;

    while range.end <= numbers.len() {
        let sum = numbers[range.clone()].iter().sum::<usize>();
        match sum.cmp(&n) {
            Ordering::Equal => return Some(range),
            Ordering::Greater => range.start += 1,
            Ordering::Less => range.end += 1,
        }
    }
    None
}

fn main() {
    let input = base::get_input(9).unwrap();

    let numbers: Vec<usize> = input
        .lines()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    let found = find(&numbers).unwrap();
    println!("Found: {}", found);

    let range = exploit(&numbers, found).unwrap();
    let max = numbers[range.clone()].iter().max().unwrap();
    let min = numbers[range].iter().min().unwrap();
    println!("{} + {} = {}", min, max, min + max);
}
