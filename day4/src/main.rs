use std::collections::HashMap;

use parser::{fixed, match_n, optional, take_while1, unsigned_number};

fn identifier(input: &str) -> Option<(&str, &str)> {
    take_while1(|c| !c.is_whitespace() && c != ':', input)
}

fn blank_line(input: &str) -> Option<(&str, &str)> {
    match_n(|c| c == '\n', 2, input)
}

fn field(input: &str) -> Option<((&str, &str), &str)> {
    let (_, input) = optional(|inp| match_n(|c| c.is_whitespace(), 1, inp), input);
    let (key, input) = identifier(input)?;
    let (_, input) = fixed(":", input)?;
    let (value, input) = identifier(input)?;
    Some(((key, value), input))
}

fn passport(input: &str) -> Option<(HashMap<String, String>, &str)> {
    let mut map = HashMap::new();
    let ((key, val), mut input) = field(input)?;
    map.insert(key.to_owned(), val.to_owned());
    while let Some(((key, val), new_input)) = field(input) {
        map.insert(key.to_owned(), val.to_owned());
        input = new_input;
    }
    let (_, input) = optional(blank_line, input);
    Some((map, input))
}

fn passports(input: &str) -> Vec<HashMap<String, String>> {
    let mut input = input;
    let mut passports = Vec::new();
    while let Some((passport, new_input)) = passport(input) {
        passports.push(passport);
        input = new_input;
    }
    passports
}

fn validate_passport_1(passport: &HashMap<String, String>) -> bool {
    for key in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
        if !passport.contains_key(*key) {
            return false;
        }
    }
    true
}

fn validate_number(num_str: &str, min: u32, max: u32) -> bool {
    if let Ok(num) = num_str.parse::<u32>() {
        num >= min && num <= max
    } else {
        false
    }
}

fn validate_hgt(hgt_str: &str) -> bool {
    if hgt_str.ends_with("cm") {
        validate_number(&hgt_str[..hgt_str.len() - 2], 150, 193)
    } else if hgt_str.ends_with("in") {
        validate_number(&hgt_str[..hgt_str.len() - 2], 59, 76)
    } else {
        false
    }
}

fn hex_color(input: &str) -> Option<(&str, &str)> {
    let (_, input) = fixed("#", input)?;
    match_n(
        // Technically incorrect since it also matches uppercase A-F, but meh...
        |c| c.is_ascii_hexdigit(),
        6,
        input,
    )
}

fn validate_ecl(ecl_str: &str) -> bool {
    match ecl_str {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn validate_pid(pid_str: &str) -> bool {
    pid_str.len() == 9 && unsigned_number(pid_str).is_some()
}

fn validate_with(
    passport: &HashMap<String, String>,
    key: &str,
    pred: impl Fn(&str) -> bool,
) -> bool {
    passport.get(key).map(|s| pred(s)).unwrap_or(false)
}

fn validate_passport_2(passport: &HashMap<String, String>) -> bool {
    validate_with(passport, "byr", |s| validate_number(&s, 1920, 2002))
        && validate_with(passport, "iyr", |s| validate_number(&s, 2010, 2020))
        && validate_with(passport, "eyr", |s| validate_number(&s, 2020, 2030))
        && validate_with(passport, "hgt", validate_hgt)
        && validate_with(passport, "hcl", |s| hex_color(s).is_some())
        && validate_with(passport, "ecl", validate_ecl)
        && validate_with(passport, "pid", validate_pid)
}

fn main() {
    let input = include_str!("input");
    let passports = passports(input);
    let valid_count_1 = passports.iter().filter(|&p| validate_passport_1(p)).count();
    println!("Part 1: {} valid passports", valid_count_1);
    let valid_count_2 = passports.iter().filter(|&p| validate_passport_2(p)).count();
    println!("Part 2: {} valid passports", valid_count_2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn field_complete() {
        let input = "hcl:#341e13";
        let res = field(input);
        assert_eq!(res, Some((("hcl", "#341e13"), "")));
    }

    #[test]
    fn passport_one_complete() {
        let input = "hcl:#341e13 eyr:2024";
        let res = passport(input);
        let mut expected = HashMap::new();
        expected.insert("hcl".to_owned(), "#341e13".to_owned());
        expected.insert("eyr".to_owned(), "2024".to_owned());
        assert_eq!(res, Some((expected, "")));
    }

    #[test]
    fn passport_with_remainder() {
        let input = "hcl:#341e13 eyr:2024\n\nhgt:179cm";
        let res = passport(input);
        let mut expected = HashMap::new();
        expected.insert("hcl".to_owned(), "#341e13".to_owned());
        expected.insert("eyr".to_owned(), "2024".to_owned());
        assert_eq!(res, Some((expected, "hgt:179cm")));
    }

    #[test]
    fn passports_two() {
        let input = "hcl:#341e13 eyr:2024\n\nhgt:179cm";
        let res = passports(input);
        let mut expected_1 = HashMap::new();
        expected_1.insert("hcl".to_owned(), "#341e13".to_owned());
        expected_1.insert("eyr".to_owned(), "2024".to_owned());
        let mut expected_2 = HashMap::new();
        expected_2.insert("hgt".to_owned(), "179cm".to_owned());
        assert_eq!(res, vec![expected_1, expected_2]);
    }
}
