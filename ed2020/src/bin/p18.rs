use ed2020 as base;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Token {
    Add,
    Mul,
    Val(usize),
    Open,
    Close,
}

fn parse(s: &str) -> Vec<Token> {
    let mut tks = Vec::new();
    let mut v = None;

    let s = s.as_bytes();
    for c in s.iter() {
        match c {
            n @ b'0'..=b'9' => {
                let v = v.get_or_insert(0);
                *v = *v * 10 + (n - b'0') as usize;
            }
            b' ' => {
                if let Some(v) = v.take() {
                    tks.push(Token::Val(v))
                }
            }
            b'+' => tks.push(Token::Add),
            b'*' => tks.push(Token::Mul),
            b'(' => tks.push(Token::Open),
            b')' => {
                if let Some(v) = v.take() {
                    tks.push(Token::Val(v));
                }
                tks.push(Token::Close);
            }
            v => panic!("Unexpected token '{}'", *v as char),
        }
    }
    if let Some(v) = v {
        tks.push(Token::Val(v));
    }
    tks
}

fn eval(mut tks: Vec<Token>) -> usize {
    use Token as Tk;
    while tks.len() > 1 {
        let mut new = Vec::new();
        let mut slice: &[Tk] = &tks;
        while let Some(rest) = match slice {
            [Tk::Val(x), Tk::Add, Tk::Val(y), rest @ ..] => {
                new.push(Tk::Val(x + y));
                new.extend_from_slice(rest);
                None
            }
            [Tk::Val(x), Tk::Mul, Tk::Val(y), rest @ ..] => {
                new.push(Tk::Val(x * y));
                new.extend_from_slice(rest);
                None
            }
            [Tk::Open, Tk::Val(x), Tk::Close, rest @ ..] => {
                new.push(Tk::Val(*x));
                new.extend_from_slice(rest);
                None
            }
            [Tk::Val(x), t @ Tk::Mul, rest @ ..] | [Tk::Val(x), t @ Tk::Add, rest @ ..] => {
                new.push(Tk::Val(*x));
                new.push(*t);
                Some(rest)
            }
            [t @ Tk::Open, rest @ ..] | [t @ Tk::Close, rest @ ..] => {
                new.push(*t);
                Some(rest)
            }
            v => panic!("Unexpected token sequence: {:?}", v),
        } {
            slice = rest;
        }
        tks = new;
    }
    match tks.as_slice() {
        [Tk::Val(x)] => *x,
        v => panic!("{:?}", v),
    }
}

fn eval_add_first(mut tks: Vec<Token>) -> usize {
    use Token as Tk;
    while tks.len() > 1 {
        let mut new = Vec::new();
        let mut slice: &[Tk] = &tks;
        while let Some(rest) = match slice {
            [Tk::Val(x), Tk::Add, Tk::Val(y), rest @ ..] => {
                new.push(Tk::Val(x + y));
                new.extend_from_slice(rest);
                None
            }
            [Tk::Val(x), Tk::Mul, Tk::Val(y), t @ Tk::Mul, rest @ ..]
            | [Tk::Val(x), Tk::Mul, Tk::Val(y), t @ Tk::Close, rest @ ..] => {
                new.push(Tk::Val(x * y));
                new.push(*t);
                new.extend_from_slice(rest);
                None
            }
            [Tk::Val(x), Tk::Mul, Tk::Val(y)] => {
                new.push(Tk::Val(x * y));
                None
            }
            [Tk::Open, Tk::Val(x), Tk::Close, rest @ ..] => {
                new.push(Tk::Val(*x));
                new.extend_from_slice(rest);
                None
            }
            [Tk::Val(x), t @ Tk::Mul, rest @ ..] | [Tk::Val(x), t @ Tk::Add, rest @ ..] => {
                new.push(Tk::Val(*x));
                new.push(*t);
                Some(rest)
            }
            [t @ Tk::Open, rest @ ..] | [t @ Tk::Close, rest @ ..] => {
                new.push(*t);
                Some(rest)
            }
            v => panic!("Unexpected token sequence: {:?}", v),
        } {
            slice = rest;
        }
        tks = new;
    }
    match tks.as_slice() {
        [Tk::Val(x)] => *x,
        v => panic!("{:?}", v),
    }
}

fn main() {
    let input = base::get_input(18).unwrap();
    let mut sum = 0;
    let mut sum_add_first = 0;
    for line in input.lines() {
        let tks = parse(line);
        sum += eval(tks.clone());
        sum_add_first += eval_add_first(tks);
    }
    println!("Result: {}", sum);
    println!("Result: {}", sum_add_first);
}
