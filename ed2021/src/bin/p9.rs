use std::{io::BufRead, mem::replace};

const INPUT: &str = include_str!("../../assets/p9.in");

struct Map<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> Map<'a> {
    fn new(data: &'a str) -> Self {
        let data = data.as_bytes();
        let width = data.iter().position(|v| v.is_ascii_whitespace()).unwrap();
        let height = data.lines().count();
        Self {
            data,
            width,
            height,
        }
    }

    fn entry(&'a self, x: usize, y: usize) -> Option<Entry<'a>> {
        Entry::new(self, x, y)
    }

    fn entries(&'a self) -> impl Iterator<Item = Entry<'a>> {
        (0..self.width).flat_map(move |x| (0..self.height).map(move |y| self.entry(x, y).unwrap()))
    }

    fn low_point_entries(&'a self) -> impl Iterator<Item = Entry<'a>> {
        self.entries().filter(|e| {
            e.neighbours()
                .into_iter()
                .all(|n| none_or_ge_by_key(n, *e, Entry::value))
        })
    }
}

impl std::ops::Index<(usize, usize)> for Map<'_> {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.data.get(x + y * (self.width + 1)).unwrap()
    }
}

#[derive(Clone, Copy)]
struct Entry<'a> {
    map: &'a Map<'a>,
    x: usize,
    y: usize,
}

impl<'a> Entry<'a> {
    fn new(map: &'a Map<'a>, x: usize, y: usize) -> Option<Self> {
        Some(Self { map, x, y })
            .filter(|_| x < map.width)
            .filter(|_| y < map.height)
    }

    fn value(self) -> u8 {
        self.map[(self.x, self.y)]
    }

    fn left(self) -> Option<Self> {
        (self.x > 0).then(|| Self::new(self.map, self.x - 1, self.y).unwrap())
    }

    fn right(self) -> Option<Self> {
        Self::new(self.map, self.x + 1, self.y)
    }

    fn up(self) -> Option<Self> {
        (self.y > 0).then(|| Self::new(self.map, self.x, self.y - 1).unwrap())
    }

    fn down(self) -> Option<Self> {
        Self::new(self.map, self.x, self.y + 1)
    }

    fn cost(self) -> usize {
        (self.value() - b'0') as usize + 1
    }

    fn neighbours(self) -> [Option<Self>; 4] {
        [self.left(), self.right(), self.up(), self.down()]
    }
}

/// None or grater than by key
fn none_or_ge_by_key<T, F, O>(a: Option<T>, b: T, f: F) -> bool
where
    F: Fn(T) -> O,
    O: PartialOrd,
{
    match a {
        None => true,
        Some(v) => f(v) > f(b),
    }
}

fn solve_1(input: &str) -> usize {
    Map::new(input).low_point_entries().map(Entry::cost).sum()
}

fn solve_2(input: &str) -> usize {
    let map = Map::new(input);
    let mut stack = Vec::with_capacity(map.width * map.height);
    let mut seen = vec![false; map.width * map.height];
    map.low_point_entries()
        .map(|entry| {
            let mut count = 0;
            stack.clear();
            stack.push(entry);
            seen.fill(false);
            while let Some(entry) = stack.pop() {
                if !replace(&mut seen[entry.x + entry.y * map.width], true) && entry.value() != b'9'
                {
                    count += 1;
                    stack.extend(entry.neighbours().into_iter().flatten());
                }
            }
            count
        })
        .fold([0, 0, 0], |[a, b, c], basin| {
            [
                basin.max(a),
                basin.min(a).max(b),
                basin.min(a).min(b).max(c),
            ]
        })
        .into_iter()
        .product()
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
        2199943210\n\
        3987894921\n\
        9856789892\n\
        8767896789\n\
        9899965678\n\
    ";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 15);
        assert_eq!(super::solve_2(TEST_INPUT), 1134);
    }
}
