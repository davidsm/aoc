use std::collections::BTreeSet;

fn main() {
    let input = include_str!("input")
        .lines()
        .filter_map(|l| l.parse::<u32>().ok());

    let mut seen = BTreeSet::new();

    let (mut part1, mut part2) = (false, false);
    for n in input {
        let remainder = 2020 - n;
        if !part1 {
            if seen.contains(&remainder) {
                println!("Part 1: Multiple is {}", n * remainder);
                part1 = true;
            }
        }
        if !part2 {
            for previous in seen.range(1..remainder) {
                let second_remainder = remainder - previous;
                if seen.contains(&second_remainder) {
                    println!("Part 2: Multiple is {}", n * previous * second_remainder);
                    part2 = true;
                    break;
                }
            }
        }
        if part1 && part2 {
            break;
        }
        seen.insert(n);
    }
}
