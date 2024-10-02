use std::iter::once;

macro_rules! for_each_line {
    ($str:expr, |$line:ident $(: usize)?| $fn:expr) => {
        let slice: &str = $str;
        let mut slice = slice.as_bytes();
        while let Some(idx) = ::aoc::const_utils::index_of_u8(slice, b'\n') {
            let ([$line @ .., b'\r'] | $line, rest) = ::aoc::const_utils::split_at(slice, idx);
            slice = rest;
            $fn
        }
    };
    (@cached $const:ident, |$line:ident $(: usize)?| $fn:expr) => {
        let mut slice: &[&[u8]] = &$const;
        while let [$line, rest @ ..] = slice {
            slice = rest;
            $fn
        }
    };
}

pub const INPUT: &str = include_str!("../../assets/p7.in");

pub const fn count_lines(str: &str) -> usize {
    let mut slice = str.as_bytes();
    let mut count = 0;
    while let [v, s @ ..] = slice {
        count += (*v == b'\n') as usize;
        slice = s;
    }
    count
}

pub const INPUT_LINES: usize = count_lines(INPUT);

pub const INPUT_LINES_ARRAY: [&[u8]; INPUT_LINES] = {
    let mut array = [b"".as_slice(); INPUT_LINES];
    let mut at = 0;
    for_each_line!(INPUT, |line| {
        array[at] = line;
        at += 1
    });
    array
};

pub const fn max_depth(input: &str) -> usize {
    let mut depth = 0;
    let mut at = 0;
    for_each_line!(@cached INPUT_LINES_ARRAY, |line| {
        let Some(entry) = Entry::parse(line) else { panic!() };
        match entry {
            Entry::Enter => at += 1,
            Entry::Exit => at -= 1,
            Entry::Root => at = 0,
            _ => (),
        }
        if depth < at {
            depth = at
        }
    });
    depth
}

#[derive(Debug)]
enum Entry {
    Exit,
    Enter,
    Root,
    List,
    File(usize),
    Dir,
}

impl Entry {
    const fn parse(mut line: &[u8]) -> Option<Self> {
        match line {
            b"$ cd /" => Some(Self::Root),
            b"$ cd .." => Some(Self::Exit),
            b"$ ls" => Some(Self::List),
            [b'$', b' ', b'c', b'd', b' ', ..] => Some(Self::Enter),
            [b'd', b'i', b'r', b' ', ..] => Some(Self::Dir),
            [b'0'..=b'9', ..] => {
                let mut size = 0;
                while let &[n @ b'0'..=b'9', ref l @ ..] = line {
                    size = size * 10 + (n - b'0') as usize;
                    line = l;
                }
                Some(Self::File(size))
            }
            _ => None,
        }
    }
}

struct Fs<F> {
    stack: [usize; max_depth(INPUT)],
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

fn solve_1(input: &str) -> usize {
    let mut sum = 0;
    let mut fs = Fs::new(|size| sum += (size <= 100000) as usize * size);
    input
        .lines()
        .map(str::as_bytes)
        .map(Entry::parse)
        .map(Option::unwrap)
        .chain(once(Entry::Root))
        .for_each(|entry| fs.execute(entry));
    sum
}

fn solve_2(input: &str) -> usize {
    let mut ordered = vec![];
    let mut fs = Fs::new(|size| {
        let Err(idx) = ordered.binary_search(&size) else { return };
        ordered.insert(idx, size)
    });
    input
        .lines()
        .map(str::as_bytes)
        .map(Entry::parse)
        .map(Option::unwrap)
        .chain(once(Entry::Root))
        .for_each(|entry| fs.execute(entry));
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
