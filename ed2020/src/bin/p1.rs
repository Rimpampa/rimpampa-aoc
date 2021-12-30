use ed2020 as base;

fn main() {
    let input = base::get_input(1).unwrap();

    // prima parte
    let numbers: Vec<usize> = input
        .lines()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    for (i, number) in numbers.iter().enumerate() {
        for other in &numbers[i + 1..] {
            if number + other == 2020 {
                println!("{} * {} = {}", number, other, number * other);
                break;
            }
        }
    }

    // seconda parte
    for (i, number) in numbers.iter().enumerate() {
        for (j, second) in numbers[i + 1..].iter().enumerate() {
            for third in numbers[i + j + 2..].iter() {
                if number + second + third == 2020 {
                    println!(
                        "{} * {} * {} = {}",
                        number,
                        second,
                        third,
                        number * second * third
                    );
                    break;
                }
            }
        }
    }
}
