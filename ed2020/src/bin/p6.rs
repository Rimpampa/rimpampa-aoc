use ed2020 as base;

const fn idx(c: char) -> usize {
    c as usize - 'a' as usize
}

fn count_any(input: &str) -> usize {
    let mut answered = 0u32;
    let mut sum = 0;

    for line in input.lines() {
        if line.is_empty() {
            sum += answered.count_ones() as usize;
            answered = 0;
        } else {
            line.chars().for_each(|i| answered |= 1 << idx(i));
        }
    }
    sum += answered.count_ones() as usize;
    sum
}

fn count_all(input: &str) -> usize {
    let mut answered = 0u32;
    let mut sum = 0;
    let mut first = true;

    for line in input.lines() {
        if line.is_empty() {
            sum += answered.count_ones() as usize;
            answered = 0;
            first = true;
        } else if first {
            line.chars().for_each(|i| answered |= 1 << idx(i));
            first = false;
        } else {
            for i in (0..26).filter(|&i| line.chars().all(|c| idx(c) != i)) {
                answered &= !(1 << i);
            }
        }
    }
    sum += answered.count_ones() as usize;
    sum
}

fn main() {
    let input = base::get_input(6).unwrap();

    println!("Any: {}", count_any(&input));
    println!("All: {}", count_all(&input));
}
