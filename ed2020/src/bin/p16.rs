use ed2020 as base;

use std::{ops, str};

struct Rule {
    name: String,
    a: ops::RangeInclusive<usize>,
    b: ops::RangeInclusive<usize>,
}

impl Rule {
    fn is_valid(&self, n: usize) -> bool {
        self.a.contains(&n) || self.b.contains(&n)
    }
}

impl str::FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = base::split_at_str(s, ": ").ok_or(())?;
        let (a, b) = base::split_at_str(rest, " or ").ok_or(())?;
        let (a1, a2) = base::split_at(a, '-').ok_or(())?;
        let (b1, b2) = base::split_at(b, '-').ok_or(())?;
        let a1 = a1.parse().or(Err(()))?;
        let a2 = a2.parse().or(Err(()))?;
        let b1 = b1.parse().or(Err(()))?;
        let b2 = b2.parse().or(Err(()))?;
        Ok(Rule {
            name: name.into(),
            a: a1..=a2,
            b: b1..=b2,
        })
    }
}

fn main() {
    let input = base::get_input(16).unwrap();
    let mut lines = input.lines();
    let mut rules = Vec::<Rule>::new();

    for line in lines.by_ref().take_while(|s| !s.is_empty()) {
        rules.push(line.parse().unwrap());
    }

    let mut invalid_for = vec![Vec::<usize>::new(); rules.len()];

    assert_eq!(lines.next(), Some("your ticket:"));

    let mut my = Vec::<usize>::with_capacity(rules.len());
    for s in lines.next().unwrap().split(',') {
        my.push(s.parse().unwrap());
    }

    lines.next();
    assert_eq!(lines.next(), Some("nearby tickets:"));

    let mut sum = 0;
    for line in lines {
        for (i, s) in line.split(',').enumerate() {
            let n: usize = s.parse().unwrap();
            if !rules.iter().any(|r| r.is_valid(n)) {
                sum += n;
            } else {
                for (j, _) in rules.iter().enumerate().filter(|(_, v)| v.is_valid(n)) {
                    invalid_for[i].push(j);
                }
            }
        }
    }

    let mut names = vec![""; rules.len()];
    let mut remaining = (0..rules.len()).sum::<usize>();
    for len in (0..rules.len()).rev() {
        for (j, list) in invalid_for.iter().enumerate() {
            if list.len() == len - 1 {
                let n = remaining - list.iter().sum::<usize>();
                names[j] = &rules[n].name;
                remaining -= n;
                for list in invalid_for.iter_mut() {
                    if let Some(i) = list.iter().position(|j| *j == n) {
                        list.remove(i);
                    }
                }
                break;
            }
        }
    }

    let mut prod = 1;
    for (i, name) in names.iter().enumerate() {
        if name.starts_with("departure ") {
            prod *= my[i];
        }
    }
    println!("Sum: {}", sum);
    println!("Product: {}", prod);
}
