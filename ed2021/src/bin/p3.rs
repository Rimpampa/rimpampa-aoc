const INPUT: &str = include_str!("../../assets/p3.in");

fn solve_1<const SIZE: usize>(input: &str) -> usize {
    let mut total = 0;
    let mut ones = [0; SIZE];
    for line in input.lines() {
        total += 1;
        line.bytes()
            .enumerate()
            .filter_map(|(i, v)| (v == b'1').then(|| i))
            .for_each(|i| ones[i] += 1);
    }
    let half = total / 2;
    let gamma = ones
        .into_iter()
        .fold(0, |res, ones| (res << 1) + (ones > half) as usize);
    let epsilon = !gamma & ((1 << SIZE) - 1);
    gamma * epsilon
}

fn solve_2<const SIZE: usize>(input: &str) -> usize {
    let mut oxy = [0; SIZE];
    let mut co2 = [0; SIZE];

    for i in 0..SIZE {
        let mut oxy_total = 0;
        let mut co2_total = 0;

        let mut oxy_count = 0;
        let mut co2_count = 0;

        for line in input.lines() {
            let (prefix, n) = line.as_bytes().split_at(i);
            let oxy_prefix = prefix == &oxy[..i];
            let co2_prefix = prefix == &co2[..i];

            oxy_total += oxy_prefix as usize;
            co2_total += co2_prefix as usize;

            oxy_count += (oxy_prefix && n[0] == b'1') as usize;
            co2_count += (co2_prefix && n[0] == b'1') as usize;
        }
        oxy[i] = b'0'
            + ((oxy_total == 1 && oxy_count == 1) || (oxy_total > 1 && oxy_count * 2 >= oxy_total))
                as u8;
        co2[i] = b'0'
            + ((co2_total == 1 && co2_count == 1) || (co2_total > 1 && co2_count * 2 < co2_total))
                as u8;
    }
    let oxy = oxy.map(|v| v - b'0').map(usize::from);
    let co2 = co2.map(|v| v - b'0').map(usize::from);
    let oxygen_gen = oxy.into_iter().fold(0, |res, bit| res * 2 + bit);
    let co2_scrub = co2.into_iter().fold(0, |res, bit| res * 2 + bit);
    oxygen_gen * co2_scrub
}

fn main() {
    println!("Answer 1: {}", solve_1::<12>(INPUT));
    println!("Answer 2: {}", solve_2::<12>(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn test() {
        assert_eq!(super::solve_1::<5>(TEST_INPUT), 198);
        assert_eq!(super::solve_2::<5>(TEST_INPUT), 230);
    }
}
