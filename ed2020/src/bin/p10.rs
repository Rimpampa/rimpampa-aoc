use ed2020 as base;

fn main() {
    let input = base::get_input(10).unwrap();

    let mut vec: Vec<usize> = input
        .lines()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    let max = vec.iter().max().unwrap() + 3;
    vec.sort_unstable();
    vec.push(max);

    let mut j = [0, 0, 0];
    vec.iter().fold(0, |p, &v| {
        if v != p {
            j[v - p - 1] += 1;
        }
        v
    });
    println!("{} * {} = {}", j[0], j[2], j[0] * j[2]);

    vec.insert(0, 0);

    let mut hist = [1, 1, 1];
    let mut cur: usize = 1;
    for i in (0..vec.len() - 3).rev() {
        for (j, &v) in vec[i + 2..].iter().enumerate().take(3) {
            if vec[i] + 4 > v {
                cur += hist[j + 1];
            }
        }
        hist[2] = hist[1];
        hist[1] = hist[0];
        hist[0] = cur;
    }

    println!("Count: {}", cur)
}
