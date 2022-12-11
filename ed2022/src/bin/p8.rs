use std::iter::repeat;

const INPUT: &str = include_str!("../../assets/p8.in");

fn grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|s| s.bytes().collect()).collect()
}

fn at<T: Copy, A: AsRef<[T]>>(idx: usize) -> impl FnMut(&A) -> T {
    move |slice| slice.as_ref()[idx]
}

fn top(grid: &[Vec<u8>], x: usize, y: usize) -> impl DoubleEndedIterator<Item = u8> + '_ {
    grid[..y].iter().map(at(x))
}

fn bottom(grid: &[Vec<u8>], x: usize, y: usize) -> impl DoubleEndedIterator<Item = u8> + '_ {
    grid[y + 1..].iter().map(at(x))
}

fn left(grid: &[Vec<u8>], x: usize, y: usize) -> impl DoubleEndedIterator<Item = u8> + '_ {
    grid[y][..x].iter().copied()
}

fn right(grid: &[Vec<u8>], x: usize, y: usize) -> impl DoubleEndedIterator<Item = u8> + '_ {
    grid[y][x + 1..].iter().copied()
}

fn is_visible(grid: &[Vec<u8>], x: usize, y: usize) -> bool {
    let lt = |tree| tree < grid[y][x];
    top(grid, x, y).all(lt)
        || bottom(grid, x, y).all(lt)
        || left(grid, x, y).all(lt)
        || right(grid, x, y).all(lt)
}

fn scenic_score(grid: &[Vec<u8>], x: usize, y: usize) -> usize {
    fn visible(tree: u8, mut iter: impl Iterator<Item = u8>) -> Option<usize> {
        iter.position(|t| t >= tree).map(|i| i + 1)
    }
    let bottom_count = grid[y + 1..].len();
    let right_count = grid[y][x + 1..].len();
    // NOTE: always go outwards from the current tree
    let top = visible(grid[y][x], top(grid, x, y).rev()).unwrap_or(y);
    let bottom = visible(grid[y][x], bottom(grid, x, y)).unwrap_or(bottom_count);
    let left = visible(grid[y][x], left(grid, x, y).rev()).unwrap_or(x);
    let right = visible(grid[y][x], right(grid, x, y)).unwrap_or(right_count);
    top * bottom * left * right
}

fn solve_1(input: &str) -> usize {
    let grid = grid(input);
    grid.iter()
        .enumerate()
        .flat_map(|(x, col)| repeat(x).zip(0..col.len()))
        .filter(|&(x, y)| is_visible(&grid, x, y))
        .count()
}

fn solve_2(input: &str) -> usize {
    let grid = grid(input);
    grid.iter()
        .enumerate()
        .flat_map(|(x, col)| repeat(x).zip(0..col.len()))
        .map(|(x, y)| scenic_score(&grid, x, y))
        .max()
        .unwrap()
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
        30373\n\
        25512\n\
        65332\n\
        33549\n\
        35390\n";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 21);
        assert_eq!(super::solve_2(TEST_INPUT), 8);
    }
}
