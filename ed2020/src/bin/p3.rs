use ed2020 as base;

fn slope(map: &str, x: usize, y: usize) -> usize {
    let mut pos = 0;
    let mut threes = 0;
    for line in map.lines().step_by(y) {
        threes += (Some('#') == line.chars().nth(pos)) as usize;
        pos = (pos + x) % line.len();
    }
    threes
}

fn main() {
    let input = base::get_input(3).unwrap();

    let threes = [
        slope(&input, 1, 1),
        slope(&input, 3, 1),
        slope(&input, 5, 1),
        slope(&input, 7, 1),
        slope(&input, 1, 2),
    ];

    println!("Threes: {}", threes[1]);

    println!(
        "Total: {:?} *= {}",
        threes,
        threes.iter().fold(1, std::ops::Mul::mul)
    );
}
