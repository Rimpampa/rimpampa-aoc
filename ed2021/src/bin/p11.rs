use std::iter::{repeat, successors};

const INPUT: &str = include_str!("../../assets/p11.in");

fn get_map<const WIDTH: usize, const HEIGHT: usize>(input: &str) -> [[u8; WIDTH]; HEIGHT] {
    input
        .lines()
        .map(str::as_bytes)
        .map(<[u8; WIDTH]>::try_from)
        .map(Result::unwrap)
        .fold((0, [[0; WIDTH]; HEIGHT]), |(i, mut map), val| {
            map[i] = val.map(|v| v - b'0');
            (i + 1, map)
        })
        .1
}

fn neighbours<const WIDTH: usize, const HEIGHT: usize>(
    (x, y): (usize, usize),
) -> [Option<(usize, usize)>; 8] {
    [
        Some(x).zip(Some(y + 1).filter(|&y| y < 10)),
        x.checked_sub(1).zip(Some(y + 1).filter(|&y| y < 10)),
        x.checked_sub(1).zip(Some(y)),
        x.checked_sub(1).zip(y.checked_sub(1)),
        Some(x).zip(y.checked_sub(1)),
        Some(x + 1).filter(|&x| x < 10).zip(y.checked_sub(1)),
        Some(x + 1).filter(|&x| x < 10).zip(Some(y)),
        Some((x + 1, y + 1)).filter(|&(x, y)| x < 10 && y < 10),
    ]
}

fn indices<const WIDTH: usize, const HEIGHT: usize>() -> impl Iterator<Item = (usize, usize)> + Clone
{
    (0..WIDTH).flat_map(|x| repeat(x).zip(0..HEIGHT))
}

fn solve_1(input: &str) -> usize {
    let map = get_map::<10, 10>(input);
    successors(Some((0, map)), |(accum, map)| {
        let map = map.map(|row| row.map(|i| i + 1));
        let flashed = [[false; 10]; 10];
        let (flashes, map, _) = successors(Some((0, map, flashed)), |(prev, mut map, flashed)| {
            let flashing = map.map(|row| row.map(|v| v > 9));
            let flashes = indices::<10, 10>()
                .filter(|&(x, y)| flashing[x][y] && !flashed[x][y])
                .count();
            indices::<10, 10>()
                .filter(|&(x, y)| flashing[x][y] && !flashed[x][y])
                .flat_map(neighbours::<10, 10>)
                .flatten()
                .map(|(x, y)| map[x][y] += 1)
                .count();
            (flashes > 0).then(|| (prev + flashes, map, flashing))
        })
        .last()
        .unwrap();
        let map = map.map(|row| row.map(|v| (v > 9).then(|| 0).unwrap_or(v)));
        Some((accum + flashes, map))
    })
    .take(100 + 1)
    .last()
    .unwrap()
    .0
}

fn solve_2(input: &str) -> usize {
    let map = get_map::<10, 10>(input);
    successors(Some(map), |map| {
        let map = map.map(|row| row.map(|i| i + 1));
        let flashed = [[false; 10]; 10];
        let (map, flashing) = successors(Some((map, flashed)), |&(mut map, flashed)| {
            let flashing = map.map(|row| row.map(|v| v > 9));
            indices::<10, 10>()
                .filter(|&(x, y)| flashing[x][y] && !flashed[x][y])
                .flat_map(neighbours::<10, 10>)
                .flatten()
                .map(|(x, y)| map[x][y] += 1)
                .count();
            (flashing != flashed).then(|| (map, flashing))
        })
        .last()
        .unwrap();
        let map = map.map(|row| row.map(|v| (v > 9).then(|| 0).unwrap_or(v)));
        Some(map).filter(|_| flashing != [[true; 10]; 10])
    })
    .enumerate()
    .last()
    .unwrap()
    .0 + 1
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
        5483143223\n\
        2745854711\n\
        5264556173\n\
        6141336146\n\
        6357385478\n\
        4167524645\n\
        2176841721\n\
        6882881134\n\
        4846848554\n\
        5283751526\n\
    ";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 1656);
        assert_eq!(super::solve_2(TEST_INPUT), 195);
    }
}
