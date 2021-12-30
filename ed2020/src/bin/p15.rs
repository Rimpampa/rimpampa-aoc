use ed2020 as base;

use std::collections::hash_map::{Entry, HashMap};

fn main() {
    let input = base::get_input(15).unwrap();

    let mut vec: Vec<(usize, usize)> = Vec::new();
    let mut map: HashMap<usize, usize> = HashMap::new();

    let mut split = input.split(',').enumerate();
    let mut spoken = split.next().unwrap().1.parse().unwrap();
    for (turn, line) in split {
        let n: usize = line.parse().unwrap();
        // spoken.map(|s| vec.push((s, turn)));
        map.insert(spoken, turn);
        spoken = n;
    }
    vec.sort_unstable_by_key(|n| n.0);

    for turn in map.len() + 1..2020 {
        let new;
        match map.entry(spoken) {
            Entry::Occupied(e) => {
                new = turn - e.get();
                let key = *e.key();
                *map.get_mut(&key).unwrap() = turn;
            }
            Entry::Vacant(e) => {
                new = 0;
                e.insert(turn);
            }
        }
        spoken = new;
    }
    println!("2020th: {}", spoken);

    for turn in 2020..30000000 {
        let new;
        match map.entry(spoken) {
            Entry::Occupied(e) => {
                new = turn - e.get();
                let key = *e.key();
                *map.get_mut(&key).unwrap() = turn;
            }
            Entry::Vacant(e) => {
                new = 0;
                e.insert(turn);
            }
        }
        spoken = new;
    }
    println!("30000000th: {}", spoken);
}
