use std::ops::RangeInclusive;

use parser::{fixed, take, take_while, take_while1, unsigned_number};

#[derive(Debug, PartialEq)]
struct Policy {
    min_max: RangeInclusive<u64>,
    character: char,
    password: String,
}

impl Policy {
    fn validate_1(&self) -> bool {
        let char_count = self
            .password
            .chars()
            .filter(|c| *c == self.character)
            .count();
        self.min_max.contains(&(char_count as u64))
    }

    fn validate_2(&self) -> bool {
        let first = *self.min_max.start() as usize;
        let end = *self.min_max.end() as usize;
        if self.password.len() < end as usize {
            return false;
        }
        let mut matching = 0;
        let mut chars = self.password.chars();
        if chars.nth(first - 1).unwrap() == self.character {
            matching += 1;
        }
        if chars.nth(end - first - 1).unwrap() == self.character {
            matching += 1;
        }
        return matching == 1;
    }
}

fn single_char(input: &str) -> Option<(char, &str)> {
    let (c, input) = take(1, input)?;
    Some((c.chars().next()?, input))
}

fn parse_password_policy(input: &str) -> Option<(Policy, &str)> {
    // Skip initial whitespace
    let (_, input) = take_while(|c| c.is_whitespace(), input);
    let (first_num, input) = unsigned_number(input)?;
    let (_, input) = fixed("-", input)?;
    let (second_num, input) = unsigned_number(input)?;
    let (_, input) = fixed(" ", input)?;
    let (character, input) = single_char(input)?;
    let (_, input) = fixed(": ", input)?;
    let (password, input) = take_while1(|c| c.is_ascii_lowercase(), input)?;
    let policy = Policy {
        min_max: first_num..=second_num,
        character: character,
        password: password.to_owned(),
    };
    Some((policy, input))
}

fn main() {
    let mut input = include_str!("input");
    let mut valid_count_1 = 0;
    let mut valid_count_2 = 0;
    while let Some((policy, rest)) = parse_password_policy(input) {
        if policy.validate_1() {
            valid_count_1 += 1;
        }
        if policy.validate_2() {
            valid_count_2 += 1;
        }
        input = rest;
    }
    println!("Part 1: {} valid passwords", valid_count_1);
    println!("Part 2: {} valid passwords", valid_count_2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_complete() {
        let input = "1-3 a: abcde";
        let res = parse_password_policy(input);
        let expected = Policy {
            min_max: 1..=3,
            character: 'a',
            password: "abcde".to_owned(),
        };

        assert_eq!(res, Some((expected, "")));
    }

    #[test]
    fn parse_two_complete() {
        let input = "1-3 a: abcde\n1-2 b: cdefg";
        let res = parse_password_policy(input);
        assert!(res.is_some());
        let (_, rest) = res.unwrap();
        let expected = Policy {
            min_max: 1..=2,
            character: 'b',
            password: "cdefg".to_owned(),
        };
        let res = parse_password_policy(rest);
        assert_eq!(res, Some((expected, "")));
    }

    #[test]
    fn parse_incomplete() {
        let input = "1-3";
        let res = parse_password_policy(input);
        assert_eq!(res, None);
    }

    #[test]
    fn validate_1_valid() {
        let policy = Policy {
            min_max: 1..=3,
            character: 'a',
            password: "abcde".to_owned(),
        };
        assert!(policy.validate_1());
    }

    #[test]
    fn validate_1_invalid() {
        let policy = Policy {
            min_max: 1..=3,
            character: 'b',
            password: "cdefg".to_owned(),
        };
        assert!(!policy.validate_1());
    }

    #[test]
    fn validate_2_valid() {
        let policy = Policy {
            min_max: 1..=3,
            character: 'a',
            password: "abcde".to_owned(),
        };
        assert!(policy.validate_2());
    }

    #[test]
    fn validate_2_invalid_no_match() {
        let policy = Policy {
            min_max: 1..=3,
            character: 'a',
            password: "cdefg".to_owned(),
        };
        assert!(!policy.validate_2());
    }

    #[test]
    fn validate_2_invalid_two_matches() {
        let policy = Policy {
            min_max: 2..=9,
            character: 'c',
            password: "ccccccccc".to_owned(),
        };
        assert!(!policy.validate_2());
    }
}
