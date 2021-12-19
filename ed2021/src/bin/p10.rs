use std::iter::successors;

const INPUT: &str = include_str!("../../assets/p10.in");

const OPENING_TOKENS: [u8; 4] = *b"([{<";
const CLOSING_TOKENS: [u8; 4] = *b")]}>";
const SCORES: [usize; 4] = [3, 57, 1197, 25137];

/// Returns the index of the first instance of v found in the iterator
fn index_of<T: PartialEq>(v: T, iter: impl IntoIterator<Item = T>) -> usize {
    iter.into_iter().position(|o| v == o).unwrap()
}

/// Returns the points of the closing token passed
fn get_points(closing: u8) -> usize {
    index_of(closing, CLOSING_TOKENS) + 1
}

/// Returns the score of the closing token passed
fn get_score(closing: u8) -> usize {
    SCORES[index_of(closing, CLOSING_TOKENS)]
}

/// Returns the opening token relative to the opening one passed
fn get_opening(closing: u8) -> u8 {
    OPENING_TOKENS[index_of(closing, CLOSING_TOKENS)]
}

/// Returns the closing token relative to the closing one passed
fn get_closing(opening: u8) -> u8 {
    CLOSING_TOKENS[index_of(opening, OPENING_TOKENS)]
}

/// Finds the first opening without a closing token in the iterator.
///
/// The iterator is considered as reversed
fn find_opening(line: impl Iterator<Item = u8>) -> Option<u8> {
    let mut depth = 0;
    for c in line {
        match depth {
            _ if CLOSING_TOKENS.contains(&c) => depth += 1,
            0 => return Some(c),
            _ => depth -= 1,
        }
    }
    None
}

/// Returns an iterator overs the indices of the closing tokens
fn closing_indices(iter: impl Iterator<Item = u8>) -> impl Iterator<Item = usize> {
    iter.enumerate()
        .filter_map(|(i, c)| CLOSING_TOKENS.contains(&c).then(|| i))
}

/// Finds the index of first invalid opening token.
/// That is the first opening token that has a diffrent closing token
fn find_invalid_opening(str: &str) -> Option<usize> {
    closing_indices(str.bytes())
        .find(|&i| find_opening(str.bytes().take(i).rev()) != Some(get_opening(str.as_bytes()[i])))
}

/// Given the points of the closing tokens, an iterator with all the
/// closing tokens in reverse order is generated
fn tokens_from_number(n: usize) -> impl Iterator<Item = u8> {
    successors(Some(n).filter(|&n| n > 0), |n| {
        Some(n / 5).filter(|&n| n > 0)
    })
    .map(|n| CLOSING_TOKENS[(n % 5) - 1])
}

/// Returns the points for the next closing token added, or None if there are no more
/// closing tokens to add
fn next_closing_token(s: &str, points: usize) -> Option<usize> {
    let next = find_opening(tokens_from_number(points).chain(s.bytes().rev()))?;
    Some(points * 5 + get_points(get_closing(next)))
}

/// Returns the points of all the closing needed for fixing the the passed string.
fn get_fixing_points(s: &str) -> usize {
    successors(Some(0), |&p| next_closing_token(s, p))
        .last()
        .unwrap()
}

fn solve_1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|s| find_invalid_opening(s).map(|i| s.as_bytes()[i]))
        .map(get_score)
        .sum()
}

fn solve_2(input: &str) -> usize {
    let mut scores: Vec<usize> = input
        .lines()
        .filter(|s| find_invalid_opening(s).is_none())
        .map(get_fixing_points)
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
        [({(<(())[]>[[{[]{<()<>>\n\
        [(()[<>])]({[<{<<[]>>(\n\
        {([(<{}[<>[]}>{[]{[(<()>\n\
        (((({<>}<{<{<>}{[]{[]{}\n\
        [[<[([]))<([[{}[[()]]]\n\
        [{[{({}]{}}([{[{{{}}([]\n\
        {<[[]]>}<{[{[{[]{()[[[]\n\
        [<(<(<(<{}))><([]([]()\n\
        <{([([[(<>()){}]>(<<{{\n\
        <{([{{}}[<[[[<>{}]]]>[]]\n\
    ";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 26397);
        assert_eq!(super::solve_2(TEST_INPUT), 288957);
    }
}
