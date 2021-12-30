use ed2020 as base;

#[derive(Copy, Clone, Debug)]
enum Cmd {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

impl Cmd {
    pub fn execute(self, prg: &mut Program) {
        match self {
            Cmd::Nop(_) => (),
            Cmd::Jmp(v) => prg.ip += v - 1,
            Cmd::Acc(v) => prg.acc += v,
        }
        prg.ip += 1;
    }
}

impl std::str::FromStr for Cmd {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg = s[4..].parse().or(Err(()))?;
        match &s[..3] {
            "nop" => Ok(Self::Nop(arg)),
            "jmp" => Ok(Self::Jmp(arg)),
            "acc" => Ok(Self::Acc(arg)),
            _ => Err(()),
        }
    }
}

struct Program {
    cmds: Vec<Cmd>,
    flags: Vec<bool>,
    acc: i32,
    ip: i32,
}

impl Program {
    fn new() -> Self {
        Self {
            cmds: Vec::new(),
            flags: Vec::new(),
            acc: 0,
            ip: 0,
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;
        self.flags.iter_mut().for_each(|f| *f = false);
    }

    fn add_cmd(&mut self, cmd: Cmd) {
        self.cmds.push(cmd);
        self.flags.push(false);
    }

    fn execute(&mut self) -> bool {
        while self.ip >= 0 && (self.ip as usize) < self.cmds.len() && !self.flags[self.ip as usize]
        {
            self.flags[self.ip as usize] = true;
            self.cmds[self.ip as usize].execute(self);
        }
        self.ip as usize == self.cmds.len()
    }

    fn fix(&mut self) {
        let mut change = 0;
        while {
            if let Some(offset) = self.cmds[change..]
                .iter()
                .position(|c| matches!(c, Cmd::Jmp(_) | Cmd::Nop(_)))
            {
                change += offset;
                match self.cmds[change] {
                    Cmd::Nop(v) => self.cmds[change] = Cmd::Jmp(v),
                    Cmd::Jmp(v) => self.cmds[change] = Cmd::Nop(v),
                    _ => unreachable!(),
                }
                !self.execute()
            } else {
                false
            }
        } {
            self.reset();
            match self.cmds[change] {
                Cmd::Nop(v) => self.cmds[change] = Cmd::Jmp(v),
                Cmd::Jmp(v) => self.cmds[change] = Cmd::Nop(v),
                _ => unreachable!(),
            }
            change += 1;
        }
    }
}

impl std::str::FromStr for Program {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut prg = Self::new();
        for line in s.lines() {
            prg.add_cmd(line.parse()?);
        }
        Ok(prg)
    }
}

fn main() {
    let input = base::get_input(8).unwrap();

    let mut prg: Program = input.parse().unwrap();
    prg.execute();
    println!("Acc: {}", prg.acc);

    prg.fix();
    println!("Acc: {}", prg.acc);
}
