use std::iter::once;

const INPUT: &str = include_str!("../../assets/p7.in");

enum Entry {
    Exit,
    Enter,
    Root,
    List,
    File(usize),
    Dir,
}

impl Entry {
    fn parse(line: &str) -> Option<Self> {
        match line.strip_prefix("$ cd ") {
            Some("/") => return Some(Self::Root),
            Some("..") => return Some(Self::Exit),
            Some(_) => return Some(Self::Enter),
            None => (),
        };
        Some(match line.split_once(' ')? {
            ("$", "ls") => Self::List,
            ("dir", _) => Self::Dir,
            (size, _) => Self::File(size.parse().ok()?),
        })
    }
}

struct Fs<F> {
    stack: Vec<usize>,
    used: usize,
    on_exit: F,
}

impl<F: FnMut(usize)> Fs<F> {
    fn new(on_exit: F) -> Self {
        Self {
            stack: vec![],
            used: 0,
            on_exit,
        }
    }

    fn execute(&mut self, entry: Entry) {
        match entry {
            Entry::Exit => {
                let size = self.stack.pop().unwrap();
                self.stack.last_mut().map(|v| *v += size);
                (self.on_exit)(size)
            }
            Entry::File(size) => {
                self.stack.last_mut().map(|v| *v += size);
                self.used += size
            }
            Entry::Enter => self.stack.push(0),
            Entry::Root => self.stack.drain(..).rev().for_each(&mut self.on_exit),
            _ => (),
        }
    }
}

fn entries(input: &str) -> impl Iterator<Item = Entry> + '_ {
    input
        .lines()
        .map(Entry::parse)
        .map(Option::unwrap)
        .chain(once(Entry::Root))
}

fn solve_1(input: &str) -> usize {
    let mut sum = 0;
    let mut fs = Fs::new(|size| sum += (size <= 100000) as usize * size);
    entries(input).for_each(|entry| fs.execute(entry));
    sum
}

fn solve_2(input: &str) -> usize {
    let mut ordered = vec![];
    let mut fs = Fs::new(|size| {
        let Err(idx) = ordered.binary_search(&size) else { return };
        ordered.insert(idx, size)
    });
    entries(input).for_each(|entry| fs.execute(entry));
    let missing = 30000000 - (70000000 - fs.used);
    let (Ok(idx) | Err(idx)) = ordered.binary_search(&missing);
    ordered[idx]
}

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
        $ cd /\n\
        $ ls\n\
        dir a\n\
        14848514 b.txt\n\
        8504156 c.dat\n\
        dir d\n\
        $ cd a\n\
        $ ls\n\
        dir e\n\
        29116 f\n\
        2557 g\n\
        62596 h.lst\n\
        $ cd e\n\
        $ ls\n\
        584 i\n\
        $ cd ..\n\
        $ cd ..\n\
        $ cd d\n\
        $ ls\n\
        4060174 j\n\
        8033020 d.log\n\
        5626152 d.ext\n\
        7214296 k\n";

    #[test]
    fn test() {
        assert_eq!(super::solve_1(TEST_INPUT), 95437);
        assert_eq!(super::solve_2(TEST_INPUT), 24933642);
    }
}
