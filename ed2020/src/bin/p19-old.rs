use ed2020 as base;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Token {
    Index(usize),
    Or,
    Value(char),
}

#[derive(Debug, Clone)]
struct Rule {
    tks: Vec<Token>,
}

impl Rule {
    fn parse(s: &str) -> Rule {
        let mut tks = Vec::new();
        for v in s.split(' ') {
            if let Ok(n) = v.parse() {
                tks.push(Token::Index(n))
            } else if v == "|" {
                tks.push(Token::Or)
            } else if v.len() == 3 && v.starts_with('"') && v.ends_with('"') {
                tks.push(Token::Value(v.chars().nth(1).unwrap()))
            } else {
                panic!("Unexpected token '{:?}'", v);
            }
        }
        Rule { tks }
    }

    fn matches(start: usize, rules: &HashMap<usize, Rule>, s: &str) -> Option<usize> {
        let rule = rules.get(&start).unwrap();
        let mut i = 0;
        let mut idx = 0;
        while idx < rule.tks.len() {
            let res = match rule.tks[idx] {
                Token::Value(x) => s.chars().nth(i).filter(|c| *c == x).map(|_| 1),
                Token::Index(x) => Self::matches(x, rules, &s[i..]),
                Token::Or => break,
            };
            if let Some(offset) = res {
                idx += 1;
                i += offset;
            } else if let Some(offset) = rule.tks[idx + 1..].iter().position(|t| *t == Token::Or) {
                idx += offset + 2;
                i = 0;
            } else {
                return None;
            }
        }
        Some(i)
    }
}

fn main() {
    let input = base::get_input(19).unwrap();
    let mut rules = HashMap::new();
    let mut lines = input.lines();

    for line in lines.by_ref().take_while(|s| !s.is_empty()) {
        let (idx, rest) = base::split_at_str(line, ": ").unwrap();
        let idx: usize = idx.parse().unwrap();
        rules.insert(idx, Rule::parse(rest));
    }

    let mut count = 0;
    for s in lines.clone() {
        if let Some(len) = Rule::matches(0, &rules, s) {
            count += (s.len() == len) as usize;
        }
    }

    println!("Matching: {}", count);
    // println!("\n\n");
    // println!("\n\n");
    // println!("\n\n");
    // println!("\n\n");
    // println!("\n\n");

    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    rules.get_mut(&8).unwrap().tks = vec![
        Token::Index(42),
        Token::Or,
        Token::Index(42),
        Token::Index(8),
    ];
    rules.get_mut(&11).unwrap().tks = vec![
        Token::Index(42),
        Token::Index(31),
        Token::Or,
        Token::Index(42),
        Token::Index(11),
        Token::Index(31),
    ];

    let mut count = 0;
    for s in lines {
        // println!("\n\n");
        if let Some(len) = Rule::matches(0, &rules, s) {
            if s.len() == len {
                println!("{}", s);
            }
            count += (s.len() == len) as usize;
        }
    }
    println!("Matching: {}", count);
}
