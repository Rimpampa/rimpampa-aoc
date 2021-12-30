use ed2020 as base;

fn check(s: &str) -> Option<()> {
    let (rule, data) = base::split_at(s, ':')?;
    let (range, ch) = base::split_at(rule, ' ')?;
    let (min, max) = base::split_at(range, '-')?;
    let min: usize = min.parse().unwrap_or(0);
    let max: usize = max.parse().unwrap_or(0);
    let ch = ch.chars().next()?;
    let count = data[1..].chars().filter(|c| ch == *c).count();
    base::opt(count >= min && count <= max)
}

fn check_pos(s: &str) -> Option<()> {
    let (rule, data) = base::split_at(s, ':')?;
    let (range, ch) = base::split_at(rule, ' ')?;
    let (min, max) = base::split_at(range, '-')?;
    let a: usize = min.parse().unwrap_or(0);
    let b: usize = max.parse().unwrap_or(0);
    let ac = data.chars().nth(a)?;
    let bc = data.chars().nth(b)?;
    let ch = ch.chars().next()?;
    base::opt((ac == ch) ^ (bc == ch))
}

fn main() {
    let input = base::get_input(2).unwrap();

    let valid = input.lines().filter_map(check).count();
    println!("Valid: {}", valid);

    let valid = input.lines().filter_map(check_pos).count();
    println!("Valid: {}", valid);
}
