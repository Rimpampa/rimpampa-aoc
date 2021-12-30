use io::Read;
use std::{io, str};

#[macro_export]
macro_rules! replace {
    ($a:tt $b:tt) => {
        $b
    };
}

#[macro_export]
macro_rules! neighbours {
    ([$b:ident $(, $d:ident)?] as $a:ident in 0..$c:expr, $([$nb:ident $(, $nd:ident)?] as $na:ident in 0..$nc:expr),* => $f:expr) => {{
        let $a = $b;
        $(let $d = 0;)?
        $(let $na = $nb; $(let $nd = 0;)?)*

        if $b + 1 < $c {
            $(let $d = 1;)?
            let $a = $b + 1;
            $f;
            $crate::neighbours!($([$nb $(, $nd)?] as $na in 0..$nc),* => $f);
        }
        if $b > 0 {
            $(let $d = -1;)?
            let $a = $b - 1;
            $f;
            $crate::neighbours!($([$nb $(, $nd)?] as $na in 0..$nc),* => $f);
        }
        $(let $d = 0;)?
        let $a = $b;
        $crate::neighbours!($([$nb $(, $nd)?] as $na in 0..$nc),* => $f);
    }};

    ([$b:expr $(, $d:ident)?] as $a:ident in 0..$c:expr => $f:expr) => {
        if $b + 1 < $c {
            $(let $d = 1;)?
            let $a = $b + 1;
            $f;
        }
        if $b > 0 {
            $(let $d = -1;)?
            let $a = $b - 1;
            $f;
        }
    }
}

pub fn get_input(n: usize) -> io::Result<String> {
    let mut s = String::new();
    let path = format!("assets/p{}.in", n);
    std::fs::File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn split_at(s: &str, c: char) -> Option<(&str, &str)> {
    let (a, b) = s.find(c).map(|i| s.split_at(i))?;
    Some((a, &b[1..]))
}

pub fn split_at_str<'a>(s: &'a str, c: &str) -> Option<(&'a str, &'a str)> {
    let (a, b) = s.find(c).map(|i| s.split_at(i))?;
    Some((a, &b[c.len()..]))
}

pub const fn opt(b: bool) -> Option<()> {
    match b {
        true => Some(()),
        false => None,
    }
}
