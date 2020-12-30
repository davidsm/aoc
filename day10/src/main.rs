use parser::{endline_terminated, many1, unsigned_number};

fn count_paths(sorted_numbers: &[u64]) -> u64 {
    let mut trail = [0; 3];
    for (i, n) in sorted_numbers.iter().enumerate().rev() {
        let mut branches = 0;
        for (j, next) in sorted_numbers[(i + 1)..].iter().take(3).enumerate() {
            if (1..=3).contains(&(next - n)) {
                branches += trail[j];
            } else {
                break;
            }
        }
        trail = [std::cmp::max(branches, 1), trail[0], trail[1]];
    }
    trail[0]
}

fn main() {
    let input = include_str!("input");
    let (mut numbers, rest) = many1(|inp| endline_terminated(unsigned_number, inp), input)
        .expect("Failed to parse input");
    assert!(rest.is_empty());

    numbers.push(0);
    numbers.sort_unstable();
    let max = *numbers.last().unwrap();
    numbers.push(max + 3);

    let (ones, threes) = numbers
        .windows(2)
        .map(|win| win[1] - win[0])
        .fold((0, 0), |(o, t), n| {
            if n == 1 {
                (o + 1, t)
            } else if n == 3 {
                (o, t + 1)
            } else {
                (o, t)
            }
        });
    println!("Part 1: Multiple of ones and threes: {}", ones * threes);
    println!("Part 2: Number of alternatives: {}", count_paths(&numbers));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_paths() {
        let mut input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3, 0, 52,
        ];
        input.sort_unstable();
        assert_eq!(count_paths(&input), 19208);
    }
}
