use parser::{endline_terminated, many1, unsigned_number};

fn number_is_valid(target_num: u64, preamble: &[u64]) -> bool {
    preamble.iter().enumerate().any(|(i, &num1)| {
        preamble
            .iter()
            .skip(i + 1)
            .any(|&num2| num1 + num2 == target_num)
    })
}

fn first_invalid_number(numbers: &[u64], preamble_size: usize) -> Option<u64> {
    numbers
        .windows(preamble_size)
        .zip(numbers.iter().skip(preamble_size))
        .find(|(window, &num)| !number_is_valid(num, window))
        .map(|(_, &num)| num)
}

fn contiguous_sum(target_num: u64, numbers: &[u64]) -> Option<&[u64]> {
    for (i, &num1) in numbers.iter().enumerate() {
        let mut sum = num1;
        for (j, &num2) in numbers.iter().skip(i + 1).enumerate() {
            sum += num2;
            if sum == target_num {
                return Some(&numbers[i..=(i + j + 1)]);
            } else if sum > target_num {
                break;
            }
        }
    }
    None
}

fn main() {
    let input = include_str!("input");
    let (numbers, rest) = many1(|inp| endline_terminated(unsigned_number, inp), input)
        .expect("Failed to parse input");
    assert!(rest.is_empty());

    const PREAMBLE_SIZE: usize = 25;
    let first_invalid = first_invalid_number(&numbers, PREAMBLE_SIZE)
        .expect("Could not find the first invalid number");
    println!("Part 1: First invalid number: {}", first_invalid);
    let sum_window =
        contiguous_sum(first_invalid, &numbers).expect("Could not find a contiguous sum");
    println!(
        "Part 2: Sum of smallest and largest: {}",
        sum_window.iter().min().unwrap() + sum_window.iter().max().unwrap()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_is_valid() {
        let preamble = [35, 50, 15, 25, 47];
        assert!(number_is_valid(40, &preamble));
        assert!(number_is_valid(50, &preamble));
        assert!(!number_is_valid(100, &preamble));
        assert!(!number_is_valid(1, &preamble));
    }

    #[test]
    fn test_invalid_numbers() {
        let numbers = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(first_invalid_number(&numbers, 5), Some(127));
    }

    #[test]
    fn test_contiguous_sum() {
        let numbers = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let target_num = 127;
        assert_eq!(
            contiguous_sum(target_num, &numbers),
            Some(&[15, 25, 47, 40][..])
        );
    }
}
