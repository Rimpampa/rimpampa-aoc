use ed2020 as base;

#[derive(Default, Debug)]
struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn parse_add(&mut self, mut s: &'a str) {
        while let Some(idx) = s.find(':') {
            let end = s[idx..]
                .chars()
                .position(char::is_whitespace)
                .map(|i| i + idx)
                .unwrap_or_else(|| s.len());
            *match &s[idx - 3..idx] {
                "byr" => &mut self.byr,
                "iyr" => &mut self.iyr,
                "eyr" => &mut self.eyr,
                "hgt" => &mut self.hgt,
                "hcl" => &mut self.hcl,
                "ecl" => &mut self.ecl,
                "pid" => &mut self.pid,
                "cid" => &mut self.cid,
                _ => unreachable!(),
            } = Some(&s[idx + 1..end]);
            s = &s[end..];
        }
    }

    fn is_strictly_valid(&self) -> Option<()> {
        match self {
            Passport {
                byr: Some(byr),
                iyr: Some(iyr),
                eyr: Some(eyr),
                hgt: Some(hgt),
                hcl: Some(hcl),
                ecl: Some(ecl),
                pid: Some(pid),
                cid: _,
            } => {
                let ubyr = byr.parse::<usize>().ok()?;
                base::opt(byr.len() == 4 && ubyr >= 1920 && ubyr <= 2002)?;

                let uiyr = iyr.parse::<usize>().ok()?;
                base::opt(iyr.len() == 4 && uiyr >= 2010 && uiyr <= 2020)?;

                let ueyr = eyr.parse::<usize>().ok()?;
                base::opt(eyr.len() == 4 && ueyr >= 2020 && ueyr <= 2030)?;

                let uhgt = hgt[..hgt.len() - 2].parse::<usize>().ok()?;
                base::opt(match hgt.as_bytes() {
                    [.., b'c', b'm'] => uhgt >= 150 && uhgt <= 193,
                    [.., b'i', b'n'] => uhgt >= 59 && uhgt <= 76,
                    _ => false,
                })?;

                base::opt(match hcl.as_bytes() {
                    [b'#', rest @ ..] if rest.len() == 6 => rest.iter().all(u8::is_ascii_hexdigit),
                    _ => false,
                })?;

                base::opt(matches!(
                    *ecl,
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                ))?;

                base::opt(pid.len() == 9 && pid.as_bytes().iter().all(u8::is_ascii_digit))
            }
            _ => None,
        }
    }

    fn is_valid(&self) -> bool {
        matches!(self, Passport {
            byr: Some(_),
            iyr: Some(_),
            eyr: Some(_),
            hgt: Some(_),
            hcl: Some(_),
            ecl: Some(_),
            pid: Some(_),
            cid: _,
        } )
    }

    fn clear(&mut self) {
        *self = Self::default()
    }
}

fn main() {
    let input = base::get_input(4).unwrap();
    let mut fields = Passport::default();
    let mut valid = 0;
    let mut strictly_valid = 0;

    for line in input.lines() {
        if line.is_empty() {
            valid += fields.is_valid() as usize;
            strictly_valid += fields.is_strictly_valid().is_some() as usize;
            fields.clear();
        } else {
            fields.parse_add(line);
        }
    }
    valid += fields.is_valid() as usize;
    strictly_valid += fields.is_strictly_valid().is_some() as usize;

    println!("Valid: {}", valid);
    println!("Strictly valid: {}", strictly_valid);
}
