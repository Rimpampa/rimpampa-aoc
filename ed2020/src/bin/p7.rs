use ed2020 as base;

#[derive(Debug)]
struct BagRules {
    colors: Vec<String>,
    contains: Vec<Vec<usize>>,
    amount: Vec<Vec<usize>>,
}

impl BagRules {
    fn new() -> Self {
        Self {
            colors: Vec::new(),
            contains: Vec::new(),
            amount: Vec::new(),
        }
    }

    fn add_bag(&mut self, color: &str) -> usize {
        if let Some(idx) = self.colors.iter().position(|c| c == color) {
            idx
        } else {
            self.colors.push(color.to_string());
            self.contains.push(Vec::new());
            self.amount.push(Vec::new());
            self.colors.len() - 1
        }
    }

    fn add_rule(&mut self, rule: &str) -> Option<()> {
        let mut split = rule.split(" bags contain ");
        let idx = self.add_bag(split.next()?);
        let contain = split.next()?.split(" bag");
        for mut color in contain {
            if color == "no other" || color.starts_with('.') || color.starts_with("s.") {
                break;
            } else if color.starts_with(", ") {
                color = &color[2..];
            } else if color.starts_with("s, ") {
                color = &color[3..];
            }
            let start = color.chars().position(|c| c == ' ')?;

            self.amount[idx].push(color[..start].parse().ok()?);

            color = &color[start + 1..];

            let idxc = self.add_bag(color);
            self.contains[idx].push(idxc);
        }

        Some(())
    }

    fn nested_bags(&self, color: &str) -> usize {
        if let Some(idx) = self.colors.iter().position(|c| c == color) {
            let mut bags = 0;
            let mut add = vec![(1, idx)];
            while !add.is_empty() {
                let (amount, idx) = add.pop().unwrap();
                bags += amount * self.amount[idx].iter().sum::<usize>();
                for (i, a) in self.contains[idx].iter().zip(self.amount[idx].iter()) {
                    add.push((a * amount, *i))
                }
            }
            bags
        } else {
            0
        }
    }

    fn which_contains(&self, color: &str) -> usize {
        if let Some(idx) = self.colors.iter().position(|c| c == color) {
            let mut sum = 0;
            let mut counted = Vec::new();
            let mut count = vec![idx];
            while !count.is_empty() {
                let idx = count.pop().unwrap();
                for (i, v) in self.contains.iter().enumerate() {
                    if !counted.contains(&i) && v.contains(&idx) {
                        counted.push(i);
                        count.push(i);
                        sum += 1;
                    }
                }
            }
            sum
        } else {
            0
        }
    }
}

use std::fmt;
impl fmt::Display for BagRules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ((color, vec), ams) in self
            .colors
            .iter()
            .zip(self.contains.iter())
            .zip(self.amount.iter())
        {
            write!(f, "{} contains ", color)?;
            for (color, amount) in vec.iter().map(|idx| &self.colors[*idx]).zip(ams.iter()) {
                write!(f, "{} {}, ", amount, color)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let input = base::get_input(7).unwrap();

    let mut rules = BagRules::new();
    for line in input.lines() {
        rules.add_rule(line);
    }
    // println!("{}", rules);

    let count = rules.which_contains("shiny gold");
    let bags = rules.nested_bags("shiny gold");

    println!("Count: {}", count);
    println!("Nested: {}", bags);
}
