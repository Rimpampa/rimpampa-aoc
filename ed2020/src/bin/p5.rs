use ed2020 as base;

#[derive(Copy, Debug, Clone)]
struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    fn get_id(self) -> u16 {
        self.row as u16 * 8 + self.column as u16
    }

    fn parse(s: &str) -> Self {
        let (row, column) = s.split_at(7);
        Self {
            row: row.chars().fold(0, |r, c| r + r + (c == 'B') as u8),
            column: column.chars().fold(0, |r, c| r + r + (c == 'R') as u8),
        }
    }
}

fn main() {
    let input = base::get_input(5).unwrap();

    let mut ids: Vec<u16> = input.lines().map(Seat::parse).map(Seat::get_id).collect();
    ids.sort_unstable();
    println!("Max: {}", ids.last().unwrap_or(&0));

    let missing = ids.windows(2).filter(|v| v[1] - v[0] > 1).map(|v| v[0] + 1);
    for id in missing {
        println!("Missing: {}", id);
    }
}
