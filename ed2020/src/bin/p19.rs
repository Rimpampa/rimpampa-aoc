use ed2020 as base;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Token {
    Index(usize),
    Or,
    Value(char),
}

#[derive(Clone, Debug)]
struct Trace<'a> {
    index: usize,
    rule: &'a Rule,
    at: usize,
    input: &'a str,
    from: usize,
    cloned: bool,
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
        let mut stack = vec![Trace {
            index: start,
            rule: rules.get(&start).unwrap(),
            at: 0,
            input: s,
            from: 0,
            cloned: false,
        }];
        let mut result: Option<usize> = None;
        while let Some(trace) = stack.last_mut() {
            if let Some(offset) = result.take() {
                trace.from += offset;
            } else {
                if let Some(offset) = trace.rule.tks[trace.at..]
                    .iter()
                    .position(|t| matches!(t, Token::Or))
                {
                    // Found it, move to the token that follows
                    trace.at += offset + 1;
                } else {
                    // Not found, doesn't match
                    stack.pop();
                }
                continue;
            }
            if trace.at < trace.rule.tks.len() {
                match trace.rule.tks[trace.at] {
                    Token::Index(i) => {
                        // push the rule index i to the top of the stack
                        trace.at += 1;
                        let input = &trace.input[trace.from..];
                        stack.push(Trace {
                            index: i,
                            rule: rules.get(&i).unwrap(),
                            at: 0,
                            input,
                            from: 0,
                            cloned: false,
                        })
                    }
                    Token::Value(c) => {
                        // get the character currently pointed by the cursor
                        if let Some(v) = trace.input.chars().nth(trace.from) {
                            if v == c {
                                // if it does move the cursor to the next one
                                trace.from += 1;
                            }
                            // if it doesn't search for an OR
                            else if let Some(offset) = trace.rule.tks[trace.at..]
                                .iter()
                                .position(|t| matches!(t, Token::Or))
                            {
                                // Found it, move to the token that follows
                                trace.at += offset + 1;
                            } else {
                                // Not found, doesn't match
                                stack.pop();
                            }
                        } else {
                            // If the string is not enough long it doens't match
                            stack.pop();
                        }
                    }
                    Token::Or => {
                        trace.at += 1;
                        let input = &trace.input[trace.from..];
                        let opt = stack.get(stack.len() - 2);
                        if opt.is_some() {
                            let t = opt.unwrap().clone();
                            stack.push(Trace {
                                index: t.index,
                                rule: rules.get(&t.index).unwrap(),
                                at: t.at,
                                input,
                                from: 0,
                                cloned: true,
                            })
                        } else {
                            // This is the first rule, thus it matches
                            result = Some(trace.from);
                            stack.pop();
                        }
                    }
                }
            } else {
                // no more tokens = matches
                result = Some(trace.from);
                if trace.cloned {
                    stack.pop();
                }
                stack.pop();
            }
        }
        None
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
