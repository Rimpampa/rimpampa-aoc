use std::{array, cmp::Ordering, iter};

const INPUT: &str = include_str!("../../assets/p13.in");

#[derive(Clone, Copy, Debug)]
enum Token {
    Open,
    Num(usize),
    Close,
}

fn tokens(mut packet: &str) -> impl Iterator<Item = Token> + '_ {
    iter::from_fn(move || {
        while let Some(rest) = packet.strip_prefix(',') {
            packet = rest;
        }
        if let Some(rest) = packet.strip_prefix('[') {
            packet = rest;
            Some(Token::Open)
        } else if let Some(rest) = packet.strip_prefix(']') {
            packet = rest;
            Some(Token::Close)
        } else {
            let digits = packet.find([',', ']'])?;
            let (digits, rest) = packet.split_at(digits);
            packet = rest;
            digits.parse().ok().map(Token::Num)
        }
    })
}

#[derive(Debug)]
enum Item {
    Num(usize),
    List(Vec<Self>),
}

impl Item {
    fn new(mut tks: impl Iterator<Item = Token>) -> Self {
        match tks.next() {
            Some(Token::Open) => Self::list(&mut tks),
            Some(Token::Num(n)) => Item::Num(n),
            _ => panic!(),
        }
    }

    fn list(tks: &mut impl Iterator<Item = Token>) -> Self {
        let mut list = vec![];
        while let Some(token) = tks.next() {
            match token {
                Token::Open => list.push(Self::list(tks)),
                Token::Num(n) => list.push(Item::Num(n)),
                Token::Close => break,
            }
        }
        Self::List(list)
    }

    fn listed(&mut self) -> &mut Self {
        match *self {
            Self::Num(n) => *self = Self::List(vec![Self::Num(n)]),
            Self::List(_) => (),
        }
        self
    }

    fn cmp(&mut self, rhs: &mut Self) -> Ordering {
        use Item::*;
        match (self, rhs) {
            (Num(l), Num(r)) => l.cmp(&r),
            (l @ List(_), r @ Num(_)) => l.cmp(r.listed()),
            (l @ Num(_), r @ List(_)) => l.listed().cmp(r),
            (List(l), List(r)) => Self::cmp_lists(l, r),
        }
    }

    fn cmp_lists(lhs: &mut [Self], rhs: &mut [Self]) -> Ordering {
        use Ordering::*;
        let [mut l, mut r] = [lhs.iter_mut(), rhs.iter_mut()];
        for items in iter::repeat_with(|| [l.next(), r.next()]) {
            return match items {
                [Some(_), None] => Greater,
                [None, Some(_)] => Less,
                [None, None] => Equal,
                [Some(l), Some(r)] => match l.cmp(r) {
                    Less => Less,
                    Greater => Greater,
                    Equal => continue,
                },
            };
        }
        unreachable!()
    }
}

fn item_pairs(input: &str) -> impl Iterator<Item = [Item; 2]> + '_ {
    let eol = input
        .contains("\n\n")
        .then_some("\n\n")
        .unwrap_or("\r\n\r\n");
    input.split(eol).map(str::lines).map(|mut lines| {
        array::from_fn(|_| lines.next())
            .map(Option::unwrap)
            .map(tokens)
            .map(Item::new)
    })
}

fn items(input: &str) -> impl Iterator<Item = Item> + '_ {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(tokens)
        .map(Item::new)
}

fn binary_search_item(items: &mut [Item], new: &mut Item) -> usize {
    let [_, ..] = items else { return 0 };
    let mid = items.len() / 2;
    match items[mid].cmp(new) {
        Ordering::Greater => binary_search_item(&mut items[..mid], new),
        Ordering::Equal => mid,
        Ordering::Less => binary_search_item(&mut items[mid + 1..], new) + mid + 1,
    }
}

fn solve_1(input: &str) -> usize {
    item_pairs(input)
        .map(|[mut l, mut r]| l.cmp(&mut r))
        .enumerate()
        .filter_map(|(i, b)| b.is_le().then_some(i + 1))
        .sum()
}

fn solve_2(input: &str) -> usize {
    use Item::*;

    let items = &mut vec![];
    for mut item in self::items(input) {
        let i = binary_search_item(items, &mut item);
        items.insert(i, item);
    }
    let div2 = 1 + binary_search_item(items, &mut List(vec![List(vec![Num(2)])]));
    let div6 = 2 + binary_search_item(items, &mut List(vec![List(vec![Num(6)])]));
    div2 * div6
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
        [1,1,3,1,1]\n\
        [1,1,5,1,1]\n\
        \n\
        [[1],[2,3,4]]\n\
        [[1],4]\n\
        \n\
        [9]\n\
        [[8,7,6]]\n\
        \n\
        [[4,4],4,4]\n\
        [[4,4],4,4,4]\n\
        \n\
        [7,7,7,7]\n\
        [7,7,7]\n\
        \n\
        []\n\
        [3]\n\
        \n\
        [[[]]]\n\
        [[]]\n\
        \n\
        [1,[2,[3,[4,[5,6,7]]]],8,9]\n\
        [1,[2,[3,[4,[5,6,0]]]],8,9]\n";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 13);
        assert_eq!(super::solve_2(TEST_INPUT), 140);
    }
}
