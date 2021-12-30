use ed2020 as base;

fn main() {
    let input = base::get_input(13).unwrap();
    let idx = input.find('\n').unwrap();
    let earl: usize = input[..idx].parse().unwrap();
    let mut min = (usize::MAX, 0);
    let mut vec = Vec::new();
    for (i, s) in input[idx + 1..].split(',').enumerate() {
        if s != "x" {
            let n: usize = s.parse().unwrap();
            let m = n - (earl % n);
            if m < min.0 {
                min = (m, n);
            }
            vec.push((n, i));
        }
    }
    println!("{} * {} = {}", min.1, min.0, min.0 * min.1);

    let mut res = 0;
    let mut by = vec[0].0;
    for (v, i) in vec[1..].iter() {
        while (res + i) % v != 0 {
            res += by;
        }
        // I know it's true but I don't know why they can't share
        // a common factor, they must be coprime
        by *= v;
    }
    println!("Earliest: {}", res);
}
