use ed2020 as base;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    /// direction: true = left, false = right
    fn rotate(&mut self, mut by: usize, direction: Direction) {
        while by >= 90 {
            match direction {
                Direction::Left => match *self {
                    Facing::North => *self = Facing::West,
                    Facing::West => *self = Facing::South,
                    Facing::South => *self = Facing::East,
                    Facing::East => *self = Facing::North,
                },
                Direction::Right => match *self {
                    Facing::North => *self = Facing::East,
                    Facing::West => *self = Facing::North,
                    Facing::South => *self = Facing::West,
                    Facing::East => *self = Facing::South,
                },
            }
            by -= 90;
        }
    }
}

#[derive(Debug)]
struct Ship {
    x: isize,
    y: isize,
    facing: Facing,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            facing: Facing::East,
        }
    }

    fn exec(&mut self, action: Action) {
        match action {
            Action::Move(facing, amount) => match facing {
                Facing::North => self.y += amount as isize,
                Facing::East => self.x += amount as isize,
                Facing::South => self.y -= amount as isize,
                Facing::West => self.x -= amount as isize,
            },
            Action::Rotate(dir, deg) => self.facing.rotate(deg, dir),
            Action::Foreward(amount) => match self.facing {
                Facing::North => self.y += amount as isize,
                Facing::East => self.x += amount as isize,
                Facing::South => self.y -= amount as isize,
                Facing::West => self.x -= amount as isize,
            },
        }
    }

    fn distance(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn rotate(&mut self, mut by: usize, direction: Direction) {
        while by >= 90 {
            *self = match direction {
                Direction::Right => Self::new(self.y, -self.x),
                Direction::Left => Self::new(-self.y, self.x),
            };
            by -= 90;
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Mul<usize> for Point {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self::new(self.x * rhs as isize, self.y * rhs as isize)
    }
}

#[derive(Debug)]
struct ShipWaypoint {
    pos: Point,
    way: Point,
}

impl ShipWaypoint {
    fn new() -> Self {
        Self {
            pos: Point::new(0, 0),
            way: Point::new(10, 1),
        }
    }

    fn exec(&mut self, action: Action) {
        match action {
            Action::Move(facing, amount) => match facing {
                Facing::North => self.way.y += amount as isize,
                Facing::East => self.way.x += amount as isize,
                Facing::South => self.way.y -= amount as isize,
                Facing::West => self.way.x -= amount as isize,
            },
            Action::Rotate(dir, deg) => self.way.rotate(deg, dir),
            Action::Foreward(amount) => self.pos += self.way * amount,
        }
    }

    fn distance(&self) -> usize {
        self.pos.x.abs() as usize + self.pos.y.abs() as usize
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Move(Facing, usize),
    Rotate(Direction, usize),
    Foreward(usize),
}

impl std::str::FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount: usize = s[1..].parse().or(Err(()))?;
        match s.chars().next().ok_or(())? {
            'N' => Ok(Action::Move(Facing::North, amount)),
            'S' => Ok(Action::Move(Facing::South, amount)),
            'E' => Ok(Action::Move(Facing::East, amount)),
            'W' => Ok(Action::Move(Facing::West, amount)),
            'L' => Ok(Action::Rotate(Direction::Left, amount)),
            'R' => Ok(Action::Rotate(Direction::Right, amount)),
            'F' => Ok(Action::Foreward(amount)),
            _ => Err(()),
        }
    }
}

fn main() {
    let input = base::get_input(12).unwrap();

    let mut ship = Ship::new();
    let mut ship_way = ShipWaypoint::new();
    for line in input.lines() {
        let act = line.parse().unwrap();
        // println!("{:?}", act);
        ship.exec(act);
        ship_way.exec(act);
        // println!("{:?}", ship);
    }
    println!("Distance: {}", ship.distance());
    println!("Distance: {}", ship_way.distance());
}
